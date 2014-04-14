use ast;
use intern;
use grammar;
use nameresolution::Bindings;
use nameresolution::{ResolvedToItem,ResolvedToNothing};
use nameresolution::{Cycle,DoubleBinding,AmbiguousBinding};
use log::macros;

use std::io;

fn setup(text: &str,
         path_texts: &[&str],
         test_body: |&ast::AST, &[ast::PathPtr]|) {
    intern::install(|| {
        let ast = grammar::parse_ast(text);
        let paths: ~[ast::PathPtr] = path_texts.iter().map(|text| {
            grammar::parse_path(*text)
        }).collect();
        test_body(&ast, paths)
    })
}

#[test]
pub fn explicit_shadows_implicit_with_struct_and_use() {
    setup(
        "mod root {
            mod a {
                use b :: *;
                struct S;
            }

            mod b {
                struct S;
                struct T;
            }
        }",

        ["a", "b",            // 0, 1
         "a :: S", "a :: T",  // 2, 3
         "b :: S", "b :: T",  // 4, 5
         "self :: T"],        // 6

        |ast, paths| {
            assert_eq!(ast.items.len(), 6);

            let b = Bindings::new(ast).ok().unwrap();

            let r = b.resolve_path_from_root(&paths[0]);
            assert_eq!(r.unwrap(), ResolvedToItem(1));

            let r = b.resolve_path_from_root(&paths[1]);
            assert_eq!(r.unwrap(), ResolvedToItem(4));

            // a :: S yields the struct S from within a:
            let r = b.resolve_path_from_root(&paths[2]);
            assert_eq!(r.unwrap(), ResolvedToItem(0));

            // b :: S yields the struct S from within b:
            let r = b.resolve_path_from_root(&paths[4]);
            assert_eq!(r.unwrap(), ResolvedToItem(2));

            // a :: T yields an error because T is not an *EXPORT*
            let r = b.resolve_path_from_root(&paths[3]);
            assert_eq!(r.unwrap(), ResolvedToNothing);

            // but self :: T from within a works
            let r = b.resolve_path_from(&paths[0], &paths[6]);
            assert_eq!(r.unwrap(), ResolvedToItem(3));

            // as does self :: T from within b
            let r = b.resolve_path_from(&paths[1], &paths[6]);
            assert_eq!(r.unwrap(), ResolvedToItem(3));

            // and of course b :: T works
            let r = b.resolve_path_from_root(&paths[5]);
            assert_eq!(r.unwrap(), ResolvedToItem(3));
        })
}

#[test]
pub fn explicit_shadows_implicit_with_pub_use() {
    setup(
        "mod root {
            mod a {
                pub use b :: *;
                struct S;
            }

            mod b {
                struct S;
                struct T;
            }
        }",

        ["a", "b",            // 0, 1
         "a :: S", "a :: T",  // 2, 3
         "b :: S", "b :: T",  // 4, 5
         "self :: T"],        // 6

        |ast, paths| {
            assert_eq!(ast.items.len(), 6);

            let b = Bindings::new(ast).ok().unwrap();

            let r = b.resolve_path_from_root(&paths[0]);
            assert_eq!(r.unwrap(), ResolvedToItem(1));

            let r = b.resolve_path_from_root(&paths[1]);
            assert_eq!(r.unwrap(), ResolvedToItem(4));

            // a :: S yields the struct S from within a:
            let r = b.resolve_path_from_root(&paths[2]);
            assert_eq!(r.unwrap(), ResolvedToItem(0));

            // b :: S yields the struct S from within b:
            let r = b.resolve_path_from_root(&paths[4]);
            assert_eq!(r.unwrap(), ResolvedToItem(2));

            // `a :: T` yields the struct S from within `b`
            let r = b.resolve_path_from_root(&paths[3]);
            assert_eq!(r.unwrap(), ResolvedToItem(3));

            // `self :: T` from within `a` works
            let r = b.resolve_path_from(&paths[0], &paths[6]);
            assert_eq!(r.unwrap(), ResolvedToItem(3));

            // as does self :: T from within b
            let r = b.resolve_path_from(&paths[1], &paths[6]);
            assert_eq!(r.unwrap(), ResolvedToItem(3));

            // and of course b :: T works
            let r = b.resolve_path_from_root(&paths[5]);
            assert_eq!(r.unwrap(), ResolvedToItem(3));
        })
}

#[test]
pub fn ambiguous_double_use_glob() {
    /*!
     * In this test, the double glob use from `b` yields an
     * ambiguity error.
     */
    setup(
        "mod root {
            mod a {
                use b :: *;
                use b :: *;
            }

            mod b {
                struct S;
                struct T;
            }
        }",

        [],

        |ast, paths| {
            match Bindings::new(ast) {
                Err(AmbiguousBinding(..)) => {}
                r => {
                    fail!("Expected ambiguity error, got: {:?}", r);
                }
            }
        })
}

