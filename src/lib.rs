pub mod parser;

#[cfg(test)]
mod tests {
    mod parser {
        mod parse_stream {
            use crate::parser::parse_stream::{ParseStream, StringStream};
            #[test]
            fn str_stream() {
                let mut ss = StringStream::new("Hello World!");
                assert!(matches!(ss.next(), Ok('H')));
                assert!(matches!(ss.try_match("Hello".chars().collect()), Err(_)));
                assert!(matches!(ss.try_match("ello ".chars().collect()), Ok(_)));
                assert!(matches!(ss.next(), Ok('W')));
                assert!(matches!(ss.try_match("orld!".chars().collect()), Ok(_)));
                assert!(matches!(ss.try_match("orld!".chars().collect()), Err(_)));
                assert!(matches!(ss.next(), Err(_)));
            }
        }
    }
}