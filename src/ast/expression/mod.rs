use crate::ast::expression::array::ArrayLiteral;
use crate::ast::expression::boolean::Boolean;
use crate::ast::expression::call::Call;
use crate::ast::expression::function::FunctionLiteral;
use crate::ast::expression::hash::HashLiteral;
use crate::ast::expression::if_expression::If;
use crate::ast::expression::index::Index;
use crate::ast::expression::infix::Infix;
use crate::ast::expression::integer::IntegerLiteral;
use crate::ast::expression::prefix::Prefix;
use crate::ast::expression::string::StringLiteral;
use crate::ast::Identifier;
use crate::ast::NodeInterface;
use std::fmt::{Display, Formatter};

pub mod array;
pub mod boolean;
pub mod call;
pub mod function;
pub mod hash;
pub mod if_expression;
pub mod index;
pub mod infix;
pub mod integer;
pub mod prefix;
pub mod string;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Expression {
    Prefix(Prefix),
    Infix(Infix),
    IntegerLiteral(IntegerLiteral),
    Identifier(Identifier),
    Boolean(Boolean),
    If(If),
    FunctionLiteral(FunctionLiteral),
    Call(Call),
    StringLiteral(StringLiteral),
    ArrayLiteral(ArrayLiteral),
    Index(Index),
    HashLiteral(HashLiteral),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Prefix(pre_exp) => write!(f, "{pre_exp}"),
            Self::Infix(infix_exp) => write!(f, "{infix_exp}"),
            Self::IntegerLiteral(integ_exp) => write!(f, "{integ_exp}"),
            Self::Identifier(ident) => write!(f, "{ident}"),
            Self::Boolean(boolean) => write!(f, "{boolean}"),
            Self::If(if_exp) => write!(f, "{if_exp}"),
            Self::FunctionLiteral(fun_exp) => write!(f, "{fun_exp}"),
            Self::Call(call_exp) => write!(f, "{call_exp}"),
            Self::StringLiteral(string_exp) => write!(f, "{string_exp}"),
            Self::ArrayLiteral(array_exp) => write!(f, "{array_exp}"),
            Self::Index(index_exp) => write!(f, "{index_exp}"),
            Self::HashLiteral(hash_literal) => write!(f, "{hash_literal}"),
        }
    }
}

impl NodeInterface for Expression {
    fn token_literal(&self) -> &str {
        match self {
            Self::Prefix(pre_exp) => pre_exp.token_literal(),
            Self::Infix(infix_exp) => infix_exp.token_literal(),
            Self::IntegerLiteral(integ_exp) => integ_exp.token_literal(),
            Self::Identifier(ident) => ident.token_literal(),
            Self::Boolean(boolean) => boolean.token_literal(),
            Self::If(if_exp) => if_exp.token_literal(),
            Self::FunctionLiteral(fun_exp) => fun_exp.token_literal(),
            Self::Call(call_exp) => call_exp.token_literal(),
            Self::StringLiteral(string_exp) => string_exp.token_literal(),
            Self::ArrayLiteral(array_exp) => array_exp.token_literal(),
            Self::Index(index_exp) => index_exp.token_literal(),
            Self::HashLiteral(hash_literal) => hash_literal.token_literal(),
        }
    }
}

impl From<Prefix> for Expression {
    fn from(pre_exp: Prefix) -> Self {
        Self::Prefix(pre_exp)
    }
}

impl From<IntegerLiteral> for Expression {
    fn from(integ_exp: IntegerLiteral) -> Self {
        Self::IntegerLiteral(integ_exp)
    }
}

impl From<Identifier> for Expression {
    fn from(identifier: Identifier) -> Self {
        Self::Identifier(identifier)
    }
}

impl From<Infix> for Expression {
    fn from(infix_exp: Infix) -> Self {
        Self::Infix(infix_exp)
    }
}

impl From<Boolean> for Expression {
    fn from(boolean: Boolean) -> Self {
        Self::Boolean(boolean)
    }
}

impl From<If> for Expression {
    fn from(if_exp: If) -> Self {
        Self::If(if_exp)
    }
}

impl From<FunctionLiteral> for Expression {
    fn from(fn_exp: FunctionLiteral) -> Self {
        Self::FunctionLiteral(fn_exp)
    }
}

impl From<Call> for Expression {
    fn from(call_exp: Call) -> Self {
        Self::Call(call_exp)
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

impl From<Index> for Expression {
    fn from(index_exp: Index) -> Self {
        Self::Index(index_exp)
    }
}

impl From<HashLiteral> for Expression {
    fn from(hash_literal: HashLiteral) -> Self {
        Self::HashLiteral(hash_literal)
    }
}
