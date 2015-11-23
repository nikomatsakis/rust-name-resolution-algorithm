use ast::*;
use intern::InternedString;
use std::collections::{HashMap, HashSet};
use std::mem;

#[cfg(test)]
mod test;

struct PathResolver<'k> {
    krate: &'k Krate,
    breadcrumbs: HashSet<(ModuleId, PathId)>,
}

type Resolution = Result<Option<ItemId>, Error>;

#[derive(Debug)]
pub enum Error {
    NonModule(ItemId),
    Ambiguity(ModuleId, InternedString),
}

impl<'k> PathResolver<'k> {
    pub fn new(krate: &'k Krate) -> PathResolver<'k> {
        PathResolver { krate: krate, breadcrumbs: HashSet::new() }
    }

    fn resolve_path(&mut self, context: ModuleId, path: PathId) -> Resolution {
        println!("resolve_path(context={:?}, path={:?})", context, path);
        if self.breadcrumbs.insert((context, path)) {
            let result = match self.krate.paths[path.0] {
                Path::Root => Ok(Some(ItemId::Module(ROOT_ID))),
                Path::This => Ok(Some(ItemId::Module(context))),
                Path::Cons(base, id) => {
                    match try!(self.resolve_path(context, base)) {
                        None => Ok(None),
                        Some(ItemId::Module(base_module_id)) =>
                            self.resolve_name(base_module_id, id),
                        Some(item_id) =>
                            Err(Error::NonModule(item_id))
                    }
                }
            };
            self.breadcrumbs.remove(&(context, path));
            result
        } else {
            Ok(None) // cycle => not an error, but no match
        }
    }

    fn resolve_name(&mut self, module: ModuleId, name: InternedString) -> Resolution {
        match try!(self.resolve_high(module, name)) {
            Some(i) => Ok(Some(i)),
            None => self.resolve_low(module, name)
        }
    }

    fn resolve_high(&mut self, module_id: ModuleId, name: InternedString) -> Resolution {
        println!("resolve_high(module_id={:?}, name={:?})", module_id, name);
        let if_name = |item_name, item_id| {
            println!("if_name(item_name={:?}, name={:?})", item_name, name);
            if item_name == name {
                Some(item_id)
            } else {
                None
            }
        };
        let mut matches: HashSet<ItemId> =
            self.krate
                .modules[module_id.0]
                .items
                .iter()
                .flat_map(|&item_id| {
                    println!("item_id={:?}", item_id);
                    match item_id {
                        ItemId::Module(module_id) =>
                            if_name(self.krate.modules[module_id.0].name, item_id),
                        ItemId::Structure(structure_id) =>
                            if_name(self.krate.structures[structure_id.0].name, item_id),
                        ItemId::MacroDef(structure_id) =>
                            if_name(self.krate.macro_defs[structure_id.0].name, item_id),
                        ItemId::Import(import_id) =>
                            if_name(self.krate.import_name(import_id), item_id),
                        ItemId::Glob(_) |
                        ItemId::MacroRef(_) |
                        ItemId::MacroHusk(_) |
                        ItemId::Code(_) =>
                            None,
                    }
                })
                .collect();
        println!("module_id={:?} matches={:?}", module_id, matches);
        if matches.len() == 0 {
            Ok(None)
        } else if matches.len() > 1 {
            Err(Error::Ambiguity(module_id, name))
        } else {
            // this is wrong, will produce false ambiguities, we
            // should evaluate the path first
            match matches.into_iter().next().unwrap() {
                ItemId::Import(import_id) => {
                    let path = self.krate.imports[import_id.0].path;
                    self.resolve_path(module_id, path)
                }
                item_id => {
                    Ok(Some(item_id))
                }
            }
        }
    }

    fn resolve_low(&mut self, module_id: ModuleId, name: InternedString) -> Resolution {
        let globs =
            self.krate.modules[module_id.0]
            .items
            .iter()
            .flat_map(|&item_id| {
                match item_id {
                    ItemId::Glob(glob_id) => Some(glob_id),
                    _ => None
                }
            });
        let mut m = None;
        for glob_id in globs {
            let glob_path = self.krate.globs[glob_id.0].path;
            match try!(self.resolve_path(module_id, glob_path)) {
                Some(ItemId::Module(module_id)) => {
                    match try!(self.resolve_name(module_id, name)) {
                        Some(item_id) => {
                            if m.is_none() {
                                m = Some(item_id);
                            } else if m != Some(item_id) {
                                return Err(Error::Ambiguity(module_id, name));
                            }
                        }
                        None => { }
                    }
                }
                Some(item_id) => {
                    return Err(Error::NonModule(item_id));
                }
                None => { }
            }
        }
        Ok(m)
    }
}

