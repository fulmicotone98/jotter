#[derive(Default)]
pub struct FileType {
    name: String,
}

impl FileType {
    pub const fn default() -> Self {
        Self {
            name: String::new(),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl From<&str> for FileType {
    fn from(slice: &str) -> Self {
        Self {
            name: String::from(slice),
        }
    }
}
