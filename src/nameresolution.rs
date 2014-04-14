use collections::hashmap::HashMap;
use std::rc::Rc;
use ast;
use intern::Id;

type ModuleMap = HashMap<Id, BindingPtr>;

enum Binding {
    ExplicitBinding(ExplicitBinding),
    GlobBinding(GlobBinding)
}

type BindingPtr = Rc<Binding>;

#[deriving(Clone)]
enum ExplicitBinding {
    ExplicitUse(ast::UseKind, ast::PathPtr),
    ExplicitItem(ast::ItemIndex),
}

type ExplicitBindingPtr = Rc<ExplicitBinding>;

#[deriving(Clone)]
struct GlobBinding {
    use_index: ast::UseIndex,
    use_kind: ast::UseKind,
    path: ast::PathPtr,
}

type GlobBindingPtr = Rc<GlobBinding>;

impl Binding {
    fn is_export(&self) -> bool {
        match *self {
            ExplicitBinding(ExplicitUse(ast::PubUse, _)) |
            ExplicitBinding(ExplicitItem(_)) |
            GlobBinding(GlobBinding { use_kind: ast::PubUse, .. }) => {
                true
            }

            ExplicitBinding(ExplicitUse(ast::ImportUse, _)) |
            GlobBinding(GlobBinding { use_kind: ast::ImportUse, .. }) => {
                false
            }
        }
    }
}

// Bindings

pub struct Bindings {
    root_index: uint,
    module_maps: HashMap<ast::ItemIndex, ModuleMap>,
}

#[deriving(Eq,Show)]
pub enum PathResolution {
    ResolvedToItem(ast::ItemIndex),
    ResolvedToNothing,
}

impl Bindings {
    pub fn new(ast: &ast::AST) -> Fallible<Bindings> {
        let mut result = Bindings { root_index: ast.root_index(),
                                    module_maps: HashMap::new() };
        {
            let mut cx = ResolutionContext { ast: ast, bindings: &mut result };

            debug!("About to seed");
            try!(cx.seed());

            debug!("About to saturate");
            try!(cx.saturate());
        }
        Ok(result)
    }

    fn insert(&mut self, mod_id: ast::ItemIndex, id: Id, binding: BindingPtr) {
        let module_map = self.module_maps.find_or_insert_with(mod_id, |_| HashMap::new());
        module_map.insert(id, binding);
    }

