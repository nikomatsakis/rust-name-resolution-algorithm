use ast::*;
use intern::InternedString;
use std::collections::{HashMap, HashSet};
use std::mem;

#[derive(Debug)]
struct ModuleContentSets {
    ticker: usize,
    module_contents: HashMap<ModuleId, ModuleContents>,
    fully_expanded: HashSet<ModuleId>,
}

#[derive(Clone, Debug)]
pub struct ModuleContents {
    pub members: HashMap<InternedString, HashSet<NameResolution>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum NameResolution {
    Seed(ItemId),
    Placeholder,
    Glob(ItemId),
}

pub enum Resolution {
    Zero,
    One(ItemId),
    Many,
    Error,
}

#[derive(Debug)]
pub enum ResolutionError {
    MultipleNames {
        module_id: ModuleId,
        name: InternedString,
    },

    InvalidPath {
        path: PathId,
        source: ItemId,
    },
}

pub fn resolve_and_expand(krate: &mut Krate) -> Result<HashMap<ModuleId, ModuleContents>, ResolutionError> {
    let mut resolutions = ModuleContentSets::new();
    let mut ticker = 0;
    while resolutions.changed_since(&mut ticker) {
        expand_macros(krate, &mut resolutions);
        seed_names(krate, &mut resolutions);
        glob_names(krate, &mut resolutions);
    }
    try!(verify_paths(krate, &resolutions));
    Ok(resolutions.module_contents)
}

fn expand_macros(krate: &mut Krate, resolutions: &mut ModuleContentSets) {
    debug!("expand_macros()");
    let module_ids = krate.module_ids();
    for container_id in module_ids {
        // no point if fully expanded
        if resolutions.fully_expanded.contains(&container_id) {
            debug!("expand_macros: {:?} is fully expanded", container_id);
            continue;
        }

        debug!("expand_macros: container_id={:?}", container_id);
        let mut new_items = vec![];
        for &item_id in &krate.modules[container_id.0].items {
            match item_id {
                ItemId::MacroRef(macro_ref_id) => {
                    let macro_path = krate.macro_refs[macro_ref_id.0].path;
                    match resolutions.resolve_path(&krate.paths, container_id, macro_path) {
                        Resolution::One(ItemId::MacroDef(macro_def_id)) => {
                            debug!("expand_macros: macro traced to {:?}", macro_def_id);
                            resolutions.tick();
                            let macro_def = &krate.macro_defs[macro_def_id.0];
                            krate.macro_husks.push(MacroHusk { path: macro_path });
                            let macro_husk_id = MacroHuskId(krate.macro_husks.len() - 1);
                            new_items.extend(
                                macro_def.items.iter()
                                               .cloned()
                                               .chain(Some(ItemId::MacroHusk(macro_husk_id))));
                        }
                        _ => {
                            // don't really care about errors
                            // right now, just don't expand
                            new_items.push(item_id);
                        }
                    }
                }
                _ => {
                    new_items.push(item_id);
                }
            }
        }

        // check whether any unexpanded macro references remain
        let fully_expanded =
            new_items.iter()
                     .all(|item_id| match *item_id {
                         ItemId::MacroRef(_) => false,
                         _ => true,
                     });

        krate.modules[container_id.0].items = new_items;

        if fully_expanded {
            resolutions.mark_fully_expanded(container_id);
        }
    }
}

fn seed_names(krate: &Krate, resolutions: &mut ModuleContentSets) {
    debug!("seed_names()");
    for container_id in krate.module_ids() {
        let module = &krate.modules[container_id.0];
        debug!("seed_names: container_id={:?} module.name={:?}", container_id, module.name);
        for &item_id in &module.items {
            match item_id {
                ItemId::Module(module_id) => {
                    let module = &krate.modules[module_id.0];
                    resolutions.seed(container_id, module.name, item_id);
                }

                ItemId::Structure(structure_id) => {
                    let structure = &krate.structures[structure_id.0];
                    resolutions.seed(container_id, structure.name, item_id);
                }

                ItemId::Import(import_id) => {
                    let import = &krate.imports[import_id.0];
                    let name = krate.import_name(import_id);
                    match resolutions.resolve_path(&krate.paths, container_id, import.path) {
                        Resolution::One(target_id) => {
                            resolutions.seed(container_id, name, target_id);
                        }
                        _ => {
                            // don't care about invalid or incomplete
                            // paths right now, but we do want to
                            // insert a placeholder so globs don't
                            // take this name later
                            resolutions.placeholder(container_id, name);
                        }
                    }
                }

                ItemId::MacroDef(macro_def_id) => {
                    let macro_def = &krate.macro_defs[macro_def_id.0];
                    resolutions.seed(container_id, macro_def.name, item_id);
                }

                ItemId::Glob(_) |
                ItemId::MacroRef(_) |
                ItemId::MacroHusk(_) |
                ItemId::Code(_) => {
                }
            }
        }
    }
}

fn glob_names(krate: &Krate, resolutions: &mut ModuleContentSets) {
    let mut ticker = 0;
    while resolutions.changed_since(&mut ticker) {
        debug!("glob_names() ticker={:?} resolutions.tick={:?}", ticker, resolutions.ticker);
        for container_id in krate.module_ids() {
            let module = &krate.modules[container_id.0];
            let fully_expanded = resolutions.fully_expanded.contains(&container_id);
            debug!("glob_names: container_id={:?}, module.name={:?}, fully_expanded={:?}",
                   container_id, module.name, fully_expanded);

            for &item_id in &module.items {
                match item_id {
                    ItemId::Glob(glob_id) => {
                        let glob = &krate.globs[glob_id.0];
                        let glob_members = resolutions.resolve_glob(&krate.paths,
                                                                    container_id,
                                                                    glob.path);
                        for (name, resolution) in glob_members {
                            match resolution {
                                NameResolution::Seed(target_id @ ItemId::MacroDef(_)) |
                                NameResolution::Glob(target_id @ ItemId::MacroDef(_)) => {
                                    // for macros, glob rules do not apply
                                    resolutions.seed(container_id, name, target_id);
                                }
                                NameResolution::Seed(target_id) |
                                NameResolution::Glob(target_id) => {
                                    if fully_expanded {
                                        resolutions.glob(container_id, name, target_id);
                                    }
                                }
                                NameResolution::Placeholder => {
                                }
                            }
                        }
                    }

                    ItemId::Module(_) |
                    ItemId::Structure(_) |
                    ItemId::Import(_) |
                    ItemId::MacroDef(_) |
                    ItemId::MacroRef(_) |
                    ItemId::MacroHusk(_) |
                    ItemId::Code(_) => {
                        // Nothing to do here but verify the paths at the end.
                    }
                }
            }
        }
    }
}

fn check_path(krate: &Krate,
              resolutions: &ModuleContentSets,
              module_id: ModuleId,
              source: ItemId,
              path: PathId)
              -> Result<(), ResolutionError> {
    match resolutions.resolve_path(&krate.paths, module_id, path) {
        Resolution::One(_) => {
            // path can be successfully resolved
            Ok(())
        }
        _ => {
            // invalid or incomplete path
            Err(ResolutionError::InvalidPath {
                path: path,
                source: source,
            })
        }
    }
}

fn check_decl(_krate: &Krate,
              resolutions: &ModuleContentSets,
              module_id: ModuleId,
              source: ItemId,
              name: InternedString)
              -> Result<(), ResolutionError> {
    match resolutions.resolve_name(module_id, name) {
        Resolution::One(id) => {
            // path can be successfully resolved
            assert_eq!(id, source);
            Ok(())
        }
        _ => {
            // invalid or incomplete path
            Err(ResolutionError::MultipleNames {
                module_id: module_id,
                name: name,
            })
        }
    }
}

fn verify_paths(krate: &Krate, resolutions: &ModuleContentSets) -> Result<(), ResolutionError> {
    for container_id in krate.module_ids() {
        let module = &krate.modules[container_id.0];
        for &item_id in &module.items {
            match item_id {
                ItemId::Module(module_id) => {
                    // this declares a name `S`. Therefore, resolving the path
                    // `use self::S` should lead to this module. Check that.
                    let module = &krate.modules[module_id.0];
                    try!(check_decl(krate, resolutions, container_id, item_id, module.name));
                }
                ItemId::Structure(structure_id) => {
                    // this declares a name `S`. Therefore, resolving the path
                    // `use self::S` should lead to this structure. Check that.
                    let structure = &krate.structures[structure_id.0];
                    try!(check_decl(krate, resolutions, container_id, item_id, structure.name));
                }
                ItemId::Import(import_id) => {
                    let import = &krate.imports[import_id.0];
                    try!(check_path(krate, resolutions, container_id, item_id, import.path));
                }

                ItemId::Glob(glob_id) => {
                    let glob = &krate.globs[glob_id.0];
                    try!(check_path(krate, resolutions, container_id, item_id, glob.path));
                }

                ItemId::MacroDef(macro_def_id) => {
                    // this declares a name `S`. Therefore, resolving the path
                    // `use self::S` should lead to this macro_def. Check that.
                    let macro_def = &krate.macro_defs[macro_def_id.0];
                    try!(check_decl(krate, resolutions, container_id, item_id, macro_def.name));
                }

                ItemId::MacroHusk(macro_husk_id) => {
                    let macro_husk = &krate.macro_husks[macro_husk_id.0];
                    try!(check_path(krate, resolutions, container_id, item_id, macro_husk.path));
                }

                ItemId::MacroRef(macro_ref_id) => {
                    // this should not occur, must be an invalid path
                    let macro_ref = &krate.macro_refs[macro_ref_id.0];
                    try!(check_path(krate, resolutions, container_id, item_id, macro_ref.path));
                    unreachable!();
                }

                ItemId::Code(code_id) => {
                    let code = &krate.codes[code_id.0];
                    for &path in &code.paths {
                        try!(check_path(krate, resolutions, container_id, item_id, path));
                    }
                }
            }
        }
    }
    Ok(())
}

impl ModuleContents {
    fn new() -> ModuleContents {
        ModuleContents {
            members: HashMap::new()
        }
    }
}


impl ModuleContentSets {
    fn new() -> ModuleContentSets {
        ModuleContentSets {
            module_contents: HashMap::new(),
            fully_expanded: HashSet::new(),
            ticker: 1,
        }
    }

