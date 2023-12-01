pub mod args {
    pub struct Arguments {
        use_ascii: bool,
        invert_ascii: bool,
        colored_ascii: bool,
        path: String,
    }

    pub enum ArgumentsError {
        Ok,
        Err(String),
    }

    impl Arguments {
        pub fn new(args: &Vec<String>) -> Result<Arguments, ArgumentsError> {
            if args.len() < 2 {
                return Err(ArgumentsError::Err(String::from(
                    "Must provide min 1 arg - /path/to/image [--ascii]",
                )));
            }

            let mut a = Arguments {
                path: String::from(""),
                colored_ascii: false,
                use_ascii: false,
                invert_ascii: false,
            };

            for (_, item) in args.iter().enumerate() {
                if item == "--ascii" {
                    a.use_ascii = true;
                } else if item == "--colored" {
                    a.colored_ascii = true;
                } else if item == "--invert" {
                    a.invert_ascii = true;
                } else {
                    a.path = item.clone();
                }
            }

            Ok(a)
        }

        pub fn use_ascii(&self) -> bool {
            self.use_ascii
        }

        pub fn colored_ascii(&self) -> bool {
            self.use_ascii && self.colored_ascii
        }

        pub fn invert_ascii(&self) -> bool {
            self.invert_ascii
        }

        pub fn path(&self) -> &str {
            self.path.as_str()
        }
    }
}

pub mod draw {
    const ASCII: [char; 70] = [
        ' ', '.', '\'', '`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '>', '<', '~', '+', '_',
        '-', '?', ']', '[', '}', '{', '1', ')', '(', '|', '\\', '/', 't', 'f', 'j', 'r', 'x', 'n',
        'u', 'v', 'c', 'z', 'X', 'Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p',
        'd', 'b', 'k', 'h', 'a', 'o', '*', '#', 'M', 'W', '&', '8', '%', 'B', '@', '$',
    ];

    fn _map(color: &[u8; 3]) -> usize {
        let i = 0.299 * color[0] as f32 + 0.587 * color[1] as f32 + 0.114 * color[2] as f32;
        let pos = map_range(i as u32, 0, 255, 0, (ASCII.len() - 1) as u32) as usize;
        pos
    }

    pub fn map(color: &[u8; 3]) -> char {
        ASCII[_map(color)]
    }

    pub fn map_invert(color: &[u8; 3]) -> char {
        let pos = (ASCII.len() - 1) - _map(color);
        ASCII[pos]
    }

    fn map_range(this: u32, in_min: u32, in_max: u32, out_min: u32, out_max: u32) -> u32 {
        (this - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
    }

    pub fn average(f: &[u8; 3], s: &[u8; 3]) -> [u8; 3] {
        let r1 = f[0] as f32;
        let g1 = f[1] as f32;
        let b1 = f[2] as f32;

        let r2 = s[0] as f32;
        let g2 = s[1] as f32;
        let b2 = s[2] as f32;

        [
            ((r1 * r1 + r2 * r2) / 2.0).sqrt() as u8,
            ((g1 * g1 + g2 * g2) / 2.0).sqrt() as u8,
            ((b1 * b1 + b2 * b2) / 2.0).sqrt() as u8,
        ]
    }
}
