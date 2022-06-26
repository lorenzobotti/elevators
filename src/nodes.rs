use std::ops::Deref;

#[forbid(unsafe_code)]
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

#[derive(Debug, PartialEq)]
pub struct Word<'a>(&'a str);

#[derive(Debug, PartialEq)]
pub struct Commment<'a>(&'a str);

//ohhhhh ma gaaawwwdddd

pub trait Node<'a>: Sized {
    fn parse_len(input: &'a str) -> Option<(Self, usize)>;
    fn parse(input: &'a str) -> Option<Self> {
        Some(Self::parse_len(input)?.0)
    }
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
    fn parse_len(input: &'a str) -> Option<(Self, usize)> {
        let node = Self(start_match(input, char::is_whitespace)?);
        Some((node, node.0.len()))
    }
}
impl<'a> Node<'a> for SingleQuote<'a> {
    fn parse_len(input: &'a str) -> Option<(Self, usize)> {
        let line = input.trim_start_matches(SINGLE_QUOTE);
        if line.len() == input.len() {
            return None;
        }

        let end = line.find(SINGLE_QUOTE)? + 1;
        let out = &input[..=end];
        Some((Self(out), out.len()))
    }
}
impl<'a> Node<'a> for DoubleQuote<'a> {
    fn parse_len(input: &'a str) -> Option<(Self, usize)> {
        let line = input.trim_start_matches(DOUBLE_QUOTE);
        if line.len() == input.len() {
            return None;
        }

        let end = line.find(DOUBLE_QUOTE)? + 1;
        let out = &input[..=end];
        Some((Self(out), out.len()))
    }
}
impl<'a> Node<'a> for Identifier<'a> {
    fn parse_len(input: &'a str) -> Option<(Self, usize)> {
        if !input.starts_with(START_IDENT) {
            return None;
        }
        // ifound MONGUS suddenbtly i found amongus aming us amongus suddently i found amongus

        // TODO: benchmark di questo
        let line = skip(input, &START_IDENT.to_string())?;
        let end = line.find(STOP_IDENT)? + 2;
        let out = Self(&input[..end]);
        Some((out, out.0.len()))
    }
}
impl<'a> Node<'a> for Comment<'a> {
    fn parse_len(input: &'a str) -> Option<(Self,usize)> {
        if !input.starts_with(';') {
            return None;
        }
        let is_not_newline = |c| !['\n', '\r'].contains(&c);

        let out = Self(start_match(input, is_not_newline)?);
        Some((out, out.0.len()))
    }
}
impl<'a> Node<'a> for Assign<'a> {
    fn parse_len(input: &'a str) -> Option<(Self, usize)> {
        if input.starts_with(ASSIGN) {
            let out = Self(&input[..(ASSIGN.len())])
            Some((out, out.0.len()))
        } else {
            None
        }
    }
}
impl<'a> Node<'a> for Separator<'a> {
    fn parse_len(input: &'a str) -> Option<(Self, usize)> {
        if input.starts_with(SEPARATOR) {
            let out = Self(&input[..1])
            Some((out, out.len()))
        } else {
            None
        }
    }
}
impl<'a> Node<'a> for Word<'a> {
    fn parse_len(input: &'a str) -> Option<(Self, usize)> {
        let out = Self(start_match(input, char::is_alphabetic)?)
        Some((out, out.0.len()))
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

impl<'a> PrimitiveNode<'a> for Word<'a> {
    fn as_str(&self) -> &'a str {
        self.0
    }
}

