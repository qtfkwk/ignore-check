# About

Check if a path is ignored

The [`ignore`] crate's primary use case is walking a directory.
It provides `Gitignore` and `GitignoreBuilder` to load `.gitignore` files but the usage is
difficult.

This crate uses [`ignore`] but provides a much easier interface.

[`ignore`]: https://crates.io/crates/ignore

# Usage

```rust
use ignore_check::ignored;

assert!(ignored("target").unwrap());
assert!(!ignored("src/lib.rs").unwrap());
```