    fn changed_since(&self, ticker: &mut usize) -> bool {
        let t = mem::replace(ticker, self.ticker);
        self.ticker > t
    }

    fn tick(&mut self) {
        self.ticker += 1;
    }

    fn mark_fully_expanded(&mut self, module_id: ModuleId) {
        if self.fully_expanded.insert(module_id) {
            self.tick();
        }
    }

    fn module_contents(&mut self, module_id: ModuleId) -> &mut ModuleContents {
        self.module_contents.entry(module_id)
                            .or_insert_with(|| ModuleContents::new())
    }

    fn seed(&mut self,
            module_id: ModuleId,
            member_name: InternedString,
            target_id: ItemId) {
        debug!("seed(in {:?}, {:?} => {:?})", module_id, member_name, target_id);
        let changed = {
            let contents = self.module_contents(module_id);
            let members = contents.members.entry(member_name).or_insert_with(|| HashSet::new());

            // if we are seeding with this name, we can't have added a glob name yet
            assert!(!members.iter().any(|m| match *m {
                NameResolution::Glob(_) => true,
                _ => false
            }));

            members.insert(NameResolution::Seed(target_id))
        };

        if changed {
            self.tick();
        }
    }

    fn placeholder(&mut self,
                   module_id: ModuleId,
                   member_name: InternedString) {
        debug!("placeholder(in {:?}, {:?})", module_id, member_name);
        let changed = {
            let contents = self.module_contents(module_id);
            let members = contents.members.entry(member_name).or_insert_with(|| HashSet::new());

            // if we are seeding with a placeholder, we can't have added a glob name yet
            assert!(!members.iter().any(|m| match *m {
                NameResolution::Glob(_) => true,
                _ => false
            }));

            if members.is_empty() {
                members.insert(NameResolution::Placeholder)
            } else {
                false
            }
        };

        if changed {
            self.tick();
        }
    }


