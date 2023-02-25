pub fn match_literal<'a>(input: &'a str, expected: &'static str) -> Option<&'a str> {
    match input.get(0..expected.len()) {
        Some(next) if next == expected => Some(&input[expected.len()..]),
        _ => None,
    }
}

pub fn is_in_mapping(input: &str) -> Result<(), &str> {
    match &input.chars().next() {
        Some(first_char) => {
            if !first_char.is_alphabetic() {
                Ok(())
            } else {
                Err(input)
            }
        }
        _ => Err(input),
    }
}
