use std::{
    env,
    io::{self, Write},
};

use crate::{
    JOTTER_VERSION,
    document::Document,
    error::JotterError,
    event::{Event, Key, read::next_event},
    sys::{self, WindowSize},
};

#[derive(Default, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    cursor_pos: Position,
    document: Document,
    should_quit: bool,
    offset: Position,
    window_size: WindowSize,
}

impl Editor {
    fn clear_current_line() -> Result<(), JotterError> {
        io::stdout().write_all(b"\x1b[2K")?;
        io::stdout().flush()?;

        Ok(())
    }

    fn clear_screen() -> Result<(), JotterError> {
        io::stdout().write_all(b"\x1b[2J")?;
        io::stdout().flush()?;

        Ok(())
    }

    fn cursor_goto(position: &Position) -> Result<(), JotterError> {
        let seq = format!("\x1b[{};{}H", position.y + 1, position.x + 1);
        io::stdout().write_all(seq.as_bytes())?;
        io::stdout().flush()?;

        Ok(())
    }

    fn cursor_hide() -> Result<(), JotterError> {
        io::stdout().write_all(b"\x1b[?25l")?;
        io::stdout().flush()?;

        Ok(())
    }

    fn cursor_show() -> Result<(), JotterError> {
        io::stdout().write_all(b"\x1b[?25h")?;
        io::stdout().flush()?;

        Ok(())
    }

    pub fn default() -> Result<Self, JotterError> {
        // Get terminal's window size.
        let ws = sys::window_size()?;

        // Enable RAW MODE
        sys::enable_raw_mode()?;

        // Get a file document, if passed.
        let args: Vec<String> = env::args().collect();
        let document = if let Some(file_name) = args.get(1) {
            Document::open(file_name)?
        } else {
            Document::default()
        };

        Ok(Self {
            cursor_pos: Position::default(),
            document,
            offset: Position::default(),
            window_size: ws,
            should_quit: false,
        })
    }

    fn draw_rows(&self) -> Result<(), JotterError> {
        let window_rows = self.window_size.y_rows;

        for terminal_row in 0..window_rows {
            Self::clear_current_line()?;
            let file_row = self.offset.y.saturating_add(terminal_row);
            if let Some(row) = self.document.row(file_row) {
                row.draw();
            } else if self.document.is_empty() && terminal_row == window_rows / 3 {
                self.welcome_page();
            } else {
                print!("~\r");
            }

            if terminal_row + 1 < window_rows {
                println!("\r");
            }
        }
        Ok(())
    }

