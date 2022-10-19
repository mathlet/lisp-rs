mod ast;
mod env;
mod eval;
mod lex;
mod parse;
mod repl;
mod token;

use crate::eval::eval;
use crate::lex::lex;
use crate::parse::parse;
use crate::token::TokenStream;
use std::{env as fs_env, fs};
use crate::repl::repl;

fn main() {
    // let dir = fs_env::current_dir().unwrap();
    // let input = fs::read_to_string(format!(
    //     "{}/examples/deadSimple.lir",
    //     dir.as_path().to_str().unwrap()
    // )).expect("Something went wrong reading the file");

    // let input = "(+ (/ 6 (* 1.5 2)) (- 1 (mod 5 2))) ; 2.0";
    // // let input = "(* (+ 2 3) 4)";
    // let mut tokens = lex(input);
    // // for t in &tokens {
    // //     println!("{:?}", t);
    // // }

    // let ast = parse(&mut TokenStream::new(tokens).peekable());

    // println!("{}", ast);
    // let val = eval(&ast, &mut env::default_env());
    // println!("{}", eval(&ast, &mut env::default_env()).expect("invalid ast"));
    repl(&mut env::default_env());
}
