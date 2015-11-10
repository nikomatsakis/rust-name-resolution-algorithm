use ast;
use std::collections::HashMap;
use std::mem;
use std::rc::Rc;
use intern::Id;

///////////////////////////////////////////////////////////////////////////
// PUBLIC INTERFACE

pub struct ProgramBindings {
    pub modules: HashMap<Id, ModuleBindings>
}

pub struct ModuleBindings {
    pub bindings: HashMap<Id, Binding>
}

pub struct Binding {
    pub privacy: ast::Privacy,
    pub item: ast::ItemIndex,
    pub kind: BindingTarget
}

#[derive(Debug,PartialEq,Clone)]
pub enum BindingTarget {
    BoundToItem(ast::ItemIndex),
    BoundRelativeToType(ast::ItemIndex, Vec<Id>),
}

use self::BindingTarget::*;

///////////////////////////////////////////////////////////////////////////
// INTERMEDIATE DATA STRUCTURES

pub struct ResolutionState<'ast> {
    ast: &'ast ast::AST,
    modules: HashMap<ast::ItemIndex, ModuleState>,
    errors: Vec<Error>,
}

#[derive(Debug)]
struct ModuleState {
    bindings: HashMap<Id, BindingState>,
}

#[derive(Debug)]
pub struct BindingState {
    precedence: Precedence,
    privacy: ast::Privacy,
    value: BindingValue,
}

#[derive(Debug)]
enum BindingValue {
    UnknownValue(Vec<BindingOption>),
    KnownValue(BindingTarget, Vec<BindingOption>),
    ErrorValue,
}

#[derive(Debug)]
pub enum Error {
    CycleError(ast::ItemIndex, ast::PathPtr),
    GlobFromNonModule(ast::ItemIndex),
    Conflict(ast::ItemIndex, Id, BindingTarget, BindingTarget),
    Unresolved(ast::ItemIndex, Id),
}

use self::Error::*;

#[derive(Debug)]
enum Precedence {
    Glob,
    Explicit,
}

#[derive(Debug,PartialEq,Clone)]
enum BindingOption {
    Definition(ast::ItemIndex),
    Redirect(Redirection)
}

#[derive(Debug,PartialEq,Clone)]
struct Redirection {
    name: Id,
    import_index: ast::ImportIndex,
    relative_to: ast::ItemIndex,
    path: ast::PathPtr
}

///////////////////////////////////////////////////////////////////////////

pub fn resolve(ast: &ast::AST) -> ResolutionState {
    let mut state = ResolutionState::new(ast);
    state.create_module_states();
    state.seed();
    state.saturate();
    state
}

