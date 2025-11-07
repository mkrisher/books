use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct GoogleBooksResponse {
    items: Option<Vec<BookItem>>,
}

#[derive(Deserialize, Debug)]
struct BookItem {
    #[serde(rename = "volumeInfo")]
    volume_info: VolumeInfo,
}

#[derive(Deserialize, Debug)]
struct VolumeInfo {
    title: String,
    authors: Vec<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct BookInfo {
    pub title: String,
    pub authors: String,
}

static URL: &str = "https://www.googleapis.com/books/v1/volumes?q=isbn";

pub async fn book(isbn: &str) -> Result<BookInfo, Box<dyn std::error::Error>> {
    println!("Fetching book with ISBN: {}", isbn);

    // Clean the ISBN by removing any hyphens or spaces
    let clean_isbn = isbn.replace("-", "").replace(" ", "");

    // Try both ISBN-10 and ISBN-13 formats
    let url = format!("{}:{}", URL, clean_isbn);

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    let books_response: GoogleBooksResponse = response.json().await?;

    if let Some(items) = books_response.items {
        if let Some(first_book) = items.first() {
            let result = BookInfo {
                title: first_book.volume_info.title.clone(),
                authors: first_book.volume_info.authors.join(", ").clone(),
            };
            return Ok(result);
        }
    }

    Err("No book found for this ISBN".into())
}
