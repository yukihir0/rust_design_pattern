use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::process;

// # Traits
trait Item {
    fn make_html(&self) -> String;
}

trait Link : Item {}

trait Tray : Item {
    fn add(&mut self, item: Box<Item>);
}

trait Page {
    fn make_html(&self) -> String;
}

trait Factory {
    type LinkObject;
    type TrayObject;
    type PageObject;

    fn create_link(&self, caption: String, url: String) -> Box<Self::LinkObject>;
    fn create_tray(&self, caption: String) -> Box<Self::TrayObject>;
    fn create_page(&self, title: String, author: String) -> Box<Self::PageObject>;
}


// # List
// ## ListLink
struct ListLink {
    caption: String,
    url: String,
}

impl ListLink {
    fn new(caption: String, url: String) -> ListLink {
        ListLink {
            caption: caption,
            url: url,
        }
    }
}

impl Item for ListLink {
    fn make_html(&self) -> String {
        format!("<li><a href=\"{}\">{}</a></li>\n", self.url, self.caption)
    }
}

impl Link for ListLink {}

// ## ListTray
struct ListTray {
    caption: String,
    tray: Vec<Box<Item>>,
}

impl ListTray {
    fn new(caption: String) -> ListTray {
        ListTray {
            caption: caption,
            tray: Vec::new(),
        }
    }
}

impl Item for ListTray {
    fn make_html(&self) -> String {
        let mut buffer = "".to_string();
        
        buffer.push_str("<li>\n");
        buffer.push_str(&self.caption);
        buffer.push_str("\n<ul>\n");
        for item in &self.tray {
            buffer.push_str(&item.make_html());
        }
        buffer.push_str("</ul>\n");
        buffer.push_str("</li>\n");

        buffer
    }
}

impl Tray for ListTray {
    fn add(&mut self, item: Box<Item>) {
        self.tray.push(item); 
    }
}

// ## ListPage
struct ListPage {
    title: String,
    author: String,
    content: Vec<Box<Item>>,
}

impl ListPage {
    fn new(title: String, author: String) -> ListPage {
        ListPage {
            title: title,
            author: author,
            content: Vec::new(),
        }
    }

    fn add(&mut self, item: Box<Item>) {
        self.content.push(item); 
    }

    #[allow(unused_must_use)]
    fn output(&self) {
        let file_name = format!("{}.html", self.title);
        let mut writer = File::create(file_name.clone()).expect("Unable to create file");

        writeln!(writer, "{}", self.make_html());
        println!("{}を作成しました。", file_name);
    }
}

impl Page for ListPage {
    fn make_html(&self) -> String {
        let mut buffer = "".to_string();
        
        buffer.push_str(&format!("<html><head><title>{}</title></head>\n", self.title));
        buffer.push_str("<body>\n");
        buffer.push_str(&format!("<h1>{}</h1>\n", self.title));
        buffer.push_str("<ul>\n");
        for item in &self.content {
            buffer.push_str(&item.make_html());
        }
        buffer.push_str("</ul>\n");
        buffer.push_str(&format!("<hr><address>{}</address>", self.author));
        buffer.push_str("</body></html>\n");

        buffer
    }
}

// ## ListFactory
struct ListFactory {}

impl Factory for ListFactory {
    type LinkObject = ListLink;
    type TrayObject = ListTray;
    type PageObject = ListPage;

    fn create_link(&self, caption: String, url: String) -> Box<Self::LinkObject> {
        Box::new(ListLink::new(caption, url))
    }

    fn create_tray(&self, caption: String) -> Box<Self::TrayObject> {
        Box::new(ListTray::new(caption))
    }

    fn create_page(&self, title: String, author: String) -> Box<Self::PageObject> {
        Box::new(ListPage::new(title, author))
    }
}


// # Table
// ## TableLink
struct TableLink {
    caption: String,
    url: String,
}

impl TableLink {
    fn new(caption: String, url: String) -> TableLink {
        TableLink {
            caption: caption,
            url: url,
        }
    }
}

impl Item for TableLink {
    fn make_html(&self) -> String {
        format!("<td><a href=\"{}\">{}</a></td>\n", self.url, self.caption)
    }
}

impl Link for TableLink {}

// ## TableTray
struct TableTray {
    caption: String,
    tray: Vec<Box<Item>>,
}

impl TableTray {
    fn new(caption: String) -> TableTray {
        TableTray {
            caption: caption,
            tray: Vec::new(),
        }
    }
}

impl Item for TableTray {
    fn make_html(&self) -> String {
        let mut buffer = "".to_string();
        
        buffer.push_str("<td>");
        buffer.push_str("<table width=\"100%\" border=\"1\"<tr>");
        buffer.push_str(&format!("<td bgcolor=\"#cccccc\" align=\"center\"
        colspan=\"{}\"><b>{}</b></td>",
        self.tray.len(), self.caption));
        buffer.push_str("</tr>\n");
        buffer.push_str("<tr>\n");
        for item in &self.tray {
            buffer.push_str(&item.make_html());
        }
        buffer.push_str("</tr></table>");
        buffer.push_str("</td>");

        buffer
    }
}