impl<'a> ResolutionState<'a> {
    fn new(ast: &'a ast::AST) -> ResolutionState<'a> {
        ResolutionState {
            ast: ast,
            modules: HashMap::new(),
            errors: Vec::new()
        }
    }

    pub fn errors(&self) -> &Vec<Error> {
        return &self.errors;
    }

    fn create_module_states(&mut self) {
        for (index, item) in self.ast.items.iter().enumerate() {
            match item.kind {
                ast::ItemKind::Module(..) => { self.modules.insert(index, ModuleState::new()); }
                ast::ItemKind::Struct => { }
            }
        }
    }

    fn seed(&mut self) {
        for (index, item) in self.ast.items.iter().enumerate() {
            match item.kind {
                ast::ItemKind::Module(ref m) => { self.seed_module(index, m); }
                ast::ItemKind::Struct => { }
            }
        }
    }

    fn seed_module(
        &mut self,
        module_index: ast::ItemIndex,
        module: &ast::Module)
    {
        for &import_index in module.imports.iter() {
            self.seed_from_import(module_index, module, import_index);
        }

        for &item_index in module.members.iter() {
            self.seed_from_member(module_index, module, item_index);
        }
    }

    fn seed_from_import(
        &mut self,
        module_index: ast::ItemIndex,
        _module: &ast::Module,
        import_index: ast::ItemIndex)
    {
        let import = self.ast.import(import_index);
        match import.id {
            ast::ImportId::Glob => {
                // Ignore globs during the seed phase.
            }

            ast::ImportId::Named(name) => {
                self.add_binding_option(module_index,
                                        Precedence::Explicit,
                                        import.privacy,
                                        name,
                                        BindingOption::Redirect(
                                            Redirection::new(name,
                                                             import_index,
                                                             module_index,
                                                             import.path.clone())));
            }
        }
    }

    fn seed_from_member(
        &mut self,
        module_index: ast::ItemIndex,
        _module: &ast::Module,
        item_index: ast::ItemIndex)
    {
        let item = self.ast.item(item_index);
        match item.kind {
            ast::ItemKind::Module(..) | ast::ItemKind::Struct(..) => {
                self.add_binding_option(module_index,
                                        Precedence::Explicit,
                                        item.privacy,
                                        item.name,
                                        BindingOption::Definition(item_index));
            }
        }
    }

    fn add_binding_option(
        &mut self,
        module_index: ast::ItemIndex,
        precedence: Precedence,
        privacy: ast::Privacy,
        name: Id,
        option: BindingOption)
        -> bool
    {
        debug!("add_binding_option(module_index=`{}`, precedence={}, privacy={}, \
               name={}, option={}",
               module_index, precedence, privacy, name, option);

        let module_state = self.modules.find_mut(&module_index).unwrap();
        match module_state.bindings.find_mut(&name) {
            Some(cur_state) => {
                return match compare_precedence(cur_state.precedence, precedence) {
                    Ignore => {
                        debug!("Current state has higher precedence: {}",
                               cur_state.precedence);
                        false
                    }
                    Append => {
                        match cur_state.value {
                            BindingValue::UnknownValue(ref mut options) |
                            BindingValue::KnownValue(_, ref mut options) => {
                                debug!("Current state unknown/known: Added option");
                                if !options.contains(&option) {
                                    options.push(option);
                                    true
                                } else {
                                    false
                                }
                            }
                            BindingValue::ErrorValue => {
                                debug!("Current state error");
                                // Already some error been reported here. No need to pile on.
                                false
                            }
                        }
                    }
                };
            }
            None => { }
        }

        debug!("New binding");

        module_state.bindings.insert(
            name,
            BindingState {
                precedence: precedence,
                privacy: privacy,
                value: BindingValue::UnknownValue(vec![option])
            });

        true
    }

    fn saturate(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;
            for (index, item) in self.ast.items.iter().enumerate() {
                match item.kind {
                    ast::ItemKind::Struct => { }
                    ast::ItemKind::Module(ref module) => {
                        for &import in module.imports.iter() {
                            changed = self.saturate_from_import(index, import) || changed;
                        }
                    }
                }
            }
        }
    }

    fn saturate_from_import(&mut self,
                            module_index: ast::ItemIndex,
                            import_index: ast::ImportIndex)
                            -> bool
    {
        let import = self.ast.import(import_index);
        let redirections = self.identify_glob(module_index, import_index);
        let mut added: usize = 0;
        for redirection in redirections.move_iter() {
            if self.add_binding_option(module_index,
                                       Precedence::Glob,
                                       import.privacy,
                                       redirection.name,
                                       BindingOption::Redirect(redirection)) {
                added += 1;
            }
        }
        return added != 0;
    }

    fn identify_glob(&mut self,
                     module_index: ast::ItemIndex,
                     import_index: ast::ImportIndex)
                     -> Vec<Redirection>
    {
        debug!("module_index={} import_index={}",
               module_index, import_index);

        let import = self.ast.import(import_index);

        let module_index = match import.id {
            ast::ImportId::Named(..) => {
                return Vec::new();
            }
            ast::ImportId::Glob => {
                match self.resolve_path(module_index, import.path.clone()) {
                    ResolvedToSuccess(BoundToItem(item_index)) => {
                        if !self.ast.is_module(item_index) {
                            self.errors.push(GlobFromNonModule(item_index));
                            return Vec::new();
                        }
                        item_index
                    }
                    ResolvedToSuccess(BoundRelativeToType(type_id, _)) => {
                        // In rustc, we might want to enable this in
                        // some cases, e.g. use some::Trait::*;, but
                        // we can't in general allow it.
                        self.errors.push(GlobFromNonModule(type_id));
                        return Vec::new();
                    }
                    ResolvedToError | ResolvedToIncomplete(..) => {
                        return Vec::new();
                    }
                }
            }
        };

        let module_state = self.modules.find_mut(&module_index).unwrap();

        module_state
            .bindings
            .iter()
            .filter(|&(_, binding_state)| {
                match binding_state.privacy {
                    ast::Privacy::Public => true,
                    ast::Privacy::Private => false,
                }
            })
            .map(|(&id, _)| {
                let path = Rc::new(ast::Subpath(import.path.clone(), id));
                Redirection::new(id, import_index, module_index, path)
            })
            .collect()
    }

