use super::error::ParseError;
use super::node::Node;
use super::rule_piece::RulePiece;
use super::tokens::*;
use crate::node_surrounded_by;
use crate::utils::take_n;

#[derive(Debug, PartialEq)]
pub struct RuleSeries<'a>(pub Vec<RulePiece<'a>>);

impl<'a> Node<'a> for RuleSeries<'a> {
    fn parse_len(input: &'a str) -> Result<(Self, usize), ParseError> {
        let (first_piece, trimmed) = RulePiece::parse_and_skip(input)?;
        let mut pieces = vec![first_piece];

        let mut trimmed = trimmed;

        'parse_loop: loop {
            let (piece, inp) = match node_surrounded_by!(RulePiece, Space, trimmed) {
                Some(res) => res,
                None => {
                    break 'parse_loop;
                }
            };

            assert!(inp.bytes().len() < trimmed.bytes().len());

            pieces.push(piece);

            let trimmed_before = trimmed;
            trimmed = inp;

            assert!(trimmed_before != trimmed);
        }

        let diff = input.bytes().len() - trimmed.bytes().len();
        match diff {
            0 => Err(ParseError::UnexpectedWhile {
                parsing: "rule series",
                input: take_n(input, 20),
                line: 0,
            }),
            _ => Ok((Self(pieces), diff)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::char_range::CharRange;
    use super::*;

    #[test]
    fn rule_series() {
        let input = r#"'hamburger' <hamburger> "automobile" [a-z] <johnny_moment>"#;
        let expected = RuleSeries(vec![
            RulePiece::Single(SingleQuote("'hamburger'")),
            RulePiece::Ident(Identifier("<hamburger>")),
            RulePiece::Double(DoubleQuote("\"automobile\"")),
            RulePiece::Range(CharRange{ from: 'a', to: 'z' }),
            RulePiece::Ident(Identifier("<johnny_moment>")),
        ]);

        let (got, _) = RuleSeries::parse_len(input).unwrap();
        assert_eq!(expected, got);
    }
}
