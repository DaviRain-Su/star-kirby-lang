use crate::ast::expression::array_literal::ArrayLiteral;
use crate::ast::expression::boolean::Boolean as AstBoolean;
use crate::ast::expression::call_expression::CallExpression;
use crate::ast::expression::function_literal::FunctionLiteral;
use crate::ast::expression::hash_literal::HashLiteral;
use crate::ast::expression::if_expression::IfExpression;
use crate::ast::expression::index_expression::IndexExpression;
use crate::ast::expression::infix_expression::InfixExpression;
use crate::ast::expression::integer_literal::IntegerLiteral as AstIntegerLiteral;
use crate::ast::expression::prefix_expression::PrefixExpression;
use crate::ast::expression::string_literal::StringLiteral;
use crate::ast::expression::Expression;
use crate::ast::statement::block_statement::BlockStatement;
use crate::ast::statement::expression_statement::ExpressionStatement;
use crate::ast::statement::let_statement::LetStatement;
use crate::ast::statement::return_statement::ReturnStatement;
use crate::ast::statement::Statement;
use crate::ast::{Identifier, NodeInterface, Program};
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

pub fn eval(node: &dyn NodeInterface, env: &mut Environment) -> anyhow::Result<Object> {
    let type_id = node.as_any().type_id();
    trace!("[eval] TypeID  is ({:?})", type_id);
    if TypeId::of::<Program>() == type_id {
        // Parser Program
        trace!("[eval] Type Program ID is ({:?})", TypeId::of::<Program>());
        let value = node
            .as_any()
            .downcast_ref::<Program>()
            .ok_or::<Error>(Error::DownCastRefProgramError.into())?;

        return eval_program(value, env);
    } else if TypeId::of::<Statement>() == type_id {
        // Parser Statement
        trace!(
            "[eval] Type Statement ID is ({:?})",
            TypeId::of::<Statement>()
        );
        let value = node
            .as_any()
            .downcast_ref::<Statement>()
            .ok_or::<Error>(Error::DownCastRefStatementError.into())?;

        let result = match value {
            Statement::Expression(exp) => eval(exp, env)?,
            Statement::Let(let_exp) => eval(let_exp, env)?,
            Statement::Return(ret_exp) => eval(ret_exp, env)?,
        };
        return Ok(result);
    } else if TypeId::of::<ExpressionStatement>() == type_id {
        // Parser ExpressionStatement
        trace!(
            "[eval] type ExpressionStatement ID is  ({:?})",
            TypeId::of::<ExpressionStatement>()
        );
        let value = node
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .ok_or::<Error>(Error::DownCastRefExpressionStatementError.into())?;

        return eval(&value.expression, env);
    } else if TypeId::of::<ReturnStatement>() == type_id {
        trace!(
            "[eval] Type ReturnStatement ID is ({:?})",
            TypeId::of::<ReturnStatement>()
        );
        let value = node
            .as_any()
            .downcast_ref::<ReturnStatement>()
            .ok_or::<Error>(Error::DownCastRefReturnStatementError.into())?;

        trace!("[eval] return_statement is ({})", value);

        let val = eval(&*value.return_value, env)?;
        trace!("[eval] return_statement eval value is  ({:?})", val);
        return Ok(ReturnValue {
            value: Box::new(val),
        }
        .into());
    } else if TypeId::of::<LetStatement>() == type_id {
        trace!(
            "[eval] Type LetStatement ID is ({:?})",
            TypeId::of::<LetStatement>()
        );
        let value = node
            .as_any()
            .downcast_ref::<LetStatement>()
            .ok_or::<Error>(Error::DownCastRefLetStatementError.into())?;

        trace!("[eval] LetStatement is ({})", value);

        let val = eval(&*value.value, env)?;

        trace!("[eval] LetStatement eval after = {:?}", val);

        env.store(value.name.value.clone(), val);

        Ok(NULL.into())
    } else if TypeId::of::<Expression>() == type_id {
        // parser Expression
        trace!(
            "[eval] Type Expression ID is ({:?})",
            TypeId::of::<Expression>()
        );
        let value = node
            .as_any()
            .downcast_ref::<Expression>()
            .ok_or::<Error>(Error::DownCastRefExpressionError.into())?;

        return match value {
            Expression::PrefixExpression(pre_exp) => eval(pre_exp, env),
            Expression::InfixExpression(infix_exp) => eval(infix_exp, env),
            Expression::IntegerLiteralExpression(integer) => eval(integer, env),
            Expression::IdentifierExpression(identifier) => eval(identifier, env),
            Expression::BooleanExpression(boolean) => eval(boolean, env),
            Expression::IfExpression(if_exp) => eval(if_exp, env),
            Expression::FunctionLiteral(function) => eval(function, env),
            Expression::CallExpression(call_exp) => eval(call_exp, env),
            Expression::StringLiteral(string_lit) => eval(string_lit, env),
            Expression::ArrayLiteral(array_lit) => eval(array_lit, env),
            Expression::IndexExpression(index_exp) => eval(index_exp, env),
            Expression::HashLiteral(hash_literal) => eval(hash_literal, env),
        };
    } else if TypeId::of::<PrefixExpression>() == type_id {
        // parser PrefixExpression
        trace!(
            "[eval] Type PrefixExpression ID is ({:?})",
            TypeId::of::<PrefixExpression>()
        );
        let value = node
            .as_any()
            .downcast_ref::<PrefixExpression>()
            .ok_or::<Error>(Error::DownCastRefPrefixExpressionError.into())?;

        trace!("[eval] PrefixExpression is ({})", value);

        let right = eval(&*value.right, env)?;
        return eval_prefix_expression(value.operator.clone(), right);
    } else if TypeId::of::<InfixExpression>() == type_id {
        // parser InfixExpression
        trace!(
            "[eval] Type InfixExpression ID is ({:?})",
            TypeId::of::<InfixExpression>()
        );
        let value = node
            .as_any()
            .downcast_ref::<InfixExpression>()
            .ok_or::<Error>(Error::DownCastRefInfixExpressionError.into())?;

        trace!("[eval] InfixExpression is ({})", value);

        let left = eval(&*value.left, env)?;
        let right = eval(&*value.right, env)?;

        return eval_infix_expression(value.operator.clone(), left, right);
    } else if TypeId::of::<AstIntegerLiteral>() == type_id {
        // parser AstIntegerLiteral
        trace!(
            "[eval] Type AstIntegerLiteral ID is ({:?})",
            TypeId::of::<AstIntegerLiteral>()
        );
        let value = node
            .as_any()
            .downcast_ref::<AstIntegerLiteral>()
            .ok_or::<Error>(Error::DownCastRefAstIntegerLiteralError.into())?;

        trace!("[eval] integer literal is ({:?})", value);

        return Ok(Integer { value: value.value }.into());
    } else if TypeId::of::<FunctionLiteral>() == type_id {
        // parser AstIntegerLiteral
        trace!(
            "[eval] Type FunctionLiteral ID is ({:?})",
            TypeId::of::<FunctionLiteral>()
        );
        let value = node
            .as_any()
            .downcast_ref::<FunctionLiteral>()
            .ok_or::<Error>(Error::DownCastRefFunctionLiteralError.into())?;

        trace!("[eval] FunctionLiteral is ({})", value);
        let params = value.parameters.clone();
        let body = value.body.clone();

        return Ok(Function {
            parameters: params,
            env: env.clone(),
            body: body.clone(),
        }
        .into());
    } else if TypeId::of::<AstBoolean>() == type_id {
        // parser AstBoolean
        trace!(
            "[eval] Type AstBoolean ID is ({:?})",
            TypeId::of::<AstBoolean>()
        );
        let value = node
            .as_any()
            .downcast_ref::<AstBoolean>()
            .ok_or::<Error>(Error::DownCastRefAstBooleanError.into())?;

        trace!("[eval]AstBoolean literal is ({})", value);

        return Ok(Boolean { value: value.value }.into());
    } else if TypeId::of::<BlockStatement>() == type_id {
        trace!(
            "[eval] Type AstBoolean ID is ({:?})",
            TypeId::of::<BlockStatement>()
        );
        let value = node
            .as_any()
            .downcast_ref::<BlockStatement>()
            .ok_or::<Error>(Error::DownCastRefBlockStatementError.into())?;

        trace!("[eval] BlockStatement literal is  ({})", value);

        return eval_block_statement(value, env);
    } else if TypeId::of::<IfExpression>() == type_id {
        trace!(
            "[eval] Type IfExpression ID is ({:?})",
            TypeId::of::<IfExpression>()
        );
        let value = node
            .as_any()
            .downcast_ref::<IfExpression>()
            .ok_or::<Error>(Error::DownCastRefIfExpressionError.into())?;

        trace!("[eval]IfExpression literal is ({})", value);

        return eval_if_expression(value.clone(), env);
    } else if TypeId::of::<Identifier>() == type_id {
        trace!(
            "[eval] Type Identifier ID is ({:?})",
            TypeId::of::<Identifier>()
        );
        let value = node
            .as_any()
            .downcast_ref::<Identifier>()
            .ok_or::<Error>(Error::DownCastRefIdentifierError.into())?;

        trace!("[eval]Identifier literal is  ({})", value);

        return eval_identifier(value.clone(), env);
    } else if TypeId::of::<CallExpression>() == type_id {
        trace!(
            "[eval] Type CallExpression ID is ({:?})",
            TypeId::of::<CallExpression>()
        );
        let value = node
            .as_any()
            .downcast_ref::<CallExpression>()
            .ok_or::<Error>(Error::DownCastRefCallExpressionError.into())?;

        trace!("[eval]CallExpression  is  ({})", value);
        if value.function.token_literal() == "quote".to_string() {
            return quote(value.arguments[0].clone());
        }
        let function = eval(&*value.function, env)?;
        trace!("[eval]CallExpression : function is ({})", function);

        let args = eval_expressions(value.arguments.clone(), env)?;
        trace!("[eval]CallExpression: args is  ({:?})", args);

        return apply_function(function, args);
    } else if TypeId::of::<StringLiteral>() == type_id {
        trace!(
            "[eval] Type StringLiteral ID is ({:?})",
            TypeId::of::<StringLiteral>()
        );
        let value = node
            .as_any()
            .downcast_ref::<StringLiteral>()
            .ok_or::<Error>(Error::DownCastRefStringLiteralError.into())?;
        trace!("[eval]StringLiteral  is  ({})", value);

        return Ok(StringObj {
            value: value.value.clone(),
        }
        .into());
    } else if TypeId::of::<ArrayLiteral>() == type_id {
        trace!(
            "[eval] Type ArrayLiteral ID is ({:?})",
            TypeId::of::<ArrayLiteral>()
        );
        let value = node
            .as_any()
            .downcast_ref::<ArrayLiteral>()
            .ok_or::<Error>(Error::DownCastRefArrayLiteralError.into())?;

        trace!("[eval]ArrayLiteral  is  ({})", value);

        let elements = eval_expressions(value.elements.clone(), env)?;

        return Ok(Array {
            elements: elements.into_iter().map(|value| Box::new(value)).collect(),
        }
        .into());
    } else if TypeId::of::<IndexExpression>() == type_id {
        trace!(
            "[eval] Type IndexExpression ID is ({:?})",
            TypeId::of::<IndexExpression>()
        );
        let value = node
            .as_any()
            .downcast_ref::<IndexExpression>()
            .ok_or::<Error>(Error::DownCastRefIndexExpressionError.into())?;
        trace!("[eval]IndexExpression  is  ({})", value);

        let left = eval(&*value.left, env)?;
        let index = eval(&*value.index, env)?;
        trace!("[eval]IndexExpression : left = ({})", left);
        trace!("[eval]IndexExpression : Index = ({})", index);

        return eval_index_expression(left, index);
    } else if TypeId::of::<HashLiteral>() == type_id {
        trace!(
            "[eval] Type HashLiteral ID is ({:?})",
            TypeId::of::<HashLiteral>()
        );
        let value = node
            .as_any()
            .downcast_ref::<HashLiteral>()
            .ok_or::<Error>(Error::DownCastRefHashLiteralError.into())?;

        trace!("[eval]HashLiteral  is  ({})", value);

        return eval_hash_literal(value.clone(), env);
    } else {
        // Parser Unknown type
        trace!("[eval] type Unknown Type!");
        trace!("[eval] Unknown Node is {:#?}", node);
        trace!(
            "[eval] Type FunctionLiteral ID is ({:?})",
            TypeId::of::<FunctionLiteral>()
        );
        Err(Error::UnknownTypeError(format!("{:?}", type_id)).into())
    }
}

