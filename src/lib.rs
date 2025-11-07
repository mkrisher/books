pub mod book;

#[cfg(test)]
mod tests {
    use super::*;

    // Using the ISBN-13 for "The Rust Programming Language" book
    #[tokio::test]
    async fn it_successfully_gets_book_info_from_google() {
        let result = book::google::book("978-1718503106").await;
        match &result {
            Ok(book_info) => {
                // println!(
                //     "Found book: {} by {} {} pages in the {} category",
                //     book_info.title, book_info.authors, book_info.page_count, book_info.categories
                // );

                assert_eq!(
                    book_info.title,
                    "The Rust Programming Language, 2nd Edition"
                );
            }
            Err(e) => println!("Error: {}", e),
        }
        assert!(result.is_ok());

        // Test with ISBN-10 format
        let result_isbn10 = book::google::book("1718503105").await;
        match &result_isbn10 {
            Ok(book_info) => {
                // println!("Found book: {} by {}", book_info.title, book_info.authors);

                assert_eq!(
                    book_info.title,
                    "The Rust Programming Language, 2nd Edition"
                );
            }
            Err(e) => println!("Error: {}", e),
        }
        assert!(result_isbn10.is_ok());
    }
}
