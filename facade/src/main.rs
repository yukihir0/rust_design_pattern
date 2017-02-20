use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;

extern crate csv;
extern crate rustc_serialize;

#[derive(RustcDecodable)]
struct Record {
    mail_addr: String,
    user_name: String,
}

struct Database {}

impl Database {
    fn get_properties(db_name: String) -> HashMap<String, String> {
        let mut prop = HashMap::new();
        let file_name = format!("{}.txt", db_name);
        
        let mut rdr = match csv::Reader::from_file(file_name) {
            Ok(r) => r.has_headers(false),
            Err(e) => panic!(e),
        };

        for record in rdr.decode() {
            let record: Record = record.unwrap();
            prop.insert(record.mail_addr, record.user_name);
        }
        prop
    }
}

struct HtmlWriter {
    writer: File,
}

impl HtmlWriter {
    fn new(writer: File) -> HtmlWriter {
        HtmlWriter {
            writer: writer,
        }
    }

    #[allow(unused_must_use)]
    fn title(&mut self, title: String) {
        write!(self.writer, "<html>");
        write!(self.writer, "<head>");
        write!(self.writer, "<title>{}</title>", title);
        write!(self.writer, "</head>");
        write!(self.writer, "<body>\n");
        write!(self.writer, "<h1>{}</h1>\n", title);
    }

    #[allow(unused_must_use)]
    fn paragraph(&mut self, msg: String) {
        write!(self.writer, "<p>{}</p>\n", msg);
    }

    fn link(&mut self, href: String, caption: String) {
        self.paragraph(format!("<a href=\"{}\">{}</a>", href, caption));
    }

    fn mail_to(&mut self, mail_addr: String, user_name: String) {
        self.link(format!("mailto:{}", mail_addr), user_name);
    }

    #[allow(unused_must_use)]
    fn close(&mut self) {
        write!(self.writer, "</body>");
        write!(self.writer, "<html>\n");
    }
}

struct PageMaker {}

impl PageMaker {
    fn make_welcome_page(mail_addr: String, file_name: String) {
        let mail_prop = Database::get_properties("maildata".to_string());
        let user_name = match mail_prop.get(&mail_addr) {
            Some(n) => n,
            None => panic!("{} is not found", mail_addr),
        };

        let file = File::create(file_name.clone()).expect("Unable to create file");
        let mut writer = HtmlWriter::new(file);
        writer.title(format!("Welcome to {}'s page!", user_name));
        writer.paragraph(format!("{}のページへようこそ。", user_name));
        writer.paragraph("メールまっていますね。".to_string());
        writer.mail_to(mail_addr.clone(), user_name.to_string());
        writer.close();
        println!("{} is created for {} ({})", file_name, mail_addr, user_name);
    }
}

fn main() {
    PageMaker::make_welcome_page("hyuki@hyuki.com".to_string(), "welcome.html".to_string());
}
