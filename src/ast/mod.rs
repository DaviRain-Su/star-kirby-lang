pub mod expression;
pub mod statement;

#[cfg(test)]
mod tests;

use crate::ast::expression::Expression;
use crate::ast::statement::Statement;
use crate::error::Error;
use crate::object::array::Array;
use crate::object::environment::Environment;
use crate::object::function::Function;
use crate::object::integer::Integer;
use crate::object::r#macro::quote::Quote;
use crate::object::return_value::ReturnValue;
use crate::object::string::StringObj;
use crate::object::Object;
use crate::token::Token;
use crate::NULL;
use crate::{ast::expression::boolean::Boolean, object::boolean::Boolean as ObjBoolean};
use log::trace;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub enum Node {
    Program(Program),
    Expression(Expression),
    Statement(Statement), // expression statement, return statement, let statement
    Object(Object),
}

impl Node {
    pub fn quote(&self) -> anyhow::Result<Object> {
        match self {
            Node::Program(program) => Err(Error::UnknownTypeError(format!("{program:?}")).into()),
            Node::Expression(expression) => Ok(Quote::new(expression.into()).into()),
            Node::Statement(statement) => Ok(Quote::new(statement.into()).into()),
            Node::Object(object) => Ok(Quote::new(object.into()).into()),
        }
    }

    pub fn eval(&self, env: &mut Environment) -> anyhow::Result<Object> {
        match self {
            Node::Program(ref program) => program.eval_program(env),
            Node::Statement(ref statement) => match statement {
                Statement::Expression(exp) => {
                    let expression_node: Node = exp.expression.clone().into();
                    expression_node.eval(env)
                }
                Statement::Let(let_statement) => {
                    let val_node = Node::from(*let_statement.value.clone());
                    let val = val_node.eval(env)?;

                    env.store(let_statement.name.value.clone(), val);

                    Ok(NULL.into())
                }
                Statement::Return(return_statement) => {
                    let val_node = Node::from(*return_statement.return_value.clone());
                    let val = val_node.eval(env)?;
                    Ok(ReturnValue::new(val).into())
                }
                Statement::BlockStatement(block_statement) => {
                    block_statement.eval_block_statement(env)
                }
            },
            Node::Expression(ref expression) => match expression {
                Expression::Prefix(prefix) => {
                    let right_node = Node::from(prefix.right().clone());
                    let right = right_node.eval(env)?;
                    Ok(right.eval_prefix_expression(prefix.operator()))
                }
                Expression::Infix(infix) => {
                    let left_node = Node::from(infix.left().clone());
                    let left = left_node.eval(env)?;
                    let right_node = Node::from(infix.right().clone());
                    let right = right_node.eval(env)?;

                    left.eval_infix_expression(infix.operator(), right)
                }
                Expression::IntegerLiteral(integer) => Ok(Integer::new(integer.value()).into()),
                Expression::Identifier(identifier) => identifier.eval_identifier(env),
                Expression::Boolean(boolean) => {
                    Ok(Object::Boolean(ObjBoolean::new(boolean.value())))
                }
                Expression::If(if_exp) => if_exp.eval_if_expression(env),
                Expression::FunctionLiteral(function) => {
                    let params = function.parameters().clone();
                    let body = function.body().clone();

                    Ok(Function::new(params, body, env.clone()).into())
                }
                Expression::Call(call_exp) => {
                    if call_exp.function().token_literal() == *"quote" {
                        return Node::from(call_exp.arguments()[0].clone()).quote();
                    }
                    let call_exp_node = Node::from(call_exp.function().clone());
                    let function = call_exp_node.eval(env)?;

                    let args = eval_expressions(call_exp.arguments().clone(), env)?;

                    function.apply_function(args)
                }
                Expression::StringLiteral(string_literal) => {
                    Ok(StringObj::new(string_literal.value().to_string()).into())
                }
                Expression::ArrayLiteral(array) => {
                    let elements = eval_expressions(array.elements().clone(), env)?;

                    Ok(Array::new(elements.into_iter().collect()).into())
                }
                Expression::Index(indx_exp) => {
                    let left_node = Node::from(indx_exp.left().clone());
                    let left = left_node.eval(env)?;
                    let index_node = Node::from(indx_exp.index().clone());
                    let index = index_node.eval(env)?;

                    left.eval_index_expression(index)
                }
                Expression::HashLiteral(hash_literal) => hash_literal.eval_hash_literal(env),
            },
            Node::Object(object) => {
                Err(Error::UnknownTypeError(format!("object: {object:?}")).into())
            }
        }
    }
}

