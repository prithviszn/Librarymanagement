use soroban_sdk::{contractimpl, Address, Env, BytesN, Vec, String, panic_with};

#[derive(Clone)]
pub struct Book {
    isbn: String, 
    title: String,
    author: String,
    available: bool,
}

#[contractimpl]
impl LibraryContract {

    pub fn initialize(env: Env, admin: Address) {
        env.storage().set(&admin, &());
    }
    pub fn add_book(env: Env, isbn: String, title: String, author: String) {
        let admin = env.storage().get(&Address::generate_from_bytes(b"admin")).unwrap();
        if env.invoker() != admin {
            panic_with(env, "Only admin can add books.");
        }
        
        let book = Book {
            isbn,
            title,
            author,
            available: true,
        };

        env.storage().set(&book.isbn, &book);
    }

    
    pub fn borrow_book(env: Env, isbn: String, borrower: Address) {
        let mut book = env.storage().get(&isbn).unwrap_or_else(|| panic_with(env, "Book not found."));

        if !book.available {
            panic_with(env, "Book is not available.");
        }

        book.available = false;
        env.storage().set(&book.isbn, &book);

    }

    
    pub fn return_book(env: Env, isbn: String) {
        let mut book = env.storage().get(&isbn).unwrap_or_else(|| panic_with(env, "Book not found."));
        book.available = true;
        env.storage().set(&book.isbn, &book);
    
    }

    pub fn list_available_books(env: Env) -> Vec<Book> {
        let mut available_books = Vec::new(&env);
        for (_isbn, book) in env.storage().iter() {
            if book.available {
                available_books.push(book.clone());
            }
        }
        available_books
    }
}

