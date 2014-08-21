use std::cell::RefCell;
use std::fmt;

local_data_key!(the_interner: RefCell<Interner>)

#[deriving(Eq,PartialEq,Clone,Hash)]
pub struct Id {
    index: uint // index into context's identifiers list
}

pub fn Id(u: uint) -> Id {
    Id { index: u }
}

pub struct Interner {
    identifiers: Vec<String>,
}

pub fn install(f: ||) {
    let interner = RefCell::new(Interner::new());
    the_interner.replace(Some(interner));
    f();
    the_interner.replace(None);
}

pub fn intern(s: &str) -> Id {
    the_interner.get().unwrap().borrow_mut().id(s)
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

impl fmt::Show for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let b = the_interner.get().unwrap();
        let b = b.borrow();
        write!(f, "{}", b.identifiers[self.index])
    }
}

