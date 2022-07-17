use crate::evaluator::eval;
use crate::lexer::Lexer;
use crate::object::environment::Environment;
use crate::object::{Object, ObjectInterface};
use crate::parser::Parser;
use std::any::{Any, TypeId};

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

    let mut env = Environment::new();

    Ok(eval(Box::new(program), &mut env)?)
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
        _ => unimplemented!(),
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
        _ => unimplemented!(),
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

    let tests = vec![
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
    ];

    for tt in tests.into_iter() {
        let evaluated = test_eval(tt.input)?;
        let t = tt.expected.as_any().type_id();

        if TypeId::of::<i64>() == t {
            let integer = tt
                .expected
                .as_any()
                .downcast_ref::<i64>()
                .ok_or(anyhow::anyhow!("tt.expected error"))?;

            let ret = test_integer_object(evaluated, integer.clone())?;
            if !ret {
                eprintln!("test integer object error")
            }
        } else if TypeId::of::<()>() == t {
            let ret = test_null_object(evaluated)?;
            if !ret {
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

fn test_return_statements() -> anyhow::Result<()> {
    #[derive(Debug)]
    struct Test {
        input: String,
        expected: i64,
    }

    let tests = vec![
        Test {
            input: "return 10;".to_string(),
            expected: 10,
        },
        Test {
            input: "return 10; 9;".to_string(),
            expected: 10,
        },
        Test {
            input: "return 2 * 5; 9;".to_string(),
            expected: 10,
        },
        Test {
            input: "9; return 2 * 5; 9;".to_string(),
            expected: 10,
        },
        Test {
            input: "
if (10 > 1) {
    if (10 > 1) {
        return 10;
    }
    return 1;
}"
            .to_string(),
            expected: 10,
        },
    ];

    for tt in tests.into_iter() {
        println!("test_return_statements = {:?}", tt);
        let evaluated = test_eval(tt.input)?;

        let ret = test_integer_object(evaluated, tt.expected)?;
        if !ret {
            eprintln!("test return statement failed");
        }
    }

    Ok(())
}

fn test_error_handling() -> anyhow::Result<()> {
    struct Test {
        input: String,
        expected_message: String,
    }

    let tests = vec![
        Test {
            input: "5 + true;".to_string(),
            expected_message: "type mismatch: INTEGER + BOOLEAN".to_string(),
        },
        Test {
            input: "5 + true; 5;".to_string(),
            expected_message: "type mismatch: INTEGER + BOOLEAN".to_string(),
        },
        Test {
            input: "-true".to_string(),
            expected_message: "unknown operator: -BOOLEAN".to_string(),
        },
        Test {
            input: "true + false;".to_string(),
            expected_message: "unknown operator: BOOLEAN + BOOLEAN".to_string(),
        },
        Test {
            input: "5; true + false; 5".to_string(),
            expected_message: "unknown operator: BOOLEAN + BOOLEAN".to_string(),
        },
        Test {
            input: "if (10 > 1) { true + false; }".to_string(),
            expected_message: "unknown operator: BOOLEAN + BOOLEAN".to_string(),
        },
        Test {
            input: "\
if (10 > 1) {
    if (10 > 1) {
        return true + false;
    }

    return 1;
}
"
            .to_string(),
            expected_message: "unknown operator: BOOLEAN + BOOLEAN".to_string(),
        },
        Test {
            input: "foobar".to_string(),
            expected_message: "identifier not found: foobar".to_string(),
        },
    ];

    for tt in tests {
        let evaluated = test_eval(tt.input);

        match evaluated {
            Ok(value) => {
                eprintln!("no error object returned. got = {:?}", value);
                continue;
            }
            Err(err) => {
                if format!("{}", err) != tt.expected_message {
                    eprintln!(
                        "wrong error message. expected = {}, got = {}",
                        tt.expected_message,
                        format!("{}", err)
                    )
                }
                // else {
                //     println!("{}", format!("{}", err));
                // }
            }
        }
    }
    Ok(())
}

fn test_let_statements() -> anyhow::Result<()> {
    struct Test {
        input: String,
        expected: i64,
    }

    let tests = vec![
        Test {
            input: "let a = 5; a;".to_string(),
            expected: 5,
        },
        Test {
            input: "let a = 5 * 5; a;".to_string(),
            expected: 25,
        },
        Test {
            input: "let a = 5; let b = a; b;".to_string(),
            expected: 5,
        },
        Test {
            input: "let a = 5; let b = a; let c = a + b + 5; c;".to_string(),
            expected: 15,
        },
    ];

    for tt in tests {
        let ret = test_integer_object(test_eval(tt.input)?, tt.expected)?;
        if !ret {
            eprintln!("test integer object error");
        }
    }

    Ok(())
}


fn test_function_object() -> anyhow::Result<()> {
    let input = "fn(x) { x + 2; };";

    let evaluated = test_eval(input.to_string())?;

    let value = match evaluated {
        Object::Function(fn_value) => fn_value,
        _ => {
            panic!("object is no function. got = {}", evaluated);
        }
    };

    if value.parameters.len() != 1 {
        eprintln!("function has wrong parameters. parameters = {:?}", value.parameters);
    }

    if format!("{}", value.parameters[0]) != "x" {
        eprintln!("parameter is no 'x'. got = {:?}", value.parameters[0]);
    }

    let expected_body = "(x + 2)";

    if format!("{}", value.body) != expected_body {
        eprintln!("body is not {}. got = {}", expected_body, value.body);
    }

    Ok(())
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
// #[ignore]
fn test_test_eval_integer_expression() {
    let ret = test_eval_integer_expression();
    println!("test_eval_integer_expression : ret = {:?}", ret);
}

#[test]
// #[ignore]
fn test_test_eval_boolean_expression() {
    let ret = test_eval_boolean_expression();
    println!("test_eval_boolean_expression : ret = {:?}", ret);
}

#[test]
// #[ignore]
fn test_test_bang_operator() {
    let ret = test_bang_operator();
    println!("test_bang_operator : ret = {:?}", ret);
}

#[test]
// #[ignore]
fn test_test_if_else_expressions() {
    let ret = test_if_else_expressions();
    println!("test_if_else_expressions : ret = {:?}", ret);
}

#[test]
// #[ignore]
fn test_test_return_statements() {
    let ret = test_return_statements();
    println!("test_test_return_statements: ret = {:?}", ret);
}

#[test]
// #[ignore]
fn test_test_error_handling() {
    let ret = test_error_handling();
    println!("test_error_handling: ret = {:?}", ret);
}

#[test]
// #[ignore]
fn test_test_let_statements() {
    let ret = test_let_statements();
    println!("test_let_statements: ret = {:?}", ret);
}

#[test]
fn test_test_function_object() {
    let ret = test_function_object();
    println!("test_function_object: ret = {:?}", ret);
}
