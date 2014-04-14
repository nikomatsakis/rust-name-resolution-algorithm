#![feature(globs,phase)]

extern crate collections;

#[phase(syntax, link)]
extern crate log;

mod ast;
mod intern;
mod nameresolution;
mod parse;
mod grammar;
mod test;

fn main() { }
