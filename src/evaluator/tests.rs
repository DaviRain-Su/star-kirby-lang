use std::any::{Any, TypeId};
use crate::evaluator::eval;
use crate::lexer::Lexer;
use crate::object::boolean::Boolean;
use crate::object::integer::Integer;
use crate::object::{Object, ObjectInterface};
use crate::parser::Parser;

fn test_eval_integer_expression() -> anyhow::Result<()> {
    struct Test {
        input: String,
        expected: i64,
    }

    let tests = vec![
        Test {
            input: "5".into(),
            expected: 5,
        },
        Test {
            input: "10".into(),
            expected: 10,
        },
        Test {
            input: "-5".into(),
            expected: -5,
        },
        Test {
            input: "-10".into(),
            expected: -10,
        },
        Test {
            input: "5 + 5 + 5 + 5 - 10".into(),
            expected: 10,
        },
        Test {
            input: "2 * 2 * 2 * 2 * 2".into(),
            expected: 32,
        },
        Test {
            input: "-50 + 100 + -50".into(),
            expected: 0,
        },
        Test {
            input: "5 * 2 + 10".into(),
            expected: 20,
        },
        Test {
            input: "5 + 2 * 10".into(),
            expected: 25,
        },
        Test {
            input: "20 + 2 * -10".into(),
            expected: 0,
        },
        Test {
            input: "50 / 2 * 2 + 10".into(),
            expected: 60,
        },
        Test {
            input: "2 * (5 + 10)".into(),
            expected: 30,
        },
        Test {
            input: "3 * 3 * 3 + 10".into(),
            expected: 37,
        },
        Test {
            input: "3 * (3 * 3) + 10".into(),
            expected: 37,
        },
        Test {
            input: "(5 + 10 * 2 + 15 /3) * 2 + -10".into(),
            expected: 50,
        },
    ];

    for tt in tests {
        let evaluated = test_eval(tt.input)?;

        test_integer_object(evaluated, tt.expected)?;
    }

    Ok(())
}

fn test_eval(input: String) -> anyhow::Result<Object> {
    let lexer = Lexer::new(input.as_str())?;

    let mut parser = Parser::new(lexer)?;

    let program = parser.parse_program()?;

    Ok(eval(Box::new(program))?)
}

fn test_integer_object(obj: Object, expected: i64) -> anyhow::Result<bool> {
    match obj {
        Object::Integer(value) => {
            if value.value != expected {
                eprintln!(
                    "object has wrong value. got = {:?}, want = {:?}",
                    value.value, expected
                );
                return Ok(false);
            }

            Ok(true)
        }
        _ => unimplemented!()
    }

}

fn test_eval_boolean_expression() -> anyhow::Result<()> {
    struct Test {
        input: String,
        expected: bool,
    }

    let tests = vec![
        Test {
            input: "true".into(),
            expected: true,
        },
        Test {
            input: "false".into(),
            expected: false,
        },
        Test {
            input: "1 < 2".into(),
            expected: true,
        },
        Test {
            input: "1 > 2".into(),
            expected: false,
        },
        Test {
            input: "1 < 1".into(),
            expected: false,
        },
        Test {
            input: "1 > 1".into(),
            expected: false,
        },
        Test {
            input: "1 == 1".into(),
            expected: true,
        },
        Test {
            input: "1 != 1".into(),
            expected: false,
        },
        Test {
            input: "1 == 2".into(),
            expected: false,
        },
        Test {
            input: "1 != 2".into(),
            expected: true,
        },
        Test {
            input: "true == true".into(),
            expected: true,
        },
        Test {
            input: "false == false".into(),
            expected: true,
        },
        Test {
            input: "true == false".into(),
            expected: false,
        },
        Test {
            input: "true != false".into(),
            expected: true,
        },
        Test {
            input: "false != true".into(),
            expected: true,
        },
        Test {
            input: "(1 < 2) == true".into(),
            expected: true,
        },
        Test {
            input: "(1 < 2) == false".into(),
            expected: false,
        },
        Test {
            input: "(1 > 2) == true".into(),
            expected: false,
        },
        Test {
            input: "(1 > 2) == false".into(),
            expected: true,
        },
    ];

    for tt in tests.iter() {
        let evaluated = test_eval(tt.input.clone())?;

        test_boolean_object(evaluated, tt.expected)?;
    }

    Ok(())
}

