use std::rc::Rc;
use intern::Id;

pub struct AST {
    pub items: Vec<Item>,
    pub imports: Vec<Import>,
}

pub type ItemIndex = usize;
pub type ImportIndex = usize;

#[derive(Debug)]
pub struct Item {
    pub name: Id,
    pub privacy: Privacy,
    pub kind: ItemKind
}

#[derive(Debug)]
pub enum ItemKind {
    Module(Module),
    Struct,
}

#[derive(Debug)]
pub struct Module {
    pub imports: Vec<ImportIndex>,
    pub members: Vec<ItemIndex>,
}

pub type ModulePtr = Rc<Module>;

#[derive(Debug)]
pub enum ImportId {
    Glob,
    Named(Id)
}

#[derive(Debug)]
pub struct Import {
    pub privacy: Privacy,
    pub path: PathPtr,
    pub id: ImportId,
}

#[derive(Debug, Clone)]
pub enum Privacy {
    Public,
    Private,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Path {
    Root(Id),
    This(Id),
    Subpath(Rc<Path>, Id),
}

pub type PathPtr = Rc<Path>;

impl AST {
    pub fn root_index(&self) -> ItemIndex {
        self.items.len() - 1
    }

    pub fn item(&self, index: usize) -> &Item {
        &self.items[index]
    }

    pub fn import(&self, index: usize) -> &Import {
        &self.imports[index]
    }

    pub fn is_module(&self, index: usize) -> bool {
        match self.item(index).kind {
            ItemKind::Module(..) => true,
            ItemKind::Struct => false
        }
    }
}

impl Path {
    pub fn tail_id(&self) -> Id {
        match *self {
            Path::Root(i) => i,
            Path::This(i) => i,
            Path::Subpath(_, i) => i,
        }
    }
}
