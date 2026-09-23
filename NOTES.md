# Notes

Decisions, open questions, known issues, and roadmap.
See [README.md](./README.md) for what the project is, [CHANGELOG.md](./CHANGELOG.md) for shipped history.


## Decisions

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
- Commits: adopting [Conventional Commits](https://www.conventionalcommits.org/) going forward.


## Known issues in `sys.rs`

- [x] `c_cflag |= !(CS8)` — fixed, now `|= CS8`.
- [x] `OPOST` — fixed, now cleared on `c_oflag` (was missing, then briefly on the wrong field).
- [x] Terminal not restoring on exit — fixed via `OnceLock` capture-before-mutate ordering.
- [x] Nonzero exit code on `Editor::run()` failure — fixed in `main()`.
- [x] `VMIN`/`VTIME` still set (`c_cc[VMIN] = 0; c_cc[VTIME] = 100;`) but vestigial now that
      `events/read.rs` uses `poll()` with its own timeout — worth revisiting.
- [x] `enable_raw_mode`/`disable_raw_mode` still manually `match tcsetattr(...) { ... }` to wrap
      the `io::Error`, even though `tcsetattr(...)?` would now work directly via `JotterError`'s
      `From<io::Error>` impl (same as `Termios::from_fd(fd)?` already does in the same functions).


## Planned

## Open question
                                                                    
## Roadmap

1. ~~Raw mode~~
2. ~~Raw input/output (in progress — `CS8`/`OPOST` above)~~
3. ~~Crate-wide `JotterError` enum (`Io(std::io::Error)` + more variants as needed)~~
4. Screen rendering: clear screen, terminal size, `~` rows, welcome message
5. Cursor movement: parse escape sequences for arrow keys (only bare `Esc` handled so far)
6. File viewer: open file from argv, render lines, scrolling
7. Editing: insert/delete characters, dirty flag, save
8. Search
9. Syntax highlighting (stretch)
