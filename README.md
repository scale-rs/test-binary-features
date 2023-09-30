# Automate testing your crate features

This crate helps you with testing any combinations of your crate's features programmatically. All
with one `cargo test`.

You'll need to set up auxiliary crates. Also, this can't run unit tests (`#[test]`) as-is - you'll
need to refactor those functions and have them called from binary crate(s) within the same project.

[![Documentation](https://img.shields.io/badge/docs-master-blue.svg)][Documentation]
![License](https://img.shields.io/crates/l/test-binary-features.svg) [![Crates
Status](https://img.shields.io/crates/v/test-binary-features.svg)][Crates.io]

Based on [crates.io/crates/test-binary](https://crates.io/crates/test-binary)
([gitlab.com/rust-test-binary/test-binary](https://gitlab.com/rust-test-binary/test-binary)).

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   <http://www.apache.org/licenses/LICENSE-2.0)>
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

[Crates.io]: https://crates.io/crates/test-binary-features
[Documentation]: https://docs.rs/test-binary-features/latest/test_binary_features
