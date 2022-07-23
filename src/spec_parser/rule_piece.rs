use crate::utils::take_n;

use super::char_range::CharRange;
use super::error::ParseError;
use super::node::Node;
use super::tokens::*;

#[derive(Debug, PartialEq)]
pub struct RulePiece<'a> {
    pub content: RulePieceContent<'a>,
    pub repetition: Repetition,
}

#[derive(Debug, PartialEq)]
pub enum RulePieceContent<'a> {
    Single(SingleQuote<'a>),
    Double(DoubleQuote<'a>),
    Ident(Identifier<'a>),
    Range(CharRange),
}

pub const REPEAT_TOGETHER: char = '+';
pub const REPEAT_SEPARATE: char = '*';

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Repetition {
    Single,
    RepeatTogether,
    RepeatSeparate,
}

impl<'a> Node<'a> for RulePiece<'a> {
    fn parse_len(input: &'a str) -> Result<(Self, usize), ParseError> {
        let (primitive, len) = if let Some((parsed, len)) = SingleQuote::parse_len(input) {
            (RulePieceContent::Single(parsed), len)
        } else if let Some((parsed, len)) = DoubleQuote::parse_len(input) {
            (RulePieceContent::Double(parsed), len)
        } else if let Some((parsed, len)) = Identifier::parse_len(input) {
            (RulePieceContent::Ident(parsed), len)
        } else if let Ok((parsed, len)) = CharRange::parse_len(input) {
            (RulePieceContent::Range(parsed), len)
        } else {
            return Err(ParseError::ExpectedWhile {
                parsing: "rule piece",
                expected: "identifier or quote",
                found: take_n(input, 20),
                line: 0,
            });
        };

        let rest = &input[len..];
        let (repetition, rest) = if rest.starts_with(REPEAT_TOGETHER) {
            (Repetition::RepeatTogether, &rest[1..])
        } else if rest.starts_with(REPEAT_SEPARATE) {
            (Repetition::RepeatSeparate, &rest[1..])
        } else {
            (Repetition::Single, rest)
        };

        let len = input.bytes().len() - rest.bytes().len();

        Ok((
            Self {
                content: primitive,
                repetition: repetition,
            },
            len,
        ))
    }
}

impl<'a> From<RulePieceContent<'a>> for RulePiece<'a> {
    fn from(content: RulePieceContent<'a>) -> Self {
        Self {
            content: content,
            repetition: Repetition::Single,
        }
    }
}

impl<'a> From<SingleQuote<'a>> for RulePieceContent<'a> {
    fn from(quote: SingleQuote<'a>) -> Self {
        Self::Single(quote)
    }
}

impl<'a> From<DoubleQuote<'a>> for RulePieceContent<'a> {
    fn from(quote: DoubleQuote<'a>) -> Self {
        Self::Double(quote)
    }
}

impl<'a> From<Identifier<'a>> for RulePieceContent<'a> {
    fn from(ident: Identifier<'a>) -> Self {
        Self::Ident(ident)
    }
}

impl<'a> From<CharRange> for RulePieceContent<'a> {
    fn from(range: CharRange) -> Self {
        Self::Range(range)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repetition() {
        let cases = [
            (
                "' '+",
                RulePiece {
                    content: RulePieceContent::Single(SingleQuote("' '")),
                    repetition: Repetition::RepeatTogether,
                },
            ),
            (
                "<key_pair>*",
                RulePiece {
                    content: RulePieceContent::Ident(Identifier("<key_pair>")),
                    repetition: Repetition::RepeatSeparate,
                },
            ),
            (
                "\"burger\"",
                RulePiece {
                    content: RulePieceContent::Double(DoubleQuote("\"burger\"")),
                    repetition: Repetition::Single,
                },
            ),
        ];

        for (input, expected) in cases {
            let (got, _) = RulePiece::parse_len(input).unwrap();
            assert_eq!(expected, got);
        }
    }
}
