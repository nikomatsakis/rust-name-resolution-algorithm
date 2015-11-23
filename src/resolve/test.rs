use ast;
use parser::{parse_Krate, parse_Module, parse_Path};
use super::PathResolver;

#[test]
fn basic_example() {
    let mut krate = ast::Krate::new();
    parse_Krate(&mut krate, "mod foo { use bar::Struct; } \
                             mod bar { struct Struct { } }").unwrap();
    let foo_Struct = parse_Path(&mut krate, "foo::Struct").unwrap();
    let mut resolver = PathResolver::new(&krate);
    let r = resolver.resolve_path(ast::ROOT_ID, foo_Struct);
    assert!(r.is_ok());
    assert!(r.unwrap().is_some());
}

#[test]
fn basic_example_1() {
    let mut krate = ast::Krate::new();
    parse_Krate(&mut krate, "mod foo { use bar::Struct; } \
                             mod bar { struct Struct { } }").unwrap();
    let bar_Struct = parse_Path(&mut krate, "bar::Struct").unwrap();
    let mut resolver = PathResolver::new(&krate);
    let r = resolver.resolve_path(ast::ROOT_ID, bar_Struct);
    assert!(r.is_ok());
    assert!(r.unwrap().is_some());
}

#[test]
fn double_def() {
    let mut krate = ast::Krate::new();
    parse_Krate(&mut krate, "mod foo { use bar::Struct; struct Struct { } } \
                             mod bar { struct Struct { } }").unwrap();
    let foo_Struct = parse_Path(&mut krate, "foo::Struct").unwrap();
    let mut resolver = PathResolver::new(&krate);
    let r = resolver.resolve_path(ast::ROOT_ID, foo_Struct);
    assert!(r.is_err());
}

#[test]
fn star_glob() {
    let mut krate = ast::Krate::new();
    parse_Krate(&mut krate, "mod foo { use bar::*; } \
                             mod bar { struct Struct { } }").unwrap();
    let foo_Struct = parse_Path(&mut krate, "foo::Struct").unwrap();
    let mut resolver = PathResolver::new(&krate);
    let r = resolver.resolve_path(ast::ROOT_ID, foo_Struct);
    assert!(r.is_ok());
    assert!(r.unwrap().is_some());
}

#[test]
fn star_glob_cycle() {
    let mut krate = ast::Krate::new();
    parse_Krate(&mut krate, "mod foo { use bar::*; } \
                             mod bar { use foo::*; struct Struct { } }").unwrap();
    let foo_Struct = parse_Path(&mut krate, "foo::Struct").unwrap();
    let mut resolver = PathResolver::new(&krate);
    let r = resolver.resolve_path(ast::ROOT_ID, foo_Struct);
    assert!(r.is_ok());
    assert!(r.unwrap().is_some());
}

#[test]
fn star_glob_multi() {
    let mut krate = ast::Krate::new();
    parse_Krate(&mut krate, "mod foo { use bar::*; use baz::*; } \
                             mod bar { use baz::*; } \
                             mod baz { struct Struct { } }").unwrap();
    let foo_Struct = parse_Path(&mut krate, "foo::Struct").unwrap();
    let mut resolver = PathResolver::new(&krate);
    let r = resolver.resolve_path(ast::ROOT_ID, foo_Struct);
    assert!(r.is_ok());
    assert!(r.unwrap().is_some());
}

#[test]
fn star_glob_multi_bad() {
    let mut krate = ast::Krate::new();
    parse_Krate(&mut krate, "mod foo { use bar::*; use baz::*; } \
                             mod bar { struct Struct { } } \
                             mod baz { struct Struct { } }").unwrap();
    let foo_Struct = parse_Path(&mut krate, "foo::Struct").unwrap();
    let mut resolver = PathResolver::new(&krate);
    let r = resolver.resolve_path(ast::ROOT_ID, foo_Struct);
    assert!(r.is_err());
}
