use crate::utils::take_n;

use super::char_range::CharRange;
use super::error::ParseError;
use super::node::Node;
use super::tokens::*;

#[derive(Debug, PartialEq)]
pub enum RulePiece<'a> {
    Single(SingleQuote<'a>),
    Double(DoubleQuote<'a>),
    Ident(Identifier<'a>),
    Range(CharRange),
}

impl<'a> Node<'a> for RulePiece<'a> {
    fn parse_len(input: &'a str) -> Result<(Self, usize), ParseError> {
        if let Some((parsed, len)) = SingleQuote::parse_len(input) {
            return Ok((Self::Single(parsed), len));
        }
        if let Some((parsed, len)) = DoubleQuote::parse_len(input) {
            return Ok((Self::Double(parsed), len));
        }
        if let Some((parsed, len)) = Identifier::parse_len(input) {
            return Ok((Self::Ident(parsed), len));
        }
        if let Ok((parsed, len)) = CharRange::parse_len(input) {
            return Ok((Self::Range(parsed), len));
        }

        Err(ParseError::ExpectedWhile {
            parsing: "rule piece",
            expected: "identifier or quote",
            found: take_n(input, 20),
            line: 0,
        })
    }
}
