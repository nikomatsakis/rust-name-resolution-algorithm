use ast;
use intern;
use grammar;
use nameresolution as nr;

fn setup(text: &str,
         tests: &[(Option<&str>, &str, nr::PathResolution)],
         errors: &[&str]) {
    intern::install(|| {
        let ast = grammar::parse_ast(text);
        let mut resolution_state = nr::resolve(&ast);

        for &(base, path_text, ref expected_resolution) in tests.iter() {
            let base_index = match base {
                None => ast.root_index(),
                Some(base_text) => {
                    let path = grammar::parse_path(base_text);
                    match resolution_state.resolve_path(ast.root_index(), path) {
                        nr::ResolvedToSuccess(nr::BoundToItem(index)) => index,
                        r => {
                            panic!(format!("root path {} yielded {}", base, r));
                        }
                    }
                }
            };

            let path = grammar::parse_path(path_text);
            let actual_resolution = resolution_state.resolve_path(base_index, path);
            assert_eq!(actual_resolution, *expected_resolution);
        }

        println!("{}", resolution_state.errors());
        for (expected_error, actual_error) in
            errors.iter().zip(
                resolution_state.errors().iter())
        {
            assert_eq!(format!("{}", actual_error), expected_error.to_owned());
        }
        assert_eq!(errors.len(), resolution_state.errors().len());
    });
}

#[test]
pub fn simple_test() {
    setup(
        "mod root {
            mod a {
                use b :: *;
                pub struct A;
                pub struct C;
            }

            mod b {
                use a :: *;
                pub struct B;
                pub struct C;
            }
        }",

        &[(None, "a :: A", nr::ResolvedToSuccess(nr::BoundToItem(0))),
          (None, "a :: B", nr::ResolvedToSuccess(nr::BoundToItem(3))),
          (None, "a :: C", nr::ResolvedToSuccess(nr::BoundToItem(1))),
          (None, "b :: A", nr::ResolvedToSuccess(nr::BoundToItem(0))),
          (None, "b :: B", nr::ResolvedToSuccess(nr::BoundToItem(3))),
          (None, "b :: C", nr::ResolvedToSuccess(nr::BoundToItem(4)))],

        &[]);
}

#[test]
pub fn conflicting_imports() {
    setup(
        "mod root {
            mod a {
                use b :: *;
                use c :: *;
            }

            mod b {
                pub struct S;
                pub struct T;
            }

            mod c {
                pub struct S;
                pub struct T;
            }
        }",

        &[(None, "a :: S", nr::ResolvedToSuccess(nr::BoundToItem(1)))],

        &[]);
}



