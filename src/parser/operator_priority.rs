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
#[ignore]
fn test_operator_priority_type() {
    let lowest = OperatorPriority::LOWEST;
    println!("lowest: {:?}", lowest as u8);
    let equals = OperatorPriority::EQUALS;
    println!("equals: {:?}", equals as u8);
    let lessgreater = OperatorPriority::LESSGREATER;
    println!("lessgreater: {:?}", lessgreater as u8);
    let sum = OperatorPriority::SUM;
    println!("sum: {:?}", sum as u8);
    let product = OperatorPriority::PRODUCT;
    println!("product: {:?}", product as u8);
    let prefix = OperatorPriority::PREFIX;
    println!("prefix: {:?}", prefix as u8);
    let call = OperatorPriority::CALL;
    println!("call: {:?}", call as u8);
}
