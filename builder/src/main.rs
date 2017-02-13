use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::process;

struct Director {
    builder: Box<Builder>,
}

impl Director {
    fn new(builder: Box<Builder>) -> Director {
        Director {
            builder: builder,
        }
    }
    
    fn construct(&mut self) {
        self.builder.make_title("Greeting".to_string()); 
        self.builder.make_string("朝から昼にかけて".to_string());
        self.builder.make_items(vec!["おはようございます。".to_string(), "こんにちは。".to_string()]);
        self.builder.make_string("夜に".to_string());
        self.builder.make_items(vec!["こんばんは。".to_string(), "おやすみなさい。".to_string(), "さようなら。".to_string()]);
        self.builder.close();
    }
}

trait Builder {
    fn make_title(&mut self, title: String);
    fn make_string(&mut self, string: String);
    fn make_items(&mut self, items: Vec<String>);
    fn close(&mut self);
    fn get_result(&self) -> String;
}

struct TextBuilder {
    buffer: String,
}

impl TextBuilder {
    fn new() -> TextBuilder {
        TextBuilder {
            buffer: String::new(),
        }
    }
}

impl Builder for TextBuilder {
    fn make_title(&mut self, title: String) {
        self.buffer.push_str("==============================\n");
        self.buffer.push_str(&format!("『{}』\n", title));
        self.buffer.push_str("\n");
   }

    fn make_string(&mut self, s: String) {
        self.buffer.push_str(&format!("■{}\n", s));
        self.buffer.push_str("\n");
    }

    fn make_items(&mut self, items: Vec<String>) {
        for item in &items {
            self.buffer.push_str(&format!(" ・{}\n", item));
        }
        self.buffer.push_str("\n");
    }

    fn close(&mut self) {
        self.buffer.push_str("==============================\n");
    }

    fn get_result(&self) -> String {
        self.buffer.clone()
    }
}

struct HTMLBuilder {
    file_name: String,
    writer: File,
}

impl HTMLBuilder {
    fn new(title: String) -> HTMLBuilder {
        let file_name = format!("{}.html", title);
        let writer = File::create(file_name.clone()).expect("Unable to create file");

        HTMLBuilder {
            file_name: file_name,
            writer: writer,
        }
    }
}

#[allow(unused_must_use)]
impl Builder for HTMLBuilder {
    fn make_title(&mut self, title: String) {
        writeln!(self.writer, "<html><head><title>{}</title></head><body>", title);
        writeln!(self.writer, "<h1>{}</h1>", title);
    }

    fn make_string(&mut self, string: String) {
        writeln!(self.writer, "<p>{}</p>", string);
    }

    fn make_items(&mut self, items: Vec<String>) {
        writeln!(self.writer, "<ul>");
        for item in &items {
            writeln!(self.writer, "<li>{}</li>", item);
        }
        writeln!(self.writer, "</ul>");
    }

    fn close(&mut self) {
        writeln!(self.writer, "</body></html>");
        self.writer.flush();
    }

    fn get_result(&self) -> String {
        self.file_name.clone() 
    }
}

fn usage() {
    println!("Usage: cargo run plain");
    println!("Usage: cargo run html");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len()-1 != 1 {
        usage();
        process::exit(0);
    }

    match args[1].as_str() {
        "plain" => {
            let text_builder = Box::new(TextBuilder::new());
            let mut director = Director::new(text_builder);
            director.construct();
            let result = director.builder.get_result();
            println!("{}", result);
        },
        "html" => {
            let html_builder = Box::new(HTMLBuilder::new("test".to_string()));
            let mut director = Director::new(html_builder);
            director.construct();
            let result = director.builder.get_result();
            println!("{}", result);
        },
        _ => {
            usage();
            process::exit(0);
        },
    }
    
}
