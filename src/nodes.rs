use crate::bnf::{skip, start_match};

pub const SINGLE_QUOTE: char = '\'';
pub const DOUBLE_QUOTE: char = '"';
pub const ASSIGN: &str = "->";
pub const START_IDENT: char = '<';
pub const STOP_IDENT: char = '>';
pub const SEPARATOR: char = '|';

#[derive(Debug, PartialEq)]
pub struct Space<'a>(&'a str);

#[derive(Debug, PartialEq)]
pub struct SingleQuote<'a>(&'a str);

#[derive(Debug, PartialEq)]
pub struct DoubleQuote<'a>(&'a str);

#[derive(Debug, PartialEq)]
pub struct Identifier<'a>(&'a str);

#[derive(Debug, PartialEq)]
pub struct Comment<'a>(&'a str);

#[derive(Debug, PartialEq)]
pub struct Assign<'a>(&'a str);

#[derive(Debug, PartialEq)]
pub struct Separator<'a>(&'a str);

//ohhhhh ma gaaawwwdddd

pub trait Node<'a>: Sized {
    fn parse(input: &'a str) -> Option<Self>;
}

pub trait PrimitiveNode<'a>: Node<'a> {
    fn as_str(&self) -> &'a str;
    fn skip_source(&self, input: &'a str) -> Option<&'a str> {
        let rest = input.trim_start_matches(self.as_str());
        let diff = input.bytes().len() - rest.bytes().len();
        if diff != 0 {
            Some(&input[diff..])
        } else {
            None
        }
    }
}

impl<'a> Node<'a> for Space<'a> {
    fn parse(input: &'a str) -> Option<Self> {
        Some(Self(start_match(input, char::is_whitespace)?))
    }
}

impl<'a> Node<'a> for SingleQuote<'a> {
    fn parse(input: &'a str) -> Option<Self> {
        let line = input.trim_start_matches(SINGLE_QUOTE);
        if line.len() == input.len() {
            return None;
        }

        let end = line.find(SINGLE_QUOTE)? + 1;
        let out = &input[..=end];
        Some(Self(out))
    }
}

impl<'a> Node<'a> for DoubleQuote<'a> {
    fn parse(input: &'a str) -> Option<Self> {
        let line = input.trim_start_matches(DOUBLE_QUOTE);
        if line.len() == input.len() {
            return None;
        }

        let end = line.find(DOUBLE_QUOTE)? + 1;
        let out = &input[..=end];
        Some(Self(out))
    }
}

impl<'a> Node<'a> for Identifier<'a> {
    fn parse(input: &'a str) -> Option<Self> {
        if !input.starts_with(START_IDENT) {
            return None;
        }
        // ifound MONGUS suddenbtly i found amongus aming us amongus suddently i found amongus

        // TODO: benchmark di questo
        let line = skip(input, &START_IDENT.to_string())?;
        let end = line.find(STOP_IDENT)? + 2;
        Some(Self(&input[..end]))
    }
}

impl<'a> Node<'a> for Comment<'a> {
    fn parse(input: &'a str) -> Option<Self> {
        if !input.starts_with(';') {
            return None;
        }
        let is_not_newline = |c| !['\n', '\r'].contains(&c);

        Some(Self(start_match(input, is_not_newline)?))
    }
}

impl<'a> Node<'a> for Assign<'a> {
    fn parse(input: &'a str) -> Option<Self> {
        if input.starts_with(ASSIGN) {
            Some(Self(&input[..(ASSIGN.len())]))
        } else {
            None
        }
    }
}

impl<'a> Node<'a> for Separator<'a> {
    fn parse(input: &'a str) -> Option<Self> {
        if input.starts_with(SEPARATOR) {
            Some(Self(&input[..1]))
        } else {
            None
        }
    }
}

