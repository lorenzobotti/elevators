use std::collections::VecDeque;

use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;

use super::grammar::Grammar;
use super::rule::Rule;
use super::rule::RuleList;
use super::rule::RuleOrs;
use super::rule::RulePiece;
use super::rule::RulePieceContent;

use crate::spec_parser::content::Content;
use crate::spec_parser::grammar::Grammar as SpecGrammar;
use crate::spec_parser::rule_line::RuleLine as SpecRuleLine;
use crate::spec_parser::rule_ors::RuleOrs as SpecRuleOrs;
use crate::spec_parser::rule_piece::RulePiece as SpecRulePiece;
use crate::spec_parser::rule_piece::RulePieceContent as SpecRulePieceContent;
use crate::spec_parser::rule_series::RuleSeries as SpecRuleSeries;

use crate::structures::id::Id;

pub trait FromSpec<'a> {
    type Element;
    fn from_spec(elem: &Self::Element, id_gen: &mut Id<&'a str>) -> Self;
}

impl<'a> FromSpec<'a> for RulePiece<'a> {
    type Element = SpecRulePiece<'a>;

    fn from_spec(elem: &Self::Element, id_gen: &mut Id<&'a str>) -> Self {
        let content = match &elem.content {
            SpecRulePieceContent::Single(quote) => {
                RulePieceContent::from(RulePieceContent::from(quote))
            }
            SpecRulePieceContent::Double(quote) => {
                RulePieceContent::from(RulePieceContent::from(quote))
            }
            SpecRulePieceContent::Ident(ident) => {
                RulePieceContent::Rule(id_gen.get(ident.content()).0)
            }
            SpecRulePieceContent::Range(range) => {
                RulePieceContent::from(RulePieceContent::from(range))
            }
        };

        let repetition = elem.repetition;
        Self {
            repetition: repetition,
            content: content,
        }
    }
}

impl<'a> FromSpec<'a> for RuleList<'a> {
    type Element = SpecRuleSeries<'a>;

    fn from_spec(elem: &Self::Element, id_gen: &mut Id<&'a str>) -> Self {
        let pieces: Vec<RulePiece<'a>> = elem
            .0
            .iter()
            .map(|piece| RulePiece::from_spec(piece, id_gen))
            .collect();

        Self(pieces)
    }
}

impl<'a> FromSpec<'a> for RuleOrs<'a> {
    type Element = SpecRuleOrs<'a>;

    fn from_spec(elem: &Self::Element, id_gen: &mut Id<&'a str>) -> Self {
        let pieces: Vec<RuleList<'a>> = elem
            .0
            .iter()
            .map(|piece| RuleList::from_spec(piece, id_gen))
            .collect();

        Self(pieces)
    }
}

impl<'a> FromSpec<'a> for Rule<'a> {
    type Element = SpecRuleLine<'a>;

    fn from_spec(elem: &Self::Element, id_gen: &mut Id<&'a str>) -> Self {
        let ors = RuleOrs::from_spec(&elem.rules, id_gen);
        Self {
            name: elem.name,
            rule: ors,
        }
    }
}

impl<'a> TryFrom<&SpecGrammar<'a>> for Grammar<'a> {
    type Error = String;

    fn try_from(gram: &SpecGrammar<'a>) -> Result<Self, Self::Error> {
        let mut rules = FxHashMap::default();

        let mut rules_to_check = VecDeque::with_capacity(gram.rules.len());
        let mut id_gen = Id::default();

        let mut parsed = FxHashSet::default();
        rules_to_check.push_back(gram.main);

        'checking_loop: loop {
            let line = match rules_to_check.pop_front() {
                Some(rule) => rule,
                None => {
                    break 'checking_loop;
                }
            };

            if parsed.contains(line) {
                continue 'checking_loop;
            }

            parsed.insert(line);
            let (id, _) = id_gen.get(line);

            let line = match gram.rules.get(line) {
                Some(rule) => rule,
                None => return Err(format!("can't find rule <{}>", line)),
            };

            let rule = Rule::from_spec(line, &mut id_gen);
            rules.insert(id, rule);

            for series in &line.rules.0 {
                'check_piece: for piece in &series.0 {
                    let name = match &piece.content {
                        SpecRulePieceContent::Ident(ident) => ident.content(),
                        _ => continue 'check_piece,
                    };

                    rules_to_check.push_back(name);
                }
            }
        }

        Ok(Self { rules })
    }
}

#[cfg(test)]
mod tests {
    use crate::rules::literal::LiteralContent;
    use crate::spec_parser::node::Node;
    use crate::spec_parser::rule_piece::Repetition;
    use crate::spec_parser::tokens::DoubleQuote;
    use crate::spec_parser::tokens::Identifier;
    use crate::spec_parser::tokens::SingleQuote;

    use super::*;

