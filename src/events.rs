pub mod read;

#[derive(Debug)]
pub enum Event {
    Key(KeyEvent),
    // Resize(u16, u16),
}

#[derive(Debug, PartialEq, Eq, Default)]
pub enum KeyEvent {
    Char(char),
    Ctrl(char),
    Esc,
    #[default]
    Unknown,
}

impl KeyEvent {
    pub const fn from_bytes(b: u8) -> Self {
        // println!("{b:b}");
        match b {
            27 => Self::Esc,

            // Ctrl-q: to quit the program.
            0..=26 | 28..=31 => Self::Ctrl((b | 0x40) as char),

            32..=127 => Self::Char(b as char),

            _ => Self::Unknown,
        }
    }
}
