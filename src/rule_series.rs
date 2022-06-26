use crate::rule_piece::RulePiece;
use crate::surrounded_by;
use crate::tokens::*;

#[derive(Debug, PartialEq)]
pub struct RuleSeries<'a>(Vec<RulePiece<'a>>);

impl<'a> Node<'a> for RuleSeries<'a> {
    fn parse_len(input: &'a str) -> Option<(Self, usize)> {
        let starting_len = input.bytes().len();

        let (first_piece, mut trimmed) = RulePiece::parse_and_skip(input)?;
        let mut pieces = vec![first_piece];
        
        'parse_loop:
        loop {
            dbg!(&pieces);

            let (piece, inp) = match surrounded_by!(RulePiece, Space, trimmed) {
                Some(res) => res,
                None => { break 'parse_loop; }
            };

            dbg!("parsed successfully");
            
            trimmed = inp;
            
            dbg!(&piece);
            dbg!(trimmed);
            pieces.push(piece);
            // dbg!(&pieces);
        }

        dbg!("returning some");

        let diff = starting_len - trimmed.bytes().len();
        Some((Self(pieces), diff))
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens::*;
    use super::*;

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
