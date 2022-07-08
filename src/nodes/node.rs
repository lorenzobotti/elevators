use serde::Serialize;

use crate::rules::grammar::Grammar;
use crate::rules::rule::Rule;
use crate::rules::rule::RuleOrs;
use crate::rules::rule::RuleList;
use crate::rules::rule::RulePiece;
use crate::take_start;

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

impl<'grammar, 'input> Node<'grammar, 'input> {
    pub fn from_grammar(gram: &'grammar Grammar<'grammar>, input: &'input str) -> Option<(Self, usize)> {
        Self::from_rule(gram, gram.main()?, input)
    }
    
    pub fn from_rule(gram: &'grammar Grammar<'grammar>, rule: &Rule<'grammar>, input: &'input str) -> Option<(Self, usize)> {
        let (mut node, len) = Self::from_rule_ors(gram, &rule.rule, input)?;
        node.name = Some(rule.name);
        
        Some((node, len))
    }

    fn from_rule_ors(gram: &'grammar Grammar<'grammar>, ors: &RuleOrs<'grammar>, input: &'input str) -> Option<(Self, usize)> {
        let mut longest: Option<(Self, usize)> = None;

        'or_loop:
        for or in &ors.0 {
            let (node, len) = match Self::from_rule_list(gram, or, input) {
                Some(parsed) => parsed,
                None => continue 'or_loop,
            };  

            if let Some((_, longest_until_now)) = longest {
                if len > longest_until_now {
                    longest = Some((node, len));
                }
            } else {
                longest = Some((node, len));
            }
        }

        longest
    }

    fn from_rule_list(gram: &'grammar Grammar<'grammar>, list: &RuleList<'grammar>, input: &'input str) -> Option<(Self, usize)> {
        let mut rest = input;
        let mut nodes = Vec::with_capacity(list.0.len());

        for node in &list.0 {
            let (parsed, len) = Self::from_rule_piece(gram, &node, rest)?;
            rest = &rest[len..];

            nodes.push(parsed);
        }

        let diff = input.bytes().len() - rest.bytes().len();

        Some((Self{
            name: None,
            content: NodeContent::Cons(nodes),
        }, diff))
    }

    fn from_rule_piece(gram: &'grammar Grammar<'grammar>, piece: &RulePiece<'grammar>, input: &'input str) -> Option<(Self, usize)> {
        match piece {
            RulePiece::Literal(matcher) => {
                let beginning = Self::match_str(input, matcher)?;
                let len = beginning.bytes().len();

                Some((Self{
                    name: None,
                    content: NodeContent::Literal(Self::match_str(input, matcher)?)
                }, len))
            },
            RulePiece::Rule(ruleref) => {
                Some(Self::from_rule(gram, gram.get(*ruleref)?, input)?)
            },
        }
    }

    fn match_str(input: &'input str, matcher: &'grammar str) -> Option<&'input str> {
        Some(take_start!(input, matcher)?)
    }
}


#[cfg(test)]
mod tests {
//     use super::*;
//     use crate::spec_parser::grammar::Grammar as SpecGrammar;
//     use crate::spec_parser::tokens::Node as NodeTrait;

//     #[test]
//     fn from_grammar() {
//         let input = "il cane";
//         let grammar_input = r#"
// <noun_phrase>: <article> ' ' <noun>;
// <article>: 'il' | 'lo';
// <noun>: 'cane' | 'gatto';
// "#.trim();

//         let (spec_grammar, _) = SpecGrammar::parse_len(grammar_input).unwrap();
//         let grammar = Grammar::try_from(&spec_grammar).unwrap();

//         let node = Node::from_grammar(&grammar, input).unwrap();

//         todo!();
//     }
}