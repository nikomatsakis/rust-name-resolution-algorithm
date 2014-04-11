use ast;
use intern::Id;
use parse::*;
use std::local_data;
use std::rc::Rc;

///////////////////////////////////////////////////////////////////////////
// Context data which must be installed to do a parse

local_data_key!(the_items: ~[ast::Item])

pub fn parse(f: |&Grammar| -> ast::Module) -> ~[ast::Item] {
    local_data::set(the_items, ~[]);
    let grammar = Grammar::new();
    let m = f(&grammar);
    let mut items = local_data::pop(the_items).unwrap();
    items.push(ast::Module(m));
    items
}

///////////////////////////////////////////////////////////////////////////
//

pub struct Grammar {
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
    return Choice(~[
        SelfKw()
            .thenr(ColonColon().thenr(Ident()))
            .then(ColonColon().thenr(Ident()).rep(0))
            .map(relative),
        Ident().then(ColonColon().thenr(Ident()).rep(0)).map(absolute)
    ]);

    fn relative((id, ids): (Id, ~[Id])) -> ast::PathPtr {
        let p = Rc::new(ast::Self(id));
        ids.move_iter().fold(p, |p, n| Rc::new(ast::Subpath(p, n)))
    }

    fn absolute((id, ids): (Id, ~[Id])) -> ast::PathPtr {
        let p = Rc::new(ast::Root(id));
        ids.move_iter().fold(p, |p, n| Rc::new(ast::Subpath(p, n)))
    }
}

///////////////////////////////////////////////////////////////////////////
// Use

fn Use() -> GParser<ast::Use> {
    // One of the following:
    //
    // [pub] use id = path
    // [pub] use path::*
    // [pub] use path

    return Choice(~[
        UseKind().then(UseKw().thenr(Ident()).then(Eq().thenr(Path())))
            .map(use_rename),

        UseKind().then(UseKw().thenr(Path()).thenl(ColonColon().then(Star())))
            .map(use_glob),

        UseKind().then(UseKw().thenr(Path()))
            .map(use_path),
    ]);

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
}

///////////////////////////////////////////////////////////////////////////
// Module

fn Module() -> GParser<ast::Module> {
    return Ref(get);

    fn get<'a>(g: &'a Grammar) -> &'a GParser<ast::Module> {
        &g.module
    }
}

fn MakeModule() -> GParser<ast::Module> {
    // mod id { use* item* }

    return ModKw().thenr(Ident()).then(Use().rep(0)).then(Item().rep(0))
        .map(module);

    fn module(((id, uses), items): ((Id, ~[ast::Use]), ~[ast::ItemIndex])) -> ast::Module {
        ast::Module { id: id, uses: uses, members: items }
    }
}

///////////////////////////////////////////////////////////////////////////
// Struct

fn Struct() -> GParser<ast::Struct> {
    // struct Id;

    return StructKw().thenr(Ident()).map(structure);

    fn structure(id: Id) -> ast::Struct {
        ast::Struct { id: id }
    }
}

///////////////////////////////////////////////////////////////////////////
// Item

fn Item() -> GParser<ast::ItemIndex> {
    // Module | Struct

    return Choice(~[
        Module().map(ast::Module),
        Struct().map(ast::Struct)])
        .map(register);

    fn register(item: ast::Item) -> ast::ItemIndex {
        let mut item = Some(item);
        local_data::get_mut(the_items, |items| {
            let items = items.unwrap();
            let index = items.len();
            items.push(item.take().unwrap());
            index
        })
    }
}
