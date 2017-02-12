trait AbstractDisplay {
    fn open(&self);
    fn print(&self);
    fn close(&self);
    fn display(&self) {
        self.open();
        for i in 1..5 {
            self.print();
        }
        self.close();
    }
}

struct CharDisplay {
    ch: char,
}

impl CharDisplay {
    fn new(ch: char) -> CharDisplay {
        CharDisplay{ ch: ch }
    }
}

impl AbstractDisplay for CharDisplay {
    fn open(&self) {
        print!("<<");
    }

    fn print(&self) {
        let mut s = String::new();
        s.push(self.ch);

        print!("{}", s);
    }

    fn close(&self) {
        println!(">>");
    }
}

struct StringDisplay {
    string : String,
    width : usize,
}

impl StringDisplay {
    fn new(string: String) -> StringDisplay {
        let bytes_len = string.len();
        let chars_len = string.chars().count();
        if bytes_len == chars_len {
            StringDisplay { string: string.clone(), width: string.len() }
        } else {
            StringDisplay { string: string.clone(), width: 2 * string.chars().count() }
        }
    }

    fn print_line(&self) {
        print!("+");
        for i in 0..self.width {
            print!("-");
        }
        println!("+");
    }
}

impl AbstractDisplay for StringDisplay {
    fn open(&self) {
        self.print_line(); 
    }

    fn print(&self) {
        println!("|{}|", self.string);
    }

    fn close(&self) {
        self.print_line();
    }
}

fn main() {
    let d1 = CharDisplay::new('H');
    let d2 = StringDisplay::new("Hello, world.".to_string());
    let d3 = StringDisplay::new("こんにちは".to_string());
    
    d1.display();
    d2.display();
    d3.display();
}