fn quote(node: Box<dyn NodeInterface>) -> anyhow::Result<Object> {
    let type_id = node.as_any().type_id();
    if TypeId::of::<Expression>() == type_id {
        trace!(
            "[eval] Type Expression ID is ({:?})",
            TypeId::of::<Expression>()
        );
        let value = node
            .as_any()
            .downcast_ref::<Expression>()
            .ok_or::<Error>(Error::DownCastRefExpressionError.into())?;

        trace!("[eval]Expression  is  ({})", value);

        return Ok(Quote {
            node: Box::new(value.clone().into()),
        }
        .into());
    } else if TypeId::of::<Statement>() == type_id {
        trace!(
            "[eval] Type HashLiteral ID is ({:?})",
            TypeId::of::<Statement>()
        );
        let value = node
            .as_any()
            .downcast_ref::<Statement>()
            .ok_or::<Error>(Error::DownCastRefStatementError.into())?;
        trace!("[eval]Statement  is  ({})", value);
        return Ok(Quote {
            node: Box::new(value.clone().into()),
        }
        .into());
    } else if TypeId::of::<Object>() == type_id {
        trace!("[eval] Type Object ID is ({:?})", TypeId::of::<Object>());
        let value = node
            .as_any()
            .downcast_ref::<Object>()
            .ok_or::<Error>(Error::DownCastRefObjectError)?;
        trace!("[eval]Object  is  ({})", value);
        return Ok(Quote {
            node: Box::new(value.clone().into()),
        }
        .into());
    } else {
        // Parser Unknown type
        trace!("[eval] type Unknown Type!");
        trace!("[eval] Unknown Node is {:#?}", node);
        Err(Error::UnknownTypeError(format!("{:?}", type_id)).into())
    }
}

fn apply_function(fn_obj: Object, args: Vec<Object>) -> anyhow::Result<Object> {
    match fn_obj {
        Object::Function(fn_value) => {
            trace!("[apply_function] function is {:#?}", fn_value);

            let mut extend_env = extend_function_env(fn_value.clone(), args);
            trace!("[apply_function] extend_env is {:?}", extend_env);

            let evaluated = eval(&fn_value.body, &mut extend_env)?;
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
        let key = eval(key_node, env)?;
        let value = eval(value_node, env)?;
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
        let evaluated = eval(&*e, env)?;
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
        result = eval(&statement, env)?;

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
        result = eval(&statement, env)?;

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
    let condition = eval(&*ie.condition, env)?;

    return if is_truthy(condition)? {
        eval(&ie.consequence.unwrap(), env)
    } else if ie.alternative.is_some() {
        eval(&ie.alternative.unwrap(), env)
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
