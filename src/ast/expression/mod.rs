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
            Self::Prefix(value) => write!(f, "{value}"),
            Self::Infix(value) => write!(f, "{value}"),
            Self::IntegerLiteral(value) => write!(f, "{value}"),
            Self::Identifier(value) => write!(f, "{value}"),
            Self::Boolean(value) => write!(f, "{value}"),
            Self::If(value) => write!(f, "{value}"),
            Self::FunctionLiteral(value) => write!(f, "{value}"),
            Self::Call(value) => write!(f, "{value}"),
            Self::StringLiteral(value) => write!(f, "{value}"),
            Self::ArrayLiteral(value) => write!(f, "{value}"),
            Self::Index(value) => write!(f, "{value}"),
            Self::HashLiteral(value) => write!(f, "{value}"),
        }
    }
}

impl NodeInterface for Expression {
    fn token_literal(&self) -> &str {
        match self {
            Self::Prefix(value) => value.token_literal(),
            Self::Infix(value) => value.token_literal(),
            Self::IntegerLiteral(value) => value.token_literal(),
            Self::Identifier(value) => value.token_literal(),
            Self::Boolean(value) => value.token_literal(),
            Self::If(value) => value.token_literal(),
            Self::FunctionLiteral(value) => value.token_literal(),
            Self::Call(value) => value.token_literal(),
            Self::StringLiteral(value) => value.token_literal(),
            Self::ArrayLiteral(value) => value.token_literal(),
            Self::Index(value) => value.token_literal(),
            Self::HashLiteral(value) => value.token_literal(),
        }
    }
}

impl From<Prefix> for Expression {
    fn from(value: Prefix) -> Self {
        Self::Prefix(value)
    }
}

impl From<IntegerLiteral> for Expression {
    fn from(value: IntegerLiteral) -> Self {
        Self::IntegerLiteral(value)
    }
}

impl From<Identifier> for Expression {
    fn from(value: Identifier) -> Self {
        Self::Identifier(value)
    }
}

impl From<Infix> for Expression {
    fn from(value: Infix) -> Self {
        Self::Infix(value)
    }
}

impl From<Boolean> for Expression {
    fn from(value: Boolean) -> Self {
        Self::Boolean(value)
    }
}

impl From<If> for Expression {
    fn from(value: If) -> Self {
        Self::If(value)
    }
}

impl From<FunctionLiteral> for Expression {
    fn from(value: FunctionLiteral) -> Self {
        Self::FunctionLiteral(value)
    }
}

impl From<Call> for Expression {
    fn from(value: Call) -> Self {
        Self::Call(value)
    }
}

impl From<StringLiteral> for Expression {
    fn from(value: StringLiteral) -> Self {
        Self::StringLiteral(value)
    }
}

impl From<ArrayLiteral> for Expression {
    fn from(value: ArrayLiteral) -> Self {
        Self::ArrayLiteral(value)
    }
}

impl From<Index> for Expression {
    fn from(value: Index) -> Self {
        Self::Index(value)
    }
}

impl From<HashLiteral> for Expression {
    fn from(value: HashLiteral) -> Self {
        Self::HashLiteral(value)
    }
}
