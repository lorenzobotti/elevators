use crate::tokens::*;

#[derive(Debug, PartialEq)]
pub enum RulePiece<'a> {
    Single(SingleQuote<'a>),
    Double(DoubleQuote<'a>),
    Ident(Identifier<'a>),
}

impl<'a> Node<'a> for RulePiece<'a> {
    fn parse_len(input: &'a str) -> Option<(Self, usize)> {
        if let Some((parsed, len)) = SingleQuote::parse_len(input) {
            // dbg!("it's a single");
            return Some((Self::Single(parsed), len));
        }
        if let Some((parsed, len)) = DoubleQuote::parse_len(input) {
            // dbg!("it's a double");
            return Some((Self::Double(parsed), len));
        }
        if let Some((parsed, len)) = Identifier::parse_len(input) {
            // dbg!("it's an identifier");
            return Some((Self::Ident(parsed), len));
        }

        // dbg!("it's nothing at all");
        // dbg!(input);
        None
    }
}
