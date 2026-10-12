# rustutils

A learning project implementing common Unix commands in Rust.

## Implemented Commands

`echo`, `pwd`, `cat`, `head`, `wc`, `ls`, `mkdir`, `tail`, `tee`, `cp`, `rm`, `sort`.

Basic functionality is implemented, with more options to be added over time.

## Usage

Requires a Rust toolchain that supports the 2024 edition.

```bash
# Print text
cargo run --bin 01_ruecho -- hello world

# Display file contents
cargo run --bin 03_rucat -- test.txt

# List the current directory
cargo run --bin 06_ruls
```

Command source files are in `src/bin/`. Use the filename without `.rs` as the `--bin` argument.
