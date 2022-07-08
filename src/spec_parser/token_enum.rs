use crate::spec_parser::tokens::*;

#[allow(dead_code)]
pub enum Token<'a> {
    Assign(Assign<'a>),
    Separator(Separator<'a>),
    SingleQuote(SingleQuote<'a>),
    DoubleQuote(DoubleQuote<'a>),
    Identifier(Identifier<'a>),
    Space(Space<'a>),
}