    /// Changes only `cursor_pos`, which is a position in the document.
    /// Its checks compare against the document.
    /// Down stops when y reaches the number of rows, and Up stops at 0.
    #[allow(clippy::option_if_let_else)]
    fn move_cursor(&mut self, key: &Key) {
        // Take the current cursor position
        let Position { mut x, mut y } = self.cursor_pos;
        let doc_height = self.document.len();
        match key {
            Key::Up => {
                y = y.saturating_sub(1);
            }
            Key::Down if y < doc_height => {
                y += 1;
            }
            Key::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    if let Some(row) = self.document.row(y) {
                        x = row.length();
                    }
                }
            }
            Key::Right => {
                if let Some(row) = self.document.row(y) {
                    if x < row.length() {
                        x += 1;
                    } else if y < doc_height {
                        y += 1;
                        x = 0;
                    }
                }
            }
            _ => (),
        }

        // Fix cursor_pos.x if the current line has not enough chars respect to the previous one.
        if let Some(row) = self.document.row(y) {
            x = x.min(row.length());
        }

        self.cursor_pos = Position { x, y };
    }

    fn process_event(&mut self) -> Result<(), JotterError> {
        let event = next_event(|_| true)?;

        match event {
            // Backspace key
            Event::Key(Key::Backspace) => (),

            // Delete key
            Event::Key(Key::Delete) => if self.cursor_pos.x > 0 && self.cursor_pos.y > 0 {},

            // Return key
            Event::Key(Key::Return) => {
                self.document.insert(&self.cursor_pos, '\n');
                // The cursor should go at the beginning of the new splitted line.
                self.move_cursor(&Key::Right);
            }

            // Arrow key
            Event::Key(key @ (Key::Up | Key::Down | Key::Left | Key::Right)) => {
                self.move_cursor(&key);
            }

            // Ctrl-Q to quit the program
            Event::Key(Key::Ctrl('Q')) => {
                self.should_quit = true;
            }

            Event::Key(Key::Char(c)) => {
                self.document.insert(&self.cursor_pos, c);
                // Every char typed moves the cursor up on the X axis.
                self.move_cursor(&Key::Right);
            }

            // Escape key
            Event::Key(Key::Esc) => {
                io::stdout().write_all(b"Escape")?;
                io::stdout().flush()?;
            }

            Event::Key(_) => {
                // TODO: Message inside a future status bar...
                io::stdout().write_all(b"Unkown event...")?;
                io::stdout().flush()?;
            }
        }
        self.scroll();

        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), JotterError> {
        Self::cursor_hide()?;
        Self::cursor_goto(&Position::default())?;

        if self.should_quit {
            Self::screen_quit()?;
            println!("Bye bye!\r");
        } else {
            self.draw_rows()?;
            Self::cursor_goto(&Position {
                x: self.cursor_pos.x.saturating_sub(self.offset.x),
                y: self.cursor_pos.y.saturating_sub(self.offset.y),
            })?;
        }

        Self::cursor_show()?;
        io::stdout().flush()?;
        Ok(())
    }

    fn reposition_cursor() -> Result<(), JotterError> {
        io::stdout().write_all(b"\x1b[H")?;
        io::stdout().flush()?;

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), JotterError> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            self.process_event()?;
        }
        Ok(())
    }

    fn screen_quit() -> Result<(), JotterError> {
        Self::clear_screen()?;
        Self::reposition_cursor()?;

        Ok(())
    }

    /// Runs right after each move and decides whether the window has to follow the cursor.
    /// It adjusts offset.y in two cases:
    /// - If the cursor is above the first visible row.
    /// - If the cursor is below the last visible row.
    const fn scroll(&mut self) {
        let Position { x, y } = self.cursor_pos;
        let window_rows = self.window_size.y_rows;
        let window_cols = self.window_size.x_columns;

        // Y axis (vertical)
        if y < self.offset.y {
            self.offset.y = y;
        } else if y >= self.offset.y.saturating_add(window_rows) {
            self.offset.y = y.saturating_sub(window_rows).saturating_add(1);
        }

        // X axis (horizontal)
        if x < self.offset.x {
            self.offset.x = x;
        } else if x >= self.offset.x.saturating_add(window_cols) {
            self.offset.x = x.saturating_sub(window_cols).saturating_add(1);
        }
    }

    fn welcome_page(&self) {
        let mut welcome_msg = format!("Welcome to Jotter! -- {JOTTER_VERSION}");

        #[allow(clippy::cast_possible_truncation)]
        let padding = (self
            .window_size
            .x_columns
            .saturating_sub(welcome_msg.chars().count()))
        .div_ceil(2);

        if padding > 0 {
            // Need to center the welcome message to the window's columns size
            for i in 0..padding {
                if i == 0 {
                    welcome_msg.insert(0, '~');
                }
                welcome_msg.insert(1, ' ');
            }

            print!("{welcome_msg}\r");
        } else {
            // Welcome message not needed, since the padding underflows because the terminal
            // size is too small.
            print!("~\r");
        }
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        // println!("INFO: Disabling raw mode...\r");
        if let Err(err_disable) = sys::disable_raw_mode() {
            eprintln!("ERROR: {err_disable}");
        }
        // println!("INFO: Raw mode disabled.\r");
    }
}
