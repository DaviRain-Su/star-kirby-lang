use crate::evaluator::eval;
use crate::lexer::Lexer;
use crate::object::boolean::Boolean;
use crate::object::integer::Integer;
use crate::object::Object;
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
    ];

    for tt in tests {
        let evaluated = test_eval(tt.input)?;

        test_integer_object(evaluated, tt.expected)?;
    }

    Ok(())
}

fn test_eval(input: String) -> anyhow::Result<Box<dyn Object>> {
    let lexer = Lexer::new(input.as_str())?;

    let mut parser = Parser::new(lexer)?;

    let program = parser.parse_program()?;

    Ok(eval(Box::new(program))?)
}

fn test_integer_object(obj: Box<dyn Object>, expected: i64) -> anyhow::Result<bool> {
    let result = obj
        .as_any()
        .downcast_ref::<Integer>()
        .ok_or(anyhow::anyhow!("object is not Integer. got = None"))?;
    println!("[test_integer_object] integer = {:#?}", result);

    if result.value != expected {
        eprintln!(
            "object has wrong value. got = {:?}, want = {:?}",
            result.value, expected
        );
        return Ok(false);
    }

    Ok(true)
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
    ];

    for tt in tests.iter() {
        let evaluated = test_eval(tt.input.clone())?;

        test_boolean_object(evaluated, tt.expected)?;
    }

    Ok(())
}

fn test_boolean_object(obj: Box<dyn Object>, expected: bool) -> anyhow::Result<bool> {
    let result = obj
        .as_any()
        .downcast_ref::<Boolean>()
        .ok_or(anyhow::anyhow!("object is not Integer. got = None"))?;

    if result.value != expected {
        eprintln!(
            "object has wrong value. got = {:?}, want = {:?}",
            result.value, expected
        );
        return Ok(false);
    }

    Ok(true)
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

#[test]
// #[ignore]
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
