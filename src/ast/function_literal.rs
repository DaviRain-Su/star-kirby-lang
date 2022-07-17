use crate::ast::statement::block_statement::BlockStatement;
use crate::ast::Identifier;
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct FunctionLiteral {
    token: Token, // 'fn'词法单元
    parameters: Vec<Identifier>,
    body: BlockStatement,
}
