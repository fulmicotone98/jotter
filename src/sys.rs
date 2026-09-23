use std::io::{self};
use std::os::fd::AsRawFd;
use std::sync::OnceLock;
use termios::{
    BRKINT, CS8, ECHO, ICANON, ICRNL, IEXTEN, INPCK, ISIG, ISTRIP, IXON, OPOST, TCSAFLUSH, Termios,
    tcsetattr,
};

use crate::error::RawModeError;

static TERMINAL_MODE_PRE_RAW: OnceLock<Termios> = OnceLock::new();

/// .
///
/// # Panics
///
/// Panics if .
///
/// # Errors
///
/// This function will return an error if
pub fn enable_raw_mode() -> Result<(), RawModeError> {
    get_pre_raw_mode()?;

    let fd = io::stdin().as_raw_fd();
    let mut termios = Termios::from_fd(fd)?;

    termios.c_cflag |= CS8;
    termios.c_iflag &= !(IXON | ICRNL | BRKINT | INPCK | ISTRIP);
    termios.c_lflag &= !(ECHO | ICANON | ISIG | IEXTEN);
    termios.c_oflag &= !(OPOST);
    // termios.c_cc[VMIN] = 0;
    // termios.c_cc[VTIME] = 100;
    tcsetattr(fd, TCSAFLUSH, &termios)?;

    Ok(())
}

/// .
///
/// # Panics
///
/// Panics if .
pub fn disable_raw_mode() -> Result<(), RawModeError> {
    let original_mode = TERMINAL_MODE_PRE_RAW
        .get()
        .ok_or(RawModeError::NotEnabled)?;
    let fd = io::stdin().as_raw_fd();

    tcsetattr(fd, TCSAFLUSH, original_mode)?;

    Ok(())
}

fn get_pre_raw_mode() -> Result<(), RawModeError> {
    let fd = io::stdin().as_raw_fd();
    if TERMINAL_MODE_PRE_RAW.get().is_none() {
        let termios = Termios::from_fd(fd)?;
        if TERMINAL_MODE_PRE_RAW.set(termios).is_ok() {
            return Ok(());
        }
    }

    Err(RawModeError::AlreadyCaptured)
}