    fn lookup<'a>(&'a self, mod_id: ast::ItemIndex, id: Id) -> Option<&'a BindingPtr> {
        match self.module_maps.find(&mod_id) {
            None => {
                None
            }

            Some(module_map) => {
                module_map.find(&id)
            }
        }
    }

    fn lookup_all<'a>(&'a self, mod_id: ast::ItemIndex) -> Option<&'a ModuleMap> {
        self.module_maps.find(&mod_id)
    }

    pub fn resolve_path(&self,
                        mod_id: ast::ItemIndex,
                        path: &ast::PathPtr)
                        -> Fallible<PathResolution> {
        let mut map = HashMap::new();
        self.resolve_path_after_checking_for_cycles(mod_id, path, &mut map)
    }

    pub fn resolve_path_from_root(&self,
                                  path: &ast::PathPtr)
                                  -> Fallible<PathResolution> {
        self.resolve_path(self.root_index, path)
    }

    pub fn resolve_path_from(&self,
                             mod_path: &ast::PathPtr,
                             path: &ast::PathPtr)
                             -> Fallible<PathResolution>
    {
        match try!(self.resolve_path_from_root(mod_path)) {
            ResolvedToNothing => Ok(ResolvedToNothing),
            ResolvedToItem(mod_id) => {
                self.resolve_path(mod_id, path)
            }
        }
    }

    fn resolve_path_after_checking_for_cycles(&self,
                                              mod_id: ast::ItemIndex,
                                              path: &ast::PathPtr,
                                              visited: &mut HashMap<ast::PathPtr,()>)
                                              -> Fallible<PathResolution> {
        // Watch out for cyclic path resolution.
        if !visited.insert((*path).clone(), ()) {
            Err(Cycle((*path).clone()))
        } else {
            self.resolve_path_no_cycle_check(mod_id, path, visited)
        }
    }

    fn resolve_path_no_cycle_check(&self,
                                   mod_id: ast::ItemIndex,
                                   path: &ast::PathPtr,
                                   visited: &mut HashMap<ast::PathPtr,()>)
                                   -> Fallible<PathResolution> {
        match **path {
            ast::Root(id) => self.search_bindings(true, self.root_index, id, visited),
            ast::Self(id) => self.search_bindings(false, mod_id, id, visited),
            ast::Subpath(ref base_path, id) => {
                match try!(self.resolve_path_no_cycle_check(mod_id, base_path, visited)) {
                    ResolvedToNothing => Ok(ResolvedToNothing),
                    ResolvedToItem(item_index) => {
                        self.search_bindings(true, item_index, id, visited)
                    }
                }
            }
        }
    }

    fn search_bindings(&self,
                       pub_use_only: bool,
                       mod_id: ast::ItemIndex,
                       id: Id,
                       visited: &mut HashMap<ast::PathPtr,()>)
                       -> Fallible<PathResolution> {
        match self.module_maps.find(&mod_id) {
            None => {
                Ok(ResolvedToNothing)
            }

            Some(module_map) => {
                match module_map.find_copy(&id) {
                    None => {
                        Ok(ResolvedToNothing)
                    }
                    Some(r) => {
                        match *r {
                            ExplicitBinding(ExplicitItem(id)) => {
                                Ok(ResolvedToItem(id))
                            }
                            GlobBinding(GlobBinding { use_kind: ast::PubUse,
                                                      path: ref path, .. }) |
                            ExplicitBinding(ExplicitUse(ast::PubUse, ref path)) => {
                                self.resolve_path_after_checking_for_cycles(mod_id, path, visited)
                            }
                            GlobBinding(GlobBinding { use_kind: ast::ImportUse,
                                                      path: ref path, .. }) |
                            ExplicitBinding(ExplicitUse(ast::ImportUse, ref path)) => {
                                if pub_use_only {
                                    Ok(ResolvedToNothing)
                                } else {
                                    self.resolve_path_after_checking_for_cycles(mod_id, path, visited)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

//

pub enum ResolutionError {
    Cycle(ast::PathPtr),
    DoubleBinding(ExplicitBindingPtr, ExplicitBindingPtr),
    AmbiguousBinding(GlobBindingPtr, GlobBindingPtr),
}

type Fallible<T> = Result<T, ResolutionError>;

struct ResolutionContext<'a> {
    ast: &'a ast::AST,
    bindings: &'a mut Bindings
}

impl<'ast> ResolutionContext<'ast> {
    fn seed(&mut self) -> Fallible<()> {
        println!("seed");
        let ast = self.ast;
        for mod_id in range(0, self.ast.items.len()) {
            match ast.items[mod_id] {
                ast::Struct(_) => { }
                ast::Module(ref m) => {
                    try!(self.seed_uses(mod_id, m));
                    try!(self.seed_members(mod_id, m));
                }
            }
        }
        Ok(())
    }

    fn seed_uses(&mut self,
                 mod_id: ast::ItemIndex,
                 m: &ast::Module)
                 -> Fallible<()> {
        for &use_index in m.uses.iter() {
            let u = &self.ast.uses[use_index];
            match u.id {
                ast::Glob => { }
                ast::Named(id) => {
                    let binding = Rc::new(
                        ExplicitBinding(ExplicitUse(u.kind,
                                                    u.path.clone())));
                    try!(self.insert_binding_if_necessary(mod_id, id, binding));
                }
            }
        }
        Ok(())
    }

    fn seed_members(&mut self,
                    mod_id: ast::ItemIndex,
                    m: &ast::Module)
                    -> Fallible<()> {
        let ast = self.ast;
        for &index in m.members.iter() {
            let id = match self.ast.items[index] {
                ast::Module(ref m) => m.id,
                ast::Struct(ref s) => s.id
            };
            let binding = Rc::new(ExplicitBinding(ExplicitItem(index)));
            try!(self.insert_binding_if_necessary(mod_id, id, binding));
        }
        Ok(())
    }

    fn insert_binding_if_necessary(&mut self,
                                   mod_id: ast::ItemIndex,
                                   id: Id,
                                   new_binding: BindingPtr)
                                   -> Fallible<()> {
        match try!(self.compare_binding(mod_id, id, new_binding)) {
            None => { }
            Some(new_binding) => {
                self.insert_binding_unconditionally(mod_id, id, new_binding)
            }
        }
        Ok(())
    }

    /**
     * Compares the existing binding for `mod_id::id` (if any)
     * to the new binding `new_binding`. Returns an error if it is
     * illegal to have both bindings simultaneously. Otherwise,
     * returns `Some(x)` if the existing binding should be replaced by
     * `x` or `None` if the existing binding is fine.
     */
    fn compare_binding(&self,
                       mod_id: ast::ItemIndex,
                       id: Id,
                       new_binding: BindingPtr)
                       -> Fallible<Option<BindingPtr>> {
        match self.bindings.lookup(mod_id, id) {
            None => {
                Ok(Some(new_binding))
            }

            Some(old_binding) => {
                combine_binding(old_binding, &new_binding)
            }
        }
    }

    fn insert_binding_unconditionally(&mut self,
                                      mod_id: ast::ItemIndex,
                                      id: Id,
                                      new_binding: BindingPtr) {
        self.bindings.insert(mod_id, id, new_binding);
    }

    fn saturate(&mut self) -> Fallible<()> {
        let mut new_bindings = ~[];
        loop {
            let ast = self.ast.clone();
            for mod_id in range(0, self.ast.items.len()) {
                match ast.items[mod_id] {
                    ast::Struct(_) => { }
                    ast::Module(ref m) => {
                        try!(self.saturate_uses(mod_id, m, &mut new_bindings));
                    }
                }
            }

            if new_bindings.len() == 0 {
                return Ok(());
            }

            loop {
                match new_bindings.pop() {
                    None => break,
                    Some((mod_id, id, binding)) => {
                        debug!("New binding: {} {}", mod_id, id);
                        self.insert_binding_unconditionally(mod_id, id, binding);
                    }
                }
            }
        }
    }

    fn saturate_uses(&mut self,
                     mod_id: ast::ItemIndex,
                     m: &ast::Module,
                     new_bindings: &mut ~[(ast::ItemIndex, Id, BindingPtr)])
                     -> Fallible<()> {
        for &use_index in m.uses.iter() {
            let u = &self.ast.uses[use_index];

            // Just select for Glob imports.
            match u.id {
                ast::Glob => { }
                ast::Named(id) => { continue; }
            }

            // This is either a USE PATH :: * or PUB USE PATH :: *.
            // We need to (1) resolve PATH against the current state
            //            (2) identify all of its current exported bindings
            //            (3) add them to our local exporting bindings
            match try!(self.bindings.resolve_path(mod_id, &u.path)) {
                ResolvedToNothing => { }
                ResolvedToItem(index) => {
                    match self.bindings.lookup_all(index) {
                        None => { }
                        Some(module_map) => {
                            try!(reexport_module_bindings(
                                self, mod_id, use_index,
                                module_map, new_bindings))
                        }
                    }
                }
            }
        }
        return Ok(());

        fn reexport_module_bindings(this: &ResolutionContext,
                                    mod_id: ast::ItemIndex,
                                    use_id: ast::UseIndex,
                                    from_module_map: &ModuleMap,
                                    new_bindings: &mut ~[(ast::ItemIndex, Id, BindingPtr)]) -> Fallible<()> {
            /*!
             * For each item X exported by `from_module_map`, create an
             * export X in `mod_id`.
             */

            let u = &this.ast.uses[use_id];
            for (&id, binding) in from_module_map.iter() {
                if binding.is_export() {
                    let path = Rc::new(ast::Subpath(u.path.clone(), id));
                    let binding = Rc::new(GlobBinding(GlobBinding {
                        use_index: use_id,
                        use_kind: u.kind,
                        path: path
                    }));
                    match try!(this.compare_binding(mod_id, id, binding)) {
                        None => { }
                        Some(new_binding) => {
                            new_bindings.push((mod_id, id, new_binding));
                        }
                    }
                }
            }
            Ok(())
        }
    }
}

/**
 * Given that we've already added the binding `old`, adjust for
 * the new binding `new`.
 */
fn combine_binding(old: &BindingPtr,
                   new: &BindingPtr)
                   -> Fallible<Option<BindingPtr>> {
    match (&**old, &**new) {
        // Let explicit bindings have precedence over globs.
        (&ExplicitBinding(_), &GlobBinding(_)) => Ok(None),
        (&GlobBinding(_), &ExplicitBinding(_)) => Ok(Some((*new).clone())),

        // Two explicit bindings is an eager error.
        (&ExplicitBinding(ref o), &ExplicitBinding(ref n)) => {
            Err(DoubleBinding(Rc::new((*o).clone()), Rc::new((*n).clone())))
        }

        // Two glob bindings to the same id are an error, unless
        // they originate from the same use statement.
        (&GlobBinding(ref o), &GlobBinding(ref n)) => {
            if o.use_index == n.use_index {
                Ok(None)
            } else {
                Err(AmbiguousBinding(Rc::new((*o).clone()),
                                     Rc::new((*n).clone())))
            }
        }
    }
}

