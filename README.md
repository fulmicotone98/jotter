# Jotter

A minimal terminal text editor, written in Rust.

## Why

This is a learning project: a from-scratch recreation of [kilo](https://github.com/antirez/kilo),
antirez's ~1000-line terminal text editor, following along with:

- [Build Your Own Text Editor](https://viewsourcecode.org/snaptoken/kilo/index.html) (the original kilo walkthrough, in C)
- [hecto: Build Your Own Text Editor in Rust](https://philippflenker.com/hecto/) (a Rust-idiomatic take on the same tutorial)

The goal is to understand how terminal editors actually work, and to get more comfortable with Rust along the way.
It isn't meant to be a daily-driver editor (yet, at least).

## Status

Early stage. Currently implemented:

- Raw mode (entered on startup, restored on exit)
- Non-blocking keypress reading

Not yet implemented: screen rendering, cursor movement, opening/editing files,
search, syntax highlighting.

See [CHANGELOG.md](./CHANGELOG.md) for the detailed history and
[NOTES.md](./NOTES.md) for design decisions and the current roadmap.

## Building & running

Requires a recent Rust toolchain (edition 2024 features are used).

```sh
cargo build
cargo run
```

## License

No license chosen yet.
