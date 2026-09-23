# Changelog

All notable changes to Jotter will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project intends to adhere to [Semantic Versioning](https://semver.org/)
once it has its first tagged release. For the reasoning behind changes and
what's coming next, see [NOTES.md](./NOTES.md).

## [Unreleased]

### Added
- Raw mode: terminal switched into raw mode on startup, restored on exit.
- Ctrl key signals (Ctrl-C, Ctrl-Z, Ctrl-S/Ctrl-Q, Ctrl-V) disabled while
  in raw mode.
- Non-blocking keypress reading via `poll()`, decoupled into its own
  `events`/`events::read` module.
- `Event`/`KeyEvent` abstraction for representing input.
- `error.rs`: crate-wide `JotterError` type (`Io`, `RawModeNotEnabled`,
  `RawModeAlreadyCaptured`), with `Display` and `From<io::Error>` impls.
  `sys.rs` now returns `Result<(), JotterError>` throughout instead of
  `io::Result`.
- Ctrl-Q now quits the editor, via a general Ctrl-byte-to-letter mapping
  (`1..=26 => Ctrl((b | 0x40) as char)`) rather than a one-off case — any
  Ctrl-A through Ctrl-Z combination is now recognized.
- `Editor::new()`: fallible constructor that enables raw mode and returns
  an `Editor` value only on success, tying raw mode's lifetime to the
  value's lifetime.
- `Editor` now implements `Drop`, restoring the terminal automatically when
  the value goes out of scope — including during a panic unwind, not just


### Fixed
- Terminal no longer stays in raw mode after the program exits (original
  termios settings are now actually captured before mutation, via `OnceLock`).
- `CS8` was being OR'd with its own bitwise NOT, effectively corrupting most
  of `c_cflag` instead of setting 8-bit character size. Now `|= CS8`.
- `OPOST` (output post-processing) is now correctly disabled on `c_oflag`;
  it had been dropped during an earlier refactor.
- `main` now exits with a nonzero status if `Editor::run()` fails, even when
  terminal cleanup afterward succeeds.
- Ctrl-Q was never recognized: the byte sent for Ctrl-Q (17) was stored as
  its raw control code and compared against the letter `'Q'`/`'q'`, which
  never match. Now mapped explicitly.
- Typing required an extra keypress before the previous one showed up: the
  read buffer was declared as `[0u8, 1]` (a 2-element array) instead of
  `[0u8; 1]` (a 1-byte buffer), so `read_exact` was blocking for a second
  byte before returning the first.
- Output appeared to start above the cursor's actual position: with `OPOST`
  now disabled, `\n` is no longer auto-translated to `\r\n`. `\r` is now
  written explicitly wherever the editor prints a newline.
- Terminal could be left raw if `Editor::run()` panicked (rather than
  returning `Err`), since nothing would run `disable_raw_mode()` on that
  path. Fixed by `Editor`'s `Drop` impl.



### Changed
- `sys::enable_raw_mode`/`disable_raw_mode` now return `io::Result<()>`
  instead of panicking internally; errors are surfaced and printed in `main`.
- Removed unused `bitflags`/`parking_lot` dependencies from `Cargo.toml`.
- `main()` now returns `std::process::ExitCode` instead of calling
  `std::process::exit()` directly, since `exit()` skips destructors and would
  have bypassed `Editor`'s new `Drop`-based cleanup on failure paths.


