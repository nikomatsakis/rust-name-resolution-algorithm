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
        Err(resolve::ResolutionError::MultipleNames { module_id, name, sources: _ }) => {
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
        Err(resolve::ResolutionError::MultipleNames { module_id, name, sources: _ }) => {
            module_id == ModuleId(1) && name == intern("Struct")
        }
        _ => {
            false
        }
    });
}
