use std::collections::HashMap;
use ast;
use intern::Id;

///////////////////////////////////////////////////////////////////////////
// PUBLIC INTERFACE

pub type ModuleMap = HashMap<Id, Binding>;

pub struct Binding {
    pub privacy: ast::Privacy,
    pub item: ast::ItemIndex,
}

///////////////////////////////////////////////////////////////////////////
// INTERMEDIATE DATA STRUCTURES

pub struct ResolutionState<'ast> {
    ast: &'ast ast::AST,
    modules: HashMap<ast::ItemIndex, ModuleState>,
    verifications: Vec<Verification>,
}

#[deriving(Show)]
struct ModuleState {
    bindings: HashMap<Id, BindingState>,
    errors: HashMap<Id, Vec<ast::Item>>
}

#[deriving(Show)]
pub struct BindingState {
    precedence: Precedence,
    privacy: ast::Privacy,
    value: BindingValue,
}

#[deriving(Show)]
enum BindingValue {
    Unknown(Vec<BindingOption>),
    Known(ast::ItemIndex),
    Error,
}

#[deriving(Show)]
enum Precedence {
    Glob,
    Explicit,
}

#[deriving(Show)]
enum BindingOption {
    ExplicitUse(Id),
    Definition(ast::ItemIndex),
    Redirect(Redirection)
}

#[deriving(Show)]
struct Redirection {
    import_index: ast::ImportIndex,
    path: ast::PathPtr
}

#[deriving(Show)]
struct Verification {
    module_index: ast::ItemIndex,
    name: Id,
    option: BindingOption,
    should_resolve_to: ast::ItemIndex
}

///////////////////////////////////////////////////////////////////////////

pub fn resolve(ast: &ast::AST) -> ResolutionState {
    let mut state = ResolutionState::new(ast);
    state.create_module_states();
    state.seed();
    state.saturate();
    state.check();
    state
}

impl<'a> ResolutionState<'a> {
    fn new(ast: &'a ast::AST) -> ResolutionState<'a> {
        ResolutionState {
            ast: ast,
            modules: HashMap::new(),
            verifications: Vec::new()
        }
    }

    fn create_module_states(&mut self) {
        for (index, item) in self.ast.items.iter().enumerate() {
            match item.kind {
                ast::Module(..) => { self.modules.insert(index, ModuleState::new()); }
                ast::Struct => { }
            }
        }
    }

    fn seed(&mut self) {
        for (index, item) in self.ast.items.iter().enumerate() {
            match item.kind {
                ast::Module(ref m) => { self.seed_module(index, m); }
                ast::Struct => { }
            }
        }
    }

    fn seed_module(&mut self, module_index: ast::ItemIndex, module: &ast::Module) {
        for &import_index in module.imports.iter() {
            self.seed_from_import(module_index, module, import_index);
        }

        for &item_index in module.members.iter() {
            self.seed_from_member(module_index, module, item_index);
        }
    }

    fn seed_from_import(&mut self,
                        module_index: ast::ItemIndex,
                        module: &ast::Module,
                        import_index: ast::ItemIndex) {
        let import = self.ast.import(import_index);
        match import.id {
            ast::Glob => {
                // Ignore globs during the seed phase.
            }

            ast::Named(name) => {
                self.add_binding_option(module_index,
                                        Explicit,
                                        import.privacy,
                                        name,
                                        Redirect(Redirection::new(import_index,
                                                                  import.path.clone())));
            }
        }
    }

    fn seed_from_member(&mut self,
                        module_index: ast::ItemIndex,
                        module: &ast::Module,
                        item_index: ast::ItemIndex) {
        let item = self.ast.item(item_index);
        self.add_binding_option(module_index,
                                Explicit,
                                item.privacy,
                                item.name,
                                Definition(item_index));
    }

    fn add_binding_option(&mut self,
                          module_index: ast::ItemIndex,
                          precedence: Precedence,
                          privacy: ast::Privacy,
                          name: Id,
                          option: BindingOption) {
        debug!("add_binding_option(module_index=`{}`, precedence={}, privacy={}, \
               name={}, option={}",
               module_index, precedence, privacy, name, option);

        let module_state = self.modules.find_mut(&module_index).unwrap();
        match module_state.bindings.find_mut(&name) {
            Some(cur_state) => {
                match compare_precedence(cur_state.precedence, precedence) {
                    Ignore => {
                        debug!("Current state has higher precedence: {}",
                               cur_state.precedence);
                    }
                    Append => {
                        match cur_state.value {
                            Unknown(ref mut options) => {
                                debug!("Current state unknown: Added option");
                                options.push(option);
                            }
                            Known(item_index) => {
                                debug!("Current state known: {}", item_index);
                                self.verifications.push(Verification::new(module_index, name,
                                                                          option, item_index));
                            }
                            Error => {
                                debug!("Current state error");
                                // Already some error been reported here. No need to pile on.
                            }
                        }
                    }
                }
                return;
            }
            None => { }
        }

        debug!("New binding");

        module_state.bindings.insert(
            name,
            BindingState {
                precedence: precedence,
                privacy: privacy,
                value: Unknown(vec![option])
            });
    }

    fn saturate(&mut self) {
    }

    fn check(&mut self) {
    }
}

impl ModuleState {
    fn new() -> ModuleState {
        ModuleState {
            bindings: HashMap::new(),
            errors: HashMap::new()
        }
    }
}

impl Redirection {
    fn new(i: ast::ImportIndex, p: ast::PathPtr) -> Redirection {
        Redirection { import_index: i, path: p }
    }
}

impl Verification {
    fn new(module_index: ast::ItemIndex,
           name: Id,
           option: BindingOption,
           item: ast::ItemIndex)
           -> Verification
    {
        Verification {
            module_index: module_index, name:name, option: option, should_resolve_to: item
        }
    }
}

enum PrecedenceComparison {
    /// Ignore the new value.
    Ignore,

    /// Append the new value.
    Append,
}

fn compare_precedence(old_prec: Precedence,
                      new_prec: Precedence)
                      -> PrecedenceComparison
{
    match (old_prec, new_prec) {
        (Explicit, Glob) => Ignore,
        (Explicit, Explicit) => Append,
        (Glob, Explicit) => fail!("Should not happen!"),
        (Glob, Glob) => Append,
    }
}
