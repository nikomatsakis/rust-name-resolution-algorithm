use ast;
use intern;
use grammar;
use nameresolution::Bindings;
use nameresolution::{ResolvedToItem,ResolvedToNothing};
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
pub fn test_the_first() {
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
