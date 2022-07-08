use crate::spec_parser::content::Content;
use crate::spec_parser::tokens::DoubleQuote;
use crate::spec_parser::tokens::SingleQuote;

pub struct Literal<'a>(pub &'a str);

impl<'a> From<SingleQuote<'a>> for Literal<'a> {
    fn from(quote: SingleQuote<'a>) -> Self {
        Self(quote.content())
    }
}

impl<'a> From<DoubleQuote<'a>> for Literal<'a> {
    fn from(quote: DoubleQuote<'a>) -> Self {
        Self(quote.content())
    }
}
