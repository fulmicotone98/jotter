pub mod read;

#[derive(Debug)]
pub enum Event {
    Key(Key),
    // Resize(u16, u16),
}

#[derive(Debug, PartialEq, Eq, Default)]
pub enum Key {
    /// Backspace.
    Backspace,
    /// Ctrl modified character.
    Ctrl(char),
    /// Normal character.
    Char(char),
    /// Delete.
    Delete,
    /// Down arrow.
    Down,
    /// Left arrow.
    Left,
    /// Return.
    Return,
    /// Right arrow.
    Right,
    /// Up arrow.
    Up,
    /// Escape.
    Esc,
    #[default]
    Unknown,
}

impl Event {
    pub fn parse_event(bytes: &[u8]) -> Self {
        Self::Key(Key::from_bytes(bytes))
    }
}

impl Key {
    fn from_bytes(bytes: &[u8]) -> Self {
        // eprintln!("{bytes:?}");
        match bytes {
            [27, b'[', b'A'] => Self::Up,           // Arrow Up
            [27, b'[', b'B'] => Self::Down,         // Arrow Down
            [27, b'[', b'C'] => Self::Right,        // Arrow Right
            [27, b'[', b'D'] => Self::Left,         // Arrow Left
            [27, b'[', b'3', b'~'] => Self::Delete, // Delete
            [27] => Self::Esc,                      // bare Esc
            // one ordinary byte
            [b] => match b {
                0..=12 | 14..=26 | 28..=31 => Self::Ctrl((b | 0x40) as char),
                13 => Self::Return,
                32..=126 => Self::Char(*b as char),
                127 => Self::Backspace,
                _ => Self::Unknown,
            },
            _ => Self::Unknown,
        }
    }
}
