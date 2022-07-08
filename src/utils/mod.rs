pub fn take_n<'a>(input: &'a str, n: usize) -> &'a str {
    let end = {
        let mut chars = input.chars();
        chars.nth(n);
        chars.as_str()
    };

    let bytes = input.bytes().len() - end.bytes().len();
    &input[..bytes]
}