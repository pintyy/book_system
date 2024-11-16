#[derive(Debug)]
enum Book {
    Book {title:String, author:String, page:String, category:String}
}

impl Book {
    fn new (title:String,author:String,page:String,category:String )-> Self {
        Book::Book { title, author, page, category }
    
    }
    fn description (&self) -> String {
        match self {
            Book::Book { title, author, page, category } =>
            {   
            format!("{} was written by {} || Category :  {} and page: {} ",title,author,category,page)
            }
            
        }
    }
    
}
#[derive(Debug)]

struct  Library {
    books : Vec<Book>
}
impl Library {
    fn new () -> Self {
        Library { books:vec![]}
    }
    fn add_book (&mut self, book : Book) {
        self.books.push(book);
    }

    fn get_book_by_title (&self, title:&str) -> Option<&Book> {
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






fn main() {
    let mut library = Library::new();
    /*let book1 = Book::Book { title:String::from("Beyaz Geceler"),author:String::from("Fyodor Dostoevski"),category:String::from("Classic"),page:String::from("202")};
    let book2 = Book::Book { title: String::from("Savaş ve Barış"), author: String::from("Leo Tolstoy"), category: String::from("Classic"), page: String::from("1225")};
    let book3 = Book ::Book{ title: String::from("Karamazov Kardeşler"), author: String::from("Fyodor Dostoevski"), category: String::from("Classic"), page: String::from("824") };
    let book4 = Book::Book { title: String::from("Anna Karenina"), author: String::from("Leo Tolstoy"), category: String::from("Classic"), page: String::from("864") };
    let book20 = Book::Book { title: String::from("Suç ve Ceza"), author: String::from("Fyodor Dostoevski"), category: String::from("Classic"), page: String::from("671") };
    let book5 = Book::Book { title: String::from("Gurur ve Önyargı"), author: String::from("Jane Austen"), category: String::from("Classic"), page: String::from("279") };
    let book6 = Book::Book { title: String::from("Uçurtma Avcısı"), author: String::from("Khaled Hosseini"), category: String::from("Fiction"), page: String::from("371") };
    let book7 = Book::Book { title: String::from("Dönüşüm"), author: String::from("Franz Kafka"), category: String::from("Classic"), page: String::from("201") };
    let book8 = Book::Book { title: String::from("1984"), author: String::from("George Orwell"), category: String::from("Dystopian"), page: String::from("328") };
    let book9 = Book::Book { title: String::from("Hayvan Çiftliği"), author: String::from("George Orwell"), category: String::from("Dystopian"), page: String::from("112") };
    let book10 = Book::Book { title: String::from("Fahrenheit 451"), author: String::from("Ray Bradbury"), category: String::from("Dystopian"), page: String::from("194") };
    let book11 = Book::Book { title: String::from("Cesur Yeni Dünya"), author: String::from("Aldous Huxley"), category: String::from("Dystopian"), page: String::from("311") };
    let book12 = Book::Book { title: String::from("Silmarillion"), author: String::from("J.R.R. Tolkien"), category: String::from("Fantasy"), page: String::from("365") };
    let book13 = Book::Book { title: String::from("Yüzüklerin Efendisi"), author: String::from("J.R.R. Tolkien"), category: String::from("Fantasy"), page: String::from("1178") };
    let book14 = Book::Book { title: String::from("Hobbit"), author: String::from("J.R.R. Tolkien"), category: String::from("Fantasy"), page: String::from("310") };
    let book15 = Book::Book { title: String::from("Harry Potter ve Felsefe Taşı"), author: String::from("J.K. Rowling"), category: String::from("Fantasy"), page: String::from("223") };
    let book16 = Book::Book { title: String::from("Harry Potter ve Azkaban Tutsağı"), author: String::from("J.K. Rowling"), category: String::from("Fantasy"), page: String::from("317") };
    let book17 = Book::Book { title: String::from("Yüzyıllık Yalnızlık"), author: String::from("Gabriel Garcia Marquez"), category: String::from("Magic Realism"), page: String::from("417") };
    let book18 = Book::Book { title: String::from("Körlük"), author: String::from("Jose Saramago"), category: String::from("Fiction"), page: String::from("352") };
    let book19 = Book::Book { title: String::from("Don Kişot"), author: String::from("Miguel de Cervantes"), category: String::from("Classic"), page: String::from("1072") };
    

    library.add_book(book1);
    library.add_book(book2);
    library.add_book(book3);
    library.add_book(book4);
    library.add_book(book5);
    library.add_book(book6);
    library.add_book(book7);
    library.add_book(book8);
    library.add_book(book9);
    library.add_book(book10);
    library.add_book(book11);
    library.add_book(book12);
    library.add_book(book13);
    library.add_book(book14);
    library.add_book(book15);
    library.add_book(book16);
    library.add_book(book17);
    library.add_book(book18);
    library.add_book(book19);
    library.add_book(book20); */
    library.add_book(Book::new(String::from("Beyaz Geceler"), String::from("Fyodor Dostoevski"), String::from("202"), String::from("Classic")));
    library.add_book(Book::new(String::from("Savaş ve Barış"), String::from("Leo Tolstoy"), String::from("1225"), String::from("Classic")));
    library.add_book(Book::new(String::from("Karamazov Kardeşler"), String::from("Fyodor Dostoevski"), String::from("824"), String::from("Classic")));
    library.add_book(Book::new(String::from("Anna Karenina"), String::from("Leo Tolstoy"), String::from("864"), String::from("Classic")));
    library.add_book(Book::new(String::from("Suç ve Ceza"), String::from("Fyodor Dostoevski"), String::from("671"), String::from("Classic")));
    library.add_book(Book::new(String::from("Gurur ve Önyargı"), String::from("Jane Austen"), String::from("279"), String::from("Classic")));
    library.add_book(Book::new(String::from("Uçurtma Avcısı"), String::from("Khaled Hosseini"), String::from("371"), String::from("Fiction")));
    library.add_book(Book::new(String::from("Dönüşüm"), String::from("Franz Kafka"), String::from("201"), String::from("Classic")));
    library.add_book(Book::new(String::from("1984"), String::from("George Orwell"), String::from("328"), String::from("Dystopian")));
    library.add_book(Book::new(String::from("Hayvan Çiftliği"), String::from("George Orwell"), String::from("112"), String::from("Dystopian")));
    library.add_book(Book::new(String::from("Fahrenheit 451"), String::from("Ray Bradbury"), String::from("194"), String::from("Dystopian")));
    library.add_book(Book::new(String::from("Cesur Yeni Dünya"), String::from("Aldous Huxley"), String::from("311"), String::from("Dystopian")));
    library.add_book(Book::new(String::from("Silmarillion"), String::from("J.R.R. Tolkien"), String::from("365"), String::from("Fantasy")));
    library.add_book(Book::new(String::from("Yüzüklerin Efendisi"), String::from("J.R.R. Tolkien"), String::from("1178"), String::from("Fantasy")));
    library.add_book(Book::new(String::from("Hobbit"), String::from("J.R.R. Tolkien"), String::from("310"), String::from("Fantasy")));
    library.add_book(Book::new(String::from("Harry Potter ve Felsefe Taşı"), String::from("J.K. Rowling"), String::from("223"), String::from("Fantasy")));
    library.add_book(Book::new(String::from("Harry Potter ve Azkaban Tutsağı"), String::from("J.K. Rowling"), String::from("317"), String::from("Fantasy")));
    library.add_book(Book::new(String::from("Yüzyıllık Yalnızlık"), String::from("Gabriel Garcia Marquez"), String::from("417"), String::from("Magic Realism")));
    library.add_book(Book::new(String::from("Körlük"), String::from("Jose Saramago"), String::from("352"), String::from("Fiction")));
    library.add_book(Book::new(String::from("Don Kişot"), String::from("Miguel de Cervantes"), String::from("1072"), String::from("Classic")));   
   
    if let Some(book) =library.get_book_by_title("cesur yeni dünya")  {
        println!("{}",book.description());
        
    }
    else {
        print!("Couldnt find the book")
    }
   // println!("{:#?}",library);
}
