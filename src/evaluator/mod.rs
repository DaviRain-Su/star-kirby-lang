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
use std::any::TypeId;
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
                return Ok(ReturnValue {
                    value: Box::new(val),
                }
                .into());
            }
            Statement::BlockStatement(block_statement) => {
                return eval_block_statement(block_statement, env);
            }
        },
        Node::Expression(ref expression) => match expression {
            Expression::PrefixExpression(prefix) => {
                let right = eval(Node::from(*prefix.right.clone()), env)?;
                return eval_prefix_expression(prefix.operator.clone(), right);
            }
            Expression::InfixExpression(infix) => {
                let left = eval(Node::from(*infix.left.clone()), env)?;
                let right = eval(Node::from(*infix.right.clone()), env)?;

                return eval_infix_expression(infix.operator.clone(), left, right);
            }
            Expression::IntegerLiteralExpression(integer) => {
                return Ok(Integer {
                    value: integer.value,
                }
                .into());
            }
            Expression::IdentifierExpression(identifier) => {
                return eval_identifier(identifier.clone(), env);
            }
            Expression::BooleanExpression(boolean) => {
                return Ok(Boolean {
                    value: boolean.value,
                }
                .into());
            }
            Expression::IfExpression(if_exp) => {
                return eval_if_expression(if_exp.clone(), env);
            }
            Expression::FunctionLiteral(function) => {
                let params = function.parameters.clone();
                let body = function.body.clone();

                return Ok(Function {
                    parameters: params,
                    env: env.clone(),
                    body: body.clone(),
                }
                .into());
            }
            Expression::CallExpression(call_exp) => {
                if call_exp.function.token_literal() == "quote".to_string() {
                    return quote(Node::from(*call_exp.arguments[0].clone()));
                }
                let function = eval(Node::from(*call_exp.function.clone()), env)?;

                let args = eval_expressions(call_exp.arguments.clone(), env)?;

                return apply_function(function, args);
            }
            Expression::StringLiteral(string_literal) => {
                return Ok(StringObj {
                    value: string_literal.value.clone(),
                }
                .into());
            }
            Expression::ArrayLiteral(array) => {
                let elements = eval_expressions(array.elements.clone(), env)?;

                return Ok(Array {
                    elements: elements.into_iter().map(|value| Box::new(value)).collect(),
                }
                .into());
            }
            Expression::IndexExpression(indx_exp) => {
                let left = eval(Node::from(*indx_exp.left.clone()), env)?;
                let index = eval(Node::from(*indx_exp.index.clone()), env)?;

                return eval_index_expression(left, index);
            }
            Expression::HashLiteral(hash_literal) => {
                return eval_hash_literal(hash_literal.clone(), env);
            }
        },
        Node::Object(object) => {
            Err(Error::UnknownTypeError(format!("object: {:?}", object)).into())
        }
    }
}

fn quote(node: Node) -> anyhow::Result<Object> {
    match node {
        Node::Program(program) => Err(Error::UnknownTypeError(format!("{:?}", program)).into()),
        Node::Expression(expression) => {
            return Ok(Quote {
                node: Box::new(expression.clone().into()),
            }
            .into());
        }
        Node::Statement(statement) => {
            return Ok(Quote {
                node: Box::new(statement.clone().into()),
            }
            .into());
        }
        Node::Object(object) => {
            return Ok(Quote {
                node: Box::new(object.clone().into()),
            }
            .into());
        }
    }
}

fn apply_function(fn_obj: Object, args: Vec<Object>) -> anyhow::Result<Object> {
    match fn_obj {
        Object::Function(fn_value) => {
            trace!("[apply_function] function is {:#?}", fn_value);

            let mut extend_env = extend_function_env(fn_value.clone(), args);
            trace!("[apply_function] extend_env is {:?}", extend_env);

            let evaluated = eval(fn_value.body.into(), &mut extend_env)?;
            trace!("[apply_function] call function result is {}", evaluated);

            Ok(evaluated)
        }
        Object::Builtin(built_in) => {
            return (built_in.built_in_function)(args);
        }
        _ => return Err(Error::NoFunction(fn_obj.r#type().to_string()).into()),
    }
}

fn eval_hash_literal(node: HashLiteral, env: &mut Environment) -> anyhow::Result<Object> {
    let mut pairs = BTreeMap::<Object, Object>::new();

    for (key_node, value_node) in node.pair.iter() {
        let key = eval(Node::from(key_node.clone()), env)?;
        let value = eval(Node::from(value_node.clone()), env)?;
        pairs.insert(key, value);
    }

    Ok(Object::Hash(Hash { pairs }))
}

fn extend_function_env(fn_obj: Function, args: Vec<Object>) -> Environment {
    let mut env = Environment::new_enclosed_environment(fn_obj.env);
    for (param_idx, param) in fn_obj.parameters.iter().enumerate() {
        env.store(param.value.clone(), args[param_idx].clone()); // TODO need imporve
    }
    env
}

fn eval_expressions(
    exps: Vec<Box<Expression>>,
    env: &mut Environment,
) -> anyhow::Result<Vec<Object>> {
    trace!("[eval_expressions] start");

    let mut result = vec![];

    for e in exps.into_iter() {
        let evaluated = eval(Node::from(*e), env)?;
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
                return Ok(*value.value.clone());
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
                    return Ok(value.clone().into());
                }
            }
            _ => continue,
        }
    }

    return Ok(result);
}

