#[allow(unused_imports)]
use crate::tokens::*;

#[macro_export]
/// Parses $token, and tries to parse leading and trailing
/// $surrounded. $token and $surrounded are Node<'a>, $input
/// is a &str or String 
macro_rules! surrounded_by {
    ($token: ty, $surrounded: ty, $input: ident) => {{
        // we can't early return from a macro, so we set this
        // nullable value, and at the end we check if it's
        // null
        let mut token: Option<$token> = None;

        let mut trimmed = $input;
        match <$surrounded>::parse_and_skip($input) {
            Some((_, inp)) => {trimmed = inp},
            None => {}
        }

        let trimmed = match <$token>::parse_and_skip(trimmed) {
            Some((tok, left)) => {
                token = Some(tok);
                left
            },
            None => trimmed,
        };
        
        let trimmed = match <$surrounded>::parse_and_skip(trimmed) {
            Some((_, inp)) => inp,
            None => trimmed,
        };

        match token {
            Some(tok) => Some((tok, trimmed)),
            None => None,
        }
    }};
}

#[cfg(test)]
mod tests {
    use crate::tokens::{Identifier, Space};

    use super::*;

    #[test]
    fn surrounded() {
        let input = " <identifier> 'john'";
        let expected_parsed = Identifier("<identifier>");
        let expected_left = "'john'";

        let (ident, left) = surrounded_by!(Identifier, Space, input).unwrap();
        assert_eq!(ident, expected_parsed);
        assert_eq!(left, expected_left);
    }
}