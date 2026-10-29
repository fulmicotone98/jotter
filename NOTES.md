# Notes

Decisions, open questions, known issues, and roadmap.
See [README.md](./README.md) for what the project is, [CHANGELOG.md](./CHANGELOG.md) for shipped history.


## Decisions

- Commits: adopting [Conventional Commits](https://www.conventionalcommits.org/) going forward.
- Raw-mode restore: `TERMINAL_MODE_PRE_RAW` is a `OnceLock<Termios>`, captured once by
  `get_pre_raw_mode()` as the first line of `enable_raw_mode()`, before any flags mutate.
  Kept as global state in `sys.rs` (not threaded through `main.rs`) so it can eventually be
  restored from other call sites (panic hook, signal handler).
- `enable_raw_mode()` is designed to be called exactly once per program run. A second call fails
  (`ErrorKind::AlreadyExists`) — this is intentional, not a bug: raw mode is enabled once at startup
  and disabled once at shutdown.
- `enable_raw_mode()` is not retryable after a failure: if it fails (e.g. tcsetattr errors),
  the program exits immediately rather than trying again.
- Errors propagate with `?`/`Result` instead of `.unwrap()`/`.expect()`.
- Coordinates: `cursor_pos` is a position in the document. `offset` is the first visible row and
  column, and only `scroll()` changes it (called once at the end of `process_event`). The screen
  position.



## Known issues

- [x] `c_cflag |= !(CS8)` — fixed, now `|= CS8`.
- [x] `OPOST` — fixed, now cleared on `c_oflag` (was missing, then briefly on the wrong field).
- [x] Terminal not restoring on exit — fixed via `OnceLock` capture-before-mutate ordering.
- [x] Nonzero exit code on `Editor::run()` failure — fixed in `main()`.
- [x] `VMIN`/`VTIME` still set (`c_cc[VMIN] = 0; c_cc[VTIME] = 100;`) but vestigial now that
      `events/read.rs` uses `poll()` with its own timeout — worth revisiting.
- [x] `enable_raw_mode`/`disable_raw_mode` still manually `match tcsetattr(...) { ... }` to wrap
      the `io::Error`, even though `tcsetattr(...)?` would now work directly via `JotterError`'s
      `From<io::Error>` impl (same as `Termios::from_fd(fd)?` already does in the same functions).
- [x] Input chunks: only the first event of a chunk is parsed, and bytes beyond the 3 read stay in
      Rust's `stdin` buffer, where `poll()` can't see them. 
- [x] `parse_event` ignores `n_bytes` and relies on zero padding; `from_bytes` indexes `bytes[2]`,
      so passing `&buffer[..n_bytes]` would panic on `ESC [`. Use slice patterns.
- [x] Typing a char does `x += 1` unchecked and the Right guard is `x != columns - 1`: after a full
      row, Right never stops. Handle with automatic new line; prefer `<` over `!=`.
- [x] `rows - 1` / `columns - 1` underflow if the terminal reports 0. Treat 0 as an error in
      `window_size()`.
- [x] `Editor::new()` returns `RawModeError`, so a failed `window_size()` is reported as
      `RawModeError::EnableDisableFailed`. Fix: return `JotterError` (add `RM(RawModeError)` +
      `From<RawModeError>`, and a `WindowSize(io::Error)` variant built with `map_err`).
- [x] Welcome message: `columns - len` underflows on a terminal narrower than the message; the
      padding also counts the trailing `\r\n`.
- [x] Smaller: `read() == 0` (stdin closed) is reported as `InvalidInput` with an empty message
      (`UnexpectedEof` fits better).
- [x] Left at the first column of the first row jumped to the end of that row — fixed.
- [ ] A missing file leaves the terminal raw: `Document::open` runs after `enable_raw_mode()`.
      The message also says "I/O error", because `Document::open` uses a bare `?` and `FileError`
      is never produced.
- [ ] **Rows wider than the window break the rendering.** Seen with a file of many lines longer
      than the terminal width (reproduced with 130-char lines at 80 columns): the cursor does not
      start on the first line, the rendering looks irregular (rows show up as continuation
      pieces), the top of the file cannot be reached with Up, and the terminal's scrollback grows by
      about 24 lines on every key press (1464 lines after 60 presses), even at the bottom of the
      file. 1000 short lines are fine. Cause: `Row::draw` prints the whole row, so one row takes
      several screen lines. `draw_rows` then writes more than `y_rows` lines and the terminal
      scrolls on every redraw. Fix (tested in a scratch copy): draw only the characters
      `offset.x..offset.x + x_columns` of each row, so a row is always one screen line. This also
      makes `offset.x`, which `scroll()` already computes, take effect (horizontal scrolling).
- [ ] `window_size()` returns `io::Error::last_os_error()` when the ioctl succeeds with a 0 size,
      so the errno can be stale.


## Planned

- Complete cursor movements with Backspace, ~Return~, and Delete.
- ~~Automatic new line (and with it the cursor bounds while typing).~~

## Open question

- ~~What if Jotter is not able to get the `WindowSize` form the terminal? -> Implement a solution.~~
                                                                    
## Roadmap

1. ~~Raw mode~~
2. ~~Raw input/output (in progress — `CS8`/`OPOST` above)~~
3. ~~Crate-wide `JotterError` enum (`Io(std::io::Error)` + more variants as needed)~~
4. ~~Screen rendering: clear screen, terminal size, `~` rows, welcome message~~
5. ~~Cursor movement: parse escape sequences for arrow keys (only bare `Esc` handled so far)~~
6. ~~File viewer: open file from argv, render lines, scrolling~~
7. Editing: insert/delete characters, dirty flag, save
8. Status bar: file name, messages from the Jotter, file type, COL:ROW, time...
9. Search
10. Syntax highlighting (stretch)
