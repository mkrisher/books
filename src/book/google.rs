use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct GoogleBooksResponse {
    items: Option<Vec<BookItem>>,
}

#[derive(Deserialize, Debug)]
struct BookItem {
    #[serde(rename = "volumeInfo")]
    volume_info: VolumeInfo,
    #[serde(rename = "saleInfo")]
    sale_info: SaleInfo,
}

#[derive(Deserialize, Debug)]
struct VolumeInfo {
    title: String,
    authors: Vec<String>,
    publisher: String,
    #[serde(rename = "publishedDate")]
    published_date: String,
    #[serde(rename = "pageCount")]
    page_count: u32,
    categories: Vec<String>,
    description: String,
    #[serde(rename = "contentVersion")]
    content_version: String,
    #[serde(rename = "imageLinks")]
    image_links: ImageLinks,
    language: String,
    #[serde(rename = "previewLink")]
    preview_link: String,
    #[serde(rename = "infoLink")]
    info_link: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct ImageLinks {
    #[serde(rename = "smallThumbnail")]
    small_thumbnail: String,
    thumbnail: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct SaleInfo {
    country: String,
    saleability: String,
    #[serde(rename = "isEbook")]
    is_ebook: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct BookInfo {
    pub title: String,
    pub authors: String,
    pub publisher: String,
    pub published_date: String,
    pub page_count: u32,
    pub categories: String,
    pub description: String,
    pub content_version: String,
    pub image_links: ImageLinks,
    pub language: String,
    pub preview_link: String,
    pub info_link: String,
    pub sale_info: SaleInfo,
}

static URL: &str = "https://www.googleapis.com/books/v1/volumes?q=isbn";

pub async fn book(isbn: &str) -> Result<Option<BookInfo>, Box<dyn std::error::Error>> {
    println!("Fetching book with ISBN: {}", isbn);

    // Clean the ISBN by removing any hyphens or spaces
    let clean_isbn = isbn.replace("-", "").replace(" ", "");

    // Try both ISBN-10 and ISBN-13 formats
    let url = format!("{}:{}", URL, clean_isbn);

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    // NOTE: response returns { "kind": "books#volumes", "totalItems": 0 } if book is not found
    let books_response: GoogleBooksResponse = response.json().await?;

    if let Some(items) = books_response.items {
        if let Some(first_book) = items.first() {
            let result = BookInfo {
                title: first_book.volume_info.title.clone(),
                authors: first_book.volume_info.authors.join(", ").clone(),
                page_count: first_book.volume_info.page_count,
                categories: first_book.volume_info.categories.join(", ").clone(),
                publisher: first_book.volume_info.publisher.clone(),
                published_date: first_book.volume_info.published_date.clone(),
                description: first_book.volume_info.description.clone(),
                content_version: first_book.volume_info.content_version.clone(),
                image_links: first_book.volume_info.image_links.clone(),
                language: first_book.volume_info.language.clone(),
                preview_link: first_book.volume_info.preview_link.clone(),
                info_link: first_book.volume_info.info_link.clone(),
                sale_info: first_book.sale_info.clone(),
            };
            return Ok(Some(result));
        }
    } else {
        return Ok(None);
    }

    Err("No book found for this ISBN".into())
}
