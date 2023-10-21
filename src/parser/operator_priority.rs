use crate::token::token_type::TokenType;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
#[repr(C)]
pub enum OperatorPriority {
    LOWEST = 1,
    EQUALS = 2,      // ==
    LESSGREATER = 3, // < or >
    SUM = 4,         // +
    PRODUCT = 5,     // *
    PREFIX = 6,      // -X or !x
    CALL = 7,        // myFcuntion(x)
    INDEX = 8,       // array[index]
}

// precedences 就是优先级表，用于将词法单元类型与其优先级相关联。
// TODO 这里不理解
lazy_static! {
    static ref PRECEDENCES: HashMap<TokenType, OperatorPriority> = {
        let mut m = HashMap::new();
        m.insert(TokenType::LPAREN, OperatorPriority::CALL);
        m.insert(TokenType::EQ, OperatorPriority::EQUALS);
        m.insert(TokenType::NOTEQ, OperatorPriority::EQUALS);
        m.insert(TokenType::LT, OperatorPriority::LESSGREATER);
        m.insert(TokenType::GT, OperatorPriority::LESSGREATER);
        m.insert(TokenType::PLUS, OperatorPriority::SUM);
        m.insert(TokenType::MINUS, OperatorPriority::SUM);
        m.insert(TokenType::SLASH, OperatorPriority::PRODUCT);
        m.insert(TokenType::ASTERISK, OperatorPriority::PRODUCT);
        m.insert(TokenType::LBRACKET, OperatorPriority::INDEX);
        m
    };
}

pub fn precedence(token_type: TokenType) -> OperatorPriority {
    match PRECEDENCES.get(&token_type) {
        Some(value) => value.clone(),
        None => OperatorPriority::LOWEST,
    }
}

#[test]
fn test_operator_priority_type() {
    assert_eq!(OperatorPriority::LOWEST as u8, 1);
    assert_eq!(OperatorPriority::EQUALS as u8, 2);
    assert_eq!(OperatorPriority::LESSGREATER as u8, 3);
    assert_eq!(OperatorPriority::SUM as u8, 4);
    assert_eq!(OperatorPriority::PRODUCT as u8, 5);
    assert_eq!(OperatorPriority::PREFIX as u8, 6);
    assert_eq!(OperatorPriority::CALL as u8, 7);
}
