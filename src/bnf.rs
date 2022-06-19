use std::{collections::HashMap, error::Error, fmt::Display};

use crate::rule::Rule;

struct Parser<'a> {
    input: &'a str,
    cursor: usize,

    rules: HashMap<String, Rule<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input,
            cursor: 0,
            rules: Default::default(),
        }
    }

    pub fn parse_rule(&mut self) -> Result<Rule, ParseError> {
        let input = self.input;

        let (name, input) = parse_and_skip(input, name).unwrap();
        let (_, input) = parse_and_skip(input, space).unwrap();
        let (_, input) = parse_and_skip(input, assign).unwrap();
        let (_, input) = parse_and_skip(input, space).unwrap();

        let (rule_piece, input) = parse_and_skip(input, rule_piece).unwrap();
        // let mut rule = Rule {
        //     name: name,
        //     content: vec![(rule_piece)],
        // };

        // loop {
        //     (_, input) = match parse_and_skip(input, space) {
        //         Some((space, input)) => (space, input),
        //         None => return Ok(rule),
        //     }
        // }

        todo!();
    }
}

pub fn rule_piece<'b>(input: &'b str) -> Option<&'b str> {
    if let Some(single) = single_quoted(input) {
        return Some(single);
    }

    if let Some(double) = double_quoted(input) {
        return Some(double);
    }

    if let Some(ident) = name(input) {
        return Some(ident);
    }

    None
}

pub fn name<'b>(input: &'b str) -> Option<&'b str> {
    if !input.starts_with('<') {
        return None;
    }
    // ifound MONGUS suddenbtly i found amongus aming us amongus suddently i found amongus

    let line = skip(input, "<")?;
    let end = line.find('>')? + 2;
    Some(&input[..end])
}

pub fn comment<'b>(input: &'b str) -> Option<&'b str> {
    if !input.starts_with(';') {
        return None;
    }

    Some(start_match(input, |c| !['\n', '\r'].contains(&c))?)
}

pub fn space<'b>(input: &'b str) -> Option<&'b str> {
    start_match(input, char::is_whitespace)
}

pub fn assign<'b>(input: &'b str) -> Option<&'b str> {
    if input.starts_with("->") {
        Some(&input[..("->".len())])
    } else {
        None
    }
}

pub fn double_quoted<'b>(input: &'b str) -> Option<&'b str> {
    let line = input.trim_start_matches('"');
    if line.len() == input.len() {
        return None;
    }

    let end = line.find('"')? + 2;
    Some(&input[..end])
}

pub fn single_quoted<'b>(input: &'b str) -> Option<&'b str> {
    let line = input.trim_start_matches('\'');
    if line.len() == input.len() {
        return None;
    }

    let end = line.find('\'')? + 1;
    let out = &input[..=end];
    Some(out)
}

pub fn skip_n<'b>(s: &'b str, n: usize) -> &'b str {
    let mut chars = s.chars();
    chars.nth(n - 1);
    chars.as_str()
}

pub fn start_match<'b, Pattern>(s: &'b str, p: Pattern) -> Option<&'b str>
where
    Pattern: Fn(char) -> bool,
{
    let trimmed = s.trim_start_matches(p);
    if trimmed.len() == s.len() {
        None
    } else {
        let diff = s.len() - trimmed.len();
        Some(&s[..diff])
    }
}

pub fn skip<'b>(line: &'b str, start: &str) -> Option<&'b str> {
    if line.starts_with(start) {
        Some(line.trim_start_matches(start))
    } else {
        None
    }
}

pub fn parse_and_skip<'b, ParseFunc>(input: &'b str, parse: ParseFunc) -> Option<(&'b str, &'b str)>
where
    ParseFunc: Fn(&'b str) -> Option<&'b str>,
{
    let parsed = parse(input)?;
    let skipped = skip(input, parsed)?;
    Some((parsed, skipped))
}

#[cfg(test)]
mod tests {
    use super::*;

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

    // #[test]
    // fn quotes_and_separator() {
    //     let input = "'burger' | \"ciao\"";
    //     let quote = super::SingleQuote::parse(input).unwrap();
    //     assert_eq!(quote, super::SingleQuote("'burger'"));

    // }
}

#[derive(Clone, Debug)]
enum ParseError {
    Expected(String, String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expected(exp, got) => write!(f, "expected {}, got {}", exp, got),
        }
    }
}
