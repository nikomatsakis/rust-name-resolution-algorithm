use ast;
use intern::Id;
use parse::*;
use std::cell::RefCell;
use std::rc::Rc;

///////////////////////////////////////////////////////////////////////////
// Context data which must be installed to do a parse

local_data_key!(the_ast: RefCell<ast::AST>)

///////////////////////////////////////////////////////////////////////////
// Public entry points

pub fn parse_ast(text: &str) -> ast::AST {
    let text_bytes = text.as_bytes();
    the_ast.replace(Some(RefCell::new(
        ast::AST { items: Vec::new(), uses: Vec::new() })));
    let grammar = Grammar::new();
    let m = match parse(&grammar, text_bytes, &Module()) {
        Ok(m) => m,
        Err(byte) => {
            fail!("Unexpected parse error for Program: {} (*) {}",
                  // assumes ascii
                  text.slice_to(byte),
                  text.slice_from(byte))
        }
    };
    let mut ast = the_ast.replace(None).unwrap().unwrap();
    ast.items.push(ast::Module(m));
    ast
}

pub fn parse_path(text: &str) -> ast::PathPtr {
    let text_bytes = text.as_bytes();
    let grammar = Grammar::new();
    match parse(&grammar, text_bytes, &Path()) {
        Ok(m) => m,
        Err(byte) => {
            fail!("Unexpected parse error for Path: {} (*) {}",
                  // assumes ascii
                  text.slice_to(byte),
                  text.slice_from(byte))
        }
    }
}

///////////////////////////////////////////////////////////////////////////
//

struct Grammar {
    module: GParser<ast::Module>,
}

impl Grammar {
    fn new() -> Grammar {
        Grammar { module: MakeModule() }
    }
}

type GParser<T> = Parser<Grammar, T>;

///////////////////////////////////////////////////////////////////////////
// Misc. keywords

pub fn SelfKw<G>() -> Parser<G,()> {
    Token("self", is_not_ident_cont)
}

pub fn ModKw<G>() -> Parser<G,()> {
    Token("mod", is_not_ident_cont)
}

pub fn StructKw<G>() -> Parser<G,()> {
    Token("struct", is_not_ident_cont)
}

pub fn PubKw<G>() -> Parser<G,()> {
    Token("pub", is_not_ident_cont)
}

pub fn UseKw<G>() -> Parser<G,()> {
    Token("use", is_not_ident_cont)
}

///////////////////////////////////////////////////////////////////////////
// Path
//
// One of the following
//    id::...::id
//    self::...::id

fn Path() -> GParser<ast::PathPtr> {
    return Choice(vec![
        SelfKw()
            .thenr(ColonColon().thenr(Ident()))
            .then(ColonColon().thenr(Ident()).rep(0))
            .map(relative),
        Ident().then(ColonColon().thenr(Ident()).rep(0)).map(absolute)
    ]);

    fn relative((id, ids): (Id, Vec<Id>)) -> ast::PathPtr {
        let p = Rc::new(ast::Self(id));
        ids.move_iter().fold(p, |p, n| Rc::new(ast::Subpath(p, n)))
    }

    fn absolute((id, ids): (Id, Vec<Id>)) -> ast::PathPtr {
        let p = Rc::new(ast::Root(id));
        ids.move_iter().fold(p, |p, n| Rc::new(ast::Subpath(p, n)))
    }
}

///////////////////////////////////////////////////////////////////////////
// Use

fn Use() -> GParser<ast::UseIndex> {
    // One of the following:
    //
    // [pub] use id = path
    // [pub] use path::*
    // [pub] use path

    return Choice(vec![
        UseKind().then(UseKw().thenr(Ident()).then(Eq().thenr(Path())))
            .map(use_rename),

        UseKind().then(UseKw().thenr(Path()).thenl(ColonColon().then(Star())))
            .map(use_glob),

        UseKind().then(UseKw().thenr(Path()))
            .map(use_path),
    ]).thenl(Semi()).map(register);

    fn UseKind() -> GParser<ast::UseKind> {
        PubKw().opt().map(to_use_kind)
    }

    fn to_use_kind(opt: Option<()>) -> ast::UseKind {
        match opt {
            None => ast::ImportUse,
            Some(()) => ast::PubUse
        }
    }

    fn use_rename((k, (id, p)): (ast::UseKind, (Id, ast::PathPtr))) -> ast::Use {
        ast::Use { kind: k, path: p, id: ast::Named(id) }
    }

    fn use_glob((k, p): (ast::UseKind, ast::PathPtr)) -> ast::Use {
        ast::Use { kind: k, path: p, id: ast::Glob }
    }

    fn use_path((k, p): (ast::UseKind, ast::PathPtr)) -> ast::Use {
        let id = p.tail_id();
        ast::Use { kind: k, path: p, id: ast::Named(id) }
    }

    fn register(u: ast::Use) -> ast::UseIndex {
        let mut u = Some(u);
        let ast = the_ast.get().unwrap();
        let mut ast = ast.borrow_mut();
        let index = ast.uses.len();
        ast.uses.push(u.take().unwrap());
        index
    }
}

///////////////////////////////////////////////////////////////////////////
// Module

pub fn Module() -> GParser<ast::Module> {
    return Ref(get);

    fn get<'a>(g: &'a Grammar) -> &'a GParser<ast::Module> {
        &g.module
    }
}

fn MakeModule() -> GParser<ast::Module> {
    // mod id { use* item* }

    return ModKw().thenr(Ident().thenl(Lbrace()))
        .then(Use().rep(0))
        .then(Item().rep(0))
        .thenl(Rbrace())
        .map(module);

    fn module(((id, uses), items): ((Id, Vec<ast::UseIndex>), Vec<ast::ItemIndex>)) -> ast::Module {
        ast::Module { id: id, uses: uses, members: items }
    }
}

///////////////////////////////////////////////////////////////////////////
// Struct

fn Struct() -> GParser<ast::Struct> {
    // struct Id;

    return StructKw().thenr(Ident()).thenl(Semi()).map(structure);

    fn structure(id: Id) -> ast::Struct {
        ast::Struct { id: id }
    }
}

///////////////////////////////////////////////////////////////////////////
// Item

fn Item() -> GParser<ast::ItemIndex> {
    // Module | Struct

    return Choice(vec![
        Module().map(ast::Module),
        Struct().map(ast::Struct)])
        .map(register);

    fn register(item: ast::Item) -> ast::ItemIndex {
        let mut item = Some(item);
        let ast = the_ast.get().unwrap();
        let mut ast = ast.borrow_mut();
        let index = ast.items.len();
        ast.items.push(item.take().unwrap());
        index
    }
}
