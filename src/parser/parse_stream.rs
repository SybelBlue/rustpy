use std::fs::read_to_string;

use crate::parser::*;

pub trait ParseStream<T> {
    fn next(&mut self) -> ParseResult<T>;
    fn try_match(&mut self, items: Vec<T>) -> ParseResult<Vec<T>>;
}

impl<T> Iterator for dyn ParseStream<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next().ok()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StringStream {
    file_pos: FilePos,
    pos: usize,
    data: Vec<char>,
}

impl StringStream {
    pub fn new(text: &str) -> Self {
        Self { file_pos: FilePos::new(), pos: 0, data: text.chars().collect() }
    }

    fn check_eof(&self) -> ParseResult<()> {
        if self.pos >= self.data.len() {
            Err(ParseError::eof(self.file_pos, None))
        } else {
            Ok(())
        }
    }
}

impl ParseStream<char> for StringStream {
    fn next(&mut self) -> ParseResult<char> {
        self.check_eof()?;
        let c = self.data[self.pos];
        self.pos += 1;
        self.file_pos.advance(c);
        Ok(c)
    }

    fn try_match(&mut self, items: Vec<char>) -> ParseResult<Vec<char>> {
        self.check_eof()?;
        let mut file_pos = self.file_pos.clone();
        for (&a, &b) in self.data[self.pos..].iter().zip(items.iter()) {
            if a != b {
                return Err(
                    ParseError::mismatch(a, b, file_pos)
                )
            } else {
                file_pos.advance(a)
            }
        }
        self.file_pos = file_pos;
        self.pos += items.len();
        Ok(items)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FileStream {
    src_path: String,
    string_stream: StringStream,
}

impl FileStream {
    pub fn new(src_path: String) -> std::io::Result<Self> {
        let s = read_to_string(src_path.clone())?;
        let string_stream = StringStream::new(s.as_str());
        Ok(Self { src_path, string_stream })
    }
}

impl ParseStream<char> for FileStream {
    fn next(&mut self) -> ParseResult<char> {
        let r = self.string_stream.next();
        r.map_err(|e| e.with_src_path(self.src_path.clone()))
    }

    fn try_match(&mut self, items: Vec<char>) -> ParseResult<Vec<char>> {
        let r = self.string_stream.try_match(items);
        r.map_err(|e| e.with_src_path(self.src_path.clone()))
    }
}

pub fn skip_whitespace<S>(stream: &mut S) where S : ParseStream<char> {
    while let Ok(_) = stream.try_match(vec![' ']) {}
}
