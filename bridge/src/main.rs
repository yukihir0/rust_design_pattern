struct Display {
    imple : Box<DisplayImpl>,
}

impl Display {
    fn new(imple: Box<DisplayImpl>) -> Display {
        Display {
            imple: imple,
        }
    }

    fn open(&self) {
        self.imple.raw_open();
    }

    fn print(&self) {
        self.imple.raw_print();
    }

    fn close(&self) {
        self.imple.raw_close();
    }

    fn display(&self) {
        self.open();
        self.print();
        self.close();
    }
}

struct CountDisplay {
    display: Box<DisplayImpl>,
}

impl CountDisplay {
    fn new(display: Box<DisplayImpl>) -> CountDisplay {
        CountDisplay {
            display: display,
        }
    }

    fn multi_display(&self, times: u32) {
        self.display.raw_open();
        for _ in 0..times {
            self.display.raw_print();
        }
        self.display.raw_close();
    }

    fn display(&self) {
        self.display.raw_open();
        self.display.raw_print();
        self.display.raw_close();
    }
}

trait DisplayImpl {
    fn raw_open(&self);
    fn raw_print(&self);
    fn raw_close(&self);
}

struct StringDisplayImpl {
    string: String,
    width: usize,
}

impl StringDisplayImpl {
    fn new(string: String) -> StringDisplayImpl {
        StringDisplayImpl {
            string: string.clone(),
            width: string.chars().count(),
        }
    }

    fn print_line(&self) {
        print!("+");
        for _ in 0..self.width {
            print!("-");
        }
        println!("+");
    }
}

impl DisplayImpl for StringDisplayImpl {
    fn raw_open(&self) {
        self.print_line();
    }

    fn raw_print(&self) {
        println!("|{}|", self.string);
    }

    fn raw_close(&self) {
        self.print_line();
    }
}

fn main() {
    let d1 = Display::new(Box::new(StringDisplayImpl::new("Hello, Japan".to_string())));
    let d2 = CountDisplay::new(Box::new(StringDisplayImpl::new("Hello, World.".to_string())));
    let d3 = CountDisplay::new(Box::new(StringDisplayImpl::new("Hello, Universe.".to_string())));
    d1.display();
    d2.display();
    d3.display();
    d3.multi_display(5);
}
