use crate::tokens::{Token, TokenType};

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Seek};
use std::iter::Iterator;

pub(crate) struct Scanner<R>
where
    R: Read,
{
    contents: R,
    start: u32,
    current: u32,
    line: u32,
    end_reached: bool,
}

impl<R: Read> Iterator for Scanner<R> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end_reached {
            return None;
        }
        let result = self.scanToken();
        if result.is_none() {
            self.end_reached = true;
            return Some(Token {
                ttype: TokenType::Eof,
                lexeme: String::from(""),
                line_num: self.line,
            });
        } else {
            return result;
        }
    }
}

impl<R: Read> Scanner<R> {
    pub fn build_scanner(reader: R) -> Scanner<R> {
        return Scanner {
            contents: reader,
            start: 0,
            current: 0,
            line: 1,
            end_reached: false,
        };
    }

    // fn is_at_end(&mut self) -> bool {
    //     let result = &self.reader.seek(std::io::SeekFrom::Current(1));
    //     if result.is_err() {
    //         println!("isAtEnd err: {}", result.as_ref().err().unwrap());
    //     }

    //     return result.is_err();
    // }

    fn scanToken(&mut self) -> Option<Token> {
        let c = self.advance(); // not caring a lot about unicodes here!!
        if c.is_none() {
            return None;
        }
        return Some(Token {
            ttype: TokenType::And,
            lexeme: String::from(c.unwrap()),
            line_num: self.line,
        });
    }

    fn advance(&mut self) -> Option<char> {
        let mut buf: [u8; 1] = [0; 1];
        let result = self.contents.read_exact(&mut buf);
        if result.is_err() {
            println!("the error is: {}", result.err().unwrap());
            return None;
        }
        let c = buf[0] as char;
        if c == '\n' {
            self.line = self.line + 1;
        }
        return Some(c);
    }
}
