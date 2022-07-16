use crate::ast::expression::boolean::Boolean as AstBoolean;
use crate::ast::expression::if_expression::IfExpression;
use crate::ast::expression::infix_expression::InfixExpression;
use crate::ast::expression::integer_literal::IntegerLiteral as AstIntegerLiteral;
use crate::ast::expression::prefix_expression::PrefixExpression;
use crate::ast::expression::Expression;
use crate::ast::statement::block_statement::BlockStatement;
use crate::ast::statement::expression_statement::ExpressionStatement;
use crate::ast::statement::return_statement::ReturnStatement;
use crate::ast::statement::Statement;
use crate::ast::{Identifier, Node, Program};
use crate::object::boolean::Boolean;
use crate::object::integer::Integer;
use crate::object::return_value::ReturnValue;
use crate::object::Object;
use log::trace;
use std::any::TypeId;

#[cfg(test)]
pub mod tests;

pub fn eval(node: Box<dyn Node>) -> anyhow::Result<Object> {
    let type_id = node.as_any().type_id();
    println!("[eval] TypeID  is ({:?})", type_id);
    if TypeId::of::<Program>() == type_id {
        // Parser Program
        println!("[eval] Type Program ID is ({:?})", TypeId::of::<Program>());
        let value = node
            .as_any()
            .downcast_ref::<Program>()
            .ok_or(anyhow::anyhow!("[eval] downcast_ref Program Error"))?;

        return Ok(eval_statements(value.statements.clone())?);
    } else if TypeId::of::<Statement>() == type_id {
        // Parser Statement
        println!("[eval] Type Statement ID is ({:?})", TypeId::of::<Statement>());
        let value = node
            .as_any()
            .downcast_ref::<Statement>()
            .ok_or(anyhow::anyhow!("[eval] downcast_ref Statement Error"))?;

        let result = match value {
            Statement::ExpressionStatement(exp) => eval(Box::new(exp.clone()))?,
            Statement::LetStatement(let_exp) => eval(Box::new(let_exp.clone()))?,
            Statement::ReturnStatement(ret_exp) => eval(Box::new(ret_exp.clone()))?,
        };
        return Ok(result);
    } else if TypeId::of::<ExpressionStatement>() == type_id {
        // Parser ExpressionStatement
        println!(
            "[eval] type ExpressionStatement ID is  ({:?})",
            TypeId::of::<ExpressionStatement>()
        );
        let value = node
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .ok_or(anyhow::anyhow!("[eval] downcast_ref ExpressionStatement Error"))?;

        return Ok(eval(Box::new(value.expression.clone()))?);
    } else if TypeId::of::<ReturnStatement>() == type_id {
        println!("[eval] Type ReturnStatement ID is ({:?})", TypeId::of::<ReturnStatement>());
        let value = node
            .as_any()
            .downcast_ref::<ReturnStatement>()
            .ok_or(anyhow::anyhow!("[eval] downcast_ref ReturnStatement Error"))?;

        println!("[eval] return_statement is ({:#?})", value);

        let val = eval(Box::new(value.return_value.clone()))?;
        println!("[eval] return_statement eval value is  ({:?})", val);
        return Ok(Object::ReturnValue(ReturnValue {
            value: Box::new(val),
        }));
    } else if TypeId::of::<Expression>() == type_id {
        // parser Expression
        println!("[eval] Type Expression ID is ({:?})", TypeId::of::<Expression>());
        let value = node
            .as_any()
            .downcast_ref::<Expression>()
            .ok_or(anyhow::anyhow!("[eval] downcast_ref Expression Error"))?;

        return match value {
            Expression::PrefixExpression(pre_exp) => Ok(eval(Box::new(pre_exp.clone()))?),
            Expression::InfixExpression(infix_exp) => Ok(eval(Box::new(infix_exp.clone()))?),
            Expression::IntegerLiteralExpression(integer) => Ok(eval(Box::new(integer.clone()))?),

            Expression::IdentifierExpression(identifier) => Ok(eval(Box::new(identifier.clone()))?),
            Expression::BooleanExpression(boolean) => Ok(eval(Box::new(boolean.clone()))?),
            Expression::IfExpression(if_exp) => Ok(eval(Box::new(if_exp.clone()))?),
            Expression::FunctionLiteral(function) => Ok(eval(Box::new(function.clone()))?),
            Expression::CallExpression(call_exp) => Ok(eval(Box::new(call_exp.clone()))?),
        };
    } else if TypeId::of::<PrefixExpression>() == type_id {
        // parser PrefixExpression
        println!(
            "[eval] Type PrefixExpression ID is ({:?})",
            TypeId::of::<PrefixExpression>()
        );
        let value = node
            .as_any()
            .downcast_ref::<PrefixExpression>()
            .ok_or(anyhow::anyhow!("[eval] downcast_ref PrefixExpression Error"))?;
        println!("[eval] PrefixExpression is ({:#?})", value);

        let right = eval(value.right.clone())?;
        return Ok(eval_prefix_expression(value.operator.clone(), right)?);
    } else if TypeId::of::<InfixExpression>() == type_id {
        // parser InfixExpression
        println!(
            "[eval] Type InfixExpression ID is ({:?})",
            TypeId::of::<InfixExpression>()
        );
        let value = node
            .as_any()
            .downcast_ref::<InfixExpression>()
            .ok_or(anyhow::anyhow!("[eval] downcast_ref InfixExpression Error"))?;
        println!("[eval] InfixExpression = {:#?}", value);

        let left = eval(value.left.clone())?;
        let right = eval(value.right.clone())?;

        return Ok(eval_infix_expression(value.operator.clone(), left, right)?);
    } else if TypeId::of::<AstIntegerLiteral>() == type_id {
        // parser AstIntegerLiteral
        println!(
            "[eval] Type AstIntegerLiteral ID is ({:?})",
            TypeId::of::<AstIntegerLiteral>()
        );
        let value = node
            .as_any()
            .downcast_ref::<AstIntegerLiteral>()
            .ok_or(anyhow::anyhow!("[eval] downcast_ref AstIntegerLiteral Error"))?;
        println!("[eval] integer literal is ({:?})", value);

        return Ok(Object::Integer(Integer { value: value.value }));
    } else if TypeId::of::<AstBoolean>() == type_id {
        // parser AstBoolean
        println!("[eval] Type AstBoolean ID is ({:?})", TypeId::of::<AstBoolean>());
        let value = node
            .as_any()
            .downcast_ref::<AstBoolean>()
            .ok_or(anyhow::anyhow!("[eval] downcast_ref AstBoolean Error"))?;
        println!("[eval]AstBoolean literal is ({:#?})", value);

        return Ok(Object::Boolean(Boolean { value: value.value }));
    } else if TypeId::of::<BlockStatement>() == type_id {
        println!("[eval] Type AstBoolean ID is ({:?})", TypeId::of::<BlockStatement>());
        let value = node
            .as_any()
            .downcast_ref::<BlockStatement>()
            .ok_or(anyhow::anyhow!("[eval] downcast_ref BlockStatement Error"))?;
        println!("[eval] BlockStatement literal = {:#?}", value);

        return Ok(eval_statements(value.statements.clone())?);
    } else if TypeId::of::<IfExpression>() == type_id {
        println!("[eval] Type IfExpression ID is ({:?})", TypeId::of::<IfExpression>());
        let value = node
            .as_any()
            .downcast_ref::<IfExpression>()
            .ok_or(anyhow::anyhow!("[eval] downcast_ref IfExpression Error"))?;
        println!("[eval]IfExpression literal is ({:#?})", value);

        return Ok(eval_if_expression(value.clone())?);
    } else if TypeId::of::<Identifier>() == type_id {
        println!("[eval] Type Identifier ID is ({:?})", TypeId::of::<Identifier>());
        let value = node
            .as_any()
            .downcast_ref::<Identifier>()
            .ok_or(anyhow::anyhow!("[eval] downcast_ref Identifier Error"))?;
        println!("[eval]Identifier literal is  ({:#?})", value);

        return Ok(Object::Integer(Integer {
            value: value.value.parse()?,
        }));
    } else {
        // Parser Unknown type
        println!("[eval] type Unknown Type!");
        Err(anyhow::anyhow!(format!("[eval] Unknown Type Error,  This type_id is ({:?})",type_id)))
    }
}

