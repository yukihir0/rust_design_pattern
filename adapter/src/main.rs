struct Banner {
    string: String,
}

impl Banner {
    fn new(string: String) -> Banner {
        Banner {
            string: string,
        }
    }

    fn show_with_paren(&self) {
        println!("({})", self.string);
    }

    fn show_with_aster(&self) {
        println!("*{}*", self.string);
    }
}

struct PrintBanner {
    banner: Banner,    
}

impl PrintBanner {
    fn new(string: String) -> PrintBanner {
        PrintBanner {
            banner: Banner::new(string),
        }
    }

    fn print_weak(&self) {
        self.banner.show_with_paren();
    }

    fn print_strong(&self) {
        self.banner.show_with_aster();
    }
}

fn main() {
    let p = PrintBanner::new("Hello".to_string());
    p.print_weak();
    p.print_strong();
}
