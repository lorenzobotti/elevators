#[macro_export]
macro_rules! surrounded_by {
    ($token: ty, $surrounded: ty, $input: ident) => {{
        let mut trimmed = $input;
        match <$surrounded>::parse_and_skip($input) {
            Some((_, inp)) => {trimmed = inp},
            None => {}
        }

        dbg!("after parsing first space");
        dbg!(trimmed);
        
        let (parsed, trimmed) = <$token>::parse_and_skip(trimmed)?;
        let mut trimmed = trimmed;
        
        dbg!("after parsing content");
        dbg!(&parsed);
        dbg!(trimmed);
        
        match <$surrounded>::parse_and_skip(trimmed) {
            Some((_, inp)) => trimmed = inp,
            None => {}
        }

        dbg!("after parsing ending space");
        dbg!(trimmed);

        let diff = $input.bytes().len() - trimmed.bytes().len();
        match diff {
            0 => None,
            _ => Some((parsed, trimmed)),
        }
    }};
}
