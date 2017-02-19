use std::fmt;

#[derive(Clone)]
struct Trouble {
    number: u32,
}

impl Trouble {
    fn new(number: u32) -> Trouble {
        Trouble {
            number: number,
        }
    }

    fn get_number(&self) -> u32 {
        self.number
    }
}

impl fmt::Display for Trouble {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Trouble {}]", self.number)
    }
}

trait Support : SupportClone {
    fn resolve(&self, trouble: Trouble) -> bool;
    fn support(&self, trouble: Trouble);
    fn set_next(&mut self, next: Box<Support>) -> Box<Support>;
}

trait SupportClone {
    fn clone_box(&self) -> Box<Support>;
}

impl<T> SupportClone for T where T: 'static + Support + Clone {
    fn clone_box(&self) -> Box<Support> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Support> {
    fn clone(&self) -> Box<Support> {
        self.clone_box()
    }
}

#[derive(Clone)]
struct NoSupport {
    name: String,
    next: Option<Box<Support>>,
}

impl NoSupport {
    fn new(name: String) -> NoSupport {
        NoSupport {
            name: name,
            next: None,
        }
    } 
    
    fn done(&self, trouble: Trouble) {
        println!("{} is resolved by {}.", trouble, self); 
    }

    fn fail(&self, trouble: Trouble) {
        println!("{} cannot be resolved.", trouble);
    }
}

impl Support for NoSupport {
    fn resolve(&self, trouble: Trouble) -> bool {
        false
    }

    fn support(&self, trouble: Trouble) {
        if self.resolve(trouble.clone()) {
            self.done(trouble);
        } else {
            match self.next.clone() {
                Some(n) => n.support(trouble),
                None => self.fail(trouble),
            }
        } 
    }

    fn set_next(&mut self, next: Box<Support>) -> Box<Support> {
        self.next = Some(next.clone());
        next
    }
}

impl fmt::Display for NoSupport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.name)
    }
}

#[derive(Clone)]
struct LimitSupport {
    name: String,
    next: Option<Box<Support>>,
    limit: u32,
}

impl LimitSupport {
     fn new(name: String, limit: u32) -> LimitSupport {
        LimitSupport {
            name: name,
            next: None,
            limit: limit,
        }
     } 
    
    fn done(&self, trouble: Trouble) {
        println!("{} is resolved by {}.", trouble, self); 
    }

    fn fail(&self, trouble: Trouble) {
        println!("{} cannot be resolved.", trouble);
    }
}

impl Support for LimitSupport {
    fn resolve(&self, trouble: Trouble) -> bool {
        if trouble.get_number() < self.limit {
            true
        } else {
            false
        }
    }

    fn support(&self, trouble: Trouble) {
        if self.resolve(trouble.clone()) {
            self.done(trouble);
        } else {
            match self.next.clone() {
                Some(n) => n.support(trouble),
                None => self.fail(trouble),
            }
        } 
    }

    fn set_next(&mut self, next: Box<Support>) -> Box<Support> {
        self.next = Some(next.clone());
        next
    }
}

impl fmt::Display for LimitSupport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.name)
    }
}

#[derive(Clone)]
struct OddSupport {
    name: String,
    next: Option<Box<Support>>,
}

impl OddSupport {
     fn new(name: String) -> OddSupport {
        OddSupport {
            name: name,
            next: None,
        }
     } 
    
    fn done(&self, trouble: Trouble) {
        println!("{} is resolved by {}.", trouble, self); 
    }

    fn fail(&self, trouble: Trouble) {
        println!("{} cannot be resolved.", trouble);
    }
}

impl Support for OddSupport {
    fn resolve(&self, trouble: Trouble) -> bool {
        if trouble.get_number() % 2 == 1 {
            true
        } else {
            false
        }
    }

    fn support(&self, trouble: Trouble) {
        if self.resolve(trouble.clone()) {
            self.done(trouble);
        } else {
            match self.next.clone() {
                Some(n) => n.support(trouble),
                None => self.fail(trouble),
            }
        } 
    }

    fn set_next(&mut self, next: Box<Support>) -> Box<Support> {
        self.next = Some(next.clone());
        next 
    }
}

impl fmt::Display for OddSupport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.name)
    }
}

#[derive(Clone)]
struct SpecialSupport {
    name: String,
    next: Option<Box<Support>>,
    number: u32,
}

impl SpecialSupport {
     fn new(name: String, number: u32) -> SpecialSupport {
        SpecialSupport {
            name: name,
            next: None,
            number: number,
        }
     } 
    
    fn done(&self, trouble: Trouble) {
        println!("{} is resolved by {}.", trouble, self); 
    }

    fn fail(&self, trouble: Trouble) {
        println!("{} cannot be resolved.", trouble);
    }
}

impl Support for SpecialSupport {
    fn resolve(&self, trouble: Trouble) -> bool {
        if trouble.get_number() == self.number {
            true
        } else {
            false
        }
    }

    fn support(&self, trouble: Trouble) {
        if self.resolve(trouble.clone()) {
            self.done(trouble);
        } else {
            match self.next.clone() {
                Some(n) => n.support(trouble),
                None => self.fail(trouble),
            }
        } 
    }
    
    fn set_next(&mut self, next: Box<Support>) -> Box<Support> {
        self.next = Some(next.clone());
        next
    }
}

impl fmt::Display for SpecialSupport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.name)
    }
}

fn main() {
    let mut alice = Box::new(NoSupport::new("Alice".to_string()));
    let mut bob = Box::new(LimitSupport::new("Bob".to_string(), 100));
    let mut charlie = Box::new(SpecialSupport::new("Charlie".to_string(), 429));
    let mut diana = Box::new(LimitSupport::new("Diana".to_string(), 200));
    let mut elmo = Box::new(OddSupport::new("Elmo".to_string()));
    let fred = Box::new(LimitSupport::new("fred".to_string(), 300));

    elmo.set_next(fred);
    diana.set_next(elmo);
    charlie.set_next(diana);
    bob.set_next(charlie);
    alice.set_next(bob);

    let mut i = 0;
    while i < 500 {
        alice.support(Trouble::new(i));
        i += 33;
    }
}
