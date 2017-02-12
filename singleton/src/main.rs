#[macro_use]
extern crate lazy_static;

mod singleton {
    lazy_static! {
        pub static ref GET_INSTANCE: Singleton = {
            Singleton::new()
        };
    }

    #[derive(PartialEq, Eq)]
    pub struct Singleton {
    }

    impl Singleton {
        fn new() -> Singleton {
            println!("インスタンスを生成しました。");
            Singleton{}
        }
    }
}

fn main() {
    println!("Start.");
    let ref obj1 = *singleton::GET_INSTANCE;
    let ref obj2 = *singleton::GET_INSTANCE;

    if obj1 == obj2 {
        println!("obj1とobj2は同じインスタンスです。");
    } else {
        println!("obj1とobj2は同じインスタンスではありません。");
    }
    println!("End.");
}