fn eval_statements(stmts: Vec<Statement>) -> anyhow::Result<Object> {
    println!("[eval_statements]  statements is ({:#?})", stmts);
    let mut result: Object = Object::Unit(());

    for statement in stmts {
        result = eval(Box::new(statement))?;

        match result {
            Object::ReturnValue(value) => {
                println!("[eval_statement] ReturnValue is ({:?})", value);
                return Ok(*value.value.clone());
            }
            _ => continue,
        }
    }

    Ok(result)
}

fn eval_prefix_expression(operator: String, right: Object) -> anyhow::Result<Object> {
    match operator.as_str() {
        "!" => {
            return Ok(eval_bang_operator_expression(right)?);
        }
        "-" => {
            return Ok(eval_minus_prefix_operator_expression(right)?);
        }
        _ => Err(anyhow::anyhow!(format!("[eval_prefix_expression({})] unimplemented!", operator))),
    }
}

fn eval_infix_expression(operator: String, left: Object, right: Object) -> anyhow::Result<Object> {
    match (left, right) {
        (Object::Integer(left_value), Object::Integer(right_value)) => {
            return Ok(eval_integer_infix_expression(
                operator,
                left_value.clone(),
                right_value.clone(),
            )?);
        }
        (Object::Boolean(left_value), Object::Boolean(right_value)) if operator == "==" => {
            return Ok(native_bool_to_boolean_object(
                left_value.value == right_value.value,
            ));
        }
        (Object::Boolean(left_value), Object::Boolean(right_value)) if operator == "!=" => {
            return Ok(native_bool_to_boolean_object(
                left_value.value != right_value.value,
            ));
        }
        (_, _) => Err(anyhow::anyhow!("[eval_infix_expression] unimplemented!")),
    }
}

