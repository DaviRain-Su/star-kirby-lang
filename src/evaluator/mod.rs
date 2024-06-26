use crate::ast::expression::hash::HashLiteral;
use crate::ast::expression::if_expression::If;
use crate::ast::expression::Expression;
use crate::ast::statement::Statement;
use crate::ast::NodeInterface;
use crate::ast::{Identifier, Node};
use crate::error::Error;
use crate::evaluator::builtins::lookup_builtin;
use crate::object::array::Array;
use crate::object::boolean::Boolean;
use crate::object::boolean::Boolean as ObjBoolean;
use crate::object::environment::Environment;
use crate::object::function::Function;
use crate::object::hash::Hash;
use crate::object::integer::Integer;
use crate::object::null::Null;
use crate::object::r#macro::quote::Quote;
use crate::object::return_value::ReturnValue;
use crate::object::string::StringObj;
use crate::object::ObjectType;
use crate::object::{Object, ObjectInterface};
use std::collections::BTreeMap;

pub mod builtins;

#[cfg(test)]
pub mod tests;

impl Node {
    pub fn quote(&self) -> anyhow::Result<Object> {
        match self {
            Node::Program(value) => Err(Error::UnknownTypeError(format!("{value:?}")).into()),
            Node::Expression(value) => Ok(Quote::new(value.into()).into()),
            Node::Statement(value) => Ok(Quote::new(value.into()).into()),
            Node::Object(value) => Ok(Quote::new(value.into()).into()),
        }
    }

    pub fn eval(&self, env: &mut Environment) -> anyhow::Result<Object> {
        match self {
            Node::Program(ref value) => value.eval_program(env),
            Node::Statement(ref value) => match value {
                Statement::Expression(value) => {
                    let expression_node: Node = value.expression().clone().into();
                    expression_node.eval(env)
                }
                Statement::Let(value) => {
                    let val_node = Node::from(value.value().clone());
                    let val = val_node.eval(env)?;
                    env.store(value.name().value.clone(), val);
                    Ok(Null.into())
                }
                Statement::Return(value) => {
                    let val_node = Node::from(value.return_value().clone());
                    let val = val_node.eval(env)?;
                    Ok(ReturnValue::new(val).into())
                }
                Statement::BlockStatement(value) => value.eval_block_statement(env),
            },
            Node::Expression(ref value) => match value {
                Expression::Prefix(value) => {
                    let right_node = Node::from(value.right().clone());
                    let right = right_node.eval(env)?;
                    Ok(right.eval_prefix_expression(value.operator()))
                }
                Expression::Infix(value) => {
                    let left_node = Node::from(value.left().clone());
                    let left = left_node.eval(env)?;
                    let right_node = Node::from(value.right().clone());
                    let right = right_node.eval(env)?;
                    left.eval_infix_expression(value.operator(), right)
                }
                Expression::IntegerLiteral(value) => Ok(Integer::new(value.value()).into()),
                Expression::Identifier(value) => value.eval_identifier(env),
                Expression::Boolean(boolean) => {
                    Ok(Object::Boolean(ObjBoolean::new(boolean.value())))
                }
                Expression::If(value) => value.eval_if_expression(env),
                Expression::FunctionLiteral(value) => {
                    let params = value.parameters();
                    let body = value.body().clone();
                    Ok(Function::new(params.into(), body, env.clone()).into())
                }
                Expression::Call(value) => {
                    if value.function().token_literal() == "quote" {
                        return Node::from(value.arguments()[0].clone()).quote();
                    }
                    let call_exp_node = Node::from(value.function().clone());
                    let function = call_exp_node.eval(env)?;

                    let args = eval_expressions(value.arguments(), env)?;

                    function.apply_function(args)
                }
                Expression::StringLiteral(value) => {
                    Ok(StringObj::new(value.value().to_string()).into())
                }
                Expression::ArrayLiteral(value) => {
                    let elements = eval_expressions(value.elements(), env)?;

                    Ok(Array::new(elements.into_iter().collect()).into())
                }
                Expression::Index(value) => {
                    let left_node = Node::from(value.left().clone());
                    let left = left_node.eval(env)?;
                    let index_node = Node::from(value.index().clone());
                    let index = index_node.eval(env)?;

                    left.eval_index_expression(index)
                }
                Expression::HashLiteral(value) => value.eval_hash_literal(env),
            },
            Node::Object(value) => {
                Err(Error::UnknownTypeError(format!("object: {value:?}")).into())
            }
        }
    }
}

