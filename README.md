# Books

A Rust library for fetching book information from various sources.

## Features

- **Google Books API Integration**: Fetch comprehensive book details using ISBN numbers
- **Rich Book Information**: Returns title, authors, publisher, publication date, page count, categories, description, cover images, and more
- **ISBN Format Support**: Works with both ISBN-10 and ISBN-13 formats
- **Async/Await**: Built with async Rust using tokio and reqwest
- **Clean API**: Simple, easy-to-use interface for book lookups

## Usage

```rust
use books::book::google;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Fetch book information using ISBN-13
    if let Some(book_info) = google::book("978-1718503106").await? {
        println!("Title: {}", book_info.title);
        println!("Authors: {}", book_info.authors);
        println!("Publisher: {}", book_info.publisher);
        println!("Published: {}", book_info.published_date);
        println!("Pages: {}", book_info.page_count);
        println!("Categories: {}", book_info.categories);
    } else {
        println!("Book not found");
    }

    // Also works with ISBN-10
    if let Some(book_info) = google::book("1718503105").await? {
        println!("Title: {}", book_info.title);
    }

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

## Book Information Returned

The `BookInfo` struct includes:
- Title
- Authors (comma-separated list)
- Publisher
- Published date
- Page count
- Categories
- Description
- Cover image links (thumbnail and small thumbnail)
- Language
- Preview and info links
- Sale information (availability, ebook status)

## Future Enhancements

- Support for additional book data sources (OpenLibrary, etc.)
- Caching layer for repeated lookups
- Batch ISBN lookup support
