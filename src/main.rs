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

use eerie::Files;

fn main() {
    let filename = env::args().nth(1).or_crash(Some("file name needed"));
    let file = fs::read_to_string(filename).or_crash(Some("can't read file"));

    let (files, _) = Files::from_str(&file).or_crash(None);

    let grammar_file = files
        .0
        .iter()
        .find(|file| file.name.ends_with(".inspi"))
        .or_crash(Some("can't find grammar file"))
        .content;

    let input_file = files
        .0
        .iter()
        .find(|file| file.name.starts_with("input"))
        .or_crash(Some("can't find input file"))
        .content;

    let (spec, _) = SpecGrammar::parse_len(grammar_file).or_crash(None);
    let grammar = Grammar::try_from(&spec).or_crash(None);

    let (tree, _) = Node::from_grammar(&grammar, input_file).or_crash(None);
    println!("{}", tree);
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
    fn or_crash(self, message: Option<&str>) -> T;
}

impl<T> Crash<T> for Option<T> {
    fn or_crash(self, message: Option<&str>) -> T {
        match self {
            Some(content) => return content,
            None => {
                match message {
                    Some(mess) => eprintln!("{}", mess),
                    None => eprintln!("empty option"),
                }

                process::exit(1);
            }
        }
    }
}

impl<T, E: Display> Crash<T> for Result<T, E> {
    fn or_crash(self, message: Option<&str>) -> T {
        match self {
            Ok(content) => content,
            Err(error) => {
                match message {
                    Some(mess) => eprintln!("{}", mess),
                    None => eprintln!("{}", error),
                }

                process::exit(1);
            }
        }
    }
}
