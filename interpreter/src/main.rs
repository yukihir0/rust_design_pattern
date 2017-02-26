use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::str::FromStr;

trait Node {
    fn parse(&mut self, context: &mut Context);
}

struct ProgramNode {}

impl ProgramNode {
    fn new() -> ProgramNode {
        ProgramNode {}
    }
}

impl Node for ProgramNode {
    fn parse(&mut self, context: &mut Context) {
        context.skip_token("program".to_string());
        let mut command_list_node = CommandListNode::new();
        print!("node = [program");
        command_list_node.parse(context);
        println!("]");
    }
}

struct CommandListNode {}

impl CommandListNode {
    fn new() -> CommandListNode {
        CommandListNode {}
    }
}

impl Node for CommandListNode {
    fn parse(&mut self, context: &mut Context) {
        print!(" [");
        loop {
            let current_token = context.current_token();

            if current_token == "".to_string() {
                panic!(format!("Missing 'end'"));
            } else if current_token == "end".to_string() {
                context.skip_token("end".to_string());
                break;
            } else {
                let mut command_node = CommandNode::new();
                command_node.parse(context);
                print!(", ");
            }
        }
        print!("]");
    }
}

struct CommandNode {}

impl CommandNode {
    fn new() -> CommandNode {
        CommandNode {}
    }
}

impl Node for CommandNode {
    fn parse(&mut self, context: &mut Context) {
        if context.current_token() == "repeat".to_string() {
            let mut node = RepeatCommandNode::new();
            print!("[");
            node.parse(context);
            print!("]");
        } else {
            let mut node = PrimitiveCommandNode::new();
            node.parse(context);
        }
    }
}

struct RepeatCommandNode {
    number: u32,
}

impl RepeatCommandNode {
    fn new() -> RepeatCommandNode {
        RepeatCommandNode {
            number: 0, 
        }
    }
}

impl Node for RepeatCommandNode {
    fn parse(&mut self, context: &mut Context) {
        context.skip_token("repeat".to_string());
        self.number = context.current_number();
        context.next_token();
        let mut command_list_node = CommandListNode::new();
        print!("repeat {}", self.number);
        command_list_node.parse(context);
    }
}

struct PrimitiveCommandNode {
    name: String,
}

impl PrimitiveCommandNode {
    fn new() -> PrimitiveCommandNode {
        PrimitiveCommandNode {
            name: "".to_string(),
        }
    }
}

impl Node for PrimitiveCommandNode {
    fn parse(&mut self, context: &mut Context) {
        self.name = context.current_token();
        context.skip_token(self.name.clone());
        print!("{}", self.name);
        if self.name != "go".to_string() &&
           self.name != "right".to_string() &&
           self.name != "left".to_string()
        {
            panic!(format!("{} is undefined", self.name));
        }
    }
}

struct Context {
    tokens: Vec<String>,
    current_token: String,
}

impl Context {
    fn new(text: String) -> Context {
        let v: Vec<String> = text.split(" ").map(|s| s.to_string()).collect();
        let tokens: Vec<String> = v.into_iter().map(|s| s).rev().collect();

        let mut c = Context {
            tokens: tokens,
            current_token: "".to_string(),
        };
        
        c.next_token();
        c
    }

    fn next_token(&mut self) -> String {
        self.current_token  = match self.tokens.pop() {
            Some(t) => t,
            None => "".to_string(),
        };

        self.current_token.clone()
    }

    fn current_token(&self) -> String {
        self.current_token.clone()
    }

    fn skip_token(&mut self, token: String) {
        if token != self.current_token {
            panic!(format!("Warning: {} is expected, but {} is found.", token, self.current_token));
        }
        self.next_token();
    }

    fn current_number(&self) -> u32 {
        u32::from_str(self.current_token.clone().as_str()).unwrap()
    }
}

fn main() {
    let file = match File::open("program.txt") {
        Ok(f) => BufReader::new(f),
        Err(e) => panic!(e),
    };

    for line in file.lines() {
        let text = line.unwrap();
        let mut node = ProgramNode::new();
        let mut context = Context::new(text.clone());
        
        println!("text = \"{}\"", text);
        node.parse(&mut context);
         
    }
}
