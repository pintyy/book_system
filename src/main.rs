mod content;

use content::book::Book;
use content::library::Library;




fn main() {
    let mut library = Library::new();
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
