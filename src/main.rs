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

    /* Flipping bits to 0:
     * - ECHO: turns off echo mode
     * - ICANON: turns off canonical mode (turning it off allows to read byte by
     *   byte the input intead of line by line)
     * - ISIG: turns off Ctrl-C and Ctrl-Z signals
     * - IXON: turns off Ctrl-S and Ctrl-Q signals of 'software control flow'
     * - IEXTEN: turns off Ctrl-V on some systems; the terminal waits for you
     *   type another character and then sends it literally
     * - ICRNL: (CR: Carriage Return, NL: New Line) turns off Ctrl-M; it will be
     *   read now as carriage return (13) like ENTER key
     * - OPOST: turns off the automatic translation of each '\n' into '\r\n'
     *   (carriage return + new line)
     * The following won´t have observable effect (or they don´t apply to modern
     * terminals), but are usually turned of by when switching to RAW_MODE
     * - BRKINT:
     * - INPCK: turns off parity checking (for old terminals)
     * - ISTRIP: turns off the stripping of each 8th bit of the input byte
     * - CS8: it is a bit mask and it sets the Character Size (CS) to 8 bits
     */

    termios.c_cflag |= !(CS8);
    termios.c_oflag &= !(OPOST);
    termios.c_iflag &= !(IXON | ICRNL | BRKINT | INPCK | ISTRIP);
    termios.c_lflag &= !(ECHO | ICANON | ISIG | IEXTEN);

    /* Then flush the new termios struct. */
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
        let b = b.unwrap();
        let c = b as char;

        /* Check if c is a control character */
        if c.is_control() {
            println!("Binary: {0:08b} ASCII: {0:#03}\r\n", b);
        } else {
            println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r\n", b, c);
        }
        if c == 'q' {
            disable_raw_mode().unwrap();
            break;
        }
    }
}
