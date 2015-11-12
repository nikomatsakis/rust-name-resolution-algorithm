extern crate lalrpop_intern as intern;

macro_rules! debug {
    ($($args:tt)*) => {
        println!($($args)*);
    }
}

mod ast;
mod parser;
mod resolve;

use ast::*;
use intern::intern;
use resolve::ResolutionError;
use parser::parse_Krate;

#[test]
fn basic_example() {
    let mut krate = ast::Krate::new();
    parse_Krate(&mut krate, r#"
mod foo { use bar::Struct; }
mod bar { struct Struct { } }
"#).unwrap();
    resolve::resolve_and_expand(&mut krate).unwrap();
}

#[test]
#[should_panic]
fn basic_error() {
    let mut krate = ast::Krate::new();
    parse_Krate(&mut krate, r#"
mod foo { use bar::StructX; }
mod bar { struct Struct { } }
"#).unwrap();
    resolve::resolve_and_expand(&mut krate).unwrap();
}

#[test]
fn multiple_names() {
    let mut krate = ast::Krate::new();
    parse_Krate(&mut krate, r#"
mod foo { use bar::*; struct Struct { } }
mod bar { struct Struct { } }
"#).unwrap();
    let result = resolve::resolve_and_expand(&mut krate);
    debug!("result = {:?}", result);
    assert!(match result {
        Err(resolve::ResolutionError::MultipleNames { module_id, name }) => {
            module_id == ModuleId(1) && name == intern("Struct")
        }
        _ => {
            false
        }
    });
}

#[test]
fn with_macro() {
    let mut krate = ast::Krate::new();
    parse_Krate(&mut krate, r#"
mod foo { use bar::*; self::make_struct!; }
mod bar { macro_rules! make_struct { struct Struct { } } }
mod baz { use foo::Struct; }
"#).unwrap();
    let result = resolve::resolve_and_expand(&mut krate);
    debug!("result = {:?}", result);
    assert!(match result {
        Ok(_) => true,
        _ => false,
    });
}

#[test]
fn with_macro_multiple() {
    let mut krate = ast::Krate::new();
    parse_Krate(&mut krate, r#"
mod foo { use bar::*; self::make_struct!; struct Struct { } }
mod bar { macro_rules! make_struct { struct Struct { } } }
mod baz { use foo::Struct; }
"#).unwrap();
    let result = resolve::resolve_and_expand(&mut krate);
    debug!("result = {:?}", result);
    assert!(match result {
        Err(resolve::ResolutionError::MultipleNames { module_id, name }) => {
            module_id == ModuleId(1) && name == intern("Struct")
        }
        _ => {
            false
        }
    });
}

#[test]
fn with_macro_1() {
    let mut krate = ast::Krate::new();
    parse_Krate(&mut krate, r#"
mod foo { use bar::*; self::make_struct!; }
mod bar { macro_rules! make_struct { struct Struct { } } }
mod baz { use foo::Struct; }
"#).unwrap();
    let result = resolve::resolve_and_expand(&mut krate);
    debug!("result = {:?}", result);
    assert!(match result {
        Ok(_) => true,
        _ => false,
    });
}

#[test]
fn with_macro_making_mod_making_macro() {
    let mut krate = ast::Krate::new();

    // Here, the macro m! generates mod x { n! }, and then invoking n! generates Struct
    parse_Krate(&mut krate, r#"
mod foo { use bar::*; self::m!; foo::x::n!; }
mod bar { macro_rules! m { mod x { macro_rules! n { struct Struct { } } } } }
mod baz { use foo::Struct; }
"#).unwrap();
    let result = resolve::resolve_and_expand(&mut krate);
    debug!("result = {:?}", result);
    assert!(match result {
        Ok(_) => true,
        _ => false,
    });
}

#[test]
fn with_macro_making_mod_making_macro_that_conflicts() {
    let mut krate = ast::Krate::new();

    // Here, the macro m! generates mod x { n! }, and then invoking n!
    // generates a (conflicting) definition for `m`. Mind-bending.
    parse_Krate(&mut krate, r#"
mod foo { use bar::*; self::m!; foo::x::n!; }
mod bar { macro_rules! m { mod x { macro_rules! n { macro_rules! m { } struct Struct { } } } } }
mod baz { use foo::Struct; }
"#).unwrap();
    let result = resolve::resolve_and_expand(&mut krate);
    debug!("result = {:?}", result);
    assert!(match result {
        Err(resolve::ResolutionError::MultipleNames { module_id, name }) => {
            module_id == ModuleId(1) && name == intern("m")
        }
        _ => {
            false
        }
    });
}

#[test]
fn simple_cycle() {
    let mut krate = ast::Krate::new();

    // Here, the macro m! generates mod x { n! }, and then invoking n!
    // generates a (conflicting) definition for `m`. Mind-bending.
    parse_Krate(&mut krate, r#"
mod foo { use bar::x; }
mod bar { use foo::x; }
"#).unwrap();
    let result = resolve::resolve_and_expand(&mut krate);
    debug!("result = {:?}", result);
    assert!(match result {
        Err(resolve::ResolutionError::InvalidPath { .. }) => true,
        _ => false,
    });
}

#[test]
fn if_a_glob_conflicts_in_a_forest() {
    let mut krate = ast::Krate::new();

    // Here, the macro m! generates mod x { n! }, and then invoking n!
    // generates a (conflicting) definition for `m`. Mind-bending.
    parse_Krate(&mut krate, r#"
mod foo { use bar::*; use baz::*; { self::T; } }
mod bar { struct S { } struct T { } }
mod baz { struct S { } }
"#).unwrap();
    let result = resolve::resolve_and_expand(&mut krate);
    debug!("result = {:?}", result);
    assert!(result.is_ok());
}

#[test]
fn if_a_glob_conflicts_in_a_forest_but_someone_sees() {
    let mut krate = ast::Krate::new();

    // Here, the macro m! generates mod x { n! }, and then invoking n!
    // generates a (conflicting) definition for `m`. Mind-bending.
    parse_Krate(&mut krate, r#"
mod foo { use bar::*; use baz::*; { self::S; } }
mod bar { struct S { } struct T { } }
mod baz { struct S { } }
"#).unwrap();
    let result = resolve::resolve_and_expand(&mut krate);
    debug!("result = {:?}", result);
    assert!(match result {
        Err(resolve::ResolutionError::InvalidPath { .. }) => true,
        _ => false,
    });
}

#[test]
fn expand_to_conflicting_macro() {
    let mut krate = ast::Krate::new();

    // Here the macro m! generates a conflict entry for m!. But we
    // have to make sure that this results in an error.

    parse_Krate(&mut krate, r#"
mod foo { use bar::*; self::m!; }
mod bar { macro_rules! m { macro_rules! m { struct S { } } } }
"#).unwrap();
    let result = resolve::resolve_and_expand(&mut krate);
    debug!("result = {:?}", result);
    assert!(result.is_err());
}

