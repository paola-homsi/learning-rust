# learning-rust

[![CI](https://github.com/pawla-homsi/learning-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/pawla-homsi/learning-rust/actions/workflows/ci.yml)

Small Rust projects I build to learn the language.

## Projects

### `cmdTool`: file statistics CLI

Prints the size and line count of a text file.

```bash
cd cmdTool
cargo run -- -n path/to/file.txt --size --lines
```

```
size: 4 bytes
lines: 2
```

| Flag | Effect |
|---|---|
| `-n`, `--name <file>` | File to inspect |
| `-s`, `--size` | Size in bytes |
| `-l`, `--lines` | Number of lines |
| `-h`, `--help` | Usage |

Errors (missing file name, unreadable file, unknown flag) print a message and exit with code 2 instead of panicking.

What I practised: enums with exhaustive `match`, `Result`-based error handling with `?`, iterator-based argument parsing, and unit tests with `cargo test`.

## Development

```bash
cd cmdTool
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

## Roadmap

- [ ] Switch argument parsing to `clap` and compare with the hand-written parser
- [ ] Word count and most frequent words
- [ ] Stream large files instead of reading them into memory

## License

MIT