impl Tray for TableTray {
    fn add(&mut self, item: Box<Item>) {
        self.tray.push(item); 
    }
}

// ## TablePage
struct TablePage {
    title: String,
    author: String,
    content: Vec<Box<Item>>,
}

impl TablePage {
    fn new(title: String, author: String) -> TablePage {
        TablePage {
            title: title,
            author: author,
            content: Vec::new(),
        }
    }

    fn add(&mut self, item: Box<Item>) {
        self.content.push(item); 
    }

    #[allow(unused_must_use)]
    fn output(&self) {
        let file_name = format!("{}.html", self.title);
        let mut writer = File::create(file_name.clone()).expect("Unable to create file");

        writeln!(writer, "{}", self.make_html());
        println!("{}を作成しました。", file_name);
    }
}

impl Page for TablePage {
    fn make_html(&self) -> String {
        let mut buffer = "".to_string();
        
        buffer.push_str(&format!("<html><head><title>{}</title></head>\n", self.title));
        buffer.push_str("<body>\n");
        buffer.push_str(&format!("<h1>{}</h1>\n", self.title));
        buffer.push_str("<ul>\n");
        for item in &self.content {
            buffer.push_str(&item.make_html());
        }
        buffer.push_str("</ul>\n");
        buffer.push_str(&format!("<hr><address>{}</address>", self.author));
        buffer.push_str("</body></html>\n");

        buffer
    }
}

// ## TableFactory
struct TableFactory {}

impl Factory for TableFactory {
    type LinkObject = TableLink;
    type TrayObject = TableTray;
    type PageObject = TablePage;

    fn create_link(&self, caption: String, url: String) -> Box<Self::LinkObject> {
        Box::new(TableLink::new(caption, url))
    }

    fn create_tray(&self, caption: String) -> Box<Self::TrayObject> {
        Box::new(TableTray::new(caption))
    }

    fn create_page(&self, title: String, author: String) -> Box<Self::PageObject> {
        Box::new(TablePage::new(title, author))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len()-1 != 1 {
        println!("Usage: cargo run list");
        println!("Usage: cargo run table");
        process::exit(0);
    }

    match args[1].as_str() {
        "list" => create_list(),
        "table" => create_table(),
        _ => process::exit(0),
    };
}

fn create_list() {
    let factory = ListFactory{};

    let asahi = factory.create_link("朝日新聞".to_string(), "http://www.asahi.com/".to_string());
    let yomiuri = factory.create_link("読売新聞".to_string(), "http://www.yomiuri.co.jp/".to_string());
    let us_yahoo = factory.create_link("Yahoo!".to_string(), "http://www.yahoo.com/".to_string());
    let jp_yahoo = factory.create_link("Yahoo!Japan".to_string(), "http://www.yahoo.co.jp/".to_string());
    let excite = factory.create_link("Excite".to_string(), "http://www.excite.com/".to_string());
    let google = factory.create_link("Google".to_string(), "http://www.google.com/".to_string());

    let mut tray_news = factory.create_tray("新聞".to_string());
    tray_news.add(asahi);
    tray_news.add(yomiuri);

    let mut tray_yahoo = factory.create_tray("Yahoo!".to_string());
    tray_yahoo.add(us_yahoo);
    tray_yahoo.add(jp_yahoo);

    let mut tray_search = factory.create_tray("サーチエンジン".to_string());
    tray_search.add(tray_yahoo);
    tray_search.add(excite);
    tray_search.add(google);

    let mut page = factory.create_page("LinkPage".to_string(), "結城　浩".to_string());
    page.add(tray_news);
    page.add(tray_search);
    page.output();
}

fn create_table() {
    let factory = TableFactory{};

    let asahi = factory.create_link("朝日新聞".to_string(), "http://www.asahi.com/".to_string());
    let yomiuri = factory.create_link("読売新聞".to_string(), "http://www.yomiuri.co.jp/".to_string());
    let us_yahoo = factory.create_link("Yahoo!".to_string(), "http://www.yahoo.com/".to_string());
    let jp_yahoo = factory.create_link("Yahoo!Japan".to_string(), "http://www.yahoo.co.jp/".to_string());
    let excite = factory.create_link("Excite".to_string(), "http://www.excite.com/".to_string());
    let google = factory.create_link("Google".to_string(), "http://www.google.com/".to_string());

    let mut tray_news = factory.create_tray("新聞".to_string());
    tray_news.add(asahi);
    tray_news.add(yomiuri);

    let mut tray_yahoo = factory.create_tray("Yahoo!".to_string());
    tray_yahoo.add(us_yahoo);
    tray_yahoo.add(jp_yahoo);

    let mut tray_search = factory.create_tray("サーチエンジン".to_string());
    tray_search.add(tray_yahoo);
    tray_search.add(excite);
    tray_search.add(google);

    let mut page = factory.create_page("LinkPage".to_string(), "結城　浩".to_string());
    page.add(tray_news);
    page.add(tray_search);
    page.output();
}
