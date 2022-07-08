use std::io::{stdin, Read};
use std::{env, process};
use std::fs;
use std::fmt::Display;

mod rules;
mod spec_parser;
mod structures;
mod nodes;
mod utils;

use nodes::node::Node;
use rules::grammar::Grammar;
use spec_parser::grammar::Grammar as SpecGrammar;
use spec_parser::node::Node as NodeTrait;

fn main() {
    let grammar_file = env::args().nth(1).expect("couldn't parse filename");
    let grammar_raw = fs::read_to_string(grammar_file).expect("couldn't read file");

    let (spec, _) = SpecGrammar::parse_and_skip(&grammar_raw).or_crash();
    let grammar = Grammar::try_from(&spec).or_crash();
    
    let input = read_input().expect("can't read stdin");
    let (parsed, _) = Node::from_grammar(&grammar, &input).expect("can't parse input");

    println!("{}", &parsed);
}

fn read_input() -> Option<String> {
    let mut buf = String::new();
    match stdin().read_to_string(&mut buf) {
        Ok(0) => None,
        Ok(_) => Some(buf),
        Err(_) => None,
    }
}

trait Crash<T> {
    fn or_crash(self) -> T;
}

impl<T, E: Display> Crash<T> for Result<T, E> {
    fn or_crash(self) -> T {
        match self {
            Ok(content) => content,
            Err(error) => {
                eprintln!("{}", error);
                process::exit(1);
            },
        }
    }
}