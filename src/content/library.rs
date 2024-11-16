use super::book::Book;
#[derive(Debug)]

pub struct  Library {
    books : Vec<Book>
}
impl Library {
    pub fn new () -> Self {
        Library { books:vec![]}
    }
    pub fn add_book (&mut self, book : Book) {
        self.books.push(book);
    }

    pub fn get_book_by_title (&self, title:&str) -> Option<&Book> {
        self.books.iter().find(|book|{
            if let Book::Book { title:book_title,..} = book{
                book_title.to_lowercase()==title.to_lowercase()
            }
            else {
                false
            }
        })
    }
}