#[tracing::instrument(level = "trace", skip(env))]
fn eval_expressions(exps: &[Expression], env: &mut Environment) -> anyhow::Result<Vec<Object>> {
    let mut result = vec![];

    for e in exps {
        let node = Node::from(e);
        let evaluated = node.eval(env)?;
        tracing::trace!("[eval_expressions] evaluated is = {:?}", evaluated);
        result.push(evaluated);
    }

    Ok(result)
}

impl HashLiteral {
    pub fn eval_hash_literal(&self, env: &mut Environment) -> anyhow::Result<Object> {
        let mut pairs = BTreeMap::<Object, Object>::new();

        for (key_node, value_node) in self.pair().iter() {
            let key_node = Node::from(key_node.clone());
            let key = key_node.eval(env)?;
            let value_node = Node::from(value_node.clone());
            let value = value_node.eval(env)?;
            pairs.insert(key, value);
        }

        Ok(Object::Hash(Hash::new(pairs)))
    }
}

impl Function {
    fn extend_function_env(&self, args: Vec<Object>) -> Environment {
        let mut env = Environment::new_enclosed_environment(self.env().clone());
        for (param_idx, param) in self.parameters().iter().enumerate() {
            env.store(param.value.clone(), args[param_idx].clone()); // TODO need imporve
        }
        env
    }
}

impl StringObj {
    // can add more operator for string
    // 如果想支持字符串比较，那么可以在这里添加==和!=，但注意不能比较字符串指针
    fn eval_string_infix_expression(
        &self,
        operator: &str,
        right: StringObj,
    ) -> anyhow::Result<Object> {
        match operator {
            "+" => {
                let left_val = self.value();
                let right_val = right.value();

                Ok(StringObj::new(format!("{left_val}{right_val}")).into())
            }
            "==" => {
                let left_val = self.value();
                let right_val = right.value();

                Ok(Boolean::new(left_val == right_val).into())
            }
            "!=" => {
                let left_val = self.value();
                let right_val = right.value();

                Ok(Boolean::new(left_val != right_val).into())
            }
            _ => Err(Error::UnknownOperator {
                left: self.object_type().to_string(),
                operator: operator.to_string(),
                right: right.object_type().to_string(),
            }
            .into()),
        }
    }
}

impl Integer {
    fn eval_integer_infix_expression(&self, operator: &str, right: Integer) -> Object {
        match operator {
            "+" => Integer::new(self.value() + right.value()).into(),
            "-" => Integer::new(self.value() - right.value()).into(),
            "*" => Integer::new(self.value() * right.value()).into(),
            "/" => Integer::new(self.value() / right.value()).into(),
            "<" => (self.value() < right.value()).into(),
            ">" => (self.value() > right.value()).into(),
            "==" => (self.value() == right.value()).into(),
            "!=" => (self.value() != right.value()).into(),
            _ => Null.into(),
        }
    }
}

