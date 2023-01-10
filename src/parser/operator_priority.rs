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
    let lowest = OperatorPriority::LOWEST;
    assert_eq!(lowest as u8, 1);
    let equals = OperatorPriority::EQUALS;
    assert_eq!(equals as u8, 2);
    let lessgreater = OperatorPriority::LESSGREATER;
    assert_eq!(lessgreater as u8, 3);
    let sum = OperatorPriority::SUM;
    assert_eq!(sum as u8, 4);
    let product = OperatorPriority::PRODUCT;
    assert_eq!(product as u8, 5);
    let prefix = OperatorPriority::PREFIX;
    assert_eq!(prefix as u8, 6);
    let call = OperatorPriority::CALL;
    assert_eq!(call as u8, 7);
}
