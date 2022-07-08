use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use super::grammar::Grammar;
use super::rule::Rule;
use super::rule::RuleList;
use super::rule::RuleOrs;
use super::rule::RulePiece;

use crate::spec_parser::content::Content;
use crate::spec_parser::grammar::Grammar as SpecGrammar;
use crate::spec_parser::rule_line::RuleLine as SpecRuleLine;
use crate::spec_parser::rule_ors::RuleOrs as SpecRuleOrs;
use crate::spec_parser::rule_piece::RulePiece as SpecRulePiece;
use crate::spec_parser::rule_series::RuleSeries as SpecRuleSeries;

use crate::structures::id::Id;

pub trait FromSpec<'a> {
    type Element;
    fn from_spec(elem: &Self::Element, id_gen: &mut Id<&'a str>) -> Self;
}

impl<'a> FromSpec<'a> for RulePiece<'a> {
    type Element = SpecRulePiece<'a>;

    fn from_spec(elem: &Self::Element, id_gen: &mut Id<&'a str>) -> Self {
        match elem {
            SpecRulePiece::Single(quote) => Self::Literal(quote.content()),
            SpecRulePiece::Double(quote) => Self::Literal(quote.content()),
            SpecRulePiece::Ident(ident) => Self::Rule(id_gen.get(ident.content()).0),
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
        dbg!(gram);

        let mut rules = HashMap::new();

        let mut rules_to_check = VecDeque::with_capacity(gram.rules.len());
        let mut id_gen = Id::default();

        let mut parsed = HashSet::new();
        rules_to_check.push_back(gram.main);

        'checking_loop: loop {
            let line = match rules_to_check.pop_front() {
                Some(rule) => rule,
                None => {
                    break 'checking_loop;
                }
            };

            dbg!(line);

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
                    let name = match piece {
                        SpecRulePiece::Ident(ident) => ident.content(),
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
    use crate::spec_parser::tokens::DoubleQuote;
    use crate::spec_parser::tokens::Identifier;
    use crate::spec_parser::tokens::SingleQuote;

    use super::*;

    #[test]
    fn from_rule_piece() {
        let mut id_gen = Id::default();
        let cases = [
            (
                SpecRulePiece::Ident(Identifier("<automobile>")),
                RulePiece::Rule(0),
            ),
            (
                SpecRulePiece::Single(SingleQuote("'burger mobile'")),
                RulePiece::Literal("burger mobile"),
            ),
            (
                SpecRulePiece::Ident(Identifier("<johnny>")),
                RulePiece::Rule(1),
            ),
            (
                SpecRulePiece::Ident(Identifier("<automobile>")),
                RulePiece::Rule(0),
            ),
        ];

        for (input, expected) in cases {
            let got = RulePiece::from_spec(&input, &mut id_gen);

            assert_eq!(expected, got);
        }
    }

    #[test]
    fn from_rule_series() {
        let mut id_gen = Id::default();
        let input = SpecRuleSeries(vec![
            SpecRulePiece::Single(SingleQuote("'the'")),
            SpecRulePiece::Ident(Identifier("<space>")),
            SpecRulePiece::Double(DoubleQuote("\"noun\"")),
            SpecRulePiece::Ident(Identifier("<space>")),
            SpecRulePiece::Double(DoubleQuote("\"was my friend\"")),
            SpecRulePiece::Ident(Identifier("<space>")),
            SpecRulePiece::Ident(Identifier("<period>")),
        ]);

        let expected = RuleList(vec![
            RulePiece::Literal("the"),
            RulePiece::Rule(0),
            RulePiece::Literal("noun"),
            RulePiece::Rule(0),
            RulePiece::Literal("was my friend"),
            RulePiece::Rule(0),
            RulePiece::Rule(1),
        ]);

        let got = RuleList::from_spec(&input, &mut id_gen);

        assert_eq!(expected, got);

        // assert_eq!(expected, got);
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
