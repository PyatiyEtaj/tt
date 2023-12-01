use ascii::args::ArgumentsError;
use crossterm::{
    cursor,
    style::{self, Color, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use image::io::Reader as ImageReader;
use std::io::{self, Write};

fn to_slice(c: &image::Rgb<u8>, out: &mut[u8;3]){
    let tmp = c.0;
    out[0] = tmp[0];
    out[1] = tmp[1];
    out[2] = tmp[2];
}

fn main() -> io::Result<()> {
    let strargs: Vec<String> = std::env::args().collect();
    
    let args = match ascii::args::Arguments::new(&strargs) {
        Ok(a) => a,
        Err(err) => {
            if let ArgumentsError::Err(msg) = err {
                println!("{}", msg);
            }

            return Ok(());
        }
    };

    let img = match ImageReader::open(args.path())?
        .with_guessed_format()?
        .decode()
    {
        Ok(img) => img,
        Err(e) => panic!("{e}"),
    };

    let termsize = match crossterm::terminal::size() {
        Ok(sz) => sz,
        _ => (60, 30),
    };

    let resized = img.resize(
        termsize.0 as u32,
        (termsize.1 * 2) as u32,
        image::imageops::FilterType::CatmullRom,
    );

    let buf = match resized.as_rgb8() {
        Some(buf) => buf,
        None => return Ok(()),
    };

    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let mut term_pos_y = 0;
    let mut c:[u8;3] = [0;3];
    let mut tmp:[u8;3] = [0;3];

    for y in (0..buf.height() - 1).step_by(2) {
        for x in 0..buf.width() - 1 {

            let mut tc = buf[(x, y)];
            to_slice(&tc, &mut c);
            let mut c: [u8; 3] = [tc[0], tc[1], tc[2]];
            if y != buf.height() - 1 {
                tc = buf[(x, y + 1)];
                to_slice(&tc, &mut tmp);
                c = ascii::draw::average(&c, &tmp);
            }
            
            let cmd = stdout.queue(cursor::MoveTo(x as u16, term_pos_y as u16))?;

            let color = Color::Rgb {
                r: c[0],
                g: c[1],
                b: c[2],
            };

            let style = if args.use_ascii() {
                let sym = if args.invert_ascii() {ascii::draw::map_invert(&c)} else {ascii::draw::map(&c)};
                if args.colored_ascii() {
                    sym.with(color)
                } else {
                    sym.white()
                }
            } else {
                ' '.on(color)
            };

            cmd.queue(style::PrintStyledContent(style))?;
        }
        term_pos_y += 1;
    }
    stdout.flush()?;

    Ok(())
}
