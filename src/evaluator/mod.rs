use crate::ast::expression::hash_literal::HashLiteral;
use crate::ast::expression::if_expression::IfExpression;
use crate::ast::expression::Expression;
use crate::ast::statement::block_statement::BlockStatement;
use crate::ast::statement::Statement;
use crate::ast::{Identifier, Node, NodeInterface, Program};
use crate::error::Error;
use crate::evaluator::builtins::lookup_builtin;
use crate::object::array::Array;
use crate::object::boolean::Boolean;
use crate::object::environment::Environment;
use crate::object::function::Function;
use crate::object::hash::Hash;
use crate::object::integer::Integer;
use crate::object::null::Null;
use crate::object::r#macro::quote::Quote;
use crate::object::return_value::ReturnValue;
use crate::object::string::StringObj;
use crate::object::ObjectType::{ArrayObj, HashObj, IntegerObj};
use crate::object::{Object, ObjectInterface, ObjectType};
use crate::{FALSE, NULL, TRUE};
use log::trace;
use std::collections::BTreeMap;

pub mod builtins;

#[cfg(test)]
pub mod tests;

pub fn eval(node: Node, env: &mut Environment) -> anyhow::Result<Object> {
    match node {
        Node::Program(ref program) => eval_program(program, env),
        Node::Statement(ref statement) => match statement {
            Statement::Expression(exp) => eval(exp.expression.clone().into(), env),
            Statement::Let(let_statement) => {
                let val = eval(Node::from(*let_statement.value.clone()), env)?;

                env.store(let_statement.name.value.clone(), val);

                Ok(NULL.into())
            }
            Statement::Return(return_statement) => {
                let val = eval(Node::from(*return_statement.return_value.clone()), env)?;
                Ok(ReturnValue::new(val).into())
            }
            Statement::BlockStatement(block_statement) => {
                eval_block_statement(block_statement, env)
            }
        },
        Node::Expression(ref expression) => match expression {
            Expression::PrefixExpression(prefix) => {
                let right = eval(Node::from(prefix.right().clone()), env)?;
                eval_prefix_expression(prefix.operator(), right)
            }
            Expression::InfixExpression(infix) => {
                let left = eval(Node::from(infix.left().clone()), env)?;
                let right = eval(Node::from(infix.right().clone()), env)?;

                eval_infix_expression(infix.operator(), left, right)
            }
            Expression::IntegerLiteralExpression(integer) => Ok(Integer::new(integer.value).into()),
            Expression::IdentifierExpression(identifier) => {
                eval_identifier(identifier.clone(), env)
            }
            Expression::BooleanExpression(boolean) => Ok(Boolean::new(boolean.value()).into()),
            Expression::IfExpression(if_exp) => eval_if_expression(if_exp.clone(), env),
            Expression::FunctionLiteral(function) => {
                let params = function.parameters().clone();
                let body = function.body().clone();

                Ok(Function::new(params, body, env.clone()).into())
            }
            Expression::CallExpression(call_exp) => {
                if call_exp.function().token_literal() == *"quote" {
                    return quote(Node::from(call_exp.arguments()[0].clone()));
                }
                let function = eval(Node::from(call_exp.function().clone()), env)?;

                let args = eval_expressions(call_exp.arguments().clone(), env)?;

                apply_function(function, args)
            }
            Expression::StringLiteral(string_literal) => {
                Ok(StringObj::new(string_literal.value().to_string()).into())
            }
            Expression::ArrayLiteral(array) => {
                let elements = eval_expressions(array.elements().clone(), env)?;

                Ok(Array::new(elements.into_iter().collect()).into())
            }
            Expression::IndexExpression(indx_exp) => {
                let left = eval(Node::from(indx_exp.left().clone()), env)?;
                let index = eval(Node::from(indx_exp.index().clone()), env)?;

                eval_index_expression(left, index)
            }
            Expression::HashLiteral(hash_literal) => eval_hash_literal(hash_literal.clone(), env),
        },
        Node::Object(object) => Err(Error::UnknownTypeError(format!("object: {object:?}")).into()),
    }
}

fn quote(node: Node) -> anyhow::Result<Object> {
    match node {
        Node::Program(program) => Err(Error::UnknownTypeError(format!("{program:?}")).into()),
        Node::Expression(expression) => Ok(Quote::new(expression.into()).into()),
        Node::Statement(statement) => Ok(Quote::new(statement.into()).into()),
        Node::Object(object) => Ok(Quote::new(object.into()).into()),
    }
}

