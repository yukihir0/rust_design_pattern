extern crate rand;

use std::{thread, time};
use rand::{thread_rng, Rng};

trait Observer {
    fn update(&self, generator: Box<&NumberGenerator>);
}

struct DigitObserver {}

impl DigitObserver {
    fn new() -> DigitObserver {
        DigitObserver {}
    }
}

impl Observer for DigitObserver {
    fn update(&self, generator: Box<&NumberGenerator>) {
        println!("DigitObserver: {}", generator.get_number());
        thread::sleep(time::Duration::from_millis(100));
    }
}

struct GraphObserver {}

impl GraphObserver {
    fn new() -> GraphObserver {
        GraphObserver {}
    }
}

impl Observer for GraphObserver {
    fn update(&self, generator: Box<&NumberGenerator>) {
        print!("GraphObserver:");
        for _ in 0..generator.get_number() {
            print!("*");
        }
        println!("");
        thread::sleep(time::Duration::from_millis(100));
    }
}

trait NumberGenerator {
    fn get_number(&self) -> u32;
    fn execute(&mut self);
}

struct RandomNumberGenerator {
    observers: Vec<Box<Observer>>,
    number: u32,
    rng: rand::ThreadRng,
}

impl RandomNumberGenerator {
    fn new() -> RandomNumberGenerator {
        RandomNumberGenerator {
            observers: Vec::new(),
            number: 0,
            rng: thread_rng(),
        }
    }

    fn add_observer(&mut self, observer: Box<Observer>) {
        self.observers.push(observer);
    }

    fn notify_observers(&self) {
        for observer in &self.observers {
            observer.update(Box::new(self));
        }
    }
}

impl NumberGenerator for RandomNumberGenerator {
    fn get_number(&self) -> u32 {
        self.number
    }

    fn execute(&mut self) {
        for _ in 0..20 {
            self.number = self.rng.gen_range(0, 50);
            self.notify_observers();
        }
    }
}


fn main() {
    let mut generator = Box::new(RandomNumberGenerator::new());
    let observer1 = Box::new(DigitObserver::new());
    let observer2 = Box::new(GraphObserver::new());
    generator.add_observer(observer1);
    generator.add_observer(observer2);
    generator.execute();
}
