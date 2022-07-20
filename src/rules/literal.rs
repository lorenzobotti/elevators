use std::fmt;
use std::str::Chars;

use super::rule::RulePiece;
use super::rule::RulePieceContent;
use crate::spec_parser::char_range::CharRange;
use crate::spec_parser::content::Content;
use crate::spec_parser::strings::trim_end;
use crate::spec_parser::strings::trim_start;
use crate::spec_parser::tokens::DoubleQuote;
use crate::spec_parser::tokens::SingleQuote;

#[derive(Debug, PartialEq, Clone)]
pub struct Literal<'a> {
    pub content: LiteralContent<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralContent<'a> {
    Str(&'a str),
    Range { from: char, to: char },
}

impl<'a> fmt::Display for Literal<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // todo: repeated

        match self.content {
            LiteralContent::Str(string) => write!(f, "{}", string),
            LiteralContent::Range { from, to } => write!(f, "[{}-{}]", from, to),
        }
    }
}

impl<'a> Literal<'a> {
    pub fn match_str<'b>(&self, input: &'b str, repeated: bool) -> Option<&'b str> {
        // todo: a little bit ugly
        match self.content {
            LiteralContent::Range { from, to } => {
                let mut chars: Chars<'b> = input.chars();
                if let Some(rune) = chars.next() {
                    if !in_range(rune, from, to) {
                        return None;
                    }
                }

                if repeated {
                    while let Some(rune) = chars.next() {
                        if !in_range(rune, from, to) {
                            break;
                        }
                    }
                }
                return Some(trim_end(input, chars.as_str()));
            }
            LiteralContent::Str(string) => {
                let mut left: &'b str = trim_start(input, string)?;
                if !repeated {
                    return Some(trim_end(input, left));
                }

                loop {
                    left = match trim_start(left, string) {
                        Some(left) => left,
                        None => return Some(trim_end(input, left)),
                    };
                }
            }
        }
    }
}

impl<'a> From<LiteralContent<'a>> for Literal<'a> {
    fn from(content: LiteralContent<'a>) -> Self {
        Self { content: content }
    }
}

fn in_range(input: char, from: char, to: char) -> bool {
    from <= input && input <= to
}

impl<'a> From<&'a str> for Literal<'a> {
    fn from(string: &'a str) -> Self {
        Self {
            content: LiteralContent::Str(string),
        }
    }
}

impl<'a> From<&'a str> for RulePieceContent<'a> {
    fn from(string: &'a str) -> Self {
        Self::Literal(string.into())
    }
}

impl<'a> From<&'a str> for RulePiece<'a> {
    fn from(string: &'a str) -> Self {
        Self {
            repeated: false,
            content: string.into(),
        }
    }
}

impl<'a> From<&SingleQuote<'a>> for Literal<'a> {
    fn from(quote: &SingleQuote<'a>) -> Self {
        quote.content().into()
    }
}

impl<'a> From<&DoubleQuote<'a>> for Literal<'a> {
    fn from(quote: &DoubleQuote<'a>) -> Self {
        quote.content().into()
    }
}

impl<'a> From<&CharRange> for LiteralContent<'a> {
    fn from(range: &CharRange) -> Self {
        Self::Range {
            from: range.from,
            to: range.to,
        }
    }
}

impl<'a> From<&CharRange> for Literal<'a> {
    fn from(range: &CharRange) -> Self {
        Self {
            content: range.into(),
        }
    }
}

impl<'a> From<&SingleQuote<'a>> for RulePieceContent<'a> {
    fn from(quote: &SingleQuote<'a>) -> Self {
        Self::Literal(quote.into())
    }
}

impl<'a> From<&DoubleQuote<'a>> for RulePieceContent<'a> {
    fn from(quote: &DoubleQuote<'a>) -> Self {
        Self::Literal(quote.into())
    }
}

impl<'a> From<&CharRange> for RulePieceContent<'a> {
    fn from(range: &CharRange) -> Self {
        Self::Literal(range.into())
    }
}