impl<'a> PrimitiveNode<'a> for Space<'a> {
    fn as_str(&self) -> &'a str {
        self.0
    }
}
impl<'a> PrimitiveNode<'a> for SingleQuote<'a> {
    fn as_str(&self) -> &'a str {
        self.0
    }
}
impl<'a> PrimitiveNode<'a> for DoubleQuote<'a> {
    fn as_str(&self) -> &'a str {
        self.0
    }
}
impl<'a> PrimitiveNode<'a> for Identifier<'a> {
    fn as_str(&self) -> &'a str {
        self.0
    }
}
impl<'a> PrimitiveNode<'a> for Comment<'a> {
    fn as_str(&self) -> &'a str {
        self.0
    }
}
impl<'a> PrimitiveNode<'a> for Assign<'a> {
    fn as_str(&self) -> &'a str {
        self.0
    }
}
impl<'a> PrimitiveNode<'a> for Separator<'a> {
    fn as_str(&self) -> &'a str {
        self.0
    }
}

pub enum Primitive<'a> {
    Space(Space<'a>),
    SingleQuote(SingleQuote<'a>),
    DoubleQuote(DoubleQuote<'a>),
    Identifier(Identifier<'a>),
    Comment(Comment<'a>),
    Assign(Assign<'a>),
    Separator(Separator<'a>),
}

fn parse_primitive<'a>(input: &'a str) -> Option<Primitive<'a>> {
    if let Some(n) = Space::parse(input) {
        return Some(Primitive::Space(n));
    }
    if let Some(n) = SingleQuote::parse(input) {
        return Some(Primitive::SingleQuote(n));
    }
    if let Some(n) = DoubleQuote::parse(input) {
        return Some(Primitive::DoubleQuote(n));
    }
    if let Some(n) = Identifier::parse(input) {
        return Some(Primitive::Identifier(n));
    }
    if let Some(n) = Comment::parse(input) {
        return Some(Primitive::Comment(n));
    }
    if let Some(n) = Assign::parse(input) {
        return Some(Primitive::Assign(n));
    }
    if let Some(n) = Separator::parse(input) {
        return Some(Primitive::Separator(n));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::SingleQuote;
    use super::*;
    #[test]
    fn quotes_and_separator() {
        let input = "'burger' | \"ciao\"";
        let quote = SingleQuote::parse(input).unwrap();
        assert_eq!(quote, SingleQuote("'burger'"));
        let input = quote.skip_source(input).unwrap();

        let space = Space::parse(input).unwrap();
        assert_eq!(space, Space(" "));
        let input = space.skip_source(input).unwrap();

        let separator = Separator::parse(input).unwrap();
        assert_eq!(separator, Separator("|"));
        let input = separator.skip_source(input).unwrap();

        let space = Space::parse(input).unwrap();
        assert_eq!(space, Space(" "));
        let input = space.skip_source(input).unwrap();

        let quote = DoubleQuote::parse(input).unwrap();
        assert_eq!(quote, DoubleQuote("\"ciao\""));
        let input = quote.skip_source(input).unwrap();

        assert_eq!(input.len(), 0);
    }

    // #[test]
    // fn quotes_and_spaces() {
    //     let input = "'hamburger'  \t \"double quotes\" 'single_quotes'";

    //     assert_eq!(Parser::double_quoted(input), None);

    //     let (pat, input) = Parser::parse_and_skip(input, Parser::single_quoted).unwrap();
    //     assert_eq!(pat, "'hamburger'");

    //     let (pat, input) = Parser::parse_and_skip(input, Parser::space).unwrap();
    //     assert_eq!(pat, "  \t ");
    // }

    // #[test]
    // fn name_and_arrow_and_comment() {
    //     let input = "<rule> -> <burger> ; hamburger mobile!";

    //     let (ident, input) = Parser::parse_and_skip(input, Parser::name).unwrap();
    //     assert_eq!(ident, "<rule>");

    //     let (_, input) = Parser::parse_and_skip(input, Parser::space).unwrap();

    //     let (arrow, input) = Parser::parse_and_skip(input, Parser::assign).unwrap();
    //     assert_eq!(arrow, "->");

    //     let (_, input) = Parser::parse_and_skip(input, Parser::space).unwrap();

    //     let (ident, input) = Parser::parse_and_skip(input, Parser::name).unwrap();
    //     assert_eq!(ident, "<burger>");

    //     let (_, input) = Parser::parse_and_skip(input, Parser::space).unwrap();

    //     let (comment, input) = Parser::parse_and_skip(input, Parser::comment).unwrap();
    //     assert_eq!(comment, "; hamburger mobile!")
    // }
}
