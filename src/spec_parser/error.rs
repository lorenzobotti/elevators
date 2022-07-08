use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError<'a> {
    // #[error("unexpected \"{input}\" at line {line}")]
    // Unexpected{ input: &'a str, line: usize },
    
    #[error("while parsing {parsing} unexpected \"{input}\" at line {line}")]
    UnexpectedWhile{ parsing: &'static str, input: &'a str, line: usize },

    // #[error("unexpected \"{found}\" (expected \"{expected}\") at line {line}")]
    // Expected{ expected: &'a str, found: &'a str, line: usize },

    #[error("while parsing {parsing} unexpected \"{found}\" (expected \"{expected}\") at line {line}")]
    ExpectedWhile{ parsing: &'static str, expected: &'static str, found: &'a str, line: usize },
}