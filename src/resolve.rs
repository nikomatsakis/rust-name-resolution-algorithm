use ast::*;
use intern::InternedString;
use std::collections::{HashMap, HashSet};
use std::mem;

#[derive(Debug)]
pub struct ModuleContentSets {
    pub module_contents: HashMap<ModuleId, ModuleContents>,
}

#[derive(Clone, Debug)]
pub struct ModuleContents {
    pub members: HashMap<InternedString, Vec<NameResolution>>,
}

#[derive(Copy, Clone, Debug)]
pub struct NameResolution {
    /// Item that name maps to.
    pub target: ItemId,

    /// Source of mapping.
    pub source: ItemId,
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

pub fn resolve_and_expand(krate: &mut Krate) -> Result<ModuleContentSets, ResolutionError> {
    let mut resolutions = ModuleContentSets::new();
    let mut changed = true;
    while mem::replace(&mut changed, false) {
        propagate_names(krate, &mut resolutions);
        changed |= expand_macros(krate, &resolutions);
    }
    try!(verify_paths(krate, &resolutions));
    Ok(resolutions)
}

fn propagate_names(krate: &Krate, resolutions: &mut ModuleContentSets) {
    let mut changed = true;
    while mem::replace(&mut changed, false) {
        for container_id in krate.module_ids() {
            let module = &krate.modules[container_id.0];
            debug!("propagate_names({:?}, {:?})", container_id, module.name);
            for &item_id in &module.items {
                match item_id {
                    ItemId::Module(module_id) => {
                        let module = &krate.modules[module_id.0];
                        changed |= resolutions.add(container_id, module.name, item_id, item_id)
                    }

                    ItemId::Structure(structure_id) => {
                        let structure = &krate.structures[structure_id.0];
                        changed |= resolutions.add(container_id, structure.name, item_id, item_id)
                    }

                    ItemId::Import(import_id) => {
                        let import = &krate.imports[import_id.0];
                        match resolutions.resolve_path(&krate.paths, container_id, import.path) {
                            Resolution::One(target_id) => {
                                let name = import.alt_name.unwrap_or_else(|| {
                                    match krate.paths[import.path.0] {
                                        Path::Root | Path::This => unreachable!(),
                                        Path::Cons(_, s) => s,
                                    }
                                });
                                changed |= resolutions.add(container_id, name, target_id, item_id);
                            }
                            _ => {
                                // don't care about invalid or incomplete paths right now
                            }
                        }
                    }

                    ItemId::Glob(glob_id) => {
                        let glob = &krate.globs[glob_id.0];
                        match resolutions.resolve_path(&krate.paths, container_id, glob.path) {
                            Resolution::One(ItemId::Module(target_id)) => {
                                let contents = resolutions.module_contents(target_id).clone();
                                for (name, module_resolutions) in contents.members {
                                    for resolution in module_resolutions {
                                        changed |= resolutions.add(container_id,
                                                                   name,
                                                                   resolution.target,
                                                                    item_id);
                                    }
                                }
                            }
                            _ => {
                                // don't care about invalid or incomplete paths right now
                            }
                        }
                    }

                    ItemId::MacroDef(macro_def_id) => {
                        let macro_def = &krate.macro_defs[macro_def_id.0];
                        changed |= resolutions.add(container_id, macro_def.name, item_id, item_id)
                    }

                    ItemId::MacroRef(_) => {
                        // This will (maybe) get expanded in the next phase.
                    }

                    ItemId::Code(_) => {
                        // Nothing to do here but verify the paths at the end.
                    }
                }
            }
        }
    }
}

fn expand_macros(krate: &mut Krate, resolutions: &ModuleContentSets) -> bool {
    let mut expanded = false;

    let module_ids = krate.module_ids();
    let macro_refs = &krate.macro_refs;
    let macro_defs = &krate.macro_defs;
    let paths = &krate.paths;
    let modules = &mut krate.modules;

    for container_id in module_ids {
        let module = &mut modules[container_id.0];
        let new_items: Vec<ItemId> =
            module.items
                  .iter()
                  .cloned()
                  .flat_map(|item_id| match item_id {
                      ItemId::MacroRef(macro_ref_id) => {
                          let macro_path = macro_refs[macro_ref_id.0].path;
                          match resolutions.resolve_path(paths, container_id, macro_path) {
                              Resolution::One(ItemId::MacroDef(macro_def_id)) => {
                                  let macro_def = &macro_defs[macro_def_id.0];
                                  expanded = true;
                                  macro_def.items.clone()
                              }
                              _ => {
                                  // don't really care about errors
                                  // right now, just don't expand
                                  vec![item_id]
                              }
                          }
                      }
                      _ => {
                          vec![item_id]
                      }
                  })
                  .collect();
            module.items = new_items;
    }
    expanded
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

fn check_decl(krate: &Krate,
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
            module_contents: HashMap::new()
        }
    }

    fn module_contents(&mut self, module_id: ModuleId) -> &mut ModuleContents {
        self.module_contents.entry(module_id)
                            .or_insert_with(|| ModuleContents::new())
    }

    fn add(&mut self,
           module_id: ModuleId,
           member_name: InternedString,
           member_id: ItemId,
           source_id: ItemId)
           -> bool {
        debug!("add({:?}, {:?}, {:?}, {:?})", module_id, member_name, member_id, source_id);
        let contents = self.module_contents(module_id);
        let members = contents.members.entry(member_name).or_insert_with(|| vec![]);
        if !members.iter().any(|m| m.target == member_id) {
            members.push(NameResolution { target: member_id, source: source_id });
            true
        } else {
            false
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
                Some(nrs) => if nrs.len() == 1 {
                    Resolution::One(nrs[0].target)
                } else {
                    Resolution::Many
                }
            },
        }
    }
}
