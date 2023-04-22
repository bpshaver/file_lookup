# `file_lookup`

For when you want to look up from the current working directory for a file by name.
## Example Usage

Find a file within your home directory, looking up from your current working directory:

```rust
use file_lookup::home_find_file;

let path = home_find_file("some_file.json").unwrap();
```

Find a file within your root directory (or anyhwere else), looking up from your current working directory:

```rust
use std::path::PathBuf;
use file_lookup::find_file;

let path = find_file("some_file.json", &PathBuf::from("/")).unwrap();
```

## Errors

These functions fail if there is a failure to ascertain your current working directory or your home directory:

```rust
pub enum FileLookupError {
    CwdNotFound,
    HomeDirNotFound,
};
```

## Installation

This crate is not on [crates.io](https://crates.io/), but you can install it with `cargo`:

```bash
$ cargo add --git
https://github.com/bpshaver/file_lookup
```
