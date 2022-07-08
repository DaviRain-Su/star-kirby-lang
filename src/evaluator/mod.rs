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

#[cfg(test)]
pub mod tests;

pub fn eval(node: Box<dyn Node>) -> anyhow::Result<Box<dyn Object>> {
    let type_id = node.as_any().type_id();
    trace!("[eval] type_id = {:?}", type_id);
    if TypeId::of::<Program>() == type_id {
        // Parser Program
        trace!("[eval] type program id = {:?}", TypeId::of::<Program>());
        let value = node
            .as_any()
            .downcast_ref::<Program>()
            .ok_or(anyhow::anyhow!("downcast_ref program error"))?;

        return Ok(eval_statements(value.statements.clone())?);
    } else if TypeId::of::<Statement>() == type_id {
        // Parser Statement
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
    } else if TypeId::of::<ExpressionStatement>() == type_id {
        // Parser ExpressionStatement
        trace!(
            "[eval] type ExpressionStatement id = {:?}",
            TypeId::of::<ExpressionStatement>()
        );
        let value = node
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .ok_or(anyhow::anyhow!("downcast_ref expression statement error"))?;

        return Ok(eval(Box::new(value.expression.clone()))?);
    } else if TypeId::of::<Expression>() == type_id {
        // parser Expression
        trace!("type Expression id = {:?}", TypeId::of::<Expression>());
        let value = node
            .as_any()
            .downcast_ref::<Expression>()
            .ok_or(anyhow::anyhow!("downcast_ref expression error"))?;

        match value {
            Expression::PrefixExpression(pre_exp) => return Ok(eval(Box::new(pre_exp.clone()))?),
            Expression::InfixExpression(infix_exp) => {
                return Ok(eval(Box::new(infix_exp.clone()))?)
            }
            Expression::IntegerLiteralExpression(integer) => {
                return Ok(eval(Box::new(integer.clone()))?)
            }

            Expression::IdentifierExpression(identifier) => {
                return Ok(eval(Box::new(identifier.clone()))?)
            }
            Expression::BooleanExpression(boolean) => return Ok(eval(Box::new(boolean.clone()))?),
            Expression::IfExpression(if_exp) => return Ok(eval(Box::new(if_exp.clone()))?),
            Expression::FunctionLiteral(function) => return Ok(eval(Box::new(function.clone()))?),
            Expression::CallExpression(call_exp) => return Ok(eval(Box::new(call_exp.clone()))?),
        }
    } else if TypeId::of::<PrefixExpression>() == type_id {
        // parser prefix_expression
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
    } else if TypeId::of::<InfixExpression>() == type_id {
        // parser infix expression
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
    } else if TypeId::of::<AstIntegerLiteral>() == type_id {
        // parser integer literals expression
        trace!(
            "type AstIntegerLiteral id = {:?}",
            TypeId::of::<AstIntegerLiteral>()
        );
        let value = node
            .as_any()
            .downcast_ref::<AstIntegerLiteral>()
            .ok_or(anyhow::anyhow!("downcast_ref integer_literal error"))?;
        trace!("[eval] integer literal = {:#?}", value);

        return Ok(Box::new(Integer { value: value.value }));
    } else if TypeId::of::<AstBoolean>() == type_id {
        // parser Expression boolean
        trace!("type AstBoolean id = {:?}", TypeId::of::<AstBoolean>());
        let value = node
            .as_any()
            .downcast_ref::<AstBoolean>()
            .ok_or(anyhow::anyhow!("downcast_ref AstBoolean error"))?;
        trace!("[eval]AstBoolean literal = {:#?}", value);

        return Ok(Box::new(Boolean { value: value.value }));
    }

    Err(anyhow::anyhow!("eval error"))
}

fn eval_statements(stmts: Vec<Statement>) -> anyhow::Result<Box<dyn Object>> {
    trace!("eval_statements stmt = {:#?}", stmts);
    let mut result: Box<dyn Object> = Box::new(Integer { value: 0 });

    for statement in stmts {
        result = eval(Box::new(statement))?;
    }

    Ok(result)
}

