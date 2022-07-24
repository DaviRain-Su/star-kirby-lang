use crate::ast::expression::array_literal::ArrayLiteral;
use crate::ast::expression::boolean::Boolean;
use crate::ast::expression::call_expression::CallExpression;
use crate::ast::expression::function_literal::FunctionLiteral;
use crate::ast::expression::if_expression::IfExpression;
use crate::ast::expression::index_expression::IndexExpression;
use crate::ast::expression::infix_expression::InfixExpression;
use crate::ast::expression::integer_literal::IntegerLiteral;
use crate::ast::expression::prefix_expression::PrefixExpression;
use crate::ast::expression::string_literal::StringLiteral;
use crate::ast::{Identifier, Node};
use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::ast::expression::hash_literal::HashLiteral;

pub mod array_literal;
pub mod boolean;
pub mod call_expression;
pub mod function_literal;
pub mod if_expression;
pub mod index_expression;
pub mod infix_expression;
pub mod integer_literal;
pub mod prefix_expression;
pub mod string_literal;
pub mod hash_literal;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Expression {
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
    IntegerLiteralExpression(IntegerLiteral),
    IdentifierExpression(Identifier),
    BooleanExpression(Boolean),
    IfExpression(IfExpression),
    FunctionLiteral(FunctionLiteral),
    CallExpression(CallExpression),
    StringLiteral(StringLiteral),
    ArrayLiteral(ArrayLiteral),
    IndexExpression(IndexExpression),
    HashLiteral(HashLiteral),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::PrefixExpression(pre_exp) => write!(f, "{}", pre_exp),
            Expression::InfixExpression(infix_exp) => write!(f, "{}", infix_exp),
            Expression::IntegerLiteralExpression(integ_exp) => write!(f, "{}", integ_exp),
            Expression::IdentifierExpression(ident) => write!(f, "{}", ident),
            Expression::BooleanExpression(boolean) => write!(f, "{}", boolean),
            Expression::IfExpression(if_exp) => write!(f, "{}", if_exp),
            Expression::FunctionLiteral(fun_exp) => write!(f, "{}", fun_exp),
            Expression::CallExpression(call_exp) => write!(f, "{}", call_exp),
            Expression::StringLiteral(string_exp) => write!(f, "{}", string_exp),
            Expression::ArrayLiteral(array_exp) => write!(f, "{}", array_exp),
            Expression::IndexExpression(index_exp) => write!(f, "{}", index_exp),
            Expression::HashLiteral(hash_literal) => write!(f, "{}", hash_literal),
        }
    }
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Self::PrefixExpression(pre_exp) => pre_exp.token_literal(),
            Self::InfixExpression(infix_exp) => infix_exp.token_literal(),
            Self::IntegerLiteralExpression(integ_exp) => integ_exp.token_literal(),
            Self::IdentifierExpression(ident) => ident.token_literal(),
            Self::BooleanExpression(boolean) => boolean.token_literal(),
            Self::IfExpression(if_exp) => if_exp.token_literal(),
            Self::FunctionLiteral(fun_exp) => fun_exp.token_literal(),
            Self::CallExpression(call_exp) => call_exp.token_literal(),
            Self::StringLiteral(string_exp) => string_exp.token_literal(),
            Self::ArrayLiteral(array_exp) => array_exp.token_literal(),
            Self::IndexExpression(index_exp) => index_exp.token_literal(),
            Self::HashLiteral(hash_literal) => hash_literal.token_literal(),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<PrefixExpression> for Expression {
    fn from(pre_exp: PrefixExpression) -> Self {
        Self::PrefixExpression(pre_exp)
    }
}

impl From<IntegerLiteral> for Expression {
    fn from(integ_exp: IntegerLiteral) -> Self {
        Self::IntegerLiteralExpression(integ_exp)
    }
}

impl From<Identifier> for Expression {
    fn from(identifier: Identifier) -> Self {
        Self::IdentifierExpression(identifier)
    }
}

impl From<InfixExpression> for Expression {
    fn from(infix_exp: InfixExpression) -> Self {
        Self::InfixExpression(infix_exp)
    }
}

impl From<Boolean> for Expression {
    fn from(boolean: Boolean) -> Self {
        Self::BooleanExpression(boolean)
    }
}

impl From<IfExpression> for Expression {
    fn from(if_exp: IfExpression) -> Self {
        Self::IfExpression(if_exp)
    }
}

impl From<FunctionLiteral> for Expression {
    fn from(fn_exp: FunctionLiteral) -> Self {
        Self::FunctionLiteral(fn_exp)
    }
}

impl From<CallExpression> for Expression {
    fn from(call_exp: CallExpression) -> Self {
        Self::CallExpression(call_exp)
    }
}

impl From<StringLiteral> for Expression {
    fn from(string_lit: StringLiteral) -> Self {
        Self::StringLiteral(string_lit)
    }
}

impl From<ArrayLiteral> for Expression {
    fn from(array_exp: ArrayLiteral) -> Self {
        Self::ArrayLiteral(array_exp)
    }
}

impl From<IndexExpression> for Expression {
    fn from(index_exp: IndexExpression) -> Self {
        Self::IndexExpression(index_exp)
    }
}

impl From<HashLiteral> for Expression {
    fn from(hash_literal: HashLiteral) -> Self {
        Self::HashLiteral(hash_literal)
    }
}
