use trait_enum::Deref;

use crate::tokens::*;

pub enum Token<'a> {
    Assign(Assign<'a>),
    Separator(Separator<'a>),
    SingleQuote(SingleQuote<'a>),
    DoubleQuote(DoubleQuote<'a>),
    Identifier(Identifier<'a>),
    Space(Space<'a>),
}

// impl<'a> Deref for Token<'a> {
//     type Target = dyn Node<'a>;
// }
