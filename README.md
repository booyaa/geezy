# geezy

A collection of Geodetical functions

# Usage

This crate is [on crates.io](https://crates.io/crates/geezy) and can be
used by adding `wordsworth` to the dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
geezy = "0.1.*"
```

and this to your crate root:

```rust
extern crate geezy;
```

# Example

```rust
use geezy;
assert_eq!(true, geezy::Coords::is_valid(-90_f64, -180_f64));
```
