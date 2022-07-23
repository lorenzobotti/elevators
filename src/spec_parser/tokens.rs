use crate::take_start;

pub const SINGLE_QUOTE: char = '\'';
pub const DOUBLE_QUOTE: char = '"';
pub const ASSIGN: &str = "->";
pub const START_IDENT: char = '<';
pub const STOP_IDENT: char = '>';
pub const SEPARATOR: char = '|';
pub const COLUMN: char = ':';
pub const SEMICOLUMN: char = ';';

pub trait PrimitiveNode<'a>: Sized {
    fn parse_len(input: &'a str) -> Option<(Self, usize)>;
    fn parse_and_skip(input: &'a str) -> Option<(Self, &'a str)> {
        let (node, len) = Self::parse_len(input)?;
        Some((node, &input[len..]))
    }
}

macro_rules! literal {
    ($type: ty, $lit: expr) => {
        impl<'a> PrimitiveNode<'a> for $type {
            fn parse_len(input: &'a str) -> Option<(Self, usize)> {
                let start = take_start!(input, $lit);
                Some((Self(start?), start?.bytes().len()))
            }
        }
    };
}

macro_rules! delimited {
    ($type: ty, $start: expr, $end: expr, $matcher: expr) => {
        impl<'a> PrimitiveNode<'a> for $type {
            fn parse_len(input: &'a str) -> Option<(Self, usize)> {
                let starting_size = input.bytes().len();

                let trimmed = input.strip_prefix($start)?;

                let content = match take_start!(trimmed, $matcher) {
                    Some(c) => c,
                    None => "",
                };

                dbg!($start, $end, content);
                let trimmed = trimmed.strip_prefix(content)?;

                take_start!(trimmed, $end)?;
                let trimmed = trimmed.strip_prefix($end)?;

                let finished_size = trimmed.bytes().len();
                let diff = starting_size - finished_size;
                Some((Self(&input[..diff]), diff))
            }
        }
    };
}

#[derive(PartialEq, Debug)]
pub struct Assign<'a>(pub &'a str);
literal!(Assign<'a>, ASSIGN);
#[derive(PartialEq, Debug)]
pub struct Separator<'a>(pub &'a str);
literal!(Separator<'a>, SEPARATOR);

#[derive(PartialEq, Debug)]
pub struct SingleQuote<'a>(pub &'a str);
delimited!(SingleQuote<'a>, SINGLE_QUOTE, SINGLE_QUOTE, |c| c
    != SINGLE_QUOTE);

#[derive(PartialEq, Debug)]
pub struct DoubleQuote<'a>(pub &'a str);
delimited!(DoubleQuote<'a>, DOUBLE_QUOTE, DOUBLE_QUOTE, |c| c
    != DOUBLE_QUOTE);

#[derive(PartialEq, Debug)]
pub struct Identifier<'a>(pub &'a str);
delimited!(Identifier<'a>, START_IDENT, STOP_IDENT, |c: char| c
    .is_alphabetic()
    || c == '_');

#[derive(PartialEq, Debug)]
pub struct Space<'a>(pub &'a str);
literal!(Space<'a>, |c| char::is_ascii_whitespace(&c));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_quote() {
        let input = r#"'hamburger mobile'"#;
        let (took, _) = SingleQuote::parse_len(input).unwrap();

        assert_eq!(took, SingleQuote(r#"'hamburger mobile'"#));
    }

    #[test]
    fn identifier() {
        let input = "<johnny_boy>";
        let (took, _) = Identifier::parse_len(input).unwrap();

        assert_eq!(took, Identifier("<johnny_boy>"));
    }
}
