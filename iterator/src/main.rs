trait Iterator {
    type Object;
    fn has_next(&self) -> bool;
    fn next(&mut self) -> Self::Object;
}

#[derive(Clone)]
struct Book {
    name: String,
}

impl Book {
    fn new(name: String) -> Book {
        Book {
            name: name,
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Clone)]
struct BookShelf {
    books: Vec<Book>,
    last: u32,
}

impl BookShelf {
    fn new(maxsize: usize) -> BookShelf {
        BookShelf {
            books: Vec::with_capacity(maxsize),
            last: 0,
        }
    }

    fn get_book_at(&self, index: u32) -> Book {
        self.books[index as usize].clone() 
    }

    fn append_book(&mut self, book: Book) {
        self.books.push(book);
        self.last += 1;
    }

    fn get_length(&self) -> u32 {
        self.last
    }

    fn iterator(&self) -> Box<Iterator<Object=Book>> {
        Box::new(BookShelfIterator::new((*self).clone()))
    }
}

struct BookShelfIterator {
    book_shelf: BookShelf,
    index: u32,
}

impl BookShelfIterator {
    fn new(book_shelf: BookShelf) -> BookShelfIterator {
        BookShelfIterator {
            book_shelf: book_shelf,
            index: 0,
        }
    }
}

impl Iterator for BookShelfIterator {
    type Object = Book;

    fn has_next(&self) -> bool {
        if self.index < self.book_shelf.get_length() {
            true
        } else {
            false
        }
    }

    fn next(&mut self) -> Self::Object {
        let book = self.book_shelf.get_book_at(self.index);
        self.index += 1;
        book
    }
}

fn main() {
    let mut book_shelf = BookShelf::new(4);
    book_shelf.append_book(Book::new("Around the World in 80 Days".to_string()));
    book_shelf.append_book(Book::new("Bible".to_string()));
    book_shelf.append_book(Book::new("Cinderella".to_string()));
    book_shelf.append_book(Book::new("Daddy-Long-Legs".to_string()));

    let mut it = book_shelf.iterator();
    while it.has_next() {
        let book = it.next();
        println!("{}", book.get_name());
    }
}
