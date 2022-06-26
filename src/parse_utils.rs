#[macro_export]
/// Parses $token, and tries to parse leading and trailing
/// $surrounded. $token and $surrounded are Node<'a>, $input
/// is a &str or String 
macro_rules! surrounded_by {
    ($token: ty, $surrounded: ty, $input: ident) => {{
        let mut token: Option<$token> = None;

        let mut trimmed = $input;
        match <$surrounded>::parse_and_skip($input) {
            Some((_, inp)) => {trimmed = inp},
            None => {}
        }

        // dbg!("after parsing first space");
        // dbg!(trimmed);
        
        let trimmed = match <$token>::parse_and_skip(trimmed) {
            Some((tok, left)) => {
                token = Some(tok);
                left
            },
            None => trimmed,
        };
        
        // dbg!("after parsing content");
        // dbg!(&parsed);
        // dbg!(trimmed);
        
        let trimmed = match <$surrounded>::parse_and_skip(trimmed) {
            Some((_, inp)) => inp,
            None => trimmed,
        };

        // dbg!("after parsing ending space");
        // dbg!(trimmed);

        let diff = $input.bytes().len() - trimmed.bytes().len();
        match token {
            Some(tok) => Some((tok, trimmed)),
            None => None,
        }
    }};
}