    #[test]
    fn from_raw_spec() {
        let input = r#"
<sentence>: <subject> <space>+ "li" <space>+ <predicate>;
<subject>: <noun_phrase>;
<noun_phrase>: <word> | <word> <space>+ <noun_phrase>;
<object>: <noun_phrase>;
<space>: ' ';
<word>: 'mi' | 'sina' | 'lape' | 'pona' | 'mute';
<predicate>: <noun_phrase>;"#
            .trim();

        // <sentence>: 0
        // <subject>: 1
        // <space>: 2
        // <predicate>: 3
        // <noun_phrase>: 4
        // <word>: 5

        let expected_sentence = Rule {
            name: "sentence",
            rule: RuleOrs(vec![RuleList(vec![
                RulePiece {
                    content: RulePieceContent::Rule(1),
                    repetition: Repetition::Single,
                },
                RulePiece {
                    content: RulePieceContent::Rule(2),
                    repetition: Repetition::RepeatTogether,
                },
                RulePiece {
                    content: RulePieceContent::Literal(LiteralContent::Str("li").into()),
                    repetition: Repetition::Single,
                },
                RulePiece {
                    content: RulePieceContent::Rule(2),
                    repetition: Repetition::RepeatTogether,
                },
                RulePiece {
                    content: RulePieceContent::Rule(3),
                    repetition: Repetition::Single,
                },
            ])]),
        };

        let (spec_grammar, _) = SpecGrammar::parse_len(input).unwrap();
        let grammar = Grammar::try_from(&spec_grammar).unwrap();
        let sentence_gotten = grammar.get(0).unwrap();

        for i in 0..grammar.rules.len() {
        }

        assert_eq!(&expected_sentence, sentence_gotten);
    }

    #[test]
    fn from_rule_piece() {
        let mut id_gen = Id::default();
        let cases = [
            (
                SpecRulePieceContent::from(Identifier("<automobile>")).into(),
                RulePieceContent::Rule(0),
            ),
            (
                SpecRulePieceContent::from(SingleQuote("'burger mobile'")).into(),
                RulePieceContent::Literal("burger mobile".into()),
            ),
            (
                SpecRulePieceContent::from(Identifier("<johnny>")).into(),
                RulePieceContent::Rule(1),
            ),
            (
                SpecRulePieceContent::from(Identifier("<automobile>")).into(),
                RulePieceContent::Rule(0),
            ),
        ];

        for (input, expected) in cases {
            let got = RulePiece::from_spec(&input, &mut id_gen).content;

            assert_eq!(expected, got);
        }
    }

    #[test]
    fn from_rule_series() {
        let mut id_gen = Id::default();
        let input = SpecRuleSeries(vec![
            SpecRulePieceContent::from(SingleQuote("'the'")).into(),
            SpecRulePieceContent::from(Identifier("<space>")).into(),
            SpecRulePieceContent::from(DoubleQuote("\"noun\"")).into(),
            SpecRulePieceContent::from(Identifier("<space>")).into(),
            SpecRulePieceContent::from(DoubleQuote("\"was my friend\"")).into(),
            SpecRulePieceContent::from(Identifier("<space>")).into(),
            SpecRulePieceContent::from(Identifier("<period>")).into(),
        ]);

        let expected = RuleList(vec![
            RulePieceContent::from("the").into(),
            RulePieceContent::from(RulePieceContent::Rule(0)).into(),
            RulePieceContent::from("noun").into(),
            RulePieceContent::from(RulePieceContent::Rule(0)).into(),
            RulePieceContent::from("was my friend").into(),
            RulePieceContent::from(RulePieceContent::Rule(0)).into(),
            RulePieceContent::from(RulePieceContent::Rule(1)).into(),
        ]);

        let got = RuleList::from_spec(&input, &mut id_gen);

        assert_eq!(expected, got);
    }

    // #[test]
    //     fn from_grammar() {
    //         let input = r#"
    // <sentence>: <subject> ' ' <verb> ' ' <object>;
    // <subject>: <article> ' ' <noun>;
    // <article>: 'il' | 'lo';
    // <noun>: 'cane' | 'zio' | 'nonno';
    // <verb>: 'mangia';
    // <object>: <subject>;
    // "#.trim();

    //         let expected = Rule {
    //             name: "sentence",
    //             rule: RuleOrs(vec![
    //                 RuleList(vec![
    //                     RulePiece::Rule(0),
    //                     RulePiece::Literal(" "),
    //                     RulePiece::Rule(1),
    //                     RulePiece::Literal(" "),
    //                     RulePiece::Rule(2),
    //                 ]),
    //             ]),
    //         };

    //         let (spec, _) = SpecGrammar::parse_len(input).unwrap();
    //         let grammar = Grammar::try_from(&spec).unwrap();
    //         //dbg!(&grammar);
    //         //dbg!(&grammar.rules.len());

    //         panic!();
    //     }
}
