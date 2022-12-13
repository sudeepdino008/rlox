use crate::tokens::{Token, TokenType};

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Seek};
use std::iter::Iterator;

pub(crate) struct Scanner<R>
where
    R: Read + Seek,
{
    reader: BufReader<R>,
    start: u32,
    current: u32,
    line: u32,
}

impl<R: Read + Seek> Iterator for Scanner<R> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        //self.reader.
        if !self.is_at_end() {
            self.start = self.current;
            return Some(self.scanToken());
        }

        return Some(Token {
            ttype: TokenType::Eof,
            lexeme: String::from(""),
            line_num: self.line,
        });
    }
}

impl<R: Read + Seek> Scanner<R> {
    fn build_scanner(reader: BufReader<R>) -> Scanner<R> {
        return Scanner {
            reader: reader,
            start: 0,
            current: 0,
            line: 1,
        };
    }

    fn is_at_end(&mut self) -> bool {
        let result = &self.reader.seek(std::io::SeekFrom::Current(1));
        if result.is_err() {
            println!("isAtEnd err: {}", result.as_ref().err().unwrap());
        }

        return result.is_err();
    }

    fn scanToken(&mut self) -> Token {
        let c = self.advance(); // not caring a lot about unicodes here!!
        return Token {
            ttype: TokenType::And,
            lexeme: String::from(c),
            line_num: self.line,
        };
    }

    fn advance(&mut self) -> char {
        let mut buf: [u8; 1] = [0; 1];
        self.reader.read_exact(&mut buf).unwrap();
        let c = buf[0] as char;
        if c == '\n' {
            self.line = self.line + 1;
        }
        return c;
    }
}
