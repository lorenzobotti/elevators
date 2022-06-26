use crate::{tokens::*, surrounded_by};
use crate::rule_series::RuleSeries;

#[derive(Debug, PartialEq)]
pub struct RuleOrs<'a>(pub Vec<RuleSeries<'a>>);

impl<'a> Node<'a> for RuleOrs<'a> {
    fn parse_len(input: &'a str) -> Option<(Self, usize)> {
        let (first, trimmed) = RuleSeries::parse_and_skip(input)?;
        let mut series = vec![first];

        let mut trimmed = trimmed;

        loop {
            let (_, left) = match surrounded_by!(Separator, Space, trimmed) {
                Some((sep, left)) => (sep, left),
                None => return Some((Self(series), input.bytes().len() - trimmed.bytes().len())),
            };

            trimmed = left;
            
            let (other_series, left) = match RuleSeries::parse_and_skip(trimmed) {
                Some((series, left)) => (series, left),
                None => panic!("expected rule series after separator"),
            };
            
            trimmed = left;
            series.push(other_series);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rule_piece::RulePiece;
    use super::*;
    
    #[test]
    fn rule_ors() {
        let input = r#"<hamburger_mobile> <space> <jimmy> | "mamma mia" '"' "burger" <moment>"#;
        let expected = RuleOrs(vec![
            RuleSeries(vec![
                RulePiece::Ident(Identifier("<hamburger_mobile>")),
                RulePiece::Ident(Identifier("<space>")),
                RulePiece::Ident(Identifier("<jimmy>")),
            ]),
            RuleSeries(vec![
                RulePiece::Double(DoubleQuote("\"mamma mia\"")),
                RulePiece::Single(SingleQuote("'\"'")),
                RulePiece::Double(DoubleQuote("\"burger\"")),
                RulePiece::Ident(Identifier("<moment>")),
            ]),
        ]);

        let (got, _) = RuleOrs::parse_len(input).unwrap();
        assert_eq!(expected, got);
    }
}