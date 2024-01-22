use core::fmt;
use std::borrow::Borrow;

// #[derive(Debug)]
struct Book {
    name: String,
    pages: u16,
    rating: f32,
    isBorrowed: bool,
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Debug for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Book details")
            .field("name", &self.name)
            .field("pages", &self.pages)
            .field("rating", &self.rating)
            .field("isBorrowed", &self.isBorrowed)
            .finish()
    }
}

impl Book {
    fn borrow_book(book_to_borrow: &mut Book) {
        book_to_borrow.isBorrowed = true;
        println!("book {} has been borrowed!", book_to_borrow);
    }

    fn display(book: &Book) {
        println!("{:?}", book);
    }
}

fn main() {
    let mut my_book = Book {
        name: "Omniscient Reader's Viewpoint".to_string(),
        pages: 664,
        rating: 10.0,
        isBorrowed: false,
    };

    Book::display(&my_book);
    Book::borrow_book(&mut my_book);
    Book::display(&my_book);
}
