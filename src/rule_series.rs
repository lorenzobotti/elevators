use crate::rule_piece::RulePiece;
use crate::surrounded_by;
use crate::tokens::*;

#[derive(Debug, PartialEq)]
pub struct RuleSeries<'a>(pub Vec<RulePiece<'a>>);

impl<'a> Node<'a> for RuleSeries<'a> {
    fn parse_len(input: &'a str) -> Option<(Self, usize)> {
        let starting_len = input.bytes().len();

        let (first_piece, trimmed) = RulePiece::parse_and_skip(input)?;
        let mut pieces = vec![first_piece];

        let mut trimmed = trimmed;

        'parse_loop: loop {
            dbg!(trimmed);

            let (piece, inp) = match surrounded_by!(RulePiece, Space, trimmed) {
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

        dbg!("returning some");

        let diff = starting_len - trimmed.bytes().len();
        Some((Self(pieces), diff))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::*;

    #[test]
    fn rule_series() {
        let input = r#"'hamburger' <hamburger> "automobile" <johnny_moment>"#;
        let expected = RuleSeries(vec![
            RulePiece::Single(SingleQuote("'hamburger'")),
            RulePiece::Ident(Identifier("<hamburger>")),
            RulePiece::Double(DoubleQuote("\"automobile\"")),
            RulePiece::Ident(Identifier("<johnny_moment>")),
        ]);

        let (got, _) = RuleSeries::parse_len(input).unwrap();
        assert_eq!(expected, got);
    }
}
