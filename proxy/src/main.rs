use std::{thread, time};
use std::io::{self, Write};

trait Printable {
    fn set_printer_name(&mut self, name: String);
    fn get_printer_name(&self) -> String;
    fn print(&mut self, string: String);
}

struct Printer {
    name: String,
}

impl Printer {
    fn new(name: String) -> Printer {
        Printer::heaby_job(format!("Printerのインスタンス({})を生成中", name));
        Printer {
            name: name,
        }
    }

    fn heaby_job(msg: String) {
        print!("{}", msg);
        io::stdout().flush().unwrap();

        for _ in 0..5 {
            thread::sleep(time::Duration::from_millis(1000));
            print!(".");
            io::stdout().flush().unwrap();
        }
        
        println!("完了。");
    }
}

impl Printable for Printer {
    fn set_printer_name(&mut self, name: String) {
        self.name = name;
    }

    fn get_printer_name(&self) -> String {
        self.name.clone()
    }

    fn print(&mut self, string: String) {
        println!("=== {} ===", self.name);
        println!("{}", string);
    }
}

struct PrinterProxy {
    name: String,
    real: Option<Box<Printable>>,
}

impl PrinterProxy {
    fn new(name: String) -> PrinterProxy {
        PrinterProxy {
            name: name,
            real: None,
        }
    }

    fn realize(&mut self) {
        if self.real.is_none() {
            self.real = Some(Box::new(Printer::new(self.name.clone())));
        }
    }
}

impl Printable for PrinterProxy {
    fn set_printer_name(&mut self, name: String) {
        if self.real.is_some() {
            match self.real.as_mut() {
                Some(real) => real.set_printer_name(name.clone()),
                None => {},
            }
        }
        self.name = name;
    }

    fn get_printer_name(&self) -> String {
        self.name.clone()
    }

    fn print(&mut self, string: String) {
        self.realize();
        match self.real.as_mut() {
            Some(real) => real.print(string),
            None => {},
        }
    } 
}

fn main() {
    let mut p = PrinterProxy::new("Alice".to_string());

    println!("名前は現在{}です。", p.get_printer_name());
    p.set_printer_name("Bob".to_string());
    println!("名前は現在{}です。", p.get_printer_name());
    p.print("Hello, world.".to_string());
}
