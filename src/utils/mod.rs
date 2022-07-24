pub fn take_n<'a>(input: &'a str, n: usize) -> &'a str {
    let end = {
        let mut chars = input.chars();
        chars.nth(n);
        chars.as_str()
    };

    let bytes = input.len() - end.len();
    &input[..bytes]
}

#[allow(dead_code)]
pub fn match_start<'a>(input: &'a str, matcher: &str) -> Option<&'a str> {
    if input.starts_with(matcher) {
        Some(&input[..matcher.len()])
    } else {
        None
    }
}
