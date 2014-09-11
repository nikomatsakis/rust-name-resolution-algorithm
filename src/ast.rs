use std::rc::Rc;
use intern::Id;

pub struct AST {
    pub items: Vec<Item>,
    pub imports: Vec<Import>,
}

pub type ItemIndex = uint;
pub type ImportIndex = uint;

#[deriving(Show)]
pub struct Item {
    pub name: Id,
    pub privacy: Privacy,
    pub kind: ItemKind
}

#[deriving(Show)]
pub enum ItemKind {
    Module(Module),
    Struct,
}

#[deriving(Show)]
pub struct Module {
    pub imports: Vec<ImportIndex>,
    pub members: Vec<ItemIndex>,
}

pub type ModulePtr = Rc<Module>;

#[deriving(Show)]
pub enum ImportId {
    Glob,
    Named(Id)
}

#[deriving(Show)]
pub struct Import {
    pub privacy: Privacy,
    pub path: PathPtr,
    pub id: ImportId,
}

#[deriving(Show,Clone)]
pub enum Privacy {
    Public,
    Private,
}

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

    pub fn item(&self, index: uint) -> &Item {
        &self.items[index]
    }

    pub fn import(&self, index: uint) -> &Import {
        &self.imports[index]
    }

    pub fn is_module(&self, index: uint) -> bool {
        match self.item(index).kind {
            Module(..) => true,
            Struct => false
        }
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