impl If {
    pub fn eval_if_expression(&self, env: &mut Environment) -> anyhow::Result<Object> {
        let node = Node::from(self.condition().clone());
        let condition = node.eval(env)?;

        if condition.is_truthy() {
            let node: Node = self.consequence().clone().unwrap().into();
            node.eval(env)
        } else if self.alternative().is_some() {
            let node: Node = self.alternative().clone().unwrap().into();
            node.eval(env)
        } else {
            Ok(Null.into())
        }
    }
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Boolean(boolean) => boolean.value(),
            _ => false,
        }
    }

    pub fn eval_array_index_expression(&self, index: Object) -> anyhow::Result<Object> {
        let array_object = match self {
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

    pub fn eval_hash_index_expression(&self, index: Object) -> anyhow::Result<Object> {
        let hash_object = Hash::try_from(self.clone())?;
        let pair = hash_object.pairs().get(&index);
        if pair.is_none() {
            return Ok(Null.into());
        }

        Ok(pair.unwrap().clone())
    }

    #[tracing::instrument(name = "eval_index_expression", skip(self), fields(index = %index))]
    pub fn eval_index_expression(&self, index: Object) -> anyhow::Result<Object> {
        if self.object_type() == ObjectType::Array && index.object_type() == ObjectType::Integer {
            self.eval_array_index_expression(index)
        } else if self.object_type() == ObjectType::Hash {
            self.eval_hash_index_expression(index)
        } else {
            Err(Error::IndexOperatorNotSupported(self.object_type().to_string()).into())
        }
    }

    fn eval_minus_prefix_operator_expression(&self) -> Object {
        match self {
            Object::Integer(value) => Integer::new(-value.value()).into(),
            _ => Null.into(),
        }
    }

    // eval ! operator expression
    pub fn eval_bang_operator_expression(&self) -> Object {
        match self {
            Object::Boolean(value) => {
                if value.value() {
                    false.into()
                } else {
                    true.into()
                }
            }
            Object::Integer(value) => {
                if value.value() != 0 {
                    false.into()
                } else {
                    true.into()
                }
            }
            Object::Null(_) => true.into(),
            _ => false.into(),
        }
    }

    pub fn eval_infix_expression(&self, operator: &str, right: Object) -> anyhow::Result<Object> {
        match (self.clone(), right) {
            (Object::Integer(left_value), Object::Integer(right_value)) => {
                Ok(left_value.eval_integer_infix_expression(operator, right_value))
            }
            (Object::Boolean(left_value), Object::Boolean(right_value)) if operator == "==" => {
                Ok((left_value.value() == right_value.value()).into())
            }
            (Object::Boolean(left_value), Object::Boolean(right_value)) if operator == "!=" => {
                Ok((left_value.value() != right_value.value()).into())
            }
            (Object::String(left), Object::String(right)) => {
                left.eval_string_infix_expression(operator, right)
            }
            (_, _) => Ok(Null.into()),
        }
    }

    pub fn eval_prefix_expression(&self, operator: &str) -> Object {
        match operator {
            "!" => self.eval_bang_operator_expression(),
            "-" => self.eval_minus_prefix_operator_expression(),
            _ => Null.into(),
        }
    }

    #[tracing::instrument(name = "apply_function", skip(self), fields(self = ?self, args = ?args))]
    pub fn apply_function(&self, args: Vec<Object>) -> anyhow::Result<Object> {
        match self.clone() {
            Object::Function(fn_value) => {
                tracing::trace!("[apply_function] function is {:#?}", fn_value);

                let mut extend_env = fn_value.extend_function_env(args);
                tracing::trace!("[apply_function] extend_env is {:?}", extend_env);

                let fn_value: Node = fn_value.body().clone().into();
                let evaluated = fn_value.eval(&mut extend_env)?;
                tracing::trace!("[apply_function] call function result is {}", evaluated);

                Ok(evaluated)
            }
            Object::Builtin(built_in) => (built_in.value())(args),
            _ => Err(Error::NoFunction(self.object_type().to_string()).into()),
        }
    }
}

impl Identifier {
    pub fn eval_identifier(&self, env: &mut Environment) -> anyhow::Result<Object> {
        let val = env.get(self.value.clone());
        if let Some(val) = val {
            return Ok(val.clone());
        }

        if let Ok(builtin) = lookup_builtin(self.value.as_str()) {
            return Ok(builtin.into());
        }

        Err(Error::IdentifierNotFound(self.value.clone()).into())
    }
}
