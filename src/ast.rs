use intern::{intern, InternedString};

pub struct Krate {
    pub modules: Vec<Module>,
    pub structures: Vec<Structure>,
    pub imports: Vec<Import>,
    pub macro_defs: Vec<MacroDef>,
    pub macro_refs: Vec<MacroRef>,
    pub paths: Vec<Path>,
    pub codes: Vec<Code>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ItemId {
    Module(ModuleId),
    Structure(StructureId),
    Import(ImportId),
    MacroDef(MacroDefId),
    MacroRef(MacroRefId),
    Code(CodeId),
}

pub const ROOT_ID: ModuleId = ModuleId(0);
pub const ROOT_PATH: PathId = PathId(0);
pub const THIS_PATH: PathId = PathId(1);
pub const SUPER_PATH: PathId = PathId(2);

impl Krate {
    pub fn new() -> Krate {
        Krate {
            modules: vec![Module { privacy: Privacy::Pub, name: intern(""), items: vec![] }],
            structures: vec![],
            imports: vec![],
            macro_defs: vec![],
            macro_refs: vec![],
            paths: vec![Path::Root, Path::This, Path::Super],
            codes: vec![],
        }
    }

    pub fn add_module(&mut self, module: Module) -> ModuleId {
        self.modules.push(module);
        ModuleId(self.modules.len() - 1)
    }

    pub fn add_structure(&mut self, structure: Structure) -> StructureId {
        self.structures.push(structure);
        StructureId(self.structures.len() - 1)
    }

    pub fn add_import(&mut self, import: Import) -> ImportId {
        self.imports.push(import);
        ImportId(self.imports.len() - 1)
    }

    pub fn add_macro_def(&mut self, macro_def: MacroDef) -> MacroDefId {
        self.macro_defs.push(macro_def);
        MacroDefId(self.macro_defs.len() - 1)
    }

    pub fn add_macro_ref(&mut self, macro_ref: MacroRef) -> MacroRefId {
        self.macro_refs.push(macro_ref);
        MacroRefId(self.macro_refs.len() - 1)
    }

    pub fn add_code(&mut self, code: Code) -> CodeId {
        self.codes.push(code);
        CodeId(self.codes.len() - 1)
    }

    pub fn add_path(&mut self, path: Path) -> PathId {
        self.paths.push(path);
        PathId(self.paths.len() - 1)
    }
}

///////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Privacy {
    Priv,
    Pub,
}

///////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ModuleId(pub usize);

#[derive(Debug)]
pub struct Module {
    pub privacy: Privacy,
    pub name: InternedString,
    pub items: Vec<ItemId>,
}

///////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StructureId(pub usize);

#[derive(Debug)]
pub struct Structure {
    pub privacy: Privacy,
    pub name: InternedString,
}

///////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ImportId(pub usize);

pub struct Import {
    pub privacy: Privacy,
    pub path: PathId,
    pub alt_name: Option<InternedString>,
}

///////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MacroDefId(pub usize);

pub struct MacroDef {
    pub privacy: Privacy,
    pub name: InternedString,
    pub items: Vec<ItemId>,
}

///////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MacroRefId(pub usize);

pub struct MacroRef {
    pub path: PathId,
}

///////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CodeId(pub usize);

pub struct Code {
    pub paths: Vec<PathId>,
}

///////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PathId(pub usize);

pub enum Path {
    Root,
    This,
    Super,
    Cons(PathId, InternedString),
}


