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
#[ignore]
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
#[ignore]
fn test_test_bang_operator() {
    let ret = test_bang_operator();
    println!("test_bang_operator : ret = {:?}", ret);
}