// eval ! operator expression
fn eval_bang_operator_expression(right: Object) -> anyhow::Result<Object> {
    match right {
        Object::Boolean(value) => {
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
        Object::ReturnValue(_) => Err(anyhow::anyhow!(
            "[eval_bang_operator_expression] unimplemented ReturnValue  Error "
        )),
        Object::Unit(_) =>  Err(anyhow::anyhow!(
            "[eval_bang_operator_expression] unimplemented Unit Error "
        )),
    }
}

fn eval_minus_prefix_operator_expression(right: Object) -> anyhow::Result<Object> {
    match right {
        Object::Integer(value) => {
            return Ok(Object::Integer(Integer {
                value: -value.value,
            }));
        }
        _ => Err(anyhow::anyhow!(
            "[eval_minus_prefix_operator_expression] unimplemented Error "
        )),
    }
}

fn eval_integer_infix_expression(operator: String, left: Integer, right: Integer) -> anyhow::Result<Object> {
    match operator.as_str() {
        "+" => Ok(Object::Integer(Integer {
            value: left.value + right.value,
        })),
        "-" => Ok(Object::Integer(Integer {
            value: left.value - right.value,
        })),
        "*" => Ok(Object::Integer(Integer {
            value: left.value * right.value,
        })),
        "/" => Ok(Object::Integer(Integer {
            value: left.value / right.value,
        })),
        "<" => Ok(native_bool_to_boolean_object(left.value < right.value)),
        ">" => Ok(native_bool_to_boolean_object(left.value > right.value)),
        "==" => Ok(native_bool_to_boolean_object(left.value == right.value)),
        "!=" => Ok(native_bool_to_boolean_object(left.value != right.value)),
        _ => Err(anyhow::anyhow!(format!("[eval_integer_infix_expression] unimplemented operator ({})", operator))),
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
    };
}

fn is_truthy(obj: Object) -> anyhow::Result<bool> {
    let type_id = obj.as_any().type_id();
    if TypeId::of::<()>() == type_id {
        Ok(false)
    } else if TypeId::of::<Boolean>() == type_id {
        let value = obj
            .as_any()
            .downcast_ref::<Boolean>()
            .ok_or(anyhow::anyhow!("[is_truthy] downcast_ref Boolean Error"))?;

        if value.value {
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        Ok(true)
    }
}
