pub mod lexer;
pub mod parse_stream;

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

    pub fn advance(&mut self, c: &char) {
        if *c == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
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

    pub fn eof(file_pos: FilePos, src_path: Option<String>) -> Self {
        Self { file_pos, src_path, msg: String::from("End of file")}
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Error parsing {}\n\t| {}", 
            self.src_path.clone().map_or(
                String::from("string"), 
                |f_p| format!("file at {}:{}", f_p, self.file_pos)
            ),
            self.msg
        )
    }
}
