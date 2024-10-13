// book_tests.rs

use super::*; 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_bookstore() {
        let bookstore = BookStore::new("Buhari".to_string());
        assert_eq!(bookstore.admin_address, "Buhari");
        assert!(bookstore.books.is_empty());
    }

    #[test]
    fn test_register_book() {
        let mut bookstore = BookStore::new("Agantem".to_string());
        let result = bookstore.register_book("Agantem", "Rust_Basics".to_string(), "Daniel".to_string(), 2021, 1);
        assert!(result.is_ok());
        assert_eq!(bookstore.books.len(), 1);
        assert_eq!(bookstore.books[0].name, "Rust_Basics");
    }
    #[test]
    fn test_update_book() {
        let mut bookstore = BookStore::new("Agantem".to_string());
        let result = bookstore.register_book("Agantem", "Rust_Basics".to_string(), "Daniel".to_string(), 2021, 1);
        
        let is_updated = bookstore.update_book("Agantem", "new_book".to_string(), "Daniel".to_string(), 2021, 1);
        assert!(result.is_ok());
        assert!(is_updated.is_ok());
        assert_eq!(bookstore.books[0].name, "new_book")
       

    }

    
}
