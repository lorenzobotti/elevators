use std::fmt::Display;
#[allow(unused_imports)]
use std::fs;
use std::io::{stdin, Read};
#[allow(unused_imports)]
use std::{env, process};

mod nodes;
mod rules;
mod spec_parser;
mod structures;
mod utils;

use nodes::node::Node;
use rules::grammar::Grammar;
use spec_parser::grammar::Grammar as SpecGrammar;
use spec_parser::node::Node as NodeTrait;

fn main() {
    let input_grammatica = r#"
    <parola>: <lettera> | <lettera> <parola>;
    <lettera>: 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G'
    | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N'
    | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U'
    | 'V' | 'W' | 'X' | 'Y' | 'Z' | 'a' | 'b'
    | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i'
    | 'j' | 'k' | 'l' | 'm' | 'n' | 'o' | 'p'
    | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w'
    | 'x' | 'y' | 'z';"#
        .trim();

    let (grammatica, _) = SpecGrammar::parse_len(input_grammatica).unwrap();
    let input = "johnny ";
    let grammatica = Grammar::try_from(&grammatica).unwrap();
    let node = Node::from_grammar(&grammatica, input).unwrap();
    dbg!(node);

    return;

    // let grammar_file = env::args().nth(1).expect("couldn't parse filename");
    // let grammar_raw = fs::read_to_string(grammar_file).expect("couldn't read file");

    // let (spec, _) = SpecGrammar::parse_and_skip(&grammar_raw).or_crash();
    // let grammar = Grammar::try_from(&spec).or_crash();

    // let input = read_input().expect("can't read stdin");
    // let (parsed, _) = Node::from_grammar(&grammar, &input).or_crash();

    // println!("{}", &parsed);
}

#[allow(dead_code)]
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
            }
        }
    }
}
