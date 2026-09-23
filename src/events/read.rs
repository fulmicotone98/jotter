/*
 * The following module represent the mapping point between the byte received on
 * stdin, and the events represented as enum in crate::event::Events.
 */
use std::io::{self, Read};
use std::os::fd::AsRawFd;

use super::KeyEvent;
use crate::events::Event;

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
        -1 => Err(io::Error::last_os_error()),

        // Data is ready in stdin
        _ => {
            let mut buffer = [0u8; 1]; // buffer of 1 byte = 1 char
            io::stdin().read_exact(&mut buffer)?;
            let raw_byte = buffer[0];

            let event = Event::Key(KeyEvent::from_bytes(raw_byte));

            Ok(Some(event))
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
