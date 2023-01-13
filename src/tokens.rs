use phf::phf_map;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum TokenType {
    // single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    Identifier,
    String(String),
    Number(f64),

    // keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub line_num: u32,
}

static RESERVED_KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "And" => TokenType::And,
    "Class" => TokenType::Class,
    "Else" => TokenType::Else,
    "False" => TokenType::False,
    "Fun" => TokenType::Fun,
    "For" => TokenType::For,
    "If" => TokenType::If,
    "Nil" => TokenType::Nil,
    "Or" => TokenType::Or,
    "Print" => TokenType::Print,
    "Return" => TokenType::Return,
    "Super" => TokenType::Super,
    "This" => TokenType::This,
    "True" => TokenType::True,
    "Var" => TokenType::Var,
    "While" => TokenType::While,
};

pub fn get_reserved_keyword(keyword: &str) -> Option<TokenType> {
    RESERVED_KEYWORDS.get(keyword).cloned()
}

pub fn new_token(ttype: TokenType) -> Token {
    Token {
        ttype,
        lexeme: "".to_string(),
        line_num: 0,
    }
}
