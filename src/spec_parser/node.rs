use super::error::ParseError;

pub trait Node<'a>: Sized {
    fn parse_len(input: &'a str) -> Result<(Self, usize), ParseError>;
    fn parse_and_skip(input: &'a str) -> Result<(Self, &'a str), ParseError> {
        let (node, len) = Self::parse_len(input)?;
        Ok((node, &input[len..]))
    }
}