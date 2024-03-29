use std::fmt;

use crate::utils::take_n;

use super::error::ParseError;
use super::node::Node;

pub const START_MATCH: char = '[';
pub const END_MATCH: char = ']';
pub const MATCH_SEPARATOR: char = '-';

#[derive(Debug, PartialEq)]
pub struct CharRange {
    pub from: char,
    pub to: char,
}

impl Node<'_> for CharRange {
    fn parse_len(input: &str) -> Result<(Self, usize), super::error::ParseError> {
        let mut chars = input.chars();

        match chars.next() {
            Some(START_MATCH) => {}
            Some(_) | None => {
                return Err(ParseError::ExpectedWhile {
                    parsing: "char range",
                    expected: "[",
                    found: take_n(chars.as_str(), 20),
                    line: 0,
                })
            }
        }

        let from = match chars.next() {
            Some(c) => c,
            None => {
                return Err(ParseError::ExpectedWhile {
                    parsing: "char range",
                    expected: "eof",
                    found: take_n(chars.as_str(), 20),
                    line: 0,
                })
            }
        };

        match chars.next() {
            Some(MATCH_SEPARATOR) => {}
            Some(_) | None => {
                return Err(ParseError::ExpectedWhile {
                    parsing: "char range",
                    expected: "-",
                    found: take_n(chars.as_str(), 20),
                    line: 0,
                })
            }
        }

        let to = match chars.next() {
            Some(c) => c,
            None => {
                return Err(ParseError::ExpectedWhile {
                    parsing: "char range",
                    expected: "eof",
                    found: take_n(chars.as_str(), 20),
                    line: 0,
                })
            }
        };

        match chars.next() {
            Some(END_MATCH) => {}
            Some(_) | None => {
                return Err(ParseError::ExpectedWhile {
                    parsing: "char range",
                    expected: "]",
                    found: take_n(chars.as_str(), 20),
                    line: 0,
                })
            }
        }

        let diff = input.len() - chars.as_str().len();
        Ok((Self { from, to }, diff))
    }
}

impl fmt::Display for CharRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}-{}]", self.from, self.to)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let input = "[a-z][A-Z]";
        let expected_first = CharRange { from: 'a', to: 'z' };
        let expected_second = CharRange { from: 'A', to: 'Z' };

        let (first, left) = CharRange::parse_and_skip(input).unwrap();
        assert_eq!(expected_first, first);

        let (second, _) = CharRange::parse_and_skip(left).unwrap();
        assert_eq!(expected_second, second);
    }
}
