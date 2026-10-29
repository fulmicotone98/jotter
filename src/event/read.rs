/*
 * The following module represent the mapping point between the bytes received on stdin, and the
 * events represented as enum in crate::event::Events.
 */
use std::io::{self, Error, Read};
use std::os::fd::AsRawFd;

use crate::event::Event;

const POLLIN: i16 = 0x0001;

#[repr(C)]
pub struct PollFd {
    fd: i32,
    events: i16,
    revents: i16,
}

unsafe extern "C" {
    fn poll(fds: *mut PollFd, nfds: usize, timeout: i32) -> i32;
}

fn poll_event(timeout_ms: i32) -> io::Result<Option<Event>> {
    let mut fds = PollFd {
        fd: io::stdin().as_raw_fd(),
        events: POLLIN,
        revents: 0,
    };

    let ready = unsafe { poll(&raw mut fds, 1, timeout_ms) };

    match ready {
        // System call timed out
        0 => Ok(None),

        // OS error occured
        -1 => Err(Error::last_os_error()),

        // Data is ready in stdin
        _ => {
            // 1 byte = 1 char
            let mut buffer = [0u8; 8];
            let n_bytes = io::stdin().read(&mut buffer)?;
            if n_bytes > 0 {
                // Passing `&buffer[..n_bytes]` instead of just `&buffer` allows to receive a buffer
                // with a size equal to n_bytes.
                // Ex:
                // - 1 byte: 'a'
                //   &buffer = [97, 0, 0]
                //   &buffer[..n_bytes] = [97]
                Ok(Some(Event::parse_event(&buffer[..n_bytes])))
            } else {
                Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "No bytes in stdin.",
                ))
            }
        }
    }
}

pub fn next_event<F>(filter: F) -> io::Result<Event>
where
    F: Fn(&Event) -> bool,
{
    loop {
        if let Some(event) = poll_event(10)?
            && filter(&event)
        {
            return Ok(event);
        }
    }
}
