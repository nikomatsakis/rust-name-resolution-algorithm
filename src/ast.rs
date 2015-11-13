#![allow(dead_code)]

use intern::{intern, InternedString};

pub struct Krate {
    pub modules: Vec<Module>,
    pub structures: Vec<Structure>,
    pub imports: Vec<Import>,
    pub globs: Vec<Glob>,
    pub macro_defs: Vec<MacroDef>,
    pub macro_refs: Vec<MacroRef>,
    pub macro_husks: Vec<MacroHusk>,
    pub paths: Vec<Path>,
    pub codes: Vec<Code>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ItemId {
    Module(ModuleId),
    Structure(StructureId),
    Import(ImportId),
    Glob(GlobId),
    MacroDef(MacroDefId),
    MacroRef(MacroRefId),
    MacroHusk(MacroHuskId),
    Code(CodeId),
}

pub const ROOT_ID: ModuleId = ModuleId(0);
pub const ROOT_PATH: PathId = PathId(0);
pub const THIS_PATH: PathId = PathId(1);

impl Krate {
    pub fn new() -> Krate {
        Krate {
            modules: vec![Module { privacy: Privacy::Pub, name: intern(""), items: vec![] }],
            structures: vec![],
            imports: vec![],
            globs: vec![],
            macro_defs: vec![],
            macro_refs: vec![],
            macro_husks: vec![],
            paths: vec![Path::Root, Path::This],
            codes: vec![],
        }
    }

    pub fn module_ids(&self) -> Vec<ModuleId> {
        (0..self.modules.len()).map(|i| ModuleId(i)).collect()
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

    pub fn add_glob(&mut self, glob: Glob) -> GlobId {
        self.globs.push(glob);
        GlobId(self.globs.len() - 1)
    }

    pub fn add_macro_def(&mut self, macro_def: MacroDef) -> MacroDefId {
        self.macro_defs.push(macro_def);
        MacroDefId(self.macro_defs.len() - 1)
    }

    pub fn add_macro_ref(&mut self, macro_ref: MacroRef) -> MacroRefId {
        self.macro_refs.push(macro_ref);
        MacroRefId(self.macro_refs.len() - 1)
    }

    pub fn add_macro_husk(&mut self, macro_husk: MacroHusk) -> MacroHuskId {
        self.macro_husks.push(macro_husk);
        MacroHuskId(self.macro_husks.len() - 1)
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
pub struct GlobId(pub usize);

pub struct Glob {
    pub privacy: Privacy,
    pub path: PathId,
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
pub struct MacroHuskId(pub usize);

pub struct MacroHusk {
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

#[derive(Clone)]
pub enum Path {
    Root,
    This,
    Cons(PathId, InternedString),
}


