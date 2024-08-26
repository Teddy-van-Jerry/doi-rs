# DOI for Rust
Digital Object Identifier (DOI) resolver for Rust

This crate provides a simple way to resolve DOIs using the [DOI](https://www.doi.org) APIs.

## Synchronous Design
This library is designed to be *synchronous*,
depending on the [`ureq` library](https://docs.rs/ureq) for HTTP requests.

## Usage
The following is a basic example of using this library:
```rust
use doi::Doi;
let doi = Doi::new("10.1109/TCSII.2024.3366282");
match doi.resolve() {
    Ok(resolved) => println!("Resolved Link: {}", resolved),
    Err(e) => eprintln!("Error: {}", e),
}
```
Please refer to the API documentation for more information.
More complicated constructions can be done using the `DoiBuilder` struct.

## License
This project is licensed under the [MIT license](LICENSE).
