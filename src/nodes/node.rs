use serde::Serialize;

use super::error::ParseError;
use crate::rules::grammar::Grammar;
use crate::rules::rule::Rule;
use crate::rules::rule::RuleList;
use crate::rules::rule::RuleOrs;
use crate::rules::rule::RulePiece;
use crate::take_start;
use crate::utils::take_n;

#[derive(Debug, PartialEq, Serialize)]
pub struct Node<'grammar, 'input> {
    pub name: Option<&'grammar str>,
    pub content: NodeContent<'grammar, 'input>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum NodeContent<'grammar, 'input> {
    Literal(&'input str),
    Cons(Vec<Node<'grammar, 'input>>),
}

impl<'g, 'i> Node<'g, 'i> {
    pub fn from_grammar(
        gram: &'g Grammar<'g>,
        input: &'i str,
    ) -> Result<(Self, usize), ParseError<'g, 'i>> {
        Self::from_rule(gram, gram.main().unwrap(), input)
    }

    pub fn from_rule(
        gram: &'g Grammar<'g>,
        rule: &Rule<'g>,
        input: &'i str,
    ) -> Result<(Self, usize), ParseError<'g, 'i>> {
        let (mut node, len) = Self::from_rule_ors(gram, &rule.rule, rule.name, input)?;
        node.name = Some(rule.name);

        Ok((node, len))
    }

    fn from_rule_ors(
        gram: &'g Grammar<'g>,
        ors: &RuleOrs<'g>,
        name: &'g str,
        input: &'i str,
    ) -> Result<(Self, usize), ParseError<'g, 'i>> {
        let mut longest: Option<(Self, usize)> = None;

        'or_loop: for or in &ors.0 {
            let (node, len) = match Self::from_rule_list(gram, or, input) {
                Ok(parsed) => parsed,
                Err(error) => match &ors.0.len() {
                    0 => return Err(error),
                    _ => continue 'or_loop,
                },
            };

            if let Some((_, longest_until_now)) = longest {
                if len > longest_until_now {
                    longest = Some((node, len));
                }
            } else {
                longest = Some((node, len));
            }
        }

        longest.ok_or(ParseError::Expected {
            parsing: name,
            expected: name,
            got: take_n(input, 20),
        })
    }

    fn from_rule_list(
        gram: &'g Grammar<'g>,
        list: &RuleList<'g>,
        input: &'i str,
    ) -> Result<(Self, usize), ParseError<'g, 'i>> {
        let mut rest = input;
        let mut nodes = Vec::with_capacity(list.0.len());

        for node in &list.0 {
            let (parsed, len) = Self::from_rule_piece(gram, &node, rest)?;
            rest = &rest[len..];

            nodes.push(parsed);
        }

        let diff = input.bytes().len() - rest.bytes().len();

        Ok((
            Self {
                name: None,
                content: NodeContent::Cons(nodes),
            },
            diff,
        ))
    }

    fn from_rule_piece(
        gram: &'g Grammar<'g>,
        piece: &RulePiece<'g>,
        input: &'i str,
    ) -> Result<(Self, usize), ParseError<'g, 'i>> {
        match piece {
            RulePiece::Literal(matcher) => {
                let beginning = Self::match_str(input, matcher).ok_or(ParseError::Expected {
                    parsing: "terminal",
                    expected: matcher,
                    got: take_n(input, 20),
                })?;

                let len = beginning.bytes().len();
                let content = NodeContent::Literal(beginning);

                Ok((
                    Self {
                        name: None,
                        content: content,
                    },
                    len,
                ))
            }
            RulePiece::Rule(ruleref) => {
                let rule = gram.get(*ruleref).unwrap();
                let node = Self::from_rule(gram, rule, input)?;
                Ok(node)
            }
        }
    }

    fn match_str(input: &'i str, matcher: &'g str) -> Option<&'i str> {
        Some(take_start!(input, matcher)?)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn ors() {
        let input = "cane";
        let rules = RuleOrs(vec![
            RuleList(vec![RulePiece::Literal("Marco")]),
            RuleList(vec![RulePiece::Literal("gallina")]),
            RuleList(vec![RulePiece::Literal("gatto")]),
            RuleList(vec![RulePiece::Literal("cane")]),
        ]);

        let rule = Rule {
            name: "animale",
            rule: rules.clone(),
        };
        let mut rules_map = HashMap::default();
        rules_map.insert(0, rule);

        let grammar = Grammar { rules: rules_map };

        let (node, _) = Node::from_grammar(&grammar, input).unwrap();

        let expected = Node {
            name: Some("animale"),
            content: NodeContent::Cons(vec![Node {
                name: None,
                content: NodeContent::Literal("cane"),
            }]),
        };

        assert_eq!(node, expected);
    }
}
