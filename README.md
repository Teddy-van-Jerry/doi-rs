# DOI for Rust
Digital Object Identifier (DOI) resolver for Rust

This crate provides a simple way to resolve DOIs and retrieve metadata
using the [DOI](https://www.doi.org) APIs.

## Basic Usage
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

## Metadata
This library also provides a way to retrieve metadata for a DOI.
The `metadata` feature is required to use this functionality (enabled by default).

### Structured Metadata
The structured metadata can be retrieved using the `metadata` method.
```rust
use doi::Doi;
let doi = Doi::new("10.1109/TCSII.2024.3366282");
match doi.metadata() {
    Ok(metadata) => println!("Paper Title: {}", metadata.title.unwrap_or("<unknown>".to_string())),
    Err(e) => eprintln!("Error: {}", e),
}
```

Supported metadata fields:
| Field | Type | Description |
| --- | --- | --- |
| `title` | `Option<String>` | Title of the document |
| `authors` | `Option<Vec<DoiMetadataPerson>>` | Author(s) of the document |
| `r#type` | `Option<String>` | Type of the document (e.g., journal, conference) |

The `DoiMetadataPerson` struct has the fields `given`, `family`, and `suffix`, which are all `Option<String>`.

### Raw JSON Metadata
The raw JSON metadata can be retrieved using the `metadata_json` method,
powered by `serde_json`.
```rust
use doi::Doi;
let doi = Doi::new("10.1109/TCSII.2024.3366282");
match doi.metadata_json() {
    Ok(metadata) => println!("Metadata: {}", metadata),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Blocking Requests
This library is designed to use blocking I/O,
depending on the [`ureq` library](https://docs.rs/ureq) for HTTP requests.

## License
This project is licensed under the [MIT license](LICENSE).
