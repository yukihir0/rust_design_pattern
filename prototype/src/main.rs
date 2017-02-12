use std::collections::HashMap;

trait Product {
    fn use_product(&self, s: String);
    fn create_clone(&self) -> Box<Product>;
}

struct Manager {
    showcase: HashMap<String, Box<Product>>,
}

impl Manager {
    fn new() -> Manager {
        Manager {
            showcase: HashMap::new(),
        }
    }

    fn register(&mut self, name: String, product: Box<Product>) {
        self.showcase.insert(name, product);
    }

    fn create(&self, product_name: String) -> Box<Product> {
        let product = self.showcase.get(&product_name);
        match product {
            Some(p) => p.create_clone(),
            _ => panic!(),
        }
    }
}

#[derive(Clone)]
struct MessageBox {
    deco_char: char,
}

impl MessageBox {
    fn new(deco_char: char) -> MessageBox {
        MessageBox {
            deco_char: deco_char,
        }
    }
}

impl Product for MessageBox {
    fn use_product(&self, s: String) {
        let length = s.chars().count();
        let mut ch = String::new();
        ch.push(self.deco_char);

        for _ in 0..(length + 4) {
            print!("{}", ch);
        }
        println!("");
        println!("{} {} {}", ch, s, ch);
        for _ in 0..(length + 4) {
            print!("{}", ch);
        }
        println!("");
    }

    fn create_clone(&self) -> Box<Product> {
        Box::new((*self).clone())
    }
}

#[derive(Clone)]
struct UnderlinePen {
    ul_char: char,
}

impl UnderlinePen {
    fn new(ul_char: char) -> UnderlinePen {
        UnderlinePen {
           ul_char: ul_char, 
        }
    }
}

impl Product for UnderlinePen {
    fn use_product(&self, s: String) {
        let length = s.chars().count();
        let mut ch = String::new();
        ch.push(self.ul_char);

        println!("\" {} \"", s);
        print!(" ");
        for _ in 0..length {
            print!("{}", ch);
        }
        println!("");
    }

    fn create_clone(&self) -> Box<Product> {
        Box::new((*self).clone())
    }
}


fn main() {
    let mut manager = Manager::new();
    let upen = Box::new(UnderlinePen::new('~'));
    let mbox = Box::new(MessageBox::new('*'));
    let sbox = Box::new(MessageBox::new('/'));
    manager.register("strong message".to_string(), upen);
    manager.register("warning box".to_string(), mbox);
    manager.register("slash box".to_string(), sbox);

    let p1 = manager.create("strong message".to_string());
    p1.use_product("Hello, world.".to_string());
    let p2 = manager.create("warning box".to_string());
    p2.use_product("Hello, world.".to_string());
    let p3 = manager.create("slash box".to_string());
    p3.use_product("Hello, world.".to_string());
}
