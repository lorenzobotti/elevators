use crate::rule_piece::RulePiece;
use crate::surrounded_by;
use crate::tokens::*;

#[derive(Debug, PartialEq)]
pub struct RuleSeries<'a>(Vec<RulePiece<'a>>);

impl<'a> Node<'a> for RuleSeries<'a> {
    fn parse_len(input: &'a str) -> Option<(Self, usize)> {
        let starting_len = input.bytes().len();
        let length_left = || input.bytes().len() - starting_len;

        let (first_piece, input) = RulePiece::parse_and_skip(input)?;
        let mut pieces = vec![first_piece];
        
        loop {
            dbg!(&pieces);
            dbg!(input);

            let input = match surrounded_by!(Separator, Space, input) {
                Some((sep, input)) => {
                    dbg!(sep);
                    input
                },
                None => {
                    dbg!("can't parse space");
                    return Some((Self(pieces), length_left()))
                },
            };

            let (piece, input) = match surrounded_by!(RulePiece, Space, input) {
                Some(res) => res,
                None => {
                    panic!("expected rule piece, found {}", &input[..10]);
                }
            };

            pieces.push(piece);
            dbg!(&pieces);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens::*;
    use super::*;

    #[test]
    fn rule_series() {
        let input = r#"'hamburger' <hamburger> "automobile" <johnny>"#;
        let expected = RuleSeries(vec![
            RulePiece::Single(SingleQuote("'hamburger'")),
            RulePiece::Ident(Identifier("<hamburger>")),
            RulePiece::Double(DoubleQuote("\"hamburger\"")),
            RulePiece::Ident(Identifier("<johnny>")),
        ]);

        let (got, _) = RuleSeries::parse_len(input).unwrap();
        assert_eq!(expected, got);

    }

}
