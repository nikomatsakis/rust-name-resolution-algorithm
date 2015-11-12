extern crate lalrpop_intern as intern;

macro_rules! debug {
    ($($args:tt)*) => {
        println!($($args)*);
    }
}

mod ast;
mod parser;
mod resolve;

#[test]
fn basic_example() {
    let mut krate = ast::Krate::new();
    parser::parse_Krate(&mut krate, r#"
mod foo { use bar::Struct; }
mod bar { struct Struct { } }
"#).unwrap();
    resolve::resolve_and_expand(&mut krate).unwrap();
}