    pub fn check(&mut self) {
        /*!
         * At this point, we have reached a fully saturated state.
         * We can now proceed to check.
         */

        let mut to_verify = Vec::new();

        for (&module_index, module_state) in self.modules.iter() {
            for (&binding_id, binding_state) in module_state.bindings.iter() {
                match binding_state.value {
                    UnknownValue(..) => {
                        // If this value is unknown, it means that
                        // nobody has touched it yet. Until they do,
                        // we can ignore the unknown state -- it may
                        // contain ambiguities etc but that's ok for
                        // time being.
                    }
                    KnownValue(ref bound_to, ref options) => {
                        to_verify.push((module_index,
                                        binding_id,
                                        (*bound_to).clone(),
                                        (*options).clone()));
                    }
                    ErrorValue => { }
                }
            }
        }

        for &(module_index, binding_id, ref item, ref options) in to_verify.iter() {
            self.verify(module_index, binding_id, item, options);
        }
    }

    fn verify(
        &mut self,
        from_module: ast::ItemIndex,
        name: Id,
        resolves_to: &BindingTarget,
        options: &Vec<BindingOption>)
    {
        for option in options.iter() {
            let result = {
                let mut path_resolver = PathResolver::new(self);
                path_resolver.evaluate(option)
            };
            match result {
                ResolvedToError => {
                    return;
                }
                ResolvedToIncomplete(item, id) => {
                    // When dealing with multiple namespaces, we
                    // really ought not to be eager to report an
                    // incomplete resolution.  In particular, it can
                    // happen that some use declarations wind up
                    // resolving to "nothing" in the type namespace
                    // but do resolve to something in the value
                    // namespace.
                    self.errors.push(Unresolved(item, id));
                }
                ResolvedToSuccess(target) => {
                    if *resolves_to != target {
                        self.errors.push(Conflict(from_module,
                                                  name,
                                                  (*resolves_to).clone(),
                                                  target));
                    }
                }
            }
        }
    }

    pub fn resolve_path(
        &mut self,
        relative_to_module: ast::ItemIndex,
        path: ast::PathPtr)
        -> PathResolution
    {
        let mut path_resolver = PathResolver::new(self);
        path_resolver.resolve_path(relative_to_module, path)
    }
}

impl ModuleState {
    fn new() -> ModuleState {
        ModuleState {
            bindings: HashMap::new(),
        }
    }
}

impl Redirection {
    fn new(name: Id,
           import_index: ast::ImportIndex,
           module_index: ast::ItemIndex,
           path: ast::PathPtr) -> Redirection {
        Redirection { name: name, import_index: import_index,
                      relative_to: module_index, path: path }
    }
}

///////////////////////////////////////////////////////////////////////////
// Precedence

enum PrecedenceComparison {
    /// Ignore the new value.
    Ignore,

    /// Append the new value.
    Append,
}

fn compare_precedence(
    old_prec: Precedence,
    new_prec: Precedence)
    -> PrecedenceComparison
{
    match (old_prec, new_prec) {
        (Explicit, Glob) => Ignore,
        (Explicit, Explicit) => Append,
        (Glob, Explicit) => panic!("Should not happen!"),
        (Glob, Glob) => Append,
    }
}

///////////////////////////////////////////////////////////////////////////
// Path Resolution

struct PathResolver<'p, 'ast> {
    resolution_state: &'p mut ResolutionState<'ast>,
    ast: &'ast ast::AST,
    stack: Vec<ast::PathPtr>,
}

#[derive(Debug, PartialEq)]
pub enum PathResolution {
    ResolvedToSuccess(BindingTarget),
    ResolvedToError,
    ResolvedToIncomplete(ast::ItemIndex, Id),
}

use self::PathResolution::*;

impl<'a, 'ast> PathResolver<'a, 'ast> {
    fn new(resolution_state: &'a mut ResolutionState<'ast>) -> PathResolver<'a, 'ast> {
        PathResolver { resolution_state: resolution_state,
                       ast: resolution_state.ast,
                       stack: Vec::new() }
    }