#[test]
pub fn ambiguous_double_use_explicit() {
    /*!
     * In this test, the double use of `b::T` yields an
     * ambiguity error.
     */
    setup(
        "mod root {
            mod a {
                use b :: T;
                use b :: T;
            }

            mod b {
                struct S;
                struct T;
            }
        }",

        [],

        |ast, paths| {
            match Bindings::new(ast) {
                Err(DoubleBinding(..)) => {}
                r => {
                    fail!("Expected ambiguity error, got: {:?}", r);
                }
            }
        })
}

#[test]
pub fn ambiguous_double_pub_use_glob() {
    /*!
     * In this test, the double glob `pub use` from `b` yields an
     * ambiguity error.
     */
    setup(
        "mod root {
            mod a {
                pub use b :: *;
                pub use b :: *;
            }

            mod b {
                struct S;
                struct T;
            }
        }",

        [],

        |ast, paths| {
            match Bindings::new(ast) {
                Err(AmbiguousBinding(..)) => {}
                r => {
                    fail!("Expected ambiguity error, got: {:?}", r);
                }
            }
        })
}

#[test]
pub fn specific_use_same_as_glob_use() {
    /*!
     * In this test, we import `T` twice, but once is with a glob,
     * so no error arises.
     */
    setup(
        "mod root {
            mod a {
                use b :: *;
                use b :: T;
            }

            mod b {
                struct S;
                struct T;
            }
        }",

        [],

        |ast, paths| {
            match Bindings::new(ast) {
                Ok(..) => {}
                r => {
                    fail!("Expected ambiguity error, got: {:?}", r);
                }
            }
        })
}

#[test]
pub fn specific_use_overrides_glob_use() {
    /*!
     * In this test, we import `T` twice, but once is with a glob,
     * so no error arises.
     */
    setup(
        "mod root {
            mod a {
                use b :: *;
                use c :: T;
            }

            mod b {
                struct S;
                struct T;
            }

            mod c {
                struct S;
                struct T;
            }
        }",

        ["a", "self :: T", "c :: T"],

        |ast, paths| {
            let b = match Bindings::new(ast) {
                Ok(b) => b,
                r => {
                    fail!("Expected ambiguity error, got: {:?}", r);
                }
            };

            // `c :: T` from within `a` works
            let c_T = b.resolve_path_from_root(&paths[2]).unwrap();
            assert_eq!(c_T, ResolvedToItem(5));

            // `self :: T` from within `a` works
            let a_T = b.resolve_path_from(&paths[0], &paths[1]).unwrap();
            assert_eq!(c_T, a_T);
        })
}

#[test]
pub fn cyclic_pub_use() {
    /*!
     * In this test, we have a true cycle.
     */
    setup(
        "mod root {
            mod a {
                pub use b :: T;
            }

            mod b {
                pub use a :: T;
            }
        }",

        ["a", "b", "self :: T",
         "a :: T", "b :: T",
         "self :: T"],

        |ast, paths| {
            let b = Bindings::new(ast).unwrap();

            for i in range(0u, 2u) {
                match b.resolve_path_from(&paths[i], &paths[2]) {
                    Err(Cycle(..)) => (),
                    r => fail!("Expected path {} to yield a cycle: {:?}",
                               i, r)
                }
            }

            for i in range(3u, 5u) {
                match b.resolve_path_from_root(&paths[i]) {
                    Err(Cycle(..)) => (),
                    r => fail!("Expected path {} to yield a cycle: {:?}",
                               i, r)
                }
            }

        })
}

#[test]
pub fn pub_use_one_another() {
    /*!
     * In this test, we have two modules that both `pub use` the
     * glob contents of the other.
     */
    setup(
        "mod root {
            mod a {
                pub use b :: *;
                struct C;
                struct T;
            }

            mod b {
                pub use a :: *;
                struct C;
                struct U;
            }
        }",

        ["a :: C", "a :: U", "a :: T",
         "b :: C", "b :: U", "b :: T"],

        |ast, paths| {
            let b = Bindings::new(ast).unwrap();

            let a_C = b.resolve_path_from_root(&paths[0]).unwrap();
            assert_eq!(a_C, ResolvedToItem(0));

            let a_U = b.resolve_path_from_root(&paths[1]).unwrap();
            assert_eq!(a_U, ResolvedToItem(4));

            let a_T = b.resolve_path_from_root(&paths[2]).unwrap();
            assert_eq!(a_T, ResolvedToItem(1));

            let b_C = b.resolve_path_from_root(&paths[3]).unwrap();
            assert_eq!(b_C, ResolvedToItem(3));

            let b_U = b.resolve_path_from_root(&paths[4]).unwrap();
            assert_eq!(b_U, ResolvedToItem(4));

            let b_T = b.resolve_path_from_root(&paths[5]).unwrap();
            assert_eq!(b_T, ResolvedToItem(1));

        })
}
