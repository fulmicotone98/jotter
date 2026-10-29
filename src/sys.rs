use libc::{STDOUT_FILENO, TIOCGWINSZ, ioctl, winsize};
use std::fs::File;
use std::io::{self};
use std::os::fd::AsRawFd;
use std::sync::OnceLock;
use termios::{
    BRKINT, CS8, ECHO, ICANON, ICRNL, IEXTEN, INPCK, ISIG, ISTRIP, IXON, OPOST, TCSAFLUSH, Termios,
    tcsetattr,
};

use crate::error::RawModeError;

static TERMINAL_MODE_PRE_RAW: OnceLock<Termios> = OnceLock::new();

pub struct WindowSize {
    pub x_columns: usize,
    pub y_rows: usize,
    pub h_pixels: usize,
    pub w_pixels: usize,
}

impl From<winsize> for WindowSize {
    fn from(value: winsize) -> Self {
        Self {
            y_rows: value.ws_row as usize,
            x_columns: value.ws_col as usize,
            w_pixels: value.ws_xpixel as usize,
            h_pixels: value.ws_ypixel as usize,
        }
    }
}

pub fn window_size() -> io::Result<WindowSize> {
    let mut w_size = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    let file = File::open("/dev/tty");
    let fd = file
        .as_ref()
        .map_or(STDOUT_FILENO, std::os::fd::AsRawFd::as_raw_fd);

    if wrap_with_result(unsafe { ioctl(fd, TIOCGWINSZ, &mut w_size) }).is_ok()
        && w_size.ws_row != 0
        && w_size.ws_col != 0
    {
        return Ok(w_size.into());
    }

    Err(io::Error::last_os_error())
}

pub fn enable_raw_mode() -> Result<(), RawModeError> {
    get_pre_raw_mode()?;

    let fd = io::stdin().as_raw_fd();
    let mut termios = Termios::from_fd(fd)?;

    termios.c_cflag |= CS8;
    termios.c_iflag &= !(IXON | ICRNL | BRKINT | INPCK | ISTRIP);
    termios.c_lflag &= !(ECHO | ICANON | ISIG | IEXTEN);
    termios.c_oflag &= !(OPOST);
    tcsetattr(fd, TCSAFLUSH, &termios)?;

    Ok(())
}

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

fn wrap_with_result(result: i32) -> io::Result<()> {
    if result == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}
