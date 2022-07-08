use crate::utils::take_n;

use super::content::Content;
use super::error::ParseError;
use super::node::Node;
use super::rule_ors::RuleOrs;
use super::tokens::PrimitiveNode;
use super::tokens::{Identifier, Space, COLUMN, SEMICOLUMN};

#[derive(Debug, PartialEq)]
pub struct RuleLine<'a> {
    pub name: &'a str,
    pub rules: RuleOrs<'a>,
}

impl<'a> Node<'a> for RuleLine<'a> {
    fn parse_len(input: &'a str) -> Result<(Self, usize), ParseError> {
        //dbg!("RuleLine");

        let (name, rest) = input.split_once(COLUMN).ok_or(ParseError::ExpectedWhile {
            parsing: "rule line",
            expected: "colon",
            found: take_n(input, 20),
            line: 0,
        })?;
        let name = name.trim();

        let (name, _) = Identifier::parse_len(name).ok_or(ParseError::ExpectedWhile {
            parsing: "rule line",
            expected: "identifier",
            found: take_n(input, 20),
            line: 0,
        })?;

        let rest = match Space::parse_and_skip(rest) {
            Some((_, rest)) => rest,
            None => rest,
        };

        let (rules, rest) = RuleOrs::parse_and_skip(rest)?;

        let rest = match Space::parse_and_skip(rest) {
            Some((_, rest)) => rest,
            None => rest,
        };

        if !rest.starts_with(SEMICOLUMN) {
            return Err(ParseError::ExpectedWhile {
                parsing: "rule line",
                expected: "semicolon",
                found: take_n(input, 20),
                line: 0,
            });
        }

        let rest = rest.trim_start_matches(SEMICOLUMN);
        let diff = input.bytes().len() - rest.bytes().len();
        Ok((
            Self {
                name: name.content(),
                rules: rules,
            },
            diff,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::spec_parser::{rule_piece::RulePiece, rule_series::RuleSeries, tokens::Identifier};

    use super::*;

    #[test]
    fn rule_line() {
        let input = "<my_rule>: <letter> | <letter> <my_rule> ;";
        let expected = RuleLine {
            name: "my_rule",
            rules: RuleOrs(vec![
                RuleSeries(vec![RulePiece::Ident(Identifier("<letter>"))]),
                RuleSeries(vec![
                    RulePiece::Ident(Identifier("<letter>")),
                    RulePiece::Ident(Identifier("<my_rule>")),
                ]),
            ]),
        };

        let (got, _) = RuleLine::parse_len(input).unwrap();
        assert_eq!(expected, got);
    }
}
