#![cfg_attr(not(test), allow(dead_code, unused_imports))]

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
fn precedence_over_glob() {
    let mut krate = ast::Krate::new();
    parse_Krate(&mut krate, r#"
mod foo { use bar::*; struct Struct { } }
mod bar { struct Struct { } }
"#).unwrap();
    let result = resolve::resolve_and_expand(&mut krate);
    debug!("result = {:?}", result);
    assert!(result.is_ok());
}

#[test]
fn precedence_over_glob_macro() {
    let mut krate = ast::Krate::new();
    // Here the macro m! generates a struct; this conflicts with
    // the struct we get from the glob, even though an explicit name would not.q
    parse_Krate(&mut krate, r#"
mod foo { use bar::*; m!; }
mod bar { macro_rules! m { struct Struct { } } struct Struct { } }
"#).unwrap();
    let result = resolve::resolve_and_expand(&mut krate);
    debug!("result = {:?}", result);
    assert!(result.is_err());
}

#[test]
fn precedence_over_glob_macro_2() {
    let mut krate = ast::Krate::new();
    // Here the macro m! generates a struct; this conflicts with
    // the struct we get from the glob, even though an explicit name would not.q
    parse_Krate(&mut krate, r#"
mod foo { use bar::*; m!; }
mod bar { macro_rules! m { struct Struct { } } m!; }
"#).unwrap();
    let result = resolve::resolve_and_expand(&mut krate);
    debug!("result = {:?}", result);
    assert!(result.is_err());
}

#[test]
fn multiple_names() {
    let mut krate = ast::Krate::new();
    parse_Krate(&mut krate, r#"
mod foo { use bar::Struct; struct Struct { } }
mod bar { struct Struct { } }
"#).unwrap();
    let result = resolve::resolve_and_expand(&mut krate);
    debug!("result = {:?}", result);
    assert!(result.is_err());
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
    assert!(result.is_err());
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


#[test]
fn expand_to_conflicting_globs() {
    let mut krate = ast::Krate::new();

    // Here the macro m! generates a conflict entry for m!. But we
    // have to make sure that this results in an error.

    parse_Krate(&mut krate, r#"
mod foo { use bar::*; self::m!; }
mod bar { macro_rules! m { use baz::*; } }
mod baz { macro_rules! m { } }
"#).unwrap();
    let result = resolve::resolve_and_expand(&mut krate);
    debug!("result = {:?}", result);
    assert!(result.is_err());
}

#[test]
fn banning_macro_globs_is_not_enough() {
    let mut krate = ast::Krate::new();

    // Here the macro m! generates a conflict entry for m!. But we
    // have to make sure that this results in an error.

    parse_Krate(&mut krate, r#"
mod a {
    use b::c::n;
    self::n!;
}
mod b {
    use c::*;
    macro_rules! m {
        mod d {
            macro_rules! n { }
        }
    }
    self::m!;
}
mod c {
    mod d {
        macro_rules! n { }
    }
}
"#).unwrap();
    let result = resolve::resolve_and_expand(&mut krate);
    debug!("result = {:?}", result);
    assert!(match result {
        Err(ResolutionError::InvalidPath { .. }) => true,
        _ => false
    });
}

#[test]
fn cyclic_macro_defs() {
    let mut krate = ast::Krate::new();

    // Check that no error results if `a` and `b` use each other's
    // macros.

    parse_Krate(&mut krate, r#"
mod a {
    use b::*;
    pub macro_rules! n {
        struct B {  }
    }
    self::m!;
}
mod b {
    use a::*;
    pub macro_rules! m {
        struct A {  }
    }
    self::n!;
}
mod c {
    use b::B;
    use a::A;
    { self::B; self::A; }
}
"#).unwrap();
    let result = resolve::resolve_and_expand(&mut krate);
    debug!("result = {:?}", result);
    assert!(result.is_ok());
}

#[test]
fn glob_brings_module_which_has_macro() {
    let mut krate = ast::Krate::new();
    parse_Krate(&mut krate, r#"
mod a {
    use b::*;
    self::c::m!;
}
mod b {
    mod c {
        pub macro_rules! m { }
    }
}
"#).unwrap();
    let result = resolve::resolve_and_expand(&mut krate);
    debug!("result = {:?}", result);
    assert!(result.is_ok());
}

#[test]
fn named_import_takes_prec_over_glob_even_if_we_cannot_yet_resolve() {
    let mut krate = ast::Krate::new();

    // Here the `use b::c` cannot be resolved in round 1, but it still
    // tells us not to bring in `d::c` from the `d::*` glob.

    parse_Krate(&mut krate, r#"
mod a {
    use b::c;
    use d::*;
    self::c::m!;
}
mod b {
    macro_rules! make_c {
        mod c { pub macro_rules! m { } }
    }
    self::make_c!;
}
mod d {
    mod c { }
}
"#).unwrap();
    let result = resolve::resolve_and_expand(&mut krate);
    debug!("result = {:?}", result);
    assert!(result.is_ok());
}

#[test]
fn macro_can_expand_to_fix_errors_observed_earlier() {
    let mut krate = ast::Krate::new();

    // The globs are in conflict, and hence resolving `self::b::m!`
    // would be an error. We ignore this error during macro resolution
    // and instead expand `self::make_b`. This creates the `mod b`
    // that overrides the globs and hence we are happy. This is kind
    // of odd though because it's a sort of time-travel, but one in
    // which the error we observed was resolved happily.

    parse_Krate(&mut krate, r#"
mod a {
    use c::*;
    use d::*;
    use e::*;

    self::b::m!;

    macro_rules! make_b {
        mod b {
            macro_rules! m { }
        }
    }

    self::make_b!;
}
mod c {
    mod b { }
}
mod d {
    mod b { }
}
mod e {
    mod b { }
}
"#).unwrap();
    let result = resolve::resolve_and_expand(&mut krate);
    debug!("result = {:?}", result);
    assert!(result.is_ok());
}

#[test]
fn using_self_import_to_fix_time_travel() {
    let mut krate = ast::Krate::new();

    // This is a time-travel violation.

    parse_Krate(&mut krate, r#"
mod a {
    use c::*;

    self::b::m!;

    macro_rules! make_b {
        mod b {
            macro_rules! m { }
        }
    }

    self::make_b!;
}
mod c {
    mod b {
        macro_rules! m { }
    }
}
"#).unwrap();
    let result = resolve::resolve_and_expand(&mut krate);
    debug!("result = {:?}", result);
    assert!(result.is_err());

    // This is not, because the `use self::b` takes precedence over
    // the glob, even though it can't be resolved right away (until
    // after first round of macro expansion, in fact). This is handy,
    // if maybe a bit odd.

    let mut krate = Krate::new();
    parse_Krate(&mut krate, r#"
mod a {
    use c::*;
    use self::b;

    self::b::m!;

    macro_rules! make_b {
        mod b {
            macro_rules! m { }
        }
    }

    self::make_b!;
}
mod c {
    mod b {
        macro_rules! m { }
    }
}
"#).unwrap();
    let result = resolve::resolve_and_expand(&mut krate);
    debug!("result = {:?}", result);
    assert!(result.is_ok());
}


#[test]
fn macro_can_expand_to_change_how_path_is_resolved() {
    let mut krate = ast::Krate::new();

    // In phase 1, self::b::m resolves through the glob,
    // but after that the `b` is different, but the `m`
    // is the same.

    parse_Krate(&mut krate, r#"
mod a {
    use c::*;
    self::b::m!;
}
mod c {
    mod b {
        macro_rules! m {
            mod b {
                pub use c::b::m;
            }
        }
    }
}
"#).unwrap();
    let result = resolve::resolve_and_expand(&mut krate);
    debug!("result = {:?}", result);
    assert!(result.is_ok());
}