    fn glob(&mut self,
            module_id: ModuleId,
            member_name: InternedString,
            target_id: ItemId) {
        debug!("glob(in {:?}, {:?} => {:?})", module_id, member_name, target_id);
        let changed = {
            let contents = self.module_contents(module_id);
            let members = contents.members.entry(member_name).or_insert_with(|| HashSet::new());

            // if we have only added globs (i.e., nothing was added during
            // seed phase), we can add more globs
            if members.iter().all(|m| match *m { NameResolution::Glob(_) => true, _ => false }) {
                if members.insert(NameResolution::Glob(target_id)) {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };

        if changed {
            self.tick();
        }
    }

    /// Given that `use a::b::c::*` occurs in some module `M`, returns
    /// the pairs `(name, resolution)` that appear in `a::b::c`.
    ///
    /// - `paths`: the list of paths from the krate
    /// - `container_id`: the module M
    /// - `globbed_path`; the path `a::b::c`
    fn resolve_glob(&mut self,
                    paths: &[Path],
                    container_id: ModuleId,
                    globbed_path: PathId)
                    -> Vec<(InternedString, NameResolution)> {
        match self.resolve_path(paths, container_id, globbed_path) {
            Resolution::One(ItemId::Module(target_id)) => {
                self.module_contents(target_id)
                    .members
                    .iter()
                    .flat_map(|(&name, resolutions)| {
                        resolutions.iter()
                                   .cloned()
                                   .map(move |r| (name, r))
                    })
                    .collect()
            }
            _ => {
                vec![]
            }
        }
    }

    fn resolve_path(&self, paths: &[Path], context: ModuleId, path: PathId) -> Resolution {
        match paths[path.0] {
            Path::Root => Resolution::One(ItemId::Module(ROOT_ID)),
            Path::This => Resolution::One(ItemId::Module(context)),
            Path::Cons(base, id) => {
                match self.resolve_path(paths, context, base) {
                    Resolution::One(ItemId::Module(base_module_id)) =>
                        self.resolve_name(base_module_id, id),
                    Resolution::One(_) =>
                        // if you have a path `a::b`, `a` had better
                        // be a module.
                        Resolution::Error,
                    Resolution::Zero =>
                        Resolution::Zero,
                    Resolution::Many =>
                        Resolution::Many,
                    Resolution::Error =>
                        Resolution::Many,
                }
            }
        }
    }

    fn resolve_name(&self, module: ModuleId, name: InternedString) -> Resolution {
        match self.module_contents.get(&module) {
            None => Resolution::Zero,
            Some(rs) => match rs.members.get(&name) {
                None => Resolution::Zero,
                Some(nrs) => {
                    let mut target_ids = nrs.iter().filter_map(|nr| match *nr {
                        NameResolution::Placeholder =>
                            None,
                        NameResolution::Seed(target_id) | NameResolution::Glob(target_id) =>
                            Some(target_id),
                    });
                    match target_ids.next() {
                        None => Resolution::Zero,
                        Some(target_id) => match target_ids.next() {
                            None => Resolution::One(target_id),
                            Some(_) => Resolution::Many,
                        }
                    }
                }
            }
        }
    }
}
