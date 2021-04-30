use std::fmt::Display;

pub mod lexer;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
    msg: String,
    src_path: Option<String>,
    line: usize,
    col: usize,
}

impl ParseError {
    fn from_str(msg: String, line: usize, col: usize) -> Self {
        Self { msg, line, col, src_path: None }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Error parsing {}\n\t{}", 
            self.src_path.clone().map_or(
                String::from("string"), 
                |f_p| format!("file at {}:{}:{}", f_p, self.line, self.col)
            ),
            self.msg
        )
    }
}

trait ParseStream<T> {
    fn next(&mut self) -> ParseResult<T>;
    fn try_match(&mut self, items: Vec<T>) -> ParseResult<Vec<T>>;
}

#[derive(Debug, PartialEq, Eq)]
struct StringStream {
    line: usize,
    col: usize,
    pos: usize,
    data: String,
}

impl ParseStream<char> for String {
    fn next(&mut self) -> ParseResult<char> {
        todo!()
    }

    fn try_match(&mut self, items: Vec<char>) -> ParseResult<Vec<char>> {
        todo!()
    }
}