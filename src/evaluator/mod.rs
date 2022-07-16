use crate::ast::expression::boolean::Boolean as AstBoolean;
use crate::ast::expression::infix_expression::InfixExpression;
use crate::ast::expression::integer_literal::IntegerLiteral as AstIntegerLiteral;
use crate::ast::expression::prefix_expression::PrefixExpression;
use crate::ast::expression::Expression;
use crate::ast::statement::expression_statement::ExpressionStatement;
use crate::ast::statement::Statement;
use crate::ast::{Node, Program};
use crate::object::boolean::Boolean;
use crate::object::integer::Integer;
use crate::object::{Object, ObjectType};
use log::trace;
use std::any::TypeId;
use crate::ast::expression::if_expression::IfExpression;
use crate::ast::statement::block_statement::BlockStatement;

#[cfg(test)]
pub mod tests;

pub fn eval(node: Box<dyn Node>) -> anyhow::Result<Object> {
    let type_id = node.as_any().type_id();
    trace!("[eval] type_id = {:?}", type_id);
    if TypeId::of::<Program>() == type_id { // Parser Program
        trace!("[eval] type program id = {:?}", TypeId::of::<Program>());
        let value = node
            .as_any()
            .downcast_ref::<Program>()
            .ok_or(anyhow::anyhow!("downcast_ref program error"))?;

        return Ok(eval_statements(value.statements.clone())?);

    } else if TypeId::of::<Statement>() == type_id { // Parser Statement
        trace!("[eval] type Statement id = {:?}", TypeId::of::<Statement>());
        let value = node
            .as_any()
            .downcast_ref::<Statement>()
            .ok_or(anyhow::anyhow!("downcast_ref statement error"))?;

        let result = match value {
            Statement::ExpressionStatement(exp) => eval(Box::new(exp.clone()))?,
            Statement::LetStatement(let_exp) => eval(Box::new(let_exp.clone()))?,
            Statement::ReturnStatement(ret_exp) => eval(Box::new(ret_exp.clone()))?,
        };
        return Ok(result);
    } else if TypeId::of::<ExpressionStatement>() == type_id { // Parser ExpressionStatement
        trace!(
            "[eval] type ExpressionStatement id = {:?}",
            TypeId::of::<ExpressionStatement>()
        );
        let value = node
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .ok_or(anyhow::anyhow!("downcast_ref expression statement error"))?;

        return Ok(eval(Box::new(value.expression.clone()))?);
    } else if TypeId::of::<Expression>() == type_id {  // parser Expression
        trace!("type Expression id = {:?}", TypeId::of::<Expression>());
        let value = node
            .as_any()
            .downcast_ref::<Expression>()
            .ok_or(anyhow::anyhow!("downcast_ref expression error"))?;

        return match value {
            Expression::PrefixExpression(pre_exp) => Ok(eval(Box::new(pre_exp.clone()))?),
            Expression::InfixExpression(infix_exp) => {
                Ok(eval(Box::new(infix_exp.clone()))?)
            }
            Expression::IntegerLiteralExpression(integer) => {
                Ok(eval(Box::new(integer.clone()))?)
            }

            Expression::IdentifierExpression(identifier) => {
                Ok(eval(Box::new(identifier.clone()))?)
            }
            Expression::BooleanExpression(boolean) => Ok(eval(Box::new(boolean.clone()))?),
            Expression::IfExpression(if_exp) => Ok(eval(Box::new(if_exp.clone()))?),
            Expression::FunctionLiteral(function) => Ok(eval(Box::new(function.clone()))?),
            Expression::CallExpression(call_exp) => Ok(eval(Box::new(call_exp.clone()))?),
        }
    } else if TypeId::of::<PrefixExpression>() == type_id { // parser PrefixExpression
        trace!(
            "type PrefixExpression id = {:?}",
            TypeId::of::<PrefixExpression>()
        );
        let value = node
            .as_any()
            .downcast_ref::<PrefixExpression>()
            .ok_or(anyhow::anyhow!("downcast_ref PrefixExpression error"))?;
        trace!("[eval] PrefixExpression = {:#?}", value);

        let right = eval(value.right.clone())?;
        return Ok(eval_prefix_expression(value.operator.clone(), right)?);
    } else if TypeId::of::<InfixExpression>() == type_id { // parser InfixExpression
        trace!(
            "type InfixExpression id = {:?}",
            TypeId::of::<InfixExpression>()
        );
        let value = node
            .as_any()
            .downcast_ref::<InfixExpression>()
            .ok_or(anyhow::anyhow!("downcast_ref InfixExpression error"))?;
        trace!("[eval] InfixExpression = {:#?}", value);

        let left = eval(value.left.clone())?;
        let right = eval(value.right.clone())?;

        return Ok(eval_infix_expression(value.operator.clone(), left, right)?);
    } else if TypeId::of::<AstIntegerLiteral>() == type_id { // parser AstIntegerLiteral
        trace!(
            "type AstIntegerLiteral id = {:?}",
            TypeId::of::<AstIntegerLiteral>()
        );
        let value = node
            .as_any()
            .downcast_ref::<AstIntegerLiteral>()
            .ok_or(anyhow::anyhow!("downcast_ref integer_literal error"))?;
        trace!("[eval] integer literal = {:#?}", value);

        return Ok(Object::Integer(Integer { value: value.value }));
    } else if TypeId::of::<AstBoolean>() == type_id {  // parser AstBoolean
        trace!("type AstBoolean id = {:?}", TypeId::of::<AstBoolean>());
        let value = node
            .as_any()
            .downcast_ref::<AstBoolean>()
            .ok_or(anyhow::anyhow!("downcast_ref AstBoolean error"))?;
        trace!("[eval]AstBoolean literal = {:#?}", value);

        return Ok(Object::Boolean(Boolean { value: value.value }));
    } else if TypeId::of::<BlockStatement>() == type_id {
        trace!("type AstBoolean id = {:?}", TypeId::of::<BlockStatement>());
        let value = node
            .as_any()
            .downcast_ref::<BlockStatement>()
            .ok_or(anyhow::anyhow!("downcast_ref BlockStatement error"))?;
        trace!("[eval]BlockStatement literal = {:#?}", value);

        return  Ok(eval_statements(value.statements.clone())?)
    } else if TypeId::of::<IfExpression>() == type_id {
        trace!("type IfExpression id = {:?}", TypeId::of::<IfExpression>());
        let value = node
            .as_any()
            .downcast_ref::<IfExpression>()
            .ok_or(anyhow::anyhow!("downcast_ref BlockStatement error"))?;
        trace!("[eval]IfExpression literal = {:#?}", value);

        return  Ok(eval_if_expression(value.clone())?)
    } else { // Parser Unknown type
        trace!("type Unknown Type!");
        Err(anyhow::anyhow!(format!("eval error: type_id = {:?}", type_id)))
    }
}

