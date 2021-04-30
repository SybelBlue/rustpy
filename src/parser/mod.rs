pub mod lexer;

pub type ParseResult<T> = Result<T, ParseError>;
pub struct ParseError {
    pub msg: String,
    pub src_path: String,
    pub line: usize,
    pub col: usize,
}