    fn resolve_path(
        &mut self,
        relative_to_module: ast::ItemIndex,
        path: ast::PathPtr)
        -> PathResolution
    {
        debug!("resolve_path(relative_to_module=`{}`, path={})",
               relative_to_module, path);

        if self.stack.contains(&path) {
            self.resolution_state.errors.push(CycleError(relative_to_module, path));
            return ResolvedToError;
        }

        self.stack.push(path.clone());

        match *path {
            ast::Root(name) => {
                self.lookup(self.ast.root_index(), name)
            }
            ast::Self(name) => {
                self.lookup(relative_to_module, name)
            }
            ast::Subpath(ref base_path, name) => {
                match self.resolve_path(relative_to_module, (*base_path).clone()) {
                    ResolvedToError => {
                        ResolvedToError
                    }
                    ResolvedToIncomplete(base_item, name) => {
                        ResolvedToIncomplete(base_item, name)
                    }
                    ResolvedToSuccess(BoundRelativeToType(base_item, mut names)) => {
                        names.push(name);
                        ResolvedToSuccess(BoundRelativeToType(base_item, names))
                    }
                    ResolvedToSuccess(BoundToItem(base_item)) => {
                        self.lookup(base_item, name)
                    }
                }
            }
        }
    }

    fn lookup(
        &mut self,
        relative_to: ast::ItemIndex,
        name: Id)
        -> PathResolution
    {
        let options = match self.lookup_options(relative_to, name) {
            Resolved(r) => { return r; }
            ResolvedToUnknown(o) => { o }
        };

        // If we get here, then we are in an "unknown" state in which
        // there are (potentially) multiple binding options. We need
        // to figure out which of those apply. The weird (and
        // annoying) thing is that some of these paths may wind up
        // evaluating to values and not types (which are effectively
        // empty). Perhaps the easiest way to handle this is
        // just...ignore it.  We'll just look for any successes and
        // ignore failures for the time being.

        let evaluations: Vec<PathResolution> =
            options.iter().map(|o| self.evaluate(o)).collect();
        match evaluations.move_iter().find(|e| e.is_successful()) {
            Some(ResolvedToSuccess(index)) => {
                let module_state = self.resolution_state.modules.find_mut(&relative_to).unwrap();
                let binding_state = module_state.bindings.find_mut(&name).unwrap();
                binding_state.value = KnownValue(index.clone(), options);
                return ResolvedToSuccess(index);
            }
            _ => {
                // If there is no success, just close our eyes and
                // call it incomplete.
                return ResolvedToIncomplete(relative_to, name);
            }
        }
    }

    fn evaluate(
        &mut self,
        option: &BindingOption)
        -> PathResolution
    {
        match *option {
            Definition(item_index) => {
                return ResolvedToSuccess(BoundToItem(item_index));
            }
            Redirect(ref r) => {
                return self.resolve_path(r.relative_to, r.path.clone());
            }
        }
    }

    fn lookup_options(
        &mut self,
        relative_to: ast::ItemIndex,
        name: Id)
        -> OptionsLookup
    {
        match self.ast.item(relative_to).kind {
            ast::Struct => {
                return Resolved(ResolvedToSuccess(BoundRelativeToType(relative_to, vec![name])));
            }
            ast::Module(..) => { }
        }
        let module_state = self.resolution_state.modules.find_mut(&relative_to).unwrap();
        let binding_state = match module_state.bindings.find_mut(&name) {
            None => { return Resolved(ResolvedToIncomplete(relative_to, name)); }
            Some(b) => b,
        };
        match binding_state.value {
            UnknownValue(..) => { }
            KnownValue(ref index, _) => { return Resolved(ResolvedToSuccess((*index).clone())); }
            ErrorValue => { return Resolved(ResolvedToError); }
        }
        let value = mem::replace(&mut binding_state.value, ErrorValue);
        match value {
            UnknownValue(options) => { return ResolvedToUnknown(options); }
            KnownValue(..) | ErrorValue => { panic!("Impossible"); }
        }
    }
}

enum OptionsLookup {
    Resolved(PathResolution),
    ResolvedToUnknown(Vec<BindingOption>)
}

impl PathResolution {
    fn is_successful(&self) -> bool {
        match *self {
            ResolvedToSuccess(_) => true,
            _ => false
        }
    }
}
