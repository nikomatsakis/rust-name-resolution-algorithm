#![feature(globs,phase)]
#![allow(non_snake_case_functions)]

#[phase(plugin, link)]
extern crate log;

use std::os;
use std::io::File;
use std::path::Path;

mod ast;
mod intern;
mod nameresolution;
mod parse;
mod grammar;
mod test;

pub fn setup(text: &str,
             path_texts: &[&str],
             test_body: |&ast::AST, &[ast::PathPtr]|) {
    intern::install(|| {
        let ast = grammar::parse_ast(text);
        let paths: Vec<ast::PathPtr> = path_texts.iter().map(|text| {
            grammar::parse_path(*text)
        }).collect();
        test_body(&ast, paths.as_slice())
    })
}

pub fn main() {
    let args = os::args();

    if args.len() < 3 {
        println!("Arguments: ast-file paths...");
        return;
    }

    let file_name = args.get(1).as_slice();
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

        let mut resolution_state = nameresolution::resolve(&ast);

        for error in resolution_state.errors().iter() {
            println!("Error: {}", error);
        }

        for path in paths.iter() {
            let result = resolution_state.resolve_path_relative_to_root((*path).clone());
            match result {
                nameresolution::ResolvedToSuccess(item_index) => {
                    println!("Path={} resolves to item #{} = {}",
                             path, item_index, ast.item(item_index));
                }
                _ => {
                    println!("Path={} resolves to {}",
                             path, result);
                }
            }
        }
    });
}
