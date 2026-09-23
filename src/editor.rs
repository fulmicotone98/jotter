use std::io::{self, Write};

use crate::{
    error::RawModeError,
    events::{Event, KeyEvent, read::next_event},
    sys,
};

#[derive(Default)]
pub struct Editor;

impl Editor {
    pub fn new() -> Result<Self, RawModeError> {
        println!("INFO: Enabling raw mode...");
        sys::enable_raw_mode()?;
        println!("INFO: Raw mode enabled.\r");
        Ok(Self)
        // if let Err(err_enable) = sys::enable_raw_mode() {
        //     Err(JotterError::RawModeEnablingFailed)
        // } else {
        //     Ok(Self)
        // }

        // println!("INFO: Enabling raw mode...");
        // if let Err(err_enable) = sys::enable_raw_mode() {
        //     Err(JotterError::RawModeEnablingFailed)
        // } else {
        //     println!("INFO: Raw mode enabled.\r");
        //     Ok(Self)
        // }
    }

    pub fn run(&self) -> io::Result<()> {
        io::stdout().write_all(b"\nWelcome to Jotter!\n\r")?;
        io::stdout().flush()?;

        loop {
            let event = next_event(|_| true)?;

            match event {
                Event::Key(KeyEvent::Ctrl('Q')) => {
                    io::stdout().write_all(b"Quitting...bye bye!\n\r")?;
                    io::stdout().flush()?;
                    break;
                }

                Event::Key(KeyEvent::Char(c)) => {
                    io::stdout().write_all(b"You pressed: ")?;
                    io::stdout().flush()?;
                    println!("{c}\r");
                }

                Event::Key(KeyEvent::Esc) => {
                    io::stdout().write_all(b"Escape")?;
                    io::stdout().flush()?;
                    println!("\r");
                }

                // Event::Resize(w, h) => {
                //     println!("Resizing to: {w}x{h}");
                // }
                Event::Key(_) => {
                    io::stdout().write_all(b"Unkown event...")?;
                    io::stdout().flush()?;
                    println!("\r");
                }
            }
        }

        Ok(())
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        println!("INFO: Disabling raw mode...\r");
        if let Err(err_disable) = sys::disable_raw_mode() {
            eprintln!("ERROR: {err_disable}");
            // std::process::exit(1)
        }
        println!("INFO: Raw mode disabled.\r");
    }
}
