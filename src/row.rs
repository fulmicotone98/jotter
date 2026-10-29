#[derive(Default)]
pub struct Row {
    string: String,
    length: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
            length: slice.chars().count(),
        }
    }
}

impl Row {
    pub fn draw(&self) {
        let mut res = String::new();
        for c in self.string.chars() {
            if c == '\t' {
                res.push(' ');
            } else {
                res.push(c);
            }
        }

        print!("{res}\r");
    }

    pub fn insert(&mut self, at: usize, ch: char) {
        if at >= self.length() {
            self.string.push(ch);
        } else {
            self.string.insert(at, ch);
        }
        self.length += 1;
    }

    pub const fn length(&self) -> usize {
        self.length
    }

    pub fn split(&mut self, at: usize) -> Self {
        // Take the right half of the row as a new row to move below.
        let right_half = Self::from(self.string.split_off(at).as_str());

        // Need to update the length of the left_half of the splitted row.
        self.length = self.string.chars().count();
        right_half
    }
}
