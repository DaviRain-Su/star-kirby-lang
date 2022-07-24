use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone, Hash, Ord, PartialOrd)]
pub enum TokenType {
    ILLEGAL, // illegal
    EOF,     // eof

    // identifier + literals
    IDENT,  // add，foobar, x, y, z,...
    INT,    // 12345
    STRING, // "String"

    // 运算符
    ASSIGN,   // =
    PLUS,     // +
    MINUS,    // -
    BANG,     // !
    ASTERISK, // *
    SLASH,    // /

    LT, // <
    GT, // >

    EQ,    // ==
    NOTEQ, // !=

    // 分隔符
    COMMA,     // ,
    SEMICOLON, // ;
    COLON,     // :

    LPAREN,   // (
    RPAREN,   // )
    LBRACE,   // {
    RBRACE,   // }
    LBRACKET, // [
    RBRACKET, // ]

    // 关键字
    FUNCTION, // fn
    LET,      // let
    TRUE,     // true
    FALSE,    // false
    IF,       // if
    ELSE,     // else
    RETURN,   // return
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ILLEGAL => write!(f, "illegal"), // illegal
            Self::EOF => write!(f, "eof"),         // eof
            // identifier + literals
            Self::IDENT => write!(f, "ident"), // add，foobar, x, y, z,...
            Self::INT => write!(f, "int"),     // 12345
            Self::STRING => write!(f, "String"), //
            // 运算符
            Self::ASSIGN => write!(f, "="),   // =
            Self::PLUS => write!(f, "+"),     // +
            Self::MINUS => write!(f, "-"),    // -
            Self::BANG => write!(f, "!"),     // !
            Self::ASTERISK => write!(f, "*"), // *
            Self::SLASH => write!(f, "/"),    //  /
            Self::LT => write!(f, "<"),       // <
            Self::GT => write!(f, ">"),       // >

            Self::EQ => write!(f, "=="),    // ==
            Self::NOTEQ => write!(f, "!="), // !=

            // 分隔符
            Self::COMMA => write!(f, ","),     // ,
            Self::SEMICOLON => write!(f, ";"), // ;
            Self::COLON => write!(f, ":"),     // :

            Self::LPAREN => write!(f, "("),   // (
            Self::RPAREN => write!(f, ")"),   // )
            Self::LBRACE => write!(f, "{{"),  // {
            Self::RBRACE => write!(f, "}}"),  // }
            Self::LBRACKET => write!(f, "["), // [
            Self::RBRACKET => write!(f, "]"), // ]

            // 关键字
            Self::FUNCTION => write!(f, "fn"),   // fn
            Self::LET => write!(f, "let"),       // let
            Self::TRUE => write!(f, "true"),     // true
            Self::FALSE => write!(f, "false"),   // false
            Self::IF => write!(f, "if"),         // if
            Self::ELSE => write!(f, "else"),     // else
            Self::RETURN => write!(f, "return"), // return
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

/// LookupIdent 通过检查关键字表来判断给定的标识符是否是关键字。如果是，则
/// 返回关键字的 TokenType 常量。如果不是，则返回 token.IDENT，这个 TokenType 表
/// 示当前是用户定义的标识符。
pub fn lookup_ident(ident: &str) -> TokenType {
    match KEYWORDS.get(ident) {
        Some(value) => value.clone(),
        None => TokenType::IDENT,
    }
}
