use std::{collections::VecDeque, fmt::Display};

use crate::parser::*;

use super::parse_stream::ParseStream;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Ident(String),
    Kywrd(Keyword),
    Symbl(char),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (a, b) = match self {
            Token::Ident(s) => ("Identifier", s.clone()),
            Token::Kywrd(k) => ("Keyword", format!("{:?}", k)),
            Token::Symbl(s) => ("Symbol", format!("'{}'", s)),
        };
        write!(f, "{} {:?}", a, b)
    }
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

#[derive(Debug, PartialEq, Eq)]
pub struct TokenStream<C> where C : Iterator<Item = char> {
    char_iter: C,
    cached: VecDeque<Token>,
    file_pos: FilePos,
}

impl<C> ParseStream<Token> for TokenStream<C> where C : Iterator<Item = char> {
    fn next(&mut self) -> ParseResult<Token> {
        if let Some(item) = self.cached.pop_front() {
            return Ok(item);
        }
        let mut built = String::new();
        for c in &mut self.char_iter {
            if c.is_whitespace() { 
                if !built.is_empty() {
                    return Ok(into_token(built));
                }
            } else if let Some(tkn) = as_symbol(c) {
                return Ok(if !built.is_empty() {
                    self.cached.push_back(tkn);
                    into_token(built)
                } else {
                    tkn
                })
            } else if matches!(c, '0'..='9' | 'a'..='z' | 'A'..='Z' | '_') {
                built.push(c);
                self.file_pos.advance(c);
            } else {
                return Err(ParseError::from_str(format!("Bad char '{}'", c), self.file_pos));
            }
        }
        
        if built.is_empty() {
            Err(ParseError::eof(self.file_pos, None))
        } else {
            Ok(into_token(built))
        }
    }

    fn try_match(&mut self, items: Vec<Token>) -> ParseResult<Vec<Token>> {
        let n = items.len() - self.cached.len();
        let mut rest = VecDeque::new();
        for _ in 0..n {
            rest.push_back(self.next()?);
        }
        self.cached.extend(rest.into_iter());
        for (a, b) in self.cached.iter().zip(items.iter()) {
            if *a != *b {
                return Err(ParseError::mismatch(a, b, self.file_pos))
            }
        }
        Ok(items)
    }
}
