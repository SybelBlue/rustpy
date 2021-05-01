use std::{collections::VecDeque, fmt::Display, iter::Peekable};

use crate::parser::*;

use super::parse_stream::ParseStream;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    WS(u8),
    Ident(String),
    Kywrd(Keyword),
    Symbl(char),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (a, b) = match self {
            Token::WS(n) => ("Whitespace", format!("{}", n)),
            Token::Ident(s) => ("Identifier", s.clone()),
            Token::Kywrd(k) => ("Keyword", format!("{:?}", k)),
            Token::Symbl(s) => ("Symbol", format!("{}", s)),
        };
        write!(f, "{} {}", a, b)
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
    Return,
    Class,
}

fn into_token(s: String) -> Token {
    use Keyword::*;
    match s.as_str() {
        "if" => Token::Kywrd(If),
        "else" => Token::Kywrd(Else),
        "def" => Token::Kywrd(Def),
        "return" => Token::Kywrd(Return),
        "class" => Token::Kywrd(Class),
        "None" => Token::Kywrd(None),
        "True" => Token::Kywrd(True),
        "False" => Token::Kywrd(False),
        &_ => Token::Ident(s),
    }
}

fn as_symbol(c: char) -> Option<Token> {
    if matches!(c, '.' | ',' | ':' | '(' | ')' |'[' | ']' | '=' | '+' | '-' | '#' | '\'' | '\\') {
        Some(Token::Symbl(c))
    } else {
        None
    }
}

#[derive(Debug)]
pub struct TokenStream<C : Iterator<Item = char>> {
    char_iter: Peekable<C>,
    cached: VecDeque<Token>,
    file_pos: FilePos,
    line_start: bool,
}

impl<C : Iterator<Item = char>> TokenStream<C> {
    pub fn new(char_iter: C) -> Self {
        Self { char_iter: char_iter.peekable(), file_pos: FilePos::new(), cached: VecDeque::new(), line_start: true }
    }
}

impl<C : Iterator<Item = char>> ParseStream<Token> for TokenStream<C> {
    fn parse_next(&mut self) -> ParseResult<Token> {
        if let Some(item) = self.cached.pop_front() {
            return Ok(item);
        }
        if self.line_start {
            self.line_start = false;
            let mut n = 0;
            while let Some(&c) = self.char_iter.peek() {
                self.file_pos.advance(c);
                match c {
                    ' ' => n += 1,
                    '\t' => n += 4,
                    _ => return Ok(Token::WS(n)),
                }
                self.char_iter.next();
            }
            return Ok(Token::WS(n));
        }
        let mut built = String::new();
        for c in &mut self.char_iter {
            let file_pos = self.file_pos;
            self.file_pos.advance(c);
            
            if c.is_whitespace() { 
                if c == '\n' {
                    self.line_start = true;
                }
                if !built.is_empty() {
                    return Ok(into_token(built))
                }
                if self.line_start {
                    return self.parse_next();
                }
            } else if let Some(tkn) = as_symbol(c) {
                return Ok(
                    if !built.is_empty() {
                        self.cached.push_back(tkn);
                        into_token(built)
                    } else {
                        tkn
                    }
                )
            } else if matches!(c, '0'..='9' | 'a'..='z' | 'A'..='Z' | '_') {
                built.push(c);
            } else {
                return Err(ParseError::from_str(format!("Bad char '{}'", c), file_pos));
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
            rest.push_back(self.parse_next()?);
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

impl<T: Iterator<Item = char>> Iterator for TokenStream<T> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_next().ok()
    }
}