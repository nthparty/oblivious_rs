# oblivious_rs

Rust implementation of the python [oblivious](https://github.com/nthparty/oblivious) library. 

## Purpose

This library is a thin wrapper over the [curve25519-dalek](https://github.com/dalek-cryptography/curve25519-dalek) library, and is designed to streamline \
NthParty's OPRF and OT protocols across implementations in different languages.

## Package Installation and Usage

Currently, the package must be cloned locally and added to your project's `Cargo.toml` as follows:

```toml
[dependencies]
oblivious_rs = { path = "<path_to_oblivious_rs>" }
```

The library can then be imported as normal:

```rust
extern crate oblivious_rs;
use oblivious_rs::point::Point;
use oblivious_rs::scalar::Scalar;
```
