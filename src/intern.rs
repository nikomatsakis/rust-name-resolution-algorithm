use std::cell::RefCell;
use std::fmt;

thread_local! {
    static THE_INTERNER: RefCell<Interner> = RefCell::new(Interner::new())
}

#[derive(Eq,PartialEq,Clone,Hash)]
pub struct Id {
    index: usize // index into context's identifiers list
}

pub fn Id(u: usize) -> Id {
    Id { index: u }
}

pub struct Interner {
    identifiers: Vec<String>,
}

pub fn install<F>(f: F) where F: FnOnce() {
    let interner = RefCell::new(Interner::new());
    THE_INTERNER.replace(Some(interner));
    f();
    THE_INTERNER.replace(None);
}

pub fn intern(s: &str) -> Id {
    THE_INTERNER.get().unwrap().borrow_mut().id(s)
}

impl Interner {
    pub fn new() -> Interner {
        Interner {
            identifiers: vec![]
        }
    }

    pub fn id(&mut self, s: &str) -> Id {
        match self.identifiers.iter().enumerate().find(|&(_,p)| p.as_slice() == s) {
            Some((i,_)) => Id(i),
            None => {
                self.identifiers.push(s.to_string());
                Id(self.identifiers.len() - 1)
            }
        }
    }
}

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let b = THE_INTERNER.get().unwrap();
        let b = b.borrow();
        write!(f, "{}", b.identifiers[self.index])
    }
}

