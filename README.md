# Ara Source

[![Actions Status](https://github.com/ara-lang/source/workflows/ci/badge.svg)](https://github.com/ara-lang/source/actions)
[![Crates.io](https://img.shields.io/crates/v/ara_source.svg)](https://crates.io/crates/ara_source)
[![Docs](https://docs.rs/ara_source/badge.svg)](https://docs.rs/ara_source/latest/ara_source/)

A Source library for Ara Programming Language 🗃

## Usage

Add `ara_source` to your `Cargo.toml`, and you're good to go!

```toml
[dependencies]
ara_source = "0.1.0"
```

## Example

```rust
use ara_source::loader;

fn main() {
    let root = format!(
        "{}/examples/fixture/",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    );

    let map = loader::load_directories(
        root.clone(),
        vec![
            format!("{}src", root),
            format!("{}vendor/foo", root),
            format!("{}vendor/bar", root),
        ],
    )
    .unwrap();

    println!("{:#?}", map);
}
```

see [examples](examples) directory for more examples.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Credits

* [Saif Eddin Gmati](https://github.com/azjezz)
* [All contributors](https://github.com/ara-lang/source/graphs/contributors)
