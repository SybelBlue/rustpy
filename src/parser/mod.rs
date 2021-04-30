pub mod lexer;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FilePos {
    pub line: usize,
    pub col: usize,
}

impl FilePos {
    pub fn new() -> Self {
        Self { line: 1, col: 1 }
    }
}

impl std::fmt::Display for FilePos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
    msg: String,
    src_path: Option<String>,
    file_pos: FilePos
}

impl ParseError {
    pub fn from_str(msg: String, file_pos: FilePos) -> Self {
        Self { msg, file_pos, src_path: None }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Error parsing {}\n\t{}", 
            self.src_path.clone().map_or(
                String::from("string"), 
                |f_p| format!("file at {}:{}", f_p, self.file_pos)
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
    file_pos: FilePos,
    pos: usize,
    data: String,
}

impl StringStream {
    pub fn new(data: String) -> Self {
        Self { file_pos: FilePos::new(), pos:0, data }
    }
}

impl ParseStream<char> for StringStream {
    fn next(&mut self) -> ParseResult<char> {
        todo!()
    }

    fn try_match(&mut self, items: Vec<char>) -> ParseResult<Vec<char>> {
        todo!()
    }
}