//#[test]
//pub fn explicit_shadows_implicit_with_struct_and_use() {
//    setup(
//        "mod root {
//            mod a {
//                use b :: *;
//                struct S;
//            }
//
//            mod b {
//                struct S;
//                struct T;
//            }
//        }",
//
//        ["a", "b",            // 0, 1
//         "a :: S", "a :: T",  // 2, 3
//         "b :: S", "b :: T",  // 4, 5
//         "self :: T"],        // 6
//
//        |ast, paths| {
//            assert_eq!(ast.items.len(), 6);
//
//            let b = Bindings::new(ast).ok().unwrap();
//
//            let r = b.resolve_path_from_root(&paths[0]);
//            assert_eq!(r.unwrap(), ResolvedToItem(1));
//
//            let r = b.resolve_path_from_root(&paths[1]);
//            assert_eq!(r.unwrap(), ResolvedToItem(4));
//
//            // a :: S yields the struct S from within a:
//            let r = b.resolve_path_from_root(&paths[2]);
//            assert_eq!(r.unwrap(), ResolvedToItem(0));
//
//            // b :: S yields the struct S from within b:
//            let r = b.resolve_path_from_root(&paths[4]);
//            assert_eq!(r.unwrap(), ResolvedToItem(2));
//
//            // a :: T yields an error because T is not an *EXPORT*
//            let r = b.resolve_path_from_root(&paths[3]);
//            assert_eq!(r.unwrap(), ResolvedToNothing);
//
//            // but self :: T from within a works
//            let r = b.resolve_path_from(&paths[0], &paths[6]);
//            assert_eq!(r.unwrap(), ResolvedToItem(3));
//
//            // as does self :: T from within b
//            let r = b.resolve_path_from(&paths[1], &paths[6]);
//            assert_eq!(r.unwrap(), ResolvedToItem(3));
//
//            // and of course b :: T works
//            let r = b.resolve_path_from_root(&paths[5]);
//            assert_eq!(r.unwrap(), ResolvedToItem(3));
//        })
//}
//
//#[test]
//pub fn explicit_shadows_implicit_with_pub_use() {
//    setup(
//        "mod root {
//            mod a {
//                pub use b :: *;
//                struct S;
//            }
//
//            mod b {
//                struct S;
//                struct T;
//            }
//        }",
//
//        ["a", "b",            // 0, 1
//         "a :: S", "a :: T",  // 2, 3
//         "b :: S", "b :: T",  // 4, 5
//         "self :: T"],        // 6
//
//        |ast, paths| {
//            assert_eq!(ast.items.len(), 6);
//
//            let b = Bindings::new(ast).ok().unwrap();
//
//            let r = b.resolve_path_from_root(&paths[0]);
//            assert_eq!(r.unwrap(), ResolvedToItem(1));
//
//            let r = b.resolve_path_from_root(&paths[1]);
//            assert_eq!(r.unwrap(), ResolvedToItem(4));
//
//            // a :: S yields the struct S from within a:
//            let r = b.resolve_path_from_root(&paths[2]);
//            assert_eq!(r.unwrap(), ResolvedToItem(0));
//
//            // b :: S yields the struct S from within b:
//            let r = b.resolve_path_from_root(&paths[4]);
//            assert_eq!(r.unwrap(), ResolvedToItem(2));
//
//            // `a :: T` yields the struct S from within `b`
//            let r = b.resolve_path_from_root(&paths[3]);
//            assert_eq!(r.unwrap(), ResolvedToItem(3));
//
//            // `self :: T` from within `a` works
//            let r = b.resolve_path_from(&paths[0], &paths[6]);
//            assert_eq!(r.unwrap(), ResolvedToItem(3));
//
//            // as does self :: T from within b
//            let r = b.resolve_path_from(&paths[1], &paths[6]);
//            assert_eq!(r.unwrap(), ResolvedToItem(3));
//
//            // and of course b :: T works
//            let r = b.resolve_path_from_root(&paths[5]);
//            assert_eq!(r.unwrap(), ResolvedToItem(3));
//        })
//}
//
//#[test]
//pub fn ambiguous_double_use_glob() {
//    /*!
//     * In this test, the double glob use from `b` yields an
//     * ambiguity error.
//     */
//    setup(
//        "mod root {
//            mod a {
//                use b :: *;
//                use b :: *;
//            }
//
//            mod b {
//                struct S;
//                struct T;
//            }
//        }",
//
//        [],
//
//        |ast, _| {
//            match Bindings::new(ast) {
//                Err(AmbiguousBinding(..)) => {}
//                r => {
//                    panic!("Expected ambiguity error, got: {}", r);
//                }
//            }
//        })
//}
//
//#[test]
//pub fn ambiguous_double_use_explicit() {
//    /*!
//     * In this test, the double use of `b::T` yields an
//     * ambiguity error.
//     */
//    setup(
//        "mod root {
//            mod a {
//                use b :: T;
//                use b :: T;
//            }
//
//            mod b {
//                struct S;
//                struct T;
//            }
//        }",
//
//        [],
//
//        |ast, _| {
//            match Bindings::new(ast) {
//                Err(DoubleBinding(..)) => {}
//                r => {
//                    panic!("Expected ambiguity error, got: {}", r);
//                }
//            }
//        })
//}
//
//#[test]
//pub fn ambiguous_double_pub_use_glob() {
//    /*!
//     * In this test, the double glob `pub use` from `b` yields an
//     * ambiguity error.
//     */
//    setup(
//        "mod root {
//            mod a {
//                pub use b :: *;
//                pub use b :: *;
//            }
//
//            mod b {
//                struct S;
//                struct T;
//            }
//        }",
//
//        [],
//
//        |ast, _| {
//            match Bindings::new(ast) {
//                Err(AmbiguousBinding(..)) => {}
//                r => {
//                    panic!("Expected ambiguity error, got: {}", r);
//                }
//            }
//        })
//}
//
//#[test]
//pub fn ambiguous_pub_use_and_item() {
//    /*!
//     * In this test, the module `a` both has a `pub use` and a struct
//     * with the same explicit name.
//     */
//    setup(
//        "mod root {
//            mod a {
//                pub use b :: S;
//                struct S;
//            }
//
//            mod b {
//                struct S;
//            }
//        }",
//
//        [],
//
//        |ast, _| {
//            match Bindings::new(ast) {
//                Err(DoubleBinding(..)) => {}
//                r => {
//                    panic!("Expected ambiguity error, got: {}", r);
//                }
//            }
//        })
//}
//
//#[test]
//pub fn specific_use_same_as_glob_use() {
//    /*!
//     * In this test, we import `T` twice, but once is with a glob,
//     * so no error arises.
//     */
//    setup(
//        "mod root {
//            mod a {
//                use b :: *;
//                use b :: T;
//            }
//
//            mod b {
//                struct S;
//                struct T;
//            }
//        }",
//
//        [],
//
//        |ast, _| {
//            match Bindings::new(ast) {
//                Ok(..) => {}
//                r => {
//                    panic!("Expected ambiguity error, got: {}", r);
//                }
//            }
//        })
//}
//
//#[test]
//pub fn specific_use_overrides_glob_use() {
//    /*!
//     * In this test, we import `T` twice, but once is with a glob,
//     * so no error arises.
//     */
//    setup(
//        "mod root {
//            mod a {
//                use b :: *;
//                use c :: T;
//            }
//
//            mod b {
//                struct S;
//                struct T;
//            }
//
//            mod c {
//                struct S;
//                struct T;
//            }
//        }",
//
//        ["a", "self :: T", "c :: T"],
//
//        |ast, paths| {
//            let b = match Bindings::new(ast) {
//                Ok(b) => b,
//                r => {
//                    panic!("Expected ambiguity error, got: {}", r);
//                }
//            };
//
//            // `c :: T` from within `a` works
//            let c_T = b.resolve_path_from_root(&paths[2]).unwrap();
//            assert_eq!(c_T, ResolvedToItem(5));
//
//            // `self :: T` from within `a` works
//            let a_T = b.resolve_path_from(&paths[0], &paths[1]).unwrap();
//            assert_eq!(c_T, a_T);
//        })
//}
//
//#[test]
//pub fn cyclic_pub_use() {
//    /*!
//     * In this test, we have a true cycle.
//     */
//    setup(
//        "mod root {
//            mod a {
//                pub use b :: T;
//            }
//
//            mod b {
//                pub use a :: T;
//            }
//        }",
//
//        ["a", "b", "self :: T",
//         "a :: T", "b :: T",
//         "self :: T"],
//
//        |ast, paths| {
//            let b = Bindings::new(ast).unwrap();
//
//            for i in range(0u, 2u) {
//                match b.resolve_path_from(&paths[i], &paths[2]) {
//                    Err(Cycle(..)) => (),
//                    r => panic!("Expected path {} to yield a cycle: {}",
//                               i, r)
//                }
//            }
//
//            for i in range(3u, 5u) {
//                match b.resolve_path_from_root(&paths[i]) {
//                    Err(Cycle(..)) => (),
//                    r => panic!("Expected path {} to yield a cycle: {}",
//                               i, r)
//                }
//            }
//
//        })
//}
//
//#[test]
//pub fn pub_use_one_another() {
//    /*!
//     * In this test, we have two modules that both `pub use` the
//     * glob contents of the other.
//     */
//    setup(
//        "mod root {
//            mod a {
//                pub use b :: *;
//                struct C;
//                struct T;
//            }
//
//            mod b {
//                pub use a :: *;
//                struct C;
//                struct U;
//            }
//        }",
//
//        ["a :: C", "a :: U", "a :: T",
//         "b :: C", "b :: U", "b :: T"],
//
//        |ast, paths| {
//            let b = Bindings::new(ast).unwrap();
//
//            let a_C = b.resolve_path_from_root(&paths[0]).unwrap();
//            assert_eq!(a_C, ResolvedToItem(0));
//
//            let a_U = b.resolve_path_from_root(&paths[1]).unwrap();
//            assert_eq!(a_U, ResolvedToItem(4));
//
//            let a_T = b.resolve_path_from_root(&paths[2]).unwrap();
//            assert_eq!(a_T, ResolvedToItem(1));
//
//            let b_C = b.resolve_path_from_root(&paths[3]).unwrap();
//            assert_eq!(b_C, ResolvedToItem(3));
//
//            let b_U = b.resolve_path_from_root(&paths[4]).unwrap();
//            assert_eq!(b_U, ResolvedToItem(4));
//
//            let b_T = b.resolve_path_from_root(&paths[5]).unwrap();
//            assert_eq!(b_T, ResolvedToItem(1));
//
//        })
//}
//
//#[test]
//pub fn use_from_child() {
//    /*!
//     * In this test, we have two modules that both `pub use` the
//     * glob contents of the other.
//     */
//    setup(
//        "mod root {
//            mod a {
//                use self :: b :: B;
//
//                mod b {
//                    struct B;
//                }
//
//                struct A;
//            }
//        }",
//
//        ["a", "self :: B"],
//
//        |ast, paths| {
//            let b = Bindings::new(ast).unwrap();
//
//            let a_B = b.resolve_path_from(&paths[0], &paths[1]).unwrap();
//            assert_eq!(a_B, ResolvedToItem(0));
//        })
//}
//
//
//#[test]
//pub fn use_cycle() {
//    /*!
//     * In this test, we have a use that uses something which doesn't
//     * exist. The result is a cycle (from within the module, anyway).
//     */
//    setup(
//        "mod root {
//            mod a {
//                use self :: C;
//            }
//        }",
//
//        ["a", "self :: C"],
//
//        |ast, paths| {
//            let b = Bindings::new(ast).unwrap();
//
//            match b.resolve_path_from(&paths[0], &paths[1]) {
//                Err(Cycle(_)) => { }
//                r => panic!("Expected cycle: {}", r),
//            }
//        })
//}
//
//#[test]
//pub fn pub_use_cycle() {
//    /*!
//     * In this test, we have a `pub use` that uses something which doesn't
//     * exist. The result is a cycle.
//     */
//    setup(
//        "mod root {
//            mod a {
//                pub use self :: C;
//            }
//        }",
//
//        ["a", "self :: C", "a :: C"],
//
//        |ast, paths| {
//            let b = Bindings::new(ast).unwrap();
//
//            match b.resolve_path_from(&paths[0], &paths[1]) {
//                Err(Cycle(_)) => { }
//                r => panic!("Expected cycle: {}", r),
//            }
//
//            match b.resolve_path_from_root(&paths[2]) {
//                Err(Cycle(_)) => { }
//                r => panic!("Expected cycle: {}", r),
//            }
//        })
//}
