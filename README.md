# Books

A Rust library for fetching book information from various sources.

## Features

- **Google Books API Integration**: Fetch book details using ISBN numbers
- **ISBN Format Support**: Works with both ISBN-10 and ISBN-13 formats
- **Async/Await**: Built with async Rust using tokio and reqwest
- **Clean API**: Simple, easy-to-use interface for book lookups

## Usage

```rust
use books::book::google;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Fetch book information using ISBN-13
    let title = google::book("978-1718503106").await?;
    println!("Book title: {}", title);

    // Also works with ISBN-10
    let title = google::book("1718503105").await?;
    println!("Book title: {}", title);

    Ok(())
}
```

## ISBN Support

The library automatically handles both ISBN formats:
- **ISBN-10**: 10-digit format (e.g., `1718503105`)
- **ISBN-13**: 13-digit format (e.g., `978-1718503106`)

Hyphens and spaces in ISBNs are automatically cleaned before querying.

## Dependencies

- `tokio`: Async runtime
- `reqwest`: HTTP client for API requests
- `serde`: JSON deserialization

## Testing

Run the test suite with:

```bash
cargo test
```

## Future Enhancements

- Return additional book metadata (author, cover art, publisher, etc.)
- Support for additional book data sources
- Caching layer for repeated lookups
