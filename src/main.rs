#![feature(globs,phase)]
#![allow(non_snake_case_functions)]

extern crate collections;

#[phase(plugin, link)]
extern crate log;

mod ast;
mod intern;
mod nameresolution;
mod parse;
mod grammar;
mod test;

pub fn main() {
    println!("Run the tests, man!");
}
