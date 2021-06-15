pub mod old_parser;
pub mod parser;

#[cfg(test)]
mod tests {
    mod old_parser {
        mod parse_stream {
            use crate::old_parser::{lexer::TokenStream, parse_stream::{ParseStream, StringStream}};
            #[test]
            fn str_stream() {
                let mut ss = StringStream::new("Hello World!");
                assert!(matches!(ss.parse_next(), Ok('H')));
                assert!(matches!(ss.try_match("Hello".chars().collect()), Err(_)));
                assert!(matches!(ss.try_match("ello ".chars().collect()), Ok(_)));
                assert!(matches!(ss.parse_next(), Ok('W')));
                assert!(matches!(ss.try_match("orld!".chars().collect()), Ok(_)));
                assert!(matches!(ss.try_match("orld!".chars().collect()), Err(_)));
                assert!(matches!(ss.parse_next(), Err(_)));
            }

            #[test]
            fn lexing() {
                use crate::old_parser::lexer::{Token::{self, *}, Keyword::*};
                let ss = StringStream::new("def defunc(a, b):\n\tx = a + \tb\n\treturn    x\n\n");
                let tokens = TokenStream::new(ss);
                let res = 
                vec![ WS(0), Kywrd(Def), Ident(String::from("defunc")), Symbl('('), Ident(String::from("a")), Symbl(','), Ident(String::from("b")), Symbl(')'), Symbl(':')
                    , WS(4), Ident(String::from("x")), Symbl('='), Ident(String::from("a")), Symbl('+'), Ident(String::from("b"))
                    , WS(4), Kywrd(Return), Ident(String::from("x"))
                    , WS(0)
                    , WS(0)
                ];
                assert_eq!(tokens.collect::<Vec<Token>>(), res);
            }
        }
    }
}