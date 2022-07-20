use serde::Serialize;

use super::error::ParseError;
use crate::rules::grammar::Grammar;
use crate::rules::literal::LiteralContent;
use crate::rules::rule::Rule;
use crate::rules::rule::RuleList;
use crate::rules::rule::RuleOrs;
use crate::rules::rule::RulePiece;
use crate::rules::rule::RulePieceContent;
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
            expected: name.into(),
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

        let name = match &piece.content {
            RulePieceContent::Literal(literal) => match literal.content {
                LiteralContent::Range { from: _, to: _ } => "char range",
                LiteralContent::Str(string) => string,
            },
            RulePieceContent::Rule(r) => gram.get(*r).unwrap().name,
        };

        match &piece.content {
            RulePieceContent::Literal(matcher) => {
                let beginning =
                    matcher
                        .match_str(input, piece.repeated)
                        .ok_or(ParseError::Expected {
                            parsing: "terminal",
                            expected: matcher.to_string(),
                            got: take_n(input, 20),
                        })?;

                let len = beginning.bytes().len();
                let content = NodeContent::Literal(beginning);

                Ok((
                    Self {
                        name: Some(name),
                        content: content,
                    },
                    len,
                ))
            }
            RulePieceContent::Rule(ruleref) => {
                let rule = gram.get(*ruleref).unwrap();
                let node = Self::from_rule(gram, rule, input)?;
                Ok(node)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rustc_hash::FxHashMap;
    use super::*;

    #[test]
    fn ors() {
        let input = "cane";
        let rules = RuleOrs(vec![
            RuleList(vec![RulePiece {
                content: RulePieceContent::Literal("Marco".into()),
                repeated: true,
            }]),
            RuleList(vec![RulePiece {
                content: RulePieceContent::Literal("gallina".into()),
                repeated: true,
            }]),
            RuleList(vec![RulePiece {
                content: RulePieceContent::Literal("gatto".into()),
                repeated: true,
            }]),
            RuleList(vec![RulePiece {
                content: RulePieceContent::Literal("cane".into()),
                repeated: true,
            }]),
        ]);

        let rule = Rule {
            name: "animale",
            rule: rules.clone(),
        };
        let mut rules_map = FxHashMap::default();
        rules_map.insert(0, rule);

        let grammar = Grammar { rules: rules_map };

        let (node, _) = Node::from_grammar(&grammar, input).unwrap();

        let expected = Node {
            name: Some("animale"),
            content: NodeContent::Cons(vec![Node {
                name: Some("cane"),
                content: NodeContent::Literal("cane"),
            }]),
        };

        assert_eq!(node, expected);
    }

    // #[test]
    // fn repeated() {
    //     let input = "giovanni                      come stai?";
    //     let rules = [
    //         Rule {
    //             name: "soggetto",
    //             rule: RuleOrs(vec![
    //                 RuleList(vec!["giovanni".into()]),
    //             ])
    //         },
    //         Rule {
    //             name: "spazio",
    //             rule: RuleOrs(vec![
    //                 RuleList(vec![
    //                     RulePiece {
    //                         repeated: true,
    //                         content: RulePieceContent::Literal(" ".into()),
    //                     }
    //                     ]),
    //                 ])
    //             },
    //             Rule {
    //                 name: "soggetto",
    //                 rule: RuleOrs(vec![
    //                     RuleList(vec!["come stai?".into()]),
    //                 ])
    //             },
    //     ];

    //     let grammar: Grammar = rules.try_into();
    // }
}
