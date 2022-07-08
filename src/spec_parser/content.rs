use super::tokens::DoubleQuote;
use super::tokens::Identifier;
use super::tokens::SingleQuote;

pub trait Content<'a> {
    fn content(&self) -> &'a str;
}

// we trust that we have parsed
// the node correctly, and that
// it begins and ends with a single
// byte character
macro_rules! derive_content {
    ($type: ty) => {
        impl<'a> Content<'a> for $type {
            fn content(&self) -> &'a str {
                let trimmed = &self.0[1..(self.0.bytes().len() - 1)];
                trimmed
            }
        }
    };
}

derive_content!(Identifier<'a>);
derive_content!(SingleQuote<'a>);
derive_content!(DoubleQuote<'a>);
