use std::io::{self, Read};
use std::os::fd::AsRawFd;
use std::sync::Mutex;
use termios::{tcsetattr, *};

static TERMINAL_MODE_PRE_RAW: Mutex<Option<Termios>> = Mutex::new(None);

fn enable_raw_mode() -> io::Result<()> {
    /* Get a terminal file descriptor */
    let fd = io::stdin().as_raw_fd();

    /* Get a termios struc with its parameters from the fd */
    let mut termios = Termios::from_fd(fd).unwrap();

    let mut original_mode = match TERMINAL_MODE_PRE_RAW.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            eprintln!("Mutax was POISONED!");
            poisoned.into_inner()
        }
    };

    if let Some(mut original_mode_ios) = original_mode.as_ref() {
        original_mode_ios = &termios;
        *original_mode = Some(original_mode_ios.to_owned());
    }

    /* Flipping the ECHO bit to 0 to disable echo inside the terminal.
     * Then flush the new termios struct. */
    termios.c_lflag &= !(ECHO);
    tcsetattr(fd, TCSAFLUSH, &termios).unwrap();

    Ok(())
}

fn disable_raw_mode() -> io::Result<()> {
    let mut original_mode = match TERMINAL_MODE_PRE_RAW.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            eprintln!("Mutex was POISONED!");
            poisoned.into_inner()
        }
    };
    match original_mode.as_ref() {
        Some(original_mode_ios) => {
            let fd = io::stdin().as_raw_fd();
            tcsetattr(fd, TCSAFLUSH, original_mode_ios).unwrap();
            *original_mode = None;
        }
        _ => (),
    }
    Ok(())
}

fn main() {
    enable_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        println!("{}", c);
        if c == 'q' {
            break;
        }
    }
    disable_raw_mode().unwrap();
}
