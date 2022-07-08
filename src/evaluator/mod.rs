use crate::ast::expression::boolean::Boolean as AstBoolean;
use crate::ast::expression::integer_literal::IntegerLiteral as AstIntegerLiteral;
use crate::ast::expression::prefix_expression::PrefixExpression;
use crate::ast::expression::Expression;
use crate::ast::statement::expression_statement::ExpressionStatement;
use crate::ast::statement::Statement;
use crate::ast::{Node, Program};
use crate::object::boolean::Boolean;
use crate::object::integer::Integer;
use crate::object::Object;
use log::trace;
use std::any::TypeId;

#[cfg(test)]
pub mod tests;

pub fn eval(node: Box<dyn Node>) -> anyhow::Result<Box<dyn Object>> {
    let type_id = node.as_any().type_id();
    println!("type_id = {:?}", type_id);
    if TypeId::of::<Program>() == type_id {
        // Parser Program
        println!("type program id = {:?}", TypeId::of::<Program>());
        let value = node
            .as_any()
            .downcast_ref::<Program>()
            .ok_or(anyhow::anyhow!("downcast_ref program error"))?;

        return Ok(eval_statements(value.statements.clone())?);
    } else if TypeId::of::<Statement>() == type_id {
        // Parser Statement
        println!("type Statement id = {:?}", TypeId::of::<Statement>());
        let value = node
            .as_any()
            .downcast_ref::<Statement>()
            .ok_or(anyhow::anyhow!("downcast_ref expression statement error"))?;

        let result = match value {
            Statement::ExpressionStatement(exp) => eval(Box::new(exp.clone()))?,
            Statement::LetStatement(let_exp) => eval(Box::new(let_exp.clone()))?,
            Statement::ReturnStatement(ret_exp) => eval(Box::new(ret_exp.clone()))?,
        };
        return Ok(result);
    } else if TypeId::of::<ExpressionStatement>() == type_id {
        // Parser ExpressionStatement
        println!(
            "type ExpressionStatement id = {:?}",
            TypeId::of::<ExpressionStatement>()
        );
        let value = node
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .ok_or(anyhow::anyhow!("downcast_ref expression statement error"))?;

        return Ok(eval(Box::new(value.expression.clone()))?);
    } else if TypeId::of::<Expression>() == type_id {
        // parser Expression
        println!("type Expression id = {:?}", TypeId::of::<Expression>());
        let value = node
            .as_any()
            .downcast_ref::<Expression>()
            .ok_or(anyhow::anyhow!("downcast_ref expression statement error"))?;

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
        println!("type Expression id = {:?}", TypeId::of::<Expression>());
        let value = node
            .as_any()
            .downcast_ref::<PrefixExpression>()
            .ok_or(anyhow::anyhow!("downcast_ref expression statement error"))?;
        println!("[eval] PrefixExpression = {:#?}", value);

        let right = eval(value.right.clone())?;
        return Ok(eval_prefix_expression(value.operator.clone(), right)?);
    } else if TypeId::of::<AstIntegerLiteral>() == type_id {
        // parser integer literals expression
        println!(
            "type AstIntegerLiteral id = {:?}",
            TypeId::of::<AstIntegerLiteral>()
        );
        let value = node
            .as_any()
            .downcast_ref::<AstIntegerLiteral>()
            .ok_or(anyhow::anyhow!("downcast_ref integer_literal error"))?;
        println!("[eval] integer literal = {:#?}", value);

        return Ok(Box::new(Integer { value: value.value }));
    } else if TypeId::of::<AstBoolean>() == type_id {
        // parser Expression boolean
        println!("type AstBoolean id = {:?}", TypeId::of::<AstBoolean>());
        let value = node
            .as_any()
            .downcast_ref::<AstBoolean>()
            .ok_or(anyhow::anyhow!("downcast_ref AstBoolean error"))?;
        println!("[eval]AstBoolean literal = {:#?}", value);

        return Ok(Box::new(Boolean {
            value: value.value,
        }));
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
        _ => Err(anyhow::anyhow!("unimplemented!")),
    }
}

// eval ! operator expression
fn eval_bang_operator_expression(right: Box<dyn Object>) -> anyhow::Result<Box<dyn Object>> {
    let type_id = right.as_any().type_id();
    if TypeId::of::<Integer>() == type_id {
        let value = right
            .as_any()
            .downcast_ref::<Integer>()
            .ok_or(anyhow::anyhow!("downcast_ref integer error"))?;
        if value.value != 0 {
            return Ok(Box::new(Boolean { value: false }));
        } else {
            return Ok(Box::new(Boolean { value: true }));
        }
    } else if TypeId::of::<Boolean>() == type_id {
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