fn apply_function(fn_obj: Object, args: Vec<Object>) -> anyhow::Result<Object> {
    match fn_obj {
        Object::Function(fn_value) => {
            trace!("[apply_function] function is {:#?}", fn_value);

            let mut extend_env = extend_function_env(fn_value.clone(), args);
            trace!("[apply_function] extend_env is {:?}", extend_env);

            let evaluated = eval(fn_value.body().clone().into(), &mut extend_env)?;
            trace!("[apply_function] call function result is {}", evaluated);

            Ok(evaluated)
        }
        Object::Builtin(built_in) => (built_in.value())(args),
        _ => Err(Error::NoFunction(fn_obj.r#type().to_string()).into()),
    }
}

fn eval_hash_literal(node: HashLiteral, env: &mut Environment) -> anyhow::Result<Object> {
    let mut pairs = BTreeMap::<Object, Object>::new();

    for (key_node, value_node) in node.pair().iter() {
        let key = eval(Node::from(key_node.clone()), env)?;
        let value = eval(Node::from(value_node.clone()), env)?;
        pairs.insert(key, value);
    }

    Ok(Object::Hash(Hash::new(pairs)))
}

fn extend_function_env(fn_obj: Function, args: Vec<Object>) -> Environment {
    let mut env = Environment::new_enclosed_environment(fn_obj.env().clone());
    for (param_idx, param) in fn_obj.parameters().iter().enumerate() {
        env.store(param.value.clone(), args[param_idx].clone()); // TODO need imporve
    }
    env
}

fn eval_expressions(exps: Vec<Expression>, env: &mut Environment) -> anyhow::Result<Vec<Object>> {
    trace!("[eval_expressions] start");

    let mut result = vec![];

    for e in exps.into_iter() {
        let evaluated = eval(Node::from(e), env)?;
        trace!("[eval_expressions] evaluated is = {:?}", evaluated);
        result.push(evaluated);
    }

    trace!("[eval_expressions] end");

    Ok(result)
}
fn eval_program(program: &Program, env: &mut Environment) -> anyhow::Result<Object> {
    trace!("[eval_program]  program is ({})", program);
    let mut result: Object = NULL.into();

    for statement in program.statements.clone().into_iter() {
        result = eval(statement.into(), env)?;

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

fn eval_block_statement(block: &BlockStatement, env: &mut Environment) -> anyhow::Result<Object> {
    trace!("[eval_block_statement]  BlockStatement is ({})", block);
    let mut result: Object = NULL.into();

    for statement in block.statements.clone().into_iter() {
        trace!("[eval_block_statement] statement is ({:#?})", statement);
        result = eval(statement.into(), env)?;

        trace!("[eval_block_statement] result is ({:?})", result);
        match result.clone() {
            Object::ReturnValue(value) => {
                if value.r#type() == ObjectType::ReturnObj {
                    return Ok(value.into());
                }
            }
            _ => continue,
        }
    }

    Ok(result)
}

fn eval_prefix_expression(operator: &str, right: Object) -> anyhow::Result<Object> {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => Ok(Null.into()),
    }
}

fn eval_infix_expression(operator: &str, left: Object, right: Object) -> anyhow::Result<Object> {
    match (left, right) {
        (Object::Integer(left_value), Object::Integer(right_value)) => {
            eval_integer_infix_expression(operator, left_value, right_value)
        }
        (Object::Boolean(left_value), Object::Boolean(right_value)) if operator == "==" => Ok(
            native_bool_to_boolean_object(left_value.value() == right_value.value()),
        ),
        (Object::Boolean(left_value), Object::Boolean(right_value)) if operator == "!=" => Ok(
            native_bool_to_boolean_object(left_value.value() != right_value.value()),
        ),
        (Object::String(left), Object::String(right)) => {
            eval_string_infix_expression(operator, left, right)
        }
        (_, _) => Ok(Null.into()),
    }
}

// can add more operator for string
// 如果想支持字符串比较，那么可以在这里添加==和!=，但注意不能比较字符串指针
fn eval_string_infix_expression(
    operator: &str,
    left: StringObj,
    right: StringObj,
) -> anyhow::Result<Object> {
    match operator {
        "+" => {
            let left_val = left.value();
            let right_val = right.value();

            Ok(StringObj::new(format!("{left_val}{right_val}")).into())
        }
        "==" => {
            let left_val = left.value();
            let right_val = right.value();

            Ok(Boolean::new(left_val == right_val).into())
        }
        "!=" => {
            let left_val = left.value();
            let right_val = right.value();

            Ok(Boolean::new(left_val != right_val).into())
        }
        _ => Err(Error::UnknownOperator {
            left: left.r#type().to_string(),
            operator: operator.to_string(),
            right: right.r#type().to_string(),
        }
        .into()),
    }
}

// eval ! operator expression
fn eval_bang_operator_expression(right: Object) -> anyhow::Result<Object> {
    match right {
        Object::Boolean(value) => {
            if value.value() {
                Ok((*FALSE).into())
            } else {
                Ok((*TRUE).into())
            }
        }
        Object::Integer(value) => {
            if value.value() != 0 {
                Ok((*FALSE).into())
            } else {
                Ok((*TRUE).into())
            }
        }
        Object::Null(_) => Ok((*TRUE).into()),
        _ => Ok((*FALSE).into()),
    }
}

fn eval_minus_prefix_operator_expression(right: Object) -> anyhow::Result<Object> {
    match right {
        Object::Integer(value) => Ok(Integer::new(-value.value()).into()),
        value if value.r#type() != IntegerObj => Ok(Null.into()),
        _ => unimplemented!(),
    }
}

fn eval_integer_infix_expression(
    operator: &str,
    left: Integer,
    right: Integer,
) -> anyhow::Result<Object> {
    match operator {
        "+" => Ok(Integer::new(left.value() + right.value()).into()),
        "-" => Ok(Integer::new(left.value() - right.value()).into()),
        "*" => Ok(Integer::new(left.value() * right.value()).into()),
        "/" => Ok(Integer::new(left.value() / right.value()).into()),
        "<" => Ok(native_bool_to_boolean_object(left.value() < right.value())),
        ">" => Ok(native_bool_to_boolean_object(left.value() > right.value())),
        "==" => Ok(native_bool_to_boolean_object(left.value() == right.value())),
        "!=" => Ok(native_bool_to_boolean_object(left.value() != right.value())),
        _ => Ok(Null.into()),
    }
}

fn eval_index_expression(left: Object, index: Object) -> anyhow::Result<Object> {
    trace!(
        "[eval_index_expression]: left = {:?}, index = {:?}",
        left,
        index
    );
    if left.r#type() == ArrayObj && index.r#type() == IntegerObj {
        eval_array_index_expression(left, index)
    } else if left.r#type() == HashObj {
        eval_hash_index_expression(left, index)
    } else {
        Err(Error::IndexOperatorNotSupported(left.r#type().to_string()).into())
    }
}

fn eval_hash_index_expression(hash: Object, index: Object) -> anyhow::Result<Object> {
    let hash_object = Hash::try_from(hash)?;
    let pair = hash_object.pairs().get(&index);
    if pair.is_none() {
        return Ok(NULL.into());
    }

    Ok(pair.unwrap().clone())
}

fn eval_array_index_expression(left: Object, index: Object) -> anyhow::Result<Object> {
    let array_object = match left {
        Object::Array(array) => array,
        _ => return Err(Error::NotArrayType.into()),
    };

    let idx = match index {
        Object::Integer(integ) => integ.value(),
        _ => return Err(Error::NotIntegerType.into()),
    };

    let max = array_object.len() - 1;
    if idx < 0 || idx as usize > max {
        return Ok(Null.into());
    }

    Ok(array_object[idx as usize].clone())
}

fn native_bool_to_boolean_object(input: bool) -> Object {
    if input {
        (*TRUE).into()
    } else {
        (*FALSE).into()
    }
}

fn eval_if_expression(ie: IfExpression, env: &mut Environment) -> anyhow::Result<Object> {
    let condition = eval(Node::from(*ie.condition), env)?;

    if is_truthy(condition)? {
        eval(ie.consequence.unwrap().into(), env)
    } else if ie.alternative.is_some() {
        eval(ie.alternative.unwrap().into(), env)
    } else {
        Ok(Null.into())
    }
}

fn is_truthy(obj: Object) -> anyhow::Result<bool> {
    match obj {
        Object::Boolean(boolean) => {
            if boolean.value() {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        _ => Ok(false),
    }
}

fn eval_identifier(node: Identifier, env: &mut Environment) -> anyhow::Result<Object> {
    let val = env.get(node.value.clone());
    if let Some(val) = val {
        return Ok(val.clone());
    }

    if let Ok(builtin) = lookup_builtin(node.value.as_str()) {
        return Ok(builtin.into());
    }

    Err(Error::IdentifierNotFound(node.value).into())
}
