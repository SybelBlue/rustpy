use crate::parser::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Ident(String),
    Kywrd(Keyword),
    Symbl(char),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Keyword {
    If,
    Else,
    Def,
    None,
    True,
    False,
}

pub fn into_tokens<C>(s: C) -> ParseResult<Vec<Token>>
    where C : Iterator<Item = char> {
    let mut out = Vec::new();
    let mut built = None;
    let mut file_pos = FilePos::new();
    for c in s {
        if c.is_whitespace() { 
            if let Some(sacc) = built {
                out.push(into_token(sacc));
                built = None;
            }
        } else if let Some(tkn) = as_symbol(c) {
            if let Some(sacc) = built {
                out.push(into_token(sacc));
                built = None;
            }
            out.push(tkn);
        } else if matches!(c, '0'..='9' | 'a'..='z' | 'A'..='Z' | '_') {
            if let Some(sacc) = &mut built {
                sacc.push(c);
            } else {
                built = Some(String::new());
            }
        } else {
            return Err(ParseError::from_str(format!("Bad char '{}'", c), file_pos));
        }
        file_pos.advance(&c);
    }
    Ok(out)
}

fn into_token(s: String) -> Token {
    use Keyword::*;
    match s.as_str() {
        "if" => Token::Kywrd(If),
        "else" => Token::Kywrd(Else),
        "def" => Token::Kywrd(Def),
        "None" => Token::Kywrd(None),
        "True" => Token::Kywrd(True),
        "False" => Token::Kywrd(False),
        &_ => Token::Ident(s),
    }
}

fn as_symbol(c: char) -> Option<Token> {
    if matches!(c, '.' | ',' | ':' | '(' | ')' |'[' | ']' | '+' | '-' | '#') {
        Some(Token::Symbl(c))
    } else {
        None
    }
}

// #[derive(Debug, PartialEq, Eq)]
// pub struct TokenStream {}

// impl ParseStream<Token> for TokenStream {
//     fn next(&mut self) -> super::ParseResult<Token> {
//         todo!()
//     }

//     fn try_match(&mut self, items: Vec<Token>) -> super::ParseResult<Vec<Token>> {
//         todo!()
//     }
// }