fn eval_expressions(exps: Vec<Expression>, env: &mut Environment) -> anyhow::Result<Vec<Object>> {
    trace!("[eval_expressions] start");

    let mut result = vec![];

    for e in exps.into_iter() {
        let node = Node::from(e);
        let evaluated = node.eval(env)?;
        trace!("[eval_expressions] evaluated is = {:?}", evaluated);
        result.push(evaluated);
    }

    trace!("[eval_expressions] end");

    Ok(result)
}

impl From<Program> for Node {
    fn from(program: Program) -> Self {
        Self::Program(program)
    }
}

impl From<Expression> for Node {
    fn from(value: Expression) -> Self {
        Self::Expression(value)
    }
}

impl From<&Expression> for Node {
    fn from(value: &Expression) -> Self {
        Self::Expression(value.clone())
    }
}

impl From<Statement> for Node {
    fn from(value: Statement) -> Self {
        Self::Statement(value)
    }
}

impl From<&Statement> for Node {
    fn from(value: &Statement) -> Self {
        Self::Statement(value.clone())
    }
}

impl From<Object> for Node {
    fn from(value: Object) -> Self {
        Self::Object(value)
    }
}

impl From<&Object> for Node {
    fn from(value: &Object) -> Self {
        Self::Object(value.clone())
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Expression(value) => write!(f, "{value}"),
            Node::Statement(value) => write!(f, "{value}"),
            Node::Object(value) => write!(f, "{value}"),
            Node::Program(value) => write!(f, "{value}"),
        }
    }
}

pub trait NodeInterface: Debug + Display {
    /// 必须提供 TokenLiteral()方法，该方法返回与其
    /// 关联的词法单元的字面量
    fn token_literal(&self) -> String;
}

/// 这个 Program 节点将成为语法分析器生成的每个 AST 的根节点。每个有效的
/// Monkey 程序都是一系列位于 Program.Statements 中的语句。Program.Statements
/// 是一个切片，其中有实现 Statement 接口的 AST 节点。
#[derive(Debug, Default, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)] // add debug trait for debug
pub struct Program {
    pub(crate) statements: Vec<Statement>,
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for statement in self.statements.iter() {
            write!(f, "{statement}")?;
        }

        Ok(())
    }
}

impl Program {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn token_literal(&self) -> String {
        if self.statements.is_empty() {
            String::new()
        } else {
            self.statements
                .first()
                .expect("never failed")
                .token_literal()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }

    pub fn len(&self) -> usize {
        self.statements.len()
    }

    pub fn eval_program(&self, env: &mut Environment) -> anyhow::Result<Object> {
        trace!("[eval_program]  program is ({})", self);
        let mut result: Object = NULL.into();

        for statement in self.statements.clone().into_iter() {
            let statement_node: Node = statement.into();
            result = statement_node.eval(env)?;

            match result {
                Object::ReturnValue(value) => {
                    trace!("[eval_statement] ReturnValue is ({:?})", value);
                    return Ok(value.value().clone());
                }
                _ => continue,
            }
        }

        Ok(result)
    }
}

impl NodeInterface for Program {
    fn token_literal(&self) -> String {
        self.token_literal()
    }
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Identifier {
    pub token: Token, // token.IDENT 词法单元
    pub value: String,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Identifier {
    pub fn new(token: Token, value: String) -> Self {
        Self { token, value }
    }
}

impl From<Token> for Identifier {
    fn from(token: Token) -> Self {
        Self {
            value: token.literal().into(),
            token,
        }
    }
}

impl From<Boolean> for Identifier {
    fn from(boolean: Boolean) -> Self {
        Self {
            value: boolean.value().to_string(),
            token: boolean.token().clone(),
        }
    }
}

impl NodeInterface for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal().into()
    }
}

impl TryFrom<Expression> for Identifier {
    type Error = anyhow::Error;

    fn try_from(expression: Expression) -> Result<Self, Self::Error> {
        match expression {
            Expression::Identifier(ident) => Ok(ident),
            Expression::IntegerLiteral(integ) => Ok(Identifier {
                value: integ.value().to_string(),
                token: integ.token().clone(),
            }),
            Expression::Boolean(boolean) => Ok(Identifier {
                token: boolean.token().clone(),
                value: boolean.value().to_string(),
            }),
            _ => {
                trace!("Expression: {expression}");
                unimplemented!()
            }
        }
    }
}
