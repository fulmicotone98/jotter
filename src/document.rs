use std::fs::{self};

use crate::{editor::Position, error::JotterError, filetype::FileType, row::Row};

#[derive(Default)]
pub struct Document {
    pub file_name: Option<String>,
    file_type: FileType,
    rows: Vec<Row>,
}

impl Document {
    pub fn file_type(&self) -> String {
        self.file_type.name()
    }

    pub fn insert(&mut self, at: &Position, ch: char) {
        if at.y > self.len() {
            return;
        }

        if ch == '\n' {
            self.insert_newline(at);
        } else if at.y == self.len() {
            let mut row = Row::default();
            row.insert(0, ch);
            self.rows.push(row);
        } else if let Some(row) = self.row_mut(at.y) {
            row.insert(at.x, ch);
        }
    }

    fn insert_newline(&mut self, at: &Position) {
        // Insert a new line is complex, we need to care of the position of the cursor.
        if let Some(current_row) = self.rows.get_mut(at.y) {
            if at.x == current_row.length() {
                // Here we have the cursor at the end of the row and we want a new line below.
                let new_empty_row = Row::default();
                self.rows.insert(at.y + 1, new_empty_row);
            } else if at.x == 0 {
                // Here we have the cursor at the beginning of the row and we want a new line above.
                let new_empty_row = Row::default();
                self.rows.insert(at.y, new_empty_row);
            } else {
                // Here we have the cursor in the middle of a row of chars, we want to split the
                // row, and move all the chars after the cursor on a new line below.
                let new_row = current_row.split(at.x);
                self.rows.insert(at.y + 1, new_row);
            }
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub const fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn open(file_name: &str) -> Result<Self, JotterError> {
        let mut rows: Vec<Row> = Vec::new();
        for line in fs::read_to_string(file_name)?.lines() {
            rows.push(Row::from(line));
        }

        Ok(Self {
            file_name: Some(file_name.to_string()),
            rows,
            file_type: FileType::from(file_name),
        })
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn row_mut(&mut self, index: usize) -> Option<&mut Row> {
        self.rows.get_mut(index)
    }
}
