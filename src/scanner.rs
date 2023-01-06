use crate::errors;
use crate::tokens::{get_reserved_keyword, Token, TokenType};

use std::io::{prelude::*, BufWriter, ErrorKind, SeekFrom};
use std::iter::Iterator;

pub(crate) struct Scanner<R>
where
    R: Read + Seek,
{
    contents: R,
    start: u32,
    current: u32,
    line: u32,
    end_reached: bool,
}

impl<R: Read + Seek> Iterator for Scanner<R> {
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

impl<R: Read + Seek> Scanner<R> {
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
            "!" => {
                let is_bang_equal = self.match_curr("=");
                let (token_type, lexeme) = if is_bang_equal {
                    (TokenType::BangEqual, "!=")
                } else {
                    (TokenType::Bang, "!")
                };
                Token {
                    ttype: token_type,
                    lexeme: lexeme.to_string(),
                    line_num: self.line,
                }
            }

            "=" => {
                let is_equal_equal = self.match_curr("=");
                let (token_type, lexeme) = if is_equal_equal {
                    (TokenType::EqualEqual, "==")
                } else {
                    (TokenType::Equal, "=")
                };
                Token {
                    ttype: token_type,
                    lexeme: lexeme.to_string(),
                    line_num: self.line,
                }
            }

            "<" => {
                let is_less_equal = self.match_curr("=");
                let (token_type, lexeme) = if is_less_equal {
                    _ = self.advance();
                    (TokenType::LessEqual, "<=")
                } else {
                    (TokenType::Less, "<")
                };
                Token {
                    ttype: token_type,
                    lexeme: lexeme.to_string(),
                    line_num: self.line,
                }
            }

            ">" => {
                let is_greater_equal = self.match_curr("=");
                let (token_type, lexeme) = if is_greater_equal {
                    (TokenType::GreaterEqual, ">=")
                } else {
                    (TokenType::Greater, ">")
                };
                Token {
                    ttype: token_type,
                    lexeme: lexeme.to_string(),
                    line_num: self.line,
                }
            }

            "/" => {
                let is_comment = self.match_curr("/");
                if is_comment {
                    // consume the comment
                    loop {
                        let next_char = self.advance();
                        if next_char.is_none() {
                            return Ok(None);
                        }

                        if next_char.unwrap() == '\n' {
                            break;
                        }
                    }

                    return self.scan_token();
                }

                Token {
                    ttype: TokenType::Slash,
                    lexeme: "/".to_string(),
                    line_num: self.line,
                }
            }

            "\"" => {
                let token = self.extract_string_token();
                if token.is_err() {
                    return Err(token.err().unwrap());
                } else {
                    let contents = token.unwrap();
                    Token {
                        ttype: TokenType::String(contents),
                        lexeme: String::new(),
                        line_num: self.line,
                    }
                }
            }

            d if d.chars().collect::<Vec<char>>()[0].is_numeric() => {
                let token = self.extract_number(d);
                if token.is_err() {
                    return Err(token.err().unwrap());
                } else {
                    Token {
                        ttype: TokenType::Number(token.unwrap()),
                        lexeme: String::new(),
                        line_num: self.line,
                    }
                }
            }

            a if a.chars().collect::<Vec<char>>()[0].is_alphanumeric() => {
                let token = self.extract_identifier(a);
                if token.is_err() {
                    return Err(token.err().unwrap());
                } else {
                    token.unwrap()
                }
            }

            // note that (for \n) this means that stdin based contents will exhaust itself, hitting eof and
            // the iterator will exhaust itself
            " " | "\n" | "\r" | "\t" => return self.scan_token(),

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

    fn match_curr(&mut self, value: &str) -> bool {
        let next_c = self.peek();
        return next_c.is_some() && next_c.unwrap().to_string() == value;
    }

    fn peek(&mut self) -> Option<char> {
        let curr_pos = self.contents.seek(SeekFrom::Current(0));
        if curr_pos.is_err() {
            panic!("seeking failed {:?}", curr_pos.err());
        }
        let curr_pos = curr_pos.unwrap();
        let mut buf: [u8; 1] = [0; 1];
        let result = self.contents.read_exact(&mut buf);
        if result.is_err() {
            match self.contents.seek(SeekFrom::Start(curr_pos)) {
                Ok(_) => {}
                Err(err) => panic!("seeking failed {:?}", err),
            };
            return None;
        }
        match self.contents.seek(SeekFrom::Start(curr_pos)) {
            Ok(_) => {}
            Err(err) => panic!("seeking failed {:?}", err),
        };

        return Some(buf[0] as char);
    }

    fn extract_string_token(&mut self) -> Result<String, String> {
        let mut string_content = String::new();
        // TODO: the extract* methods seem to have a common pattern of peek/advance, filter, accumulate etc. Might
        // have a lambda accepting function here.
        loop {
            let c = self.advance();

            if c.is_none() {
                return Err("unterminated string".to_string());
            }
            let c = c.unwrap();
            if c == '"' {
                return Ok(string_content);
            }
            string_content.push_str(c.to_string().as_str());
        }
    }

    fn extract_number(&mut self, start: &str) -> Result<f64, String> {
        let mut content = String::new();
        content.push_str(start);

        loop {
            let next_c = self.peek();
            if next_c.is_none() {
                break;
            }

            let next_c = next_c.unwrap();
            if !next_c.is_numeric() && next_c != '.' {
                break;
            }

            _ = self.advance();
            content.push_str(next_c.to_string().as_str());
        }

        let num = content.parse::<f64>();
        if num.is_err() {
            return Err(num.err().unwrap().to_string());
        } else {
            return Ok(num.unwrap());
        }
    }

    fn extract_identifier(&mut self, start: &str) -> Result<Token, String> {
        let mut content = String::new();
        content.push_str(start);

        loop {
            let next_c = self.peek();
            if next_c.is_none() {
                break;
            }

            let next_c = next_c.unwrap();
            if !next_c.is_alphanumeric() {
                break;
            }

            _ = self.advance();
            content.push_str(next_c.to_string().as_str());
        }

        let mut token = Token {
            ttype: TokenType::Identifier,
            lexeme: content.clone(),
            line_num: self.line,
        };

        if let Some(keyword) = get_reserved_keyword(content.as_str()) {
            token.ttype = keyword;
        }

        return Ok(token);
    }
}
