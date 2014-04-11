use std::local_data;
use std::fmt;

local_data_key!(the_interner: Interner)

#[deriving(Eq,TotalEq,Clone,Hash)]
pub struct Id {
    index: uint // index into context's identifiers list
}

pub fn Id(u: uint) -> Id {
    Id { index: u }
}

pub struct Interner {
    identifiers: ~[~str],
}

pub fn install(f: ||) {
    let interner = Interner::new();
    local_data::set(the_interner, interner);
    f();
    local_data::pop(the_interner);
}

pub fn intern(s: &str) -> Id {
    local_data::get_mut(the_interner, |i| {
        i.unwrap().id(s)
    })
}

impl Interner {
    pub fn new() -> Interner {
        Interner {
            identifiers: ~[]
        }
    }

    pub fn to_str<'a>(&'a self, id: Id) -> &'a str {
        self.identifiers[id.index].slice_from(0)
    }

    pub fn id(&mut self, s: &str) -> Id {
        match self.identifiers.iter().enumerate().find(|&(_,p)| p.slice(0, p.len()) == s) {
            Some((i,_)) => Id(i),
            None => {
                self.identifiers.push(s.to_owned());
                Id(self.identifiers.len() - 1)
            }
        }
    }

    pub fn int_identifier(&mut self, i: uint) -> Id {
        self.id(format!("{}", i))
    }
}

impl fmt::Show for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        local_data::get(the_interner, |i| {
            write!(f.buf, "{}", i.unwrap().identifiers[self.index])
        })
    }
}