fn eval_prefix_expression(operator: String, right: Object) -> anyhow::Result<Object> {
    match operator.as_str() {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => Ok(Null.into()),
    }
}

fn eval_infix_expression(operator: String, left: Object, right: Object) -> anyhow::Result<Object> {
    match (left, right) {
        (Object::Integer(left_value), Object::Integer(right_value)) => {
            eval_integer_infix_expression(operator, left_value.clone(), right_value.clone())
        }
        (Object::Boolean(left_value), Object::Boolean(right_value)) if operator == "==" => Ok(
            native_bool_to_boolean_object(left_value.value == right_value.value),
        ),
        (Object::Boolean(left_value), Object::Boolean(right_value)) if operator == "!=" => Ok(
            native_bool_to_boolean_object(left_value.value != right_value.value),
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
    operator: String,
    left: StringObj,
    right: StringObj,
) -> anyhow::Result<Object> {
    match operator.as_str() {
        "+" => {
            let left_val = left.value.clone();
            let right_val = right.value.clone();

            Ok(StringObj {
                value: format!("{}{}", left_val, right_val),
            }
            .into())
        }
        "==" => {
            let left_val = left.value.clone();
            let right_val = right.value.clone();

            Ok(Boolean {
                value: left_val == right_val,
            }
            .into())
        }
        "!=" => {
            let left_val = left.value.clone();
            let right_val = right.value.clone();

            Ok(Boolean {
                value: left_val != right_val,
            }
            .into())
        }
        _ => Err(Error::UnknownOperator {
            left: left.r#type().to_string(),
            operator: operator.clone(),
            right: right.r#type().to_string(),
        }
        .into()),
    }
}

// eval ! operator expression
fn eval_bang_operator_expression(right: Object) -> anyhow::Result<Object> {
    match right {
        Object::Boolean(value) => {
            if value.value {
                Ok(FALSE.into())
            } else {
                Ok(TRUE.into())
            }
        }
        Object::Integer(value) => {
            if value.value != 0 {
                Ok(FALSE.into())
            } else {
                Ok(TRUE.into())
            }
        }
        Object::Null(_) => Ok(TRUE.into()),
        _ => Ok(FALSE.into()),
    }
}

fn eval_minus_prefix_operator_expression(right: Object) -> anyhow::Result<Object> {
    match right.clone() {
        Object::Integer(value) => Ok(Integer {
            value: -value.value,
        }
        .into()),
        value if value.r#type() != IntegerObj => {
            return Ok(Null.into());
        }
        _ => unimplemented!(),
    }
}

fn eval_integer_infix_expression(
    operator: String,
    left: Integer,
    right: Integer,
) -> anyhow::Result<Object> {
    match operator.as_str() {
        "+" => Ok(Integer {
            value: left.value + right.value,
        }
        .into()),
        "-" => Ok(Integer {
            value: left.value - right.value,
        }
        .into()),
        "*" => Ok(Integer {
            value: left.value * right.value,
        }
        .into()),
        "/" => Ok(Integer {
            value: left.value / right.value,
        }
        .into()),
        "<" => Ok(native_bool_to_boolean_object(left.value < right.value)),
        ">" => Ok(native_bool_to_boolean_object(left.value > right.value)),
        "==" => Ok(native_bool_to_boolean_object(left.value == right.value)),
        "!=" => Ok(native_bool_to_boolean_object(left.value != right.value)),
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
    let pair = hash_object.pairs.get(&index);
    if pair.is_none() {
        return Ok(NULL.into());
    }

    return Ok(pair.unwrap().clone());
}

fn eval_array_index_expression(left: Object, index: Object) -> anyhow::Result<Object> {
    let array_object = match left {
        Object::Array(array) => array,
        _ => return Err(Error::NotArrayType.into()),
    };

    let idx = match index {
        Object::Integer(integ) => integ.value,
        _ => return Err(Error::NotIntegerType.into()),
    };

    let max = array_object.elements.len() - 1;
    if idx < 0 || idx as usize > max {
        return Ok(Null.into());
    }

    Ok(*array_object.elements[idx as usize].clone())
}

fn native_bool_to_boolean_object(input: bool) -> Object {
    if input {
        TRUE.into()
    } else {
        FALSE.into()
    }
}

fn eval_if_expression(ie: IfExpression, env: &mut Environment) -> anyhow::Result<Object> {
    let condition = eval(Node::from(*ie.condition), env)?;

    return if is_truthy(condition)? {
        eval(ie.consequence.unwrap().into(), env)
    } else if ie.alternative.is_some() {
        eval(ie.alternative.unwrap().into(), env)
    } else {
        Ok(Null.into())
    };
}

fn is_truthy(obj: Object) -> anyhow::Result<bool> {
    let type_id = ObjectInterface::as_any(&obj).type_id();
    if TypeId::of::<()>() == type_id {
        Ok(false)
    } else if TypeId::of::<Boolean>() == type_id {
        let value = ObjectInterface::as_any(&obj)
            .downcast_ref::<Boolean>()
            .ok_or::<Error>(Error::DownCastRefBooleanError.into())?;

        if value.value {
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        Ok(true)
    }
}

fn eval_identifier(node: Identifier, env: &mut Environment) -> anyhow::Result<Object> {
    let val = env.get(node.value.clone());
    if val.is_some() {
        return Ok(val.unwrap().clone());
    }

    if let Ok(builtin) = lookup_builtin(node.value.as_str()) {
        return Ok(builtin.into());
    }

    Err(Error::IdentifierNotFound(node.value.to_string()).into())
}
