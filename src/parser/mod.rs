pub mod lexer;

pub type ParseResult<T> = Result<T, ParseError>;
pub struct ParseError {
    pub msg: String,
    pub src_path: String,
    pub line: usize,
    pub col: usize,
}

trait ParseStream<T> {
    fn next(&mut self) -> ParseResult<T>;
    fn try_match(&mut self, items: Vec<T>) -> ParseResult<Vec<T>>;
}
