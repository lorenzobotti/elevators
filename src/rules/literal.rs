use crate::spec_parser::content::Content;
use crate::spec_parser::tokens::DoubleQuote;
use crate::spec_parser::tokens::SingleQuote;

pub struct Literal<'a> {
    repeated: bool,
    content: LiteralContent<'a>,
}

pub enum LiteralContent<'a> {
    Str(&'a str),
    Range { from: char, to: char },
}

impl<'a> From<SingleQuote<'a>> for Literal<'a> {
    fn from(quote: SingleQuote<'a>) -> Self {
        Self {
            repeated: false,
            content: LiteralContent::Str(quote.content()),
        }
    }
}

impl<'a> From<DoubleQuote<'a>> for Literal<'a> {
    fn from(quote: DoubleQuote<'a>) -> Self {
        Self {
            repeated: false,
            content: LiteralContent::Str(quote.content()),
        }
    }
}