fn eval_statements(stmts: Vec<Statement>) -> anyhow::Result<Object> {
    trace!("eval_statements stmt = {:#?}", stmts);
    let mut result: Object = Object::Unit(());

    for statement in stmts {
        result = eval(Box::new(statement))?;
    }

    Ok(result)
}

fn eval_prefix_expression(
    operator: String,
    right: Object,
) -> anyhow::Result<Object> {
    match operator.as_str() {
        "!" => {
            return Ok(eval_bang_operator_expression(right)?);
        }
        "-" => {
            return Ok(eval_minus_prefix_operator_expression(right)?);
        }
        _ => Err(anyhow::anyhow!("unimplemented!")),
    }
}

fn eval_infix_expression(
    operator: String,
    left: Object,
    right: Object,
) -> anyhow::Result<Object> {
    match (left, right ) {
        (Object::Integer(left_value), Object::Integer(right_value)) => {
            return Ok(eval_integer_infix_expression(
                operator,
                left_value.clone(),
                right_value.clone(),
            ));
        },
        (Object::Boolean(left_value), Object::Boolean(right_value)) if operator == "=="  => {
            return Ok(native_bool_to_boolean_object(left_value.value == right_value.value));
        },
        (Object::Boolean(left_value), Object::Boolean(right_value)) if operator == "!="  => {
            return Ok(native_bool_to_boolean_object(left_value.value != right_value.value));
        },
        (_, _) => unimplemented!()
    }
}

// eval ! operator expression
fn eval_bang_operator_expression(right: Object) -> anyhow::Result<Object> {
    match right {
        Object::Boolean(value ) => {
            if value.value {
                Ok(Object::Boolean(Boolean { value: false }))
            } else {
                Ok(Object::Boolean(Boolean { value: true }))
            }
        }
        Object::Integer(value) => {
            if value.value != 0 {
                Ok(Object::Boolean(Boolean { value: false }))
            } else {
                Ok(Object::Boolean(Boolean { value: true }))
            }
        }
        Object::Unit(_) => Err(anyhow::anyhow!("eval bang operator expression error"))
    }
}

fn eval_minus_prefix_operator_expression(
    right: Object,
) -> anyhow::Result<Object> {
    match right {
        Object::Integer(value) => {
            return Ok(Object::Integer(Integer {
                value: -value.value,
            }));
        }
        _ => Err(anyhow::anyhow!(
        "eval_minus_prefix_operator_expression error "
        ))
    }
}

fn eval_integer_infix_expression(
    operator: String,
    left: Integer,
    right: Integer,
) -> Object {
    match operator.as_str() {
        "+" => Object::Integer(Integer {
            value: left.value + right.value,
        }),
        "-" =>Object::Integer(Integer {
            value: left.value - right.value,
        }),
        "*" => Object::Integer(Integer {
            value: left.value * right.value,
        }),
        "/" => Object::Integer(Integer {
            value: left.value / right.value,
        }),
        "<" => native_bool_to_boolean_object(left.value < right.value),
        ">" => native_bool_to_boolean_object(left.value > right.value),
        "==" => native_bool_to_boolean_object(left.value == right.value),
        "!=" => native_bool_to_boolean_object(left.value != right.value),
        _ => unimplemented!(),
    }
}

fn native_bool_to_boolean_object(input: bool) -> Object {
    if input {
        Object::Boolean(Boolean { value: true })
    } else {
        Object::Boolean(Boolean { value: false })
    }
}

fn eval_if_expression(ie: IfExpression) -> anyhow::Result<Object> {
    let condition = eval(ie.condition)?;

    return if is_truthy(condition)? {
        eval(Box::new(ie.consequence.unwrap()))
    } else if ie.alternative.is_some() {
        eval(Box::new(ie.alternative.unwrap()))
    } else {
        Ok(Object::Unit(()))
    }
}

fn is_truthy(obj: Object) -> anyhow::Result<bool> {
    let type_id = obj.as_any().type_id();
    if TypeId::of::<()>() == type_id  {
        Ok(false)
    } else if TypeId::of::<Boolean>() == type_id {
        let value = obj
            .as_any()
            .downcast_ref::<Boolean>()
            .ok_or(anyhow::anyhow!("downcast_ref boolean error"))?;

        if value.value {
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        Ok(true)
    }
}