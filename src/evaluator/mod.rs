use crate::ast::expression::boolean::Boolean as AstBoolean;
use crate::ast::expression::if_expression::IfExpression;
use crate::ast::expression::infix_expression::InfixExpression;
use crate::ast::expression::integer_literal::IntegerLiteral as AstIntegerLiteral;
use crate::ast::expression::prefix_expression::PrefixExpression;
use crate::ast::expression::Expression;
use crate::ast::statement::block_statement::BlockStatement;
use crate::ast::statement::expression_statement::ExpressionStatement;
use crate::ast::statement::let_statement::LetStatement;
use crate::ast::statement::return_statement::ReturnStatement;
use crate::ast::statement::Statement;
use crate::ast::{Identifier, Node, Program};
use crate::object::boolean::Boolean;
use crate::object::environment::Environment;
use crate::object::integer::Integer;
use crate::object::return_value::ReturnValue;
use crate::object::ObjectType::INTEGER_OBJ;
use crate::object::{Object, ObjectInterface, ObjectType};
use log::trace;
use std::any::TypeId;
use std::clone;
use crate::ast::expression::call_expression::CallExpression;
use crate::ast::expression::function_literal::FunctionLiteral;
use crate::object::function::Function;

#[cfg(test)]
pub mod tests;

pub fn eval(node: Box<dyn Node>, env: &mut Environment) -> anyhow::Result<Object> {
    let type_id = node.as_any().type_id();
    println!("[eval] TypeID  is ({:?})", type_id);
    if TypeId::of::<Program>() == type_id {
        // Parser Program
        println!("[eval] Type Program ID is ({:?})", TypeId::of::<Program>());
        let value = node
            .as_any()
            .downcast_ref::<Program>()
            .ok_or(anyhow::anyhow!("[eval] downcast_ref Program Error"))?;

        return Ok(eval_program(value, env)?);
    } else if TypeId::of::<Statement>() == type_id {
        // Parser Statement
        println!(
            "[eval] Type Statement ID is ({:?})",
            TypeId::of::<Statement>()
        );
        let value = node
            .as_any()
            .downcast_ref::<Statement>()
            .ok_or(anyhow::anyhow!("[eval] downcast_ref Statement Error"))?;

        let result = match value {
            Statement::ExpressionStatement(exp) => eval(Box::new(exp.clone()), env)?,
            Statement::LetStatement(let_exp) => eval(Box::new(let_exp.clone()), env)?,
            Statement::ReturnStatement(ret_exp) => eval(Box::new(ret_exp.clone()), env)?,
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
            .ok_or(anyhow::anyhow!(
                "[eval] downcast_ref ExpressionStatement Error"
            ))?;

        return Ok(eval(Box::new(value.expression.clone()), env)?);
    } else if TypeId::of::<ReturnStatement>() == type_id {
        println!(
            "[eval] Type ReturnStatement ID is ({:?})",
            TypeId::of::<ReturnStatement>()
        );
        let value = node
            .as_any()
            .downcast_ref::<ReturnStatement>()
            .ok_or(anyhow::anyhow!("[eval] downcast_ref ReturnStatement Error"))?;

        println!("[eval] return_statement is ({})", value);

        let val = eval(Box::new(*value.return_value.clone()), env)?;
        println!("[eval] return_statement eval value is  ({:?})", val);
        return Ok(Object::ReturnValue(ReturnValue {
            value: Box::new(val),
        }));
    } else if TypeId::of::<LetStatement>() == type_id {
        println!(
            "[eval] Type LetStatement ID is ({:?})",
            TypeId::of::<LetStatement>()
        );
        let value = node
            .as_any()
            .downcast_ref::<LetStatement>()
            .ok_or(anyhow::anyhow!("[eval] downcast_ref LetStatement Error"))?;

        println!("[eval] LetStatement is ({})", value);

        let val = eval(Box::new(*value.value.clone()), env)?;

        println!("[eval] LetStatement eval after = {:?}", val);

        env.store(value.name.value.clone(), val);

        Ok(Object::Unit(()))
    } else if TypeId::of::<Expression>() == type_id {
        // parser Expression
        println!(
            "[eval] Type Expression ID is ({:?})",
            TypeId::of::<Expression>()
        );
        let value = node
            .as_any()
            .downcast_ref::<Expression>()
            .ok_or(anyhow::anyhow!("[eval] downcast_ref Expression Error"))?;

        return match value {
            Expression::PrefixExpression(pre_exp) => Ok(eval(Box::new(pre_exp.clone()), env)?),
            Expression::InfixExpression(infix_exp) => Ok(eval(Box::new(infix_exp.clone()), env)?),
            Expression::IntegerLiteralExpression(integer) => {
                Ok(eval(Box::new(integer.clone()), env)?)
            }

            Expression::IdentifierExpression(identifier) => {
                Ok(eval(Box::new(identifier.clone()), env)?)
            }
            Expression::BooleanExpression(boolean) => Ok(eval(Box::new(boolean.clone()), env)?),
            Expression::IfExpression(if_exp) => Ok(eval(Box::new(if_exp.clone()), env)?),
            Expression::FunctionLiteral(function) => Ok(eval(Box::new(function.clone()), env)?),
            Expression::CallExpression(call_exp) => Ok(eval(Box::new(call_exp.clone()), env)?),
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
            .ok_or(anyhow::anyhow!(
                "[eval] downcast_ref PrefixExpression Error"
            ))?;
        println!("[eval] PrefixExpression is ({})", value);

        let right = eval(value.right.clone(), env)?;
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
        println!("[eval] InfixExpression is ({})", value);

        let left = eval(value.left.clone(), env)?;
        let right = eval(value.right.clone(), env)?;

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
            .ok_or(anyhow::anyhow!(
                "[eval] downcast_ref AstIntegerLiteral Error"
            ))?;
        println!("[eval] integer literal is ({:?})", value);

        return Ok(Object::Integer(Integer { value: value.value }));
    } else if TypeId::of::<FunctionLiteral>() == type_id {
        // parser AstIntegerLiteral
        println!(
            "[eval] Type FunctionLiteral ID is ({:?})",
            TypeId::of::<FunctionLiteral>()
        );
        let value = node
            .as_any()
            .downcast_ref::<FunctionLiteral>()
            .ok_or(anyhow::anyhow!(
                "[eval] downcast_ref FunctionLiteral Error"
            ))?;
        println!("[eval] FunctionLiteral is ({})", value);
        let params = value.parameters.clone();
        let body = value.body.clone();

        return Ok(Object::Function(Function {
            parameters: params,
            env: env.clone(),
            body: body.clone(),
        }));

    } else if TypeId::of::<AstBoolean>() == type_id {
        // parser AstBoolean
        println!(
            "[eval] Type AstBoolean ID is ({:?})",
            TypeId::of::<AstBoolean>()
        );
        let value = node
            .as_any()
            .downcast_ref::<AstBoolean>()
            .ok_or(anyhow::anyhow!("[eval] downcast_ref AstBoolean Error"))?;
        println!("[eval]AstBoolean literal is ({})", value);

        return Ok(Object::Boolean(Boolean { value: value.value }));
    } else if TypeId::of::<BlockStatement>() == type_id {
        println!(
            "[eval] Type AstBoolean ID is ({:?})",
            TypeId::of::<BlockStatement>()
        );
        let value = node
            .as_any()
            .downcast_ref::<BlockStatement>()
            .ok_or(anyhow::anyhow!("[eval] downcast_ref BlockStatement Error"))?;
        println!("[eval] BlockStatement literal is  ({})", value);

        return Ok(eval_block_statement(value, env)?);
    } else if TypeId::of::<IfExpression>() == type_id {
        println!(
            "[eval] Type IfExpression ID is ({:?})",
            TypeId::of::<IfExpression>()
        );
        let value = node
            .as_any()
            .downcast_ref::<IfExpression>()
            .ok_or(anyhow::anyhow!("[eval] downcast_ref IfExpression Error"))?;
        println!("[eval]IfExpression literal is ({})", value);

        return Ok(eval_if_expression(value.clone(), env)?);
    } else if TypeId::of::<Identifier>() == type_id {
        println!(
            "[eval] Type Identifier ID is ({:?})",
            TypeId::of::<Identifier>()
        );
        let value = node
            .as_any()
            .downcast_ref::<Identifier>()
            .ok_or(anyhow::anyhow!("[eval] downcast_ref Identifier Error"))?;
        println!("[eval]Identifier literal is  ({})", value);

        return eval_identifier(value.clone(), env);
    } else if TypeId::of::<CallExpression>() == type_id {
        println!(
            "[eval] Type CallExpression ID is ({:?})",
            TypeId::of::<CallExpression>()
        );
        let value = node
            .as_any()
            .downcast_ref::<CallExpression>()
            .ok_or(anyhow::anyhow!("[eval] downcast_ref CallExpression Error"))?;
        println!("[eval]CallExpression  is  ({})", value);

        let function = eval(Box::new(*value.function.clone()), env)?;
        println!("[eval]CallExpression : function is ({})", function);

        let args = eval_expressions(value.arguments.clone(), env)?;
        println!("[eval]CallExpression: args is  ({:?})", args);

        return apply_function(function, args);
    } else {
        // Parser Unknown type
        println!("[eval] type Unknown Type!");
        println!("[eval] Unknown Node is {:#?}", node);
        println!(
            "[eval] Type FunctionLiteral ID is ({:?})",
            TypeId::of::<FunctionLiteral>()
        );
        Err(anyhow::anyhow!(format!(
            "[eval] Unknown Type Error,  This type_id is ({:?})",
            type_id
        )))
    }
}

fn apply_function(fn_obj: Object, args: Vec<Object>) -> anyhow::Result<Object> {
    let function = match fn_obj {
        Object::Function(fn_value) => fn_value,
        _ => return Err(anyhow::anyhow!(format!("not a function: {}", fn_obj.r#type()))),
    };

    println!("[apply_function] function is {:#?}", function);

    let mut extend_env = extend_function_env(function.clone(), args);
    println!("[apply_function] extend_env is {:?}", extend_env);

    let evaluated = eval(Box::new(function.body), &mut extend_env)?;
    println!("[apply_function] call function result is {}", evaluated);

    // return unwrap_return_value(evaluated);
    Ok(evaluated)
}


fn extend_function_env(fn_obj: Function, args: Vec<Object>) -> Environment {
    let mut env = Environment::new_enclosed_environment(fn_obj.env);
    for (param_idx, param) in fn_obj.parameters.iter().enumerate() {
        env.store(param.value.clone(), args[param_idx].clone()); // TODO need imporve
    }
    env
}

// fn unwrap_return_value(obj: Object) -> anyhow::Result<Object> {
//     match obj {
//         // support return ReturnValue
//         // example fn(x) { return x; }
//         Object::ReturnValue(value) => Ok(*value.value),
//         // todo(daivian) don't know why need this.
//         // Support return expression,
//         // example fn(x) { x; }
//         Object::Integer(val) => Ok(Object::Integer(val)),
//         _ => {
//             eprintln!("[unwrap_return_value] object is = {:?}", obj);
//             Err(anyhow::anyhow!("unwrap_return_value error"))
//         },
//     }
// }

fn eval_expressions(exps: Vec<Box<Expression>>, env: &mut Environment) -> anyhow::Result<Vec<Object>> {
    println!("[eval_expressions] start");

    let mut result = vec![];

    for e in exps.into_iter() {
        let evaluated = eval(e, env)?;
        println!("[eval_expressions] evaluated is = {:?}", evaluated);
        result.push(evaluated);
    }

    println!("[eval_expressions] end");

    Ok(result)
}
fn eval_program(program: &Program, env: &mut Environment) -> anyhow::Result<Object> {
    println!("[eval_program]  program is ({})", program);
    let mut result: Object = Object::Unit(());

    for statement in program.statements.clone().into_iter() {
        result = eval(Box::new(statement), env)?;

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
fn eval_statements(stmts: Vec<Statement>, env: &mut Environment) -> anyhow::Result<Object> {
    println!("[eval_statements]  statements is ({:?})", stmts);
    let mut result: Object = Object::Unit(());

    for statement in stmts {
        result = eval(Box::new(statement), env)?;

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
fn eval_block_statement(block: &BlockStatement, env: &mut Environment) -> anyhow::Result<Object> {
    println!("[eval_block_statement]  BlockStatement is ({})", block);
    let mut result: Object = Object::Unit(());

    for statement in block.statements.clone().into_iter() {
        println!("[eval_block_statement] statement is ({:#?})", statement);
        result = eval(Box::new(statement), env)?;

        println!("[eval_block_statement] result is ({:?})", result);
        match result.clone() {
            Object::ReturnValue(value) => {
                if value.r#type() == ObjectType::RETURN_OBJ {
                    return Ok(Object::ReturnValue(value.clone()));
                }
            }
            _ => continue,
        }
    }

    return Ok(result);
}

fn eval_prefix_expression(operator: String, right: Object) -> anyhow::Result<Object> {
    match operator.as_str() {
        "!" => Ok(eval_bang_operator_expression(right)?),
        "-" => Ok(eval_minus_prefix_operator_expression(right)?),
        _ => Err(anyhow::anyhow!(format!(
            "unknown operator: {}{}",
            operator,
            right.r#type()
        ))),
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
        (left, right) => {
            if left.r#type() != right.r#type() {
                Err(anyhow::anyhow!(format!(
                    "type mismatch: {} {} {}",
                    left.r#type(),
                    operator,
                    right.r#type()
                )))
            } else {
                Err(anyhow::anyhow!(format!(
                    "unknown operator: {} {} {}",
                    left.r#type(),
                    operator,
                    right.r#type()
                )))
            }
        }
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
        _ => Err(anyhow::anyhow!(
            "[eval_bang_operator_expression] unimplemented  Error "
        )),
    }
}

fn eval_minus_prefix_operator_expression(right: Object) -> anyhow::Result<Object> {
    match right.clone() {
        Object::Integer(value) => Ok(Object::Integer(Integer {
            value: -value.value,
        })),
        value if value.r#type() != INTEGER_OBJ => Err(anyhow::anyhow!(format!(
            "unknown operator: -{}",
            right.r#type()
        ))),
        _ => unimplemented!(),
    }
}

fn eval_integer_infix_expression(
    operator: String,
    left: Integer,
    right: Integer,
) -> anyhow::Result<Object> {
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
        _ => Err(anyhow::anyhow!(format!(
            "unknown operator: {} {} {}",
            left.r#type(),
            operator,
            right.r#type()
        ))),
    }
}

fn native_bool_to_boolean_object(input: bool) -> Object {
    if input {
        Object::Boolean(Boolean { value: true })
    } else {
        Object::Boolean(Boolean { value: false })
    }
}

fn eval_if_expression(ie: IfExpression, env: &mut Environment) -> anyhow::Result<Object> {
    let condition = eval(ie.condition, env)?;

    return if is_truthy(condition)? {
        eval(Box::new(ie.consequence.unwrap()), env)
    } else if ie.alternative.is_some() {
        eval(Box::new(ie.alternative.unwrap()), env)
    } else {
        Ok(Object::Unit(()))
    };
}

fn is_truthy(obj: Object) -> anyhow::Result<bool> {
    let type_id = ObjectInterface::as_any(&obj).type_id();
    if TypeId::of::<()>() == type_id {
        Ok(false)
    } else if TypeId::of::<Boolean>() == type_id {
        let value = ObjectInterface::as_any(&obj)
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

fn eval_identifier(node: Identifier, env: &mut Environment) -> anyhow::Result<Object> {
    let val = env.get(node.value.clone());
    if val.is_none() {
        Err(anyhow::anyhow!(format!(
            "identifier not found: {}",
            node.value
        )))
    } else {
        Ok(val.unwrap().clone())
    }
}
