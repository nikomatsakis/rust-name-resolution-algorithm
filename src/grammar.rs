use ast;
use intern::Id;
use parse::*;
use std::cell::RefCell;
use std::rc::Rc;

///////////////////////////////////////////////////////////////////////////
// Context data which must be installed to do a parse

thread_local!(static THE_AST: RefCell<Option<ast::AST>> = RefCell::new(None));

///////////////////////////////////////////////////////////////////////////
// Public entry points

pub fn parse_ast(text: &str) -> ast::AST {
    let text_bytes = text.as_bytes();
    THE_AST.replace(Some(RefCell::new(
        ast::AST { items: Vec::new(), imports: Vec::new() })));
    let grammar = Grammar::new();
    let m = match parse(&grammar, text_bytes, &Module()) {
        Ok(m) => m,
        Err(byte) => {
            panic!("Unexpected parse error for Program: {} (*) {}",
                   // assumes ascii
                   text.slice_to(byte),
                   text.slice_from(byte))
        }
    };
    let mut ast = THE_AST.replace(None).unwrap().unwrap();
    ast.items.push(m);
    ast
}

pub fn parse_path(text: &str) -> ast::PathPtr {
    let text_bytes = text.as_bytes();
    let grammar = Grammar::new();
    match parse(&grammar, text_bytes, &Path()) {
        Ok(m) => m,
        Err(byte) => {
            panic!("Unexpected parse error for Path: {} (*) {}",
                   // assumes ascii
                   text.slice_to(byte),
                   text.slice_from(byte))
        }
    }
}

///////////////////////////////////////////////////////////////////////////
//

struct Grammar {
    module: GParser<ast::Item>,
}

impl Grammar {
    fn new() -> Grammar {
        Grammar { module: MakeModule() }
    }
}

type GParser<T> = Parser<Grammar, T>;

///////////////////////////////////////////////////////////////////////////
// Misc.

pub fn SelfKw<G>() -> Parser<G,()> {
    Token("self", is_not_ident_cont)
}

pub fn ModKw<G>() -> Parser<G,()> {
    Token("mod", is_not_ident_cont)
}

pub fn StructKw<G>() -> Parser<G,()> {
    Token("struct", is_not_ident_cont)
}

pub fn FnKw<G>() -> Parser<G,()> {
    Token("fn", is_not_ident_cont)
}

pub fn PubKw<G>() -> Parser<G,()> {
    Token("pub", is_not_ident_cont)
}

pub fn UseKw<G>() -> Parser<G,()> {
    Token("use", is_not_ident_cont)
}

fn Privacy<G>() -> Parser<G,ast::Privacy> {
    return PubKw().opt().map(to_privacy);

    fn to_privacy(o: Option<()>) -> ast::Privacy {
        match o {
            Some(()) => ast::Public,
            None => ast::Private
        }
    }
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

fn Use() -> GParser<ast::ImportIndex> {
    // One of the following:
    //
    // [pub] use id = path
    // [pub] use path::*
    // [pub] use path

    return Choice(vec![
        Privacy().then(UseKw().thenr(Ident()).then(Eq().thenr(Path())))
            .map(use_rename),

        Privacy().then(UseKw().thenr(Path()).thenl(ColonColon().then(Star())))
            .map(use_glob),

        Privacy().then(UseKw().thenr(Path()))
            .map(use_path),
    ]).thenl(Semi()).map(register);

    fn use_rename((k, (id, p)): (ast::Privacy, (Id, ast::PathPtr))) -> ast::Import {
        ast::Import { privacy: k, path: p, id: ast::Named(id) }
    }

    fn use_glob((k, p): (ast::Privacy, ast::PathPtr)) -> ast::Import {
        ast::Import { privacy: k, path: p, id: ast::Glob }
    }

    fn use_path((k, p): (ast::Privacy, ast::PathPtr)) -> ast::Import {
        let id = p.tail_id();
        ast::Import { privacy: k, path: p, id: ast::Named(id) }
    }

    fn register(u: ast::Import) -> ast::ImportIndex {
        let mut u = Some(u);
        let ast = THE_AST.get().unwrap();
        let mut ast = ast.borrow_mut();
        let index = ast.imports.len();
        ast.imports.push(u.take().unwrap());
        index
    }
}

///////////////////////////////////////////////////////////////////////////
// Module

pub fn Module() -> GParser<ast::Item> {
    return Ref(get);

    fn get<'a>(g: &'a Grammar) -> &'a GParser<ast::Item> {
        &g.module
    }
}

fn MakeModule() -> GParser<ast::Item> {
    // mod id { use* item* }

    return Privacy().then(ModKw().thenr(Ident().thenl(Lbrace()))
        .then(Use().rep(0))
        .then(Item().rep(0))
        .thenl(Rbrace()))
        .map(module);

    fn module((privacy, ((id, imports), items)):
              (ast::Privacy, ((Id, Vec<ast::ImportIndex>), Vec<ast::ItemIndex>)))
              -> ast::Item
    {
        ast::Item {
            name: id,
            privacy: privacy,
            kind: ast::Module(ast::Module { imports: imports, members: items })
        }
    }
}

///////////////////////////////////////////////////////////////////////////
// Item

fn Item() -> GParser<ast::ItemIndex> {
    // Module | Struct

    return Choice(vec![Module(), Struct()]).map(register);

    fn Struct() -> GParser<ast::Item> {
        // struct Id;

        return Privacy().then(StructKw().thenr(Ident()).thenl(Semi())).map(structure);

        fn structure((p, id): (ast::Privacy, Id)) -> ast::Item {
            ast::Item { privacy: p, name: id, kind: ast::Struct }
        }
    }

//    fn Fn() -> GParser<ast::Item> {
//        // fn Id;
//
//        return Privacy().then(FnKw().thenr(Ident()).thenl(Semi())).map(function);
//
//        fn function((p, id): (ast::Privacy, Id)) -> ast::Item {
//            ast::Item { privacy: p, name: id, kind: ast::Function }
//        }
//    }

    fn register(item: ast::Item) -> ast::ItemIndex {
        let mut item = Some(item);
        let ast = THE_AST.get().unwrap();
        let mut ast = ast.borrow_mut();
        let index = ast.items.len();
        ast.items.push(item.take().unwrap());
        index
    }
}

