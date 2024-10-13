mod book_tests;

#[derive(Clone, Debug)]
struct Book {
    id: u8,
    name: String,
    book_author: String,
    book_pub: u16,
}

struct BookStore {
    books: Vec<Book>,
    admin_address: String,
}

#[derive(Debug)]
enum BookStoreError {
    NotAuthorized,
    BookNotFound,
    BookIdAlreadyExists,
}

impl BookStore {
    fn new(admin_address: String) -> Self {
        BookStore {
            books: Vec::new(),
            admin_address,
        }
    }

    fn register_book(
        &mut self,
        caller: &str,
        name: String,
        book_author: String,
        book_pub: u16,
        book_id: u8,
    ) -> Result<(), BookStoreError> {
        if caller != self.admin_address {
            return Err(BookStoreError::NotAuthorized);
        }
        if self.books.iter().any(|book| book.id == book_id) {
            return Err(BookStoreError::BookIdAlreadyExists);
        }
        let book = Book {
            id: book_id,
            name,
            book_author,
            book_pub,
        };
        self.books.push(book);
        println!("Book added: {}", book_id);
        Ok(())
    }

    fn update_book(
        &mut self,
        caller: &str,
        name: String,
        book_author: String,
        book_pub: u16,
        book_id: u8,
    ) -> Result<(), BookStoreError> {
        if caller != self.admin_address {
            return Err(BookStoreError::NotAuthorized);
        }
        let book = self
            .books
            .iter_mut()
            .find(|b| b.id == book_id)
            .ok_or(BookStoreError::BookNotFound)?;
        book.name = name;
        book.book_author = book_author;
        book.book_pub = book_pub;
        println!("Book updated: {}", book_id);
        Ok(())
    }

    fn access_book(&self, book_id: u8) -> Result<&Book, BookStoreError> {
        self.books
            .iter()
            .find(|&book| book.id == book_id)
            .ok_or(BookStoreError::BookNotFound)
    }

    fn delete_book(&mut self, caller: &str, book_id: u8) -> Result<(), BookStoreError> {
        if caller != self.admin_address {
            return Err(BookStoreError::NotAuthorized);
        }
        let index = self
            .books
            .iter()
            .position(|book| book.id == book_id)
            .ok_or(BookStoreError::BookNotFound)?;

        let last_index = self.books.len() - 1;
        if index != self.books.len() - 1 {
            // Swap the target with the last element if it's not already the last one
            self.books.swap(index, last_index);
        }
        self.books.remove(index);
        println!("Book deleted: {}", book_id);
        Ok(())
    }
}

fn main() {
    let mut book_store = BookStore::new("Buhari".to_string());

    // Register a book
    if let Err(e) = book_store.register_book(
        "Buhari",
        "Rust Programming".to_string(),
        "daniel agantem".to_string(),
        2019,
        1,
    ) {
        println!("Error: {:?}", e);
    } else {
        println!("Book registered successfully");
    }

    // Try to register a book without admin rights
    if let Err(e) = book_store.register_book(
        "MrEmmanuel",
        "EVM Basics".to_string(),
        "Emmanuel okeke".to_string(),
        2020,
        2,
    ) {
        println!("Error: {:?}", e);
    } else {
        println!("Book registered successfully");
    }

    // Access a book
    match book_store.access_book(1) {
        Ok(book) => println!("Accessed book: {:?}", book),
        Err(e) => println!("Error: {:?}", e),
    }

    // Update a book
    if let Err(e) = book_store.update_book(
        "Buhari",
        "Rust Programming 2nd Ed".to_string(),
        "daniel agantem".to_string(),
        2021,
        1,
    ) {
        println!("Error: {:?}", e);
    } else {
        println!("Book updated successfully");
    }

    // Delete a book
    if let Err(e) = book_store.delete_book("Buhari", 1) {
        println!("Error: {:?}", e);
    } else {
        println!("Book deleted successfully");
    }
}
