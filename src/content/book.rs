#[derive(Debug)]
pub enum Book {
    Book {title:String, author:String, page:String, category:String}
}

impl Book {
    pub fn new (title:String,author:String,page:String,category:String )-> Self {
        Book::Book { title, author, page, category }
    
    }
    pub fn description (&self) -> String {
        match self {
            Book::Book { title, author, page, category } =>
            {   
            format!("{} was written by {} || Category :  {} and page: {} ",title,author,category,page)
            }
            
        }
    }
    
}