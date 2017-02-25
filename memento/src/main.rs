extern crate rand;

use rand::{thread_rng, Rng};
use std::{thread, time};
use std::fmt;

#[derive(Clone)]
struct Memento {
    money: u32,
    fruits: Vec<String>,
}

impl Memento {
    fn new(money: u32) -> Memento {
        Memento {
            money: money,
            fruits: Vec::new(),
        }
    }

    fn get_money(&self) -> u32 {
        self.money
    }

    fn add_fruit(&mut self, fruit: String) {
        self.fruits.push(fruit);
    }

    fn get_fruits(&self) -> Vec<String> {
        self.fruits.clone()
    }
}

struct Gamer {
    money: u32,
    fruits: Vec<String>,
    rng: rand::ThreadRng,
}

impl Gamer {
    fn new(money: u32) -> Gamer {
        Gamer {
            money: money,
            fruits: Vec::new(),
            rng: thread_rng(),
        }
    }

    fn get_money(&self) -> u32 {
        self.money
    }

    fn bet(&mut self) {
        let dice = self.rng.gen_range(0, 7);

        if dice == 1 {
            self.money += 100;
            println!("所持金が増えました。");
        } else if dice == 2 {
            self.money /= 2;
            println!("所持金が半分になりました。");
        } else if dice == 6 {
            let f = self.get_fruit();
            println!("フルーツ（{}）をもらいました。", f);
            self.fruits.push(f);
        } else {
            println!("何も起こりませんでした。");
        }
    }

    fn create_memento(&self) -> Memento {
        let mut m = Memento::new(self.money);
        
        for f in &self.fruits {
            m.add_fruit(f.clone());
        }

        m
    }

    fn restore_memento(&mut self, memento: Memento) {
        self.money = memento.get_money();
        self.fruits = memento.get_fruits();
    }

    fn get_fruit(&mut self) -> String {
        let mut prefix = "".to_string();
        let coin = self.rng.gen_range(0, 1);
        if coin == 0 {
            prefix = "おいしい".to_string();
        }

        let fruits = ["リンゴ", "ぶどう", "バナナ", "みかん"];
        let index = self.rng.gen_range(0, fruits.len());
        format!("{}{}", prefix, fruits[index].to_string())
    }
}

impl fmt::Display for Gamer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[money = {} + fruits = {:?}]", self.money, self.fruits)
    }
}

fn main() {
    let mut gamer = Gamer::new(100);
    let mut memento = gamer.create_memento();
    for i in 0..100 {
        println!("===={}", i);
        println!("現状:{}", gamer);

        gamer.bet();

        println!("所持金は{}円になりました。", gamer.get_money());

        if gamer.get_money() > memento.get_money() {
            println!("      (だいぶ増えたので、現在の状態を保存しておこう)");
            memento = gamer.create_memento();
        } else if gamer.get_money() < (memento.get_money() / 2) {
            println!("      (だいぶ減ったので、以前の状態に復帰しよう)");
            gamer.restore_memento(memento.clone());
        }

        thread::sleep(time::Duration::from_millis(1000));
        println!();
    }
}