fn eval_prefix_expression(
    operator: String,
    right: Box<dyn Object>,
) -> anyhow::Result<Box<dyn Object>> {
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
    left: Box<dyn Object>,
    right: Box<dyn Object>,
) -> anyhow::Result<Box<dyn Object>> {
    let type_id_left = left.as_any().type_id();
    let type_id_right = right.as_any().type_id();
    if TypeId::of::<Integer>() == type_id_left
        && TypeId::of::<Integer>() == type_id_right
        && left.r#type() == ObjectType::INTEGER_OBJ
        && right.r#type() == ObjectType::INTEGER_OBJ
    {
        let left = left
            .as_any()
            .downcast_ref::<Integer>()
            .ok_or(anyhow::anyhow!("downcast_ref integer error"))?;

        let right = right
            .as_any()
            .downcast_ref::<Integer>()
            .ok_or(anyhow::anyhow!("downcast_ref integer error"))?;

        return Ok(eval_integer_infix_expression(
            operator,
            left.clone(),
            right.clone(),
        )?);
    } else if TypeId::of::<Boolean>() == type_id_left
        && TypeId::of::<Boolean>() == type_id_right
        && operator == "=="
    {
        let left = left
            .as_any()
            .downcast_ref::<Boolean>()
            .ok_or(anyhow::anyhow!("downcast_ref integer error"))?
            .value;

        let right = right
            .as_any()
            .downcast_ref::<Boolean>()
            .ok_or(anyhow::anyhow!("downcast_ref integer error"))?
            .value;

        return Ok(native_bool_to_boolean_object(left == right));
    } else if TypeId::of::<Boolean>() == type_id_left
        && TypeId::of::<Boolean>() == type_id_right
        && operator == "!="
    {
        let left = left
            .as_any()
            .downcast_ref::<Boolean>()
            .ok_or(anyhow::anyhow!("downcast_ref integer error"))?
            .value;

        let right = right
            .as_any()
            .downcast_ref::<Boolean>()
            .ok_or(anyhow::anyhow!("downcast_ref integer error"))?
            .value;

        return Ok(native_bool_to_boolean_object(left != right));
    }
    Err(anyhow::anyhow!("eval infix expression error"))
}

// eval ! operator expression
fn eval_bang_operator_expression(right: Box<dyn Object>) -> anyhow::Result<Box<dyn Object>> {
    let type_id = right.as_any().type_id();
    let type_name = right.r#type();
    if TypeId::of::<Integer>() == type_id && ObjectType::INTEGER_OBJ == type_name {
        let value = right
            .as_any()
            .downcast_ref::<Integer>()
            .ok_or(anyhow::anyhow!("downcast_ref integer error"))?;
        if value.value != 0 {
            return Ok(Box::new(Boolean { value: false }));
        } else {
            return Ok(Box::new(Boolean { value: true }));
        }
    } else if TypeId::of::<Boolean>() == type_id && ObjectType::BOOLEAN_OBJ == type_name {
        let value = right
            .as_any()
            .downcast_ref::<Boolean>()
            .ok_or(anyhow::anyhow!("downcast_ref boolean error"))?;

        if value.value {
            return Ok(Box::new(Boolean { value: false }));
        } else {
            return Ok(Box::new(Boolean { value: true }));
        }
    }
    Err(anyhow::anyhow!("eval bang operator expression error"))
}

fn eval_minus_prefix_operator_expression(
    right: Box<dyn Object>,
) -> anyhow::Result<Box<dyn Object>> {
    let type_id = right.as_any().type_id();
    let type_name = right.r#type();
    if TypeId::of::<Integer>() == type_id && ObjectType::INTEGER_OBJ == type_name {
        let value = right
            .as_any()
            .downcast_ref::<Integer>()
            .ok_or(anyhow::anyhow!("downcast_ref boolean error"))?;

        return Ok(Box::new(Integer {
            value: -value.value,
        }));
    }

    Err(anyhow::anyhow!(
        "eval_minus_prefix_operator_expression error "
    ))
}

fn eval_integer_infix_expression(
    operator: String,
    left: Integer,
    right: Integer,
) -> anyhow::Result<Box<dyn Object>> {
    return match operator.as_str() {
        "+" => Ok(Box::new(Integer {
            value: left.value + right.value,
        })),
        "-" => Ok(Box::new(Integer {
            value: left.value - right.value,
        })),
        "*" => Ok(Box::new(Integer {
            value: left.value * right.value,
        })),
        "/" => Ok(Box::new(Integer {
            value: left.value / right.value,
        })),
        "<" => Ok(native_bool_to_boolean_object(left.value < right.value)),
        ">" => Ok(native_bool_to_boolean_object(left.value > right.value)),
        "==" => Ok(native_bool_to_boolean_object(left.value == right.value)),
        "!=" => Ok(native_bool_to_boolean_object(left.value != right.value)),
        _ => Err(anyhow::anyhow!("eval_integer_infix_expression error")),
    };
}

fn native_bool_to_boolean_object(input: bool) -> Box<dyn Object> {
    if input {
        Box::new(Boolean { value: true })
    } else {
        Box::new(Boolean { value: false })
    }
}