fn test_boolean_object(obj: Object, expected: bool) -> anyhow::Result<bool> {
    match obj {
        Object::Boolean(value) => {
            if value.value != expected {
                eprintln!(
                    "object has wrong value. got = {:?}, want = {:?}",
                    value.value, expected
                );
                return Ok(false);
            }

            Ok(true)
        }
        _ => unimplemented!()
    }
}

fn test_bang_operator() -> anyhow::Result<()> {
    struct Test {
        input: String,
        expected: bool,
    }

    let tests = vec![
        Test {
            input: "!true".into(),
            expected: false,
        },
        Test {
            input: "!false".into(),
            expected: true,
        },
        Test {
            input: "!5".into(),
            expected: false,
        },
        Test {
            input: "!!true".into(),
            expected: true,
        },
        Test {
            input: "!!false".into(),
            expected: false,
        },
        Test {
            input: "!!5".into(),
            expected: true,
        },
    ];

    for tt in tests {
        let evaluated = test_eval(tt.input)?;

        test_boolean_object(evaluated, tt.expected)?;
    }

    Ok(())
}

fn test_if_else_expressions() -> anyhow::Result<()> {
    struct Test {
        input: String,
        expected: Box<dyn Interface>,
    }

    let tests = vec! {
        Test {
            input: "if (true) { 10 }".to_string(),
            expected: Box::new(10),
        },
        Test {
            input: "if (false) { 10 }".to_string(),
            expected: Box::new(()),
        },
        Test {
            input: "if (1) { 10 }".to_string(),
            expected: Box::new(10),
        },
        Test {
            input: "if (1 < 2) { 10 }".to_string(),
            expected: Box::new(10),
        },
        Test {
            input: "if (1 > 2) { 10 }".to_string(),
            expected: Box::new(()),
        },
        Test {
            input: "if (1 > 2) { 10 } else { 20 }".to_string(),
            expected: Box::new(20),
        },
        Test {
            input: "if (1 < 2) { 10 } else { 20 }".to_string(),
            expected: Box::new(10),
        },
    };

    for tt in tests.into_iter() {
        let evaluated = test_eval(tt.input)?;
        let t = tt.expected.as_any().type_id();

        if TypeId::of::<i64>() == t {
            let integer = tt.expected
                .as_any()
                .downcast_ref::<i64>()
                .ok_or(anyhow::anyhow!("tt.expected error"))?;

            let ret = test_integer_object(evaluated, integer.clone())?;
            if !ret {
                eprintln!("test integer object error")
            }
        } else if TypeId::of::<()>() == t {
            let ret = test_null_object(evaluated)?;
            if !ret  {
                eprintln!("test null object error");
            }
        }
    }

    Ok(())
}

fn test_null_object(obj: Object) -> anyhow::Result<bool> {
    let ret = obj.inspect();
    println!("parser object is {}", ret);
    Ok(true)
}

trait Interface {
    fn as_any(&self) -> &dyn Any;
}

impl Interface for i64 {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<i64> for Box<dyn Interface> {
    fn from(value: i64) -> Self {
        Box::new(value)
    }
}

impl Interface for bool {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<bool> for Box<dyn Interface> {
    fn from(value: bool) -> Self {
        Box::new(value)
    }
}

impl Interface for () {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<()> for Box<dyn Interface> {
    fn from(val: ()) -> Self {
        Box::new(val)
    }
}

#[test]
#[ignore]
fn test_test_eval_integer_expression() {
    let ret = test_eval_integer_expression();
    println!("test_eval_integer_expression : ret = {:?}", ret);
}

#[test]
#[ignore]
fn test_test_eval_boolean_expression() {
    let ret = test_eval_boolean_expression();
    println!("test_eval_boolean_expression : ret = {:?}", ret);
}

#[test]
#[ignore]
fn test_test_bang_operator() {
    let ret = test_bang_operator();
    println!("test_bang_operator : ret = {:?}", ret);
}

#[test]
fn test_test_if_else_expressions() {
    let ret = test_if_else_expressions();
    println!("test_if_else_expressions : ret = {:?}", ret);
}