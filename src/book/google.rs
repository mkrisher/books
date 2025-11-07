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
}

pub async fn book(isbn: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("Fetching book with ISBN: {}", isbn);

    // Clean the ISBN by removing any hyphens or spaces
    let clean_isbn = isbn.replace("-", "").replace(" ", "");

    // Try both ISBN-10 and ISBN-13 formats
    let url = format!(
        "https://www.googleapis.com/books/v1/volumes?q=isbn:{}",
        clean_isbn
    );

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    let books_response: GoogleBooksResponse = response.json().await?;

    // TODO: currently only returns the title of the first found book
    // return items like author and cover art
    if let Some(items) = books_response.items {
        if let Some(first_book) = items.first() {
            return Ok(first_book.volume_info.title.clone());
        }
    }

    Err("No book found for this ISBN".into())
}
