use std::fmt;

trait Visitor {
    fn visit_file(&self, file: Box<&File>, cur_dir: String);
    fn visit_directory(&self, directory: Box<&Directory>, cur_dir: String);
}

trait Element {
    fn accept(&self, v: Box<&Visitor>, cur_dir: String);
}

trait Entry : Element {
    fn get_name(&self) -> String;
    fn get_size(&self) -> u32;
}

struct File {
    name: String,
    size: u32,
}

impl File {
    fn new(name: String, size: u32) -> File {
        File {
            name: name,
            size: size,
        }
    }
}

impl Element for File {
    fn accept(&self, v: Box<&Visitor>, cur_dir: String) {
        v.visit_file(Box::new(self), cur_dir);
    }
}

impl Entry for File {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_size(&self) -> u32 {
        self.size
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.get_name(), self.get_size())
    }
}

struct Directory {
    name: String,
    dir: Vec<Box<Entry>>
}

impl Directory {
    fn new(name: String) -> Directory {
        Directory {
            name: name,
            dir: Vec::new(),
        }
    }

    fn add(&mut self, entry: Box<Entry>) {
        self.dir.push(entry);
    }
}

impl Element for Directory {
    fn accept(&self, v: Box<&Visitor>, cur_dir: String) {
        v.visit_directory(Box::new(self), cur_dir);
    }
}

impl Entry for Directory {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_size(&self) -> u32{
        let mut size = 0;
        for entry in &self.dir {
            size += entry.get_size();
        }
        size
    }
}

impl fmt::Display for Directory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.get_name(), self.get_size())
    }
}

struct ListVisitor {}

impl ListVisitor {
    fn new() -> ListVisitor {
        ListVisitor {}
    }
}

impl Visitor for ListVisitor {
    fn visit_file(&self, file: Box<&File>, cur_dir: String) {
        println!("{}/{}", cur_dir, file);
    }

    fn visit_directory(&self, directory: Box<&Directory>, cur_dir: String) {
         println!("{}/{}", cur_dir, directory);
        
         let current_dir = if cur_dir == "".to_string() {
            format!("/{}", directory.get_name())
        } else {
            format!("{}/{}", cur_dir, directory.get_name())
        };

        for entry in &directory.dir {
            entry.accept(Box::new(self), current_dir.clone());
        }
    }
}

fn main() {
    println!("Making root entries...");
    let mut root_dir = Box::new(Directory::new("root".to_string()));
    let mut bin_dir = Box::new(Directory::new("bin".to_string()));
    let tmp_dir = Box::new(Directory::new("tmp".to_string()));
    let mut usr_dir = Box::new(Directory::new("usr".to_string()));

    bin_dir.add(Box::new(File::new("vi".to_string(), 10000)));
    bin_dir.add(Box::new(File::new("latex".to_string(), 20000)));
    
    root_dir.add(bin_dir);
    root_dir.add(tmp_dir);

    root_dir.accept(Box::new(&ListVisitor::new()), "".to_string());

    println!();
    println!("Making user entries...");
    let mut yuki = Box::new(Directory::new("yuki".to_string()));
    yuki.add(Box::new(File::new("diary.html".to_string(), 100)));
    yuki.add(Box::new(File::new("Composite.java".to_string(), 200)));
    
    let mut hanako = Box::new(Directory::new("hanako".to_string()));
    hanako.add(Box::new(File::new("memo.tex".to_string(), 300)));
    
    let mut tomura = Box::new(Directory::new("tomura".to_string()));
    tomura.add(Box::new(File::new("game.doc".to_string(), 400)));
    tomura.add(Box::new(File::new("jumk.mail".to_string(), 500)));

    usr_dir.add(yuki);
    usr_dir.add(hanako);
    usr_dir.add(tomura);

    root_dir.add(usr_dir);

    root_dir.accept(Box::new(&ListVisitor::new()), "".to_string());
}
