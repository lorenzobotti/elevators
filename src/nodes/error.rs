use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError<'grammar, 'input> {
    #[error("while parsing {parsing}: expected \"{expected}\", found \"{got}\"")]
    Expected {
        parsing: &'grammar str,
        expected: String,
        got: &'input str,
    },
}
