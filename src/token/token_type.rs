use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone, Hash, Ord, PartialOrd)]
pub enum TokenType {
    /// illegal
    ILLEGAL,
    /// eof
    EOF,

    // identifier + literals
    /// add，foobar, x, y, z,...
    IDENT,
    /// 12345
    INT,
    /// "String"
    STRING,

    /// =
    ASSIGN,
    /// +
    PLUS,
    /// -
    MINUS,
    /// !
    BANG,
    /// *
    ASTERISK,
    /// /
    SLASH,

    /// <
    LT,
    /// >
    GT,

    /// ==
    EQ,
    /// !=
    NOTEQ,

    /// ,
    COMMA,
    /// ;
    SEMICOLON,
    /// :
    COLON,

    /// (
    LPAREN,
    /// )
    RPAREN,
    /// {
    LBRACE,
    /// }
    RBRACE,
    /// [
    LBRACKET,
    /// ]
    RBRACKET,

    /// fn
    FUNCTION,
    /// let
    LET,
    /// true
    TRUE,
    /// false
    FALSE,
    /// if
    IF,
    /// else
    ELSE,
    /// return
    RETURN,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ILLEGAL => write!(f, "illegal"),
            Self::EOF => write!(f, "eof"),
            Self::IDENT => write!(f, "ident"),
            Self::INT => write!(f, "int"),
            Self::STRING => write!(f, "String"),
            Self::ASSIGN => write!(f, "="),
            Self::PLUS => write!(f, "+"),
            Self::MINUS => write!(f, "-"),
            Self::BANG => write!(f, "!"),
            Self::ASTERISK => write!(f, "*"),
            Self::SLASH => write!(f, "/"),
            Self::LT => write!(f, "<"),
            Self::GT => write!(f, ">"),
            Self::EQ => write!(f, "=="),
            Self::NOTEQ => write!(f, "!="),
            Self::COMMA => write!(f, ","),
            Self::SEMICOLON => write!(f, ";"),
            Self::COLON => write!(f, ":"),
            Self::LPAREN => write!(f, "("),
            Self::RPAREN => write!(f, ")"),
            Self::LBRACE => write!(f, "{{"),
            Self::RBRACE => write!(f, "}}"),
            Self::LBRACKET => write!(f, "["),
            Self::RBRACKET => write!(f, "]"),
            Self::FUNCTION => write!(f, "fn"),
            Self::LET => write!(f, "let"),
            Self::TRUE => write!(f, "true"),
            Self::FALSE => write!(f, "false"),
            Self::IF => write!(f, "if"),
            Self::ELSE => write!(f, "else"),
            Self::RETURN => write!(f, "return"),
        }
    }
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("fn", TokenType::FUNCTION);
        m.insert("let", TokenType::LET);
        m.insert("true", TokenType::TRUE);
        m.insert("false", TokenType::FALSE);
        m.insert("if", TokenType::IF);
        m.insert("else", TokenType::ELSE);
        m.insert("return", TokenType::RETURN);
        m
    };
}

pub fn lookup_ident(ident: &str) -> TokenType {
    match KEYWORDS.get(ident) {
        Some(value) => value.clone(),
        None => TokenType::IDENT,
    }
}

pub fn lookup_char(ch: char) -> TokenType {
    match ch {
        '/' => TokenType::SLASH,
        '*' => TokenType::ASTERISK,
        '<' => TokenType::LT,
        '>' => TokenType::GT,
        ';' => TokenType::SEMICOLON,
        '(' => TokenType::LPAREN,
        ')' => TokenType::RPAREN,
        ',' => TokenType::COMMA,
        '+' => TokenType::PLUS,
        '{' => TokenType::LBRACE,
        '}' => TokenType::RBRACE,
        '[' => TokenType::LBRACKET,
        ']' => TokenType::RBRACKET,
        ':' => TokenType::COLON,
        '-' => TokenType::MINUS,
        '!' => TokenType::BANG,
        '=' => TokenType::ASSIGN,
        _ => TokenType::ILLEGAL,
    }
}
