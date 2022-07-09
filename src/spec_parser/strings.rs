#[macro_export]
/// Like trim_start_matches, but only takes the start
/// of a string that matches a pattern
///
macro_rules! take_start {
    ($input: expr, $pattern: expr) => {{
        use crate::spec_parser::strings::trim_end;

        let initial_len = $input.bytes().len();
        let trimmed: &str = $input.trim_start_matches($pattern);
        let diff = initial_len - trimmed.bytes().len();

        match diff {
            0 => None,
            _ => Some(trim_end($input, trimmed)),
        }
    }};
}

pub fn trim_end<'a>(input: &'a str, end: &str) -> &'a str {
    assert!(input.len() >= end.len());

    let diff = input.bytes().len() - end.bytes().len();
    &input[..diff]
}

#[cfg(test)]
mod tests {
    #[test]
    fn take_start() {
        let input = "!= burger";
        let expected = "!=";
        let func = |c| ['!', '=', ')'].contains(&c);

        let got = take_start!(input, func).unwrap();
        assert_eq!(got, expected);
    }
}
