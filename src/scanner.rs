use crate::errors;
use crate::tokens::{Token, TokenType};

use std::io::{prelude::*, ErrorKind};
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
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end_reached {
            return None;
        }
        let result = self.scan_token();
        return if result.is_ok() {
            let maybe_token = result.unwrap();
            if maybe_token.is_none() {
                self.end_reached = true;
                Some(Ok(Token {
                    ttype: TokenType::Eof,
                    lexeme: String::from(""),
                    line_num: self.line,
                }))
            } else {
                Some(Ok(maybe_token.unwrap()))
            }
        } else {
            Some(Err(result.unwrap_err()))
        };
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

    fn scan_token(&mut self) -> Result<Option<Token>, String> {
        let next_char = self.advance(); // not caring a lot about unicodes here!!
        if next_char.is_none() {
            return Ok(None);
        }

        let next_char = String::from(next_char.unwrap());

        let token = match next_char.as_str() {
            ";" => Token {
                ttype: TokenType::Semicolon,
                lexeme: next_char.to_string(),
                line_num: self.line,
            },
            "," => Token {
                ttype: TokenType::Comma,
                lexeme: next_char.to_string(),
                line_num: self.line,
            },
            "." => Token {
                ttype: TokenType::Dot,
                lexeme: next_char.to_string(),
                line_num: self.line,
            },
            "(" => Token {
                ttype: TokenType::LeftParen,
                lexeme: next_char.to_string(),
                line_num: self.line,
            },
            ")" => Token {
                ttype: TokenType::RightParen,
                lexeme: next_char.to_string(),
                line_num: self.line,
            },
            "{" => Token {
                ttype: TokenType::LeftBrace,
                lexeme: next_char.to_string(),
                line_num: self.line,
            },
            "}" => Token {
                ttype: TokenType::RightBrace,
                lexeme: next_char.to_string(),
                line_num: self.line,
            },
            "-" => Token {
                ttype: TokenType::Minus,
                lexeme: next_char.to_string(),
                line_num: self.line,
            },
            "+" => Token {
                ttype: TokenType::Plus,
                lexeme: next_char.to_string(),
                line_num: self.line,
            },
            "*" => Token {
                ttype: TokenType::Star,
                lexeme: next_char.to_string(),
                line_num: self.line,
            },

            // note that this means that stdin based contents will exhaust itself, hitting eof and
            // the iterator will exhaust itself
            "\n" => return self.scan_token(),

            _ => {
                // unhandled
                let msg = format!("error scanning token: {}", next_char);
                println!("{}", msg);
                return Err(msg);
            }
        };

        return Ok(Some(token));
    }

    fn advance(&mut self) -> Option<char> {
        let mut buf: [u8; 1] = [0; 1];
        let result = self.contents.read_exact(&mut buf);
        if result.is_err() {
            let err = result.unwrap_err();
            if err.kind() != ErrorKind::UnexpectedEof {
                println!("the error is: {}", err);
            }

            return None;
        }
        let c = buf[0] as char;
        if c == '\n' {
            self.line = self.line + 1;
        }
        return Some(c);
    }
}
