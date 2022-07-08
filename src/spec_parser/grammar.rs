use std::collections::HashMap;

use crate::spec_parser::error::ParseError;
use crate::spec_parser::node::Node;
use crate::spec_parser::rule_line::RuleLine;

#[derive(Debug)]
pub struct Grammar<'a> {
    pub main: &'a str,
    pub rules: HashMap<&'a str, RuleLine<'a>>,
}

impl<'a> Node<'a> for Grammar<'a> {
    fn parse_len(input: &'a str) -> Result<(Self, usize), ParseError> {
        let mut rules = HashMap::new();
        let mut first_rule = None;

        let mut left = input;
        'parse_loop: loop {
            let rule = match RuleLine::parse_and_skip(left) {
                Ok((rule, trim)) => {
                    left = trim;
                    rule
                }
                Err(error) => match first_rule {
                    Some(_) => break 'parse_loop,
                    None => return Err(error),
                },
            };

            if first_rule.is_none() {
                first_rule = Some(rule.name)
            }

            rules.insert(rule.name, rule);
        }

        let diff = input.bytes().len() - left.bytes().len();
        match rules.len() {
            0 => unreachable!(),
            _ => Ok((
                Self {
                    main: first_rule.unwrap(),
                    rules: rules,
                },
                diff,
            )),
        }
    }
}