#[derive(Debug, PartialEq)]
pub enum Primitive<'a> {
    Space(Space<'a>),
    SingleQuote(SingleQuote<'a>),
    DoubleQuote(DoubleQuote<'a>),
    Identifier(Identifier<'a>),
    Comment(Comment<'a>),
    Assign(Assign<'a>),
    Separator(Separator<'a>),
    // Word(Word<'a>),
}
impl<'a> Primitive<'a> {
    fn parse(input: &'a str) -> Option<Self> {
        if let Some(n) = Space::parse(input) {
            return Some(Self::Space(n));
        }
        if let Some(n) = SingleQuote::parse(input) {
            return Some(Self::SingleQuote(n));
        }
        if let Some(n) = DoubleQuote::parse(input) {
            return Some(Self::DoubleQuote(n));
        }
        if let Some(n) = Identifier::parse(input) {
            return Some(Self::Identifier(n));
        }
        if let Some(n) = Comment::parse(input) {
            return Some(Self::Comment(n));
        }
        if let Some(n) = Assign::parse(input) {
            return Some(Self::Assign(n));
        }
        if let Some(n) = Separator::parse(input) {
            return Some(Self::Separator(n));
        }

        None
    }

    pub fn as_str(&self) -> &'a str {
        match self {
            Self::Space(n) => n.as_str(),
            Self::SingleQuote(n) => n.as_str(),
            Self::DoubleQuote(n) => n.as_str(),
            Self::Identifier(n) => n.as_str(),
            Self::Comment(n) => n.as_str(),
            Self::Assign(n) => n.as_str(),
            Self::Separator(n) => n.as_str(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RulePiece<'a> {
    Identifier(Identifier<'a>),
    SingleQuote(SingleQuote<'a>),
    DoubleQuote(DoubleQuote<'a>),
}
impl<'a> RulePiece<'a> {
    fn as_str(&self) -> &'a str {
        match self {
            Self::Identifier(n) => n.as_str(),
            Self::SingleQuote(n) => n.as_str(),
            Self::DoubleQuote(n) => n.as_str(),
        }
    }
}
impl<'a> Node<'a> for RulePiece<'a> {
    fn parse_len(input: &'a str) -> Option<(Self, usize)> {
        if let Some(ident) = Identifier::parse(input) {
            return Some((Self::Identifier(ident), ident.0.len()));
        }

        if let Some(quote) = SingleQuote::parse(input) {
            return Some((Self::SingleQuote(quote), quote.0.len()));
        }

        if let Some(quote) = DoubleQuote::parse(input) {
            return Some((Self::DoubleQuote(quote), quote.0.len()));
        }

        None
    }
}

pub struct RulePieces<'a>(Vec<RulePiece<'a>>);
impl<'a> RuleSpec<'a> {

}

pub struct RuleSpec<'a>(Vec<RulePiece<'a>>);
impl<'a> RuleSpec<'a> {
    fn parse_separator(input: &'a str) -> Option<&'a str> {
        let mut trimmed = input;

        if let Some(space) = Space::parse(trimmed) {
            trimmed = space.skip_source(trimmed).unwrap();
        }

        let separator = Separator::parse(trimmed)?;
        trimmed = separator.skip_source(trimmed)?;

        if let Some(space) = Space::parse(trimmed) {
            trimmed = space.skip_source(trimmed).unwrap();
        }

        let diff = input.bytes().len() - trimmed.bytes().len();
        Some(&input[..diff])
    }
}
impl<'a> Node<'a> for RuleSpec<'a> {
    fn parse_len(input: &'a str) -> Option<(Self, usize)> {
        let mut trimmed = input;
        let mut rules = Vec::new();


        let piece = RulePiece::parse(trimmed);
        if let Some(rule) = piece {
            trimmed = skip(trimmed, rule.as_str()).unwrap();
            rules.push(rule);
        } else {
            return None
        }

        loop {
            let between = Self::parse_separator(trimmed);
            match between {
                None => break,
                Some(s) => trimmed = skip(trimmed, s).unwrap(),
            }

            let piece = RulePiece::parse(trimmed);
            if let Some(rule) = piece {
                trimmed = skip(trimmed, rule.as_str()).unwrap();
                rules.push(rule);
            } else {
                panic!("expected rule piece after separator");
            }
        }

        if rules.is_empty() {
            None
        } else {
            Some(Self(rules))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quotes_and_separator() {
        let mut input = "'burger' | \"ciao\"";
        let expected = [
            Primitive::SingleQuote(SingleQuote("'burger'")),
            Primitive::Space(Space(" ")),
            Primitive::Separator(Separator("|")),
            Primitive::Space(Space(" ")),
            Primitive::DoubleQuote(DoubleQuote("\"ciao\"")),
        ];

        for exp in expected {
            let got = Primitive::parse(input).unwrap();
            assert_eq!(exp, got);

            input = skip(input, got.as_str()).unwrap();
        }

        assert_eq!(input.len(), 0);
    }

    #[test]
    fn quotes_and_spaces() {
        let input = "'hamburger'  \t \"double quotes\" 'single_quotes'";
        
        let single = SingleQuote::parse(input).unwrap();
        assert_eq!(single, SingleQuote("'hamburger'"));
        let input = single.skip_source(input).unwrap();
        
        let space = Space::parse(input).unwrap();
        assert_eq!(space, Space("  \t "));
        let input = space.skip_source(input).unwrap();
        
        let double = DoubleQuote::parse(input).unwrap();
        assert_eq!(double, DoubleQuote("\"double quotes\""));
        let input = double.skip_source(input).unwrap();
        
        let space = Space::parse(input).unwrap();
        assert_eq!(space, Space(" "));
        let input = space.skip_source(input).unwrap();
        
        let single = SingleQuote::parse(input).unwrap();
        assert_eq!(single, SingleQuote("'single_quotes'"));

        assert_eq!(single.skip_source(input).unwrap().len(), 0);
    }

    #[test]
    fn rule_spec() {
        let input = "<identifier> | 'burger' | <johnny> 'burger'";
        let piece = RuleSpec::parse(input).unwrap();

        assert_eq!(piece.0[0], RulePiece::Identifier(Identifier("<identifier>")));
        assert_eq!(piece.0[1], RulePiece::SingleQuote(SingleQuote("'burger'")));
    }
}
