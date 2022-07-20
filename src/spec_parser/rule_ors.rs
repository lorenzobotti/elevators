use super::error::ParseError;
use super::node::Node;
use super::rule_series::RuleSeries;
use super::rule_piece::RulePieceContent;
use super::tokens::*;
use crate::primitive_surrounded_by;
use crate::utils::take_n;

#[derive(Debug, PartialEq)]
pub struct RuleOrs<'a>(pub Vec<RuleSeries<'a>>);

impl<'a> Node<'a> for RuleOrs<'a> {
    fn parse_len(input: &'a str) -> Result<(Self, usize), ParseError> {
        let (first, trimmed) = RuleSeries::parse_and_skip(input)?;
        let mut series = vec![first];

        let mut trimmed = trimmed;

        loop {
            let (_, left) = match primitive_surrounded_by!(Separator, Space, trimmed) {
                Some((sep, left)) => (sep, left),
                None => {
                    return Ok((Self(series), input.bytes().len() - trimmed.bytes().len()));
                }
            };

            trimmed = left;

            let (other_series, left) = match RuleSeries::parse_and_skip(trimmed) {
                Ok((series, left)) => (series, left),
                Err(_) => {
                    return Err(ParseError::ExpectedWhile {
                        parsing: "rule ors",
                        expected: "rule series",
                        found: take_n(input, 20),
                        line: 0,
                    })
                }
            };

            trimmed = left;
            series.push(other_series);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec_parser::rule_piece::RulePiece;

    #[test]
    fn rule_ors() {
        let input =
            r#"<hamburger_mobile> <space> <jimmy> | "mamma mia" '"' "burger" <moment> | "hi""#;
        let expected = RuleOrs(vec![
            RuleSeries(vec![
                RulePieceContent::from(Identifier("<hamburger_mobile>")).into(),
                RulePieceContent::from(Identifier("<space>")).into(),
                RulePieceContent::from(Identifier("<jimmy>")).into(),
            ]),
            RuleSeries(vec![
                RulePieceContent::from(DoubleQuote("\"mamma mia\"")).into(),
                RulePieceContent::from(SingleQuote("'\"'")).into(),
                RulePieceContent::from(DoubleQuote("\"burger\"")).into(),
                RulePieceContent::from(Identifier("<moment>")).into(),
            ]),
            RuleSeries(vec![RulePieceContent::from(DoubleQuote("\"hi\"")).into()]),
        ]);

        let (got, _) = RuleOrs::parse_len(input).unwrap();
        assert_eq!(expected, got);
    }
}
