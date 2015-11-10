#![feature(globs,phase)]
#![allow(non_snake_case_functions)]

use std::os;
use std::fs::File;
use std::path::Path;
use nameresolution as nr;

macro_rules! debug {
    ($($x:tt)*) => { }
}

mod ast;
mod intern;
mod nameresolution;
mod parse;
mod grammar;
mod test;

pub fn main() {
    let args = os::args();

    if args.len() < 3 {
        println!("Arguments: ast-file paths...");
        return;
    }

    let file_name = args[1].as_slice();
    let ast_string = match File::open(&Path::new(file_name.as_slice())).read_to_end() {
        Ok(s) => s,
        Err(_) => {
            println!("Error accessing `{}`", file_name);
            return;
        }
    };
    let ast_string = match String::from_utf8(ast_string) {
        Ok(s) => s,
        Err(b) => {
            println!("Source file not utf-8: {}", b);
            return;
        }
    };

    let path_strings = args.slice_from(2);

    intern::install(|| {
        let ast = grammar::parse_ast(ast_string.as_slice());
        let paths: Vec<ast::PathPtr> = path_strings.iter().map(|text| {
            grammar::parse_path(text.as_slice())
        }).collect();

        let mut resolution_state = nr::resolve(&ast);

        for path in paths.iter() {
            let result = resolution_state.resolve_path(ast.root_index(),
                                                       (*path).clone());
            match result {
                nr::ResolvedToSuccess(nr::BoundToItem(item_index)) => {
                    println!("Path={} resolves to item #{} = {}",
                             path, item_index, ast.item(item_index));
                }
                nr::ResolvedToSuccess(nr::BoundRelativeToType(relative_to, names)) => {
                    println!("Path={} resolves to type-related path item #{} = {}, {}",
                             path, relative_to, ast.item(relative_to), names);
                }
                _ => {
                    println!("Path={} resolves to {}",
                             path, result);
                }
            }
        }

        resolution_state.check();

        for error in resolution_state.errors().iter() {
            println!("Error: {}", error);
        }
    });
}
