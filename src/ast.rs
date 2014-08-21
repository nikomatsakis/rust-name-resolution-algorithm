use std::rc::Rc;
use intern::Id;

pub struct AST {
    pub items: Vec<Item>,
    pub uses: Vec<Use>,
}

pub type ItemIndex = uint;
pub type UseIndex = uint;

#[deriving(Show)]
pub enum Item {
    Module(Module),
    Struct(Struct),
}

#[deriving(Show)]
pub struct Module {
    pub id: Id,
    pub uses: Vec<UseIndex>,
    pub members: Vec<ItemIndex>,
}

pub type ModulePtr = Rc<Module>;

#[deriving(Show)]
pub enum UseId {
    Glob,
    Named(Id)
}

#[deriving(Show)]
pub struct Use {
    pub kind: UseKind,
    pub path: PathPtr,
    pub id: UseId,
}

#[deriving(Show,Clone)]
pub enum UseKind {
    ImportUse,
    PubUse
}

#[deriving(Show)]
pub struct Struct {
    pub id: Id
}

pub type StructPtr = Rc<Struct>;

#[deriving(Show, Hash, Eq, PartialEq)]
pub enum Path {
    Root(Id),
    Self(Id),
    Subpath(Rc<Path>, Id),
}

pub type PathPtr = Rc<Path>;

impl AST {
    pub fn root_index(&self) -> ItemIndex {
        self.items.len() - 1
    }
}

impl Path {
    pub fn tail_id(&self) -> Id {
        match *self {
            Root(i) => i,
            Self(i) => i,
            Subpath(_, i) => i,
        }
    }
}
