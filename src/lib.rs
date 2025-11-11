pub mod book;

#[cfg(test)]
mod tests {
    use super::*;

    // Using the ISBN-13 for "The Rust Programming Language" book
    #[tokio::test]
    async fn it_successfully_gets_book_info_from_google() {
        let result = book::google::book("978-1718503106").await;
        match &result {
            Ok(result) => {
                // println!(
                //     "Found book: {} by {} {} pages in the {} category",
                //     book_info.title, book_info.authors, book_info.page_count, book_info.categories
                // );

                assert!(result.is_some());
            }
            Err(e) => println!("Error: {}", e),
        }
        assert!(result.is_ok());

        // Test with ISBN-10 format
        let result_isbn10 = book::google::book("1718503105").await;
        match &result_isbn10 {
            Ok(result) => {
                // println!("Found book: {} by {}", book_info.title, book_info.authors);

                assert!(result.is_some());
            }
            Err(e) => println!("Error: {}", e),
        }
        assert!(result_isbn10.is_ok());
    }

    // Using an unknown ISBN
    #[tokio::test]
    async fn it_returns_none() {
        let result = book::google::book("978").await;
        match &result {
            Ok(result) => {
                println!("Book not found",);

                assert!(result.is_none());
            }
            Err(e) => println!("Error: {}", e),
        }
        assert!(result.is_ok());
    }
}
