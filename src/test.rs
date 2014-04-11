use ast;
use intern;
use grammar;
use parse;

fn setup(text: &str,
         test_body: |&ast::AST|) {
    let bytes = text.as_bytes();
    intern::install(|| {
        let items = grammar::parse(|g| {
            match parse::parse(g, bytes, &grammar::Module()) {
                Ok(m) => m,
                Err(byte) => {
                    fail!("Unexpected parse error: {} (*) {}",
                          // assumes ascii
                          text.slice_to(byte),
                          text.slice_from(byte))
                }
            }
        });
        let ast = ast::AST { items: items };
        test_body(&ast)
    })
}

#[test]
pub fn test_the_first() {
    setup(
        "mod a { }",
        |ast| {
            
        })
}
