use crate::evaluator::eval;
use crate::lexer::Lexer;
use crate::object::array::Array;
use crate::object::boolean::Boolean;
use crate::object::environment::Environment;
use crate::object::function::Function;
use crate::object::hash::Hash;
use crate::object::integer::Integer;
use crate::object::null::Null;
use crate::object::r#macro::quote::Quote;
use crate::object::string::StringObj;
use crate::object::Object;
use crate::parser::Parser;
use crate::{FALSE, NULL, TRUE};

use std::collections::BTreeMap;

fn test_eval_integer_expression() -> anyhow::Result<()> {
    struct Test {
        input: String,
        expected: isize,
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
    println!("[test_eval] program: {:#?}", program);

    let mut env = Environment::new();

    eval(program.into(), &mut env)
}

fn test_integer_object(obj: Object, expected: isize) -> anyhow::Result<bool> {
    let value = Integer::try_from(obj);
    match value {
        Ok(integer) => {
            if integer.value() != expected {
                eprintln!(
                    "object has wrong value. got = {:?}, want = {:?}",
                    integer.value(),
                    expected
                );
                Ok(false)
            } else {
                Ok(true)
            }
        }
        Err(err) => Err(err),
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
    let value = Boolean::try_from(obj);
    match value {
        Ok(boolean) => {
            if boolean.value() != expected {
                eprintln!(
                    "object has wrong value. got = {:?}, want = {:?}",
                    boolean.value(),
                    expected
                );
                Ok(false)
            } else {
                Ok(true)
            }
        }
        Err(err) => Err(err),
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
        expected: Interface,
    }

    let tests = vec![
        Test {
            input: "if (true) { 10 }".to_string(),
            expected: Interface::Isize(10),
        },
        Test {
            input: "if (false) { 10 }".to_string(),
            expected: Interface::Null(NULL),
        },
        Test {
            input: "if (1) { 10 }".to_string(),
            expected: Interface::Isize(10),
        },
        Test {
            input: "if (1 < 2) { 10 }".to_string(),
            expected: Interface::Isize(10),
        },
        Test {
            input: "if (1 > 2) { 10 }".to_string(),
            expected: Interface::Null(NULL),
        },
        Test {
            input: "if (1 > 2) { 10 } else { 20 }".to_string(),
            expected: Interface::Isize(20),
        },
        Test {
            input: "if (1 < 2) { 10 } else { 20 }".to_string(),
            expected: Interface::Isize(10),
        },
    ];

    for tt in tests.into_iter() {
        let evaluated = test_eval(tt.input)?;

        println!(
            "[test_test_if_else_expressions] evaluated = {:?}",
            evaluated
        );

        match tt.expected {
            Interface::Isize(integer) => {
                let ret = test_integer_object(evaluated, integer)?;
                if !ret {
                    eprintln!("test integer object error")
                }
            }
            Interface::Null(_) => {
                println!(
                    "[test_test_if_else_expressions] evaluated = {:?}",
                    evaluated
                );
                let ret = test_null_object(evaluated)?;
                if !ret {
                    eprintln!("test null object error");
                }
            }
            unknown => eprintln!("unspport {unknown:?}"),
        }
    }

    Ok(())
}

fn test_null_object(obj: Object) -> anyhow::Result<bool> {
    let value = Null::try_from(obj);
    if value.is_err() {
        Ok(false)
    } else {
        Ok(true)
    }
}

fn test_return_statements() -> anyhow::Result<()> {
    #[derive(Debug)]
    struct Test {
        input: String,
        expected: isize,
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
            input: r#"
if (10 > 1) {
    if (10 > 1) {
        return 10;
    }
    return 1;
}"#
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
            input: r#"
if (10 > 1) {
    if (10 > 1) {
        return true + false;
    }

    return 1;
}
"#
            .to_string(),
            expected_message: "unknown operator: BOOLEAN + BOOLEAN".to_string(),
        },
        Test {
            input: "foobar".to_string(),
            expected_message: "identifier not found: foobar".to_string(),
        },
        Test {
            input: r#""Hello" - "World""#.to_string(),
            expected_message: "unknown operator: STRING - STRING".to_string(),
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
                        format_args!("{}", err)
                    )
                }
            }
        }
    }
    Ok(())
}

fn test_let_statements() -> anyhow::Result<()> {
    struct Test {
        input: String,
        expected: isize,
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
    let value = Function::try_from(evaluated)?;

    if value.parameters().len() != 1 {
        eprintln!(
            "function has wrong parameters. parameters = {:?}",
            value.parameters()
        );
    }

    if format!("{}", value.parameters()[0]) != "x" {
        eprintln!("parameter is no 'x'. got = {:?}", value.parameters()[0]);
    }

    let expected_body = "(x + 2);";

    if format!("{}", value.body()) != expected_body {
        eprintln!("body is not {}. got = {}", expected_body, value.body());
    }

    Ok(())
}

fn test_function_application() -> anyhow::Result<()> {
    struct Test {
        input: String,
        expected: isize,
    }

    let tests = vec![
        Test {
            input: "let identity = fn(x) { x; }; identity(5);".to_string(),
            expected: 5,
        },
        Test {
            input: "let identity = fn(x) { return x; }; identity(5);".to_string(),
            expected: 5,
        },
        Test {
            input: "let double = fn(x) { return x * 2; }; double(5);".to_string(),
            expected: 10,
        },
        Test {
            input: "let add = fn(x, y) { return x + y; }; add(5, 5);".to_string(),
            expected: 10,
        },
        Test {
            input: "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));".to_string(),
            expected: 20,
        },
        Test {
            input: "fn(x) { x; }(5)".to_string(),
            expected: 5,
        },
    ];

    for tt in tests {
        let ret = test_integer_object(test_eval(tt.input)?, tt.expected)?;
        if !ret {
            eprintln!("test integer object failed");
        }
    }

    Ok(())
}

fn test_closures() -> anyhow::Result<()> {
    let input = r#"
let newAddr = fn(x) {
    fn(y) { x + y };
};
let addTwo = newAddr(2);
addTwo(2);"#
        .to_string();

    let ret = test_integer_object(test_eval(input)?, 4)?;
    if !ret {
        eprintln!("test integer object failed");
    }

    Ok(())
}

fn test_string_literal() -> anyhow::Result<()> {
    let input = r#""Hello World!""#;
    let evaluated = test_eval(input.to_string())?;
    let str_lit = StringObj::try_from(evaluated)?;
    println!("test string literal = {:?}", str_lit);

    if str_lit.value() != "Hello World!" {
        eprintln!("String has wrong value. got = {}", str_lit.value());
    }
    Ok(())
}

fn test_string_concatenation() -> anyhow::Result<()> {
    let input = r#""Hello" + " " + "World!""#;

    let evaluated = test_eval(input.to_string())?;
    let str_lit = StringObj::try_from(evaluated)?;

    if str_lit.value() != "Hello World!" {
        return Err(anyhow::anyhow!(format!(
            "String has wrong value. got = {}",
            str_lit.value()
        )));
    }

    Ok(())
}

fn test_string_not_equal() -> anyhow::Result<()> {
    let input = r#""Hello" != "World!""#;

    let evaluated = test_eval(input.to_string())?;
    let bool_str = Boolean::try_from(evaluated)?;

    if !bool_str.value() {
        return Err(anyhow::anyhow!(format!(
            "Boolean has wrong value. got = {}",
            bool_str.value()
        )));
    }

    Ok(())
}

fn test_string_equal() -> anyhow::Result<()> {
    let input = r#""Hello" == "Hello""#;

    let evaluated = test_eval(input.to_string())?;
    let bool_str = Boolean::try_from(evaluated)?;

    if !bool_str.value() {
        return Err(anyhow::anyhow!(format!(
            "Boolean has wrong value. got = {}",
            bool_str.value()
        )));
    }

    Ok(())
}

fn test_builtin_functions() -> anyhow::Result<()> {
    struct Test {
        input: String,
        expected: Interface,
    }

    let tests = vec![
        Test {
            input: r#"len("")"#.to_string(),
            expected: Interface::Isize(0),
        },
        Test {
            input: r#"len("four")"#.to_string(),
            expected: Interface::Isize(4),
        },
        Test {
            input: r#"len("hello world")"#.to_string(),
            expected: Interface::Isize(11),
        },
        Test {
            input: r#"len(1)"#.to_string(),
            expected: Interface::StaticStr("argument to `len` not supported, got INTEGER"),
        },
        Test {
            input: r#"len("one", "two")"#.to_string(),
            expected: Interface::String("wrong number of arguments. got=2, want=1".to_string()),
        },
    ];

    for tt in tests {
        let evaluated = test_eval(tt.input);
        println!("[test_builtin_functions] evaluated = {:?}", evaluated);

        match tt.expected {
            Interface::Isize(value) => {
                test_integer_object(evaluated?, value)?;
            }
            Interface::String(value) => {
                if let Err(error) = evaluated {
                    let error_obj_message = format!("{}", error);
                    if error_obj_message.as_str() != value.as_str() {
                        eprintln!(
                            "wrong error message. expected: {}, got = {}",
                            value, error_obj_message
                        );
                    }
                } else {
                    eprintln!("object is not Error. got = {}", evaluated?);
                }
            }
            Interface::StaticStr(value) => {
                if let Err(error) = evaluated {
                    if error.to_string() != value {
                        eprintln!("wrong error message. expected: {}, got = {}", value, error);
                    }
                } else {
                    eprintln!("object is not Error. got = {}", evaluated?);
                }
            }
            unknown => eprintln!("type({unknown:?}) of exp not handle."),
        }
    }

    Ok(())
}

fn test_array_literals() -> anyhow::Result<()> {
    let input = "[1, 2 * 2, 3 + 3]";

    let evaluated = test_eval(input.to_string())?;
    let result = Array::try_from(evaluated)?;

    if result.len() != 3 {
        eprintln!("array has wrong num of elements. got={}", result.len());
    }

    test_integer_object(result[0].clone(), 1)?;
    test_integer_object(result[1].clone(), 4)?;
    test_integer_object(result[2].clone(), 6)?;

    Ok(())
}

fn test_array_index_expressions() -> anyhow::Result<()> {
    struct Test {
        input: String,
        expected: Interface,
    }

    let tests = vec![
        Test {
            input: "[1, 2, 3][0]".to_string(),
            expected: Interface::Isize(1),
        },
        Test {
            input: "[1, 2, 3][1]".to_string(),
            expected: Interface::Isize(2),
        },
        Test {
            input: "[1, 2, 3][2]".to_string(),
            expected: Interface::Isize(3),
        },
        Test {
            input: "let i = 0; [1][i];".to_string(),
            expected: Interface::Isize(1),
        },
        Test {
            input: "[1, 2, 3][1 + 1]".to_string(),
            expected: Interface::Isize(3),
        },
        Test {
            input: "let myArray = [1, 2, 3]; myArray[2]".to_string(),
            expected: Interface::Isize(3),
        },
        Test {
            input: "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2]".to_string(),
            expected: Interface::Isize(6),
        },
        Test {
            input: "let myArray = [1, 2, 3]; let i =  myArray[0]; myArray[i]".to_string(),
            expected: Interface::Isize(2),
        },
        Test {
            input: "[1, 2, 3][3]".to_string(),
            expected: Interface::Null(NULL),
        },
        Test {
            input: "[1, 2, 3][-1]".to_string(),
            expected: Interface::Null(NULL),
        },
    ];

    for tt in tests {
        let evaluated = test_eval(tt.input)?;

        println!("[test_array_index_expressions] evaluated = {:?}", evaluated);

        match tt.expected {
            Interface::Isize(integer) => {
                let ret = test_integer_object(evaluated, integer)?;
                if !ret {
                    eprintln!("test integer object error")
                }
            }
            Interface::Null(_) => {
                let ret = test_null_object(evaluated)?;
                if !ret {
                    eprintln!("test Null object error")
                }
            }
            unknown => eprint!("unspport type({unknown:?})"),
        }
    }

    Ok(())
}

fn test_hash_literals() -> anyhow::Result<()> {
    let input = r#"
let two = "two";
{
    "one": 10 - 9,
    "two": 1 + 1,
    "thr" + "ee": 6 / 2,
    4: 4,
    true: 5,
    false: 6
}
"#;

    let evaluated = test_eval(input.to_string())?;
    let result = Hash::try_from(evaluated)?;

    let mut expected = BTreeMap::<Object, isize>::new();
    expected.insert(Object::String(StringObj::new("one".to_string())), 1);
    expected.insert(Object::String(StringObj::new("two".to_string())), 2);
    expected.insert(Object::String(StringObj::new("three".to_string())), 3);
    expected.insert(Object::Integer(Integer::new(4)), 4);
    expected.insert(Object::Boolean(*TRUE), 5);
    expected.insert(Object::Boolean(*FALSE), 6);

    if result.len() != expected.len() {
        eprintln!("hash has wrong num of paris. got={}", result.len());
    }

    for (expected_key, expected_value) in expected.iter() {
        let value = result.pairs().get(expected_key).unwrap();

        let ret = test_integer_object(value.clone(), *expected_value)?;
        if !ret {
            eprintln!("test integer object erorr");
        }
    }

    Ok(())
}

fn test_hash_index_expressions() -> anyhow::Result<()> {
    struct Test {
        input: String,
        expected: Interface,
    }

    let tests = vec![
        Test {
            input: r#"{"foo": 5}["foo"]"#.to_string(),
            expected: Interface::Isize(5),
        },
        Test {
            input: r#"{"foo": 5}["bar"]"#.to_string(),
            expected: Interface::Null(NULL),
        },
        Test {
            input: r#"let key = "foo"; {"foo": 5}[key]"#.to_string(),
            expected: Interface::Isize(5),
        },
        Test {
            input: r#"{}["foo"]"#.to_string(),
            expected: Interface::Null(NULL),
        },
        Test {
            input: r#"{5: 5}[5]"#.to_string(),
            expected: Interface::Isize(5),
        },
        Test {
            input: r#"{true: 5}[true]"#.to_string(),
            expected: Interface::Isize(5),
        },
        Test {
            input: r#"{false: 5}[false]"#.to_string(),
            expected: Interface::Isize(5),
        },
    ];

    for tt in tests {
        let evaluated = test_eval(tt.input)?;

        match tt.expected {
            Interface::Isize(integer) => {
                let ret = test_integer_object(evaluated, integer)?;
                if !ret {
                    eprintln!("test integer object error")
                }
            }
            Interface::Null(_) => {
                let ret = test_null_object(evaluated)?;
                if !ret {
                    eprintln!("test Null object error")
                }
            }
            unknown => eprint!("unsppport type({unknown:?})"),
        }
    }

    Ok(())
}

fn test_quote() -> anyhow::Result<()> {
    struct Test {
        input: String,
        expected: String,
    }

    let tests = vec![
        Test {
            input: "quote(5)".to_string(),
            expected: "5".to_string(),
        },
        Test {
            input: "quote(5 + 8)".to_string(),
            expected: "(5 + 8)".to_string(),
        },
        Test {
            input: "quote(foobar)".to_string(),
            expected: "foobar".to_string(),
        },
        Test {
            input: "quote(foobar + barfoo)".to_string(),
            expected: "(foobar + barfoo)".to_string(),
        },
    ];

    for tt in tests {
        let evaluated = test_eval(tt.input)?;
        let quote = Quote::try_from(evaluated)?;
        println!("evaluated: {}", quote);

        if format!("{}", quote.node()) == *"null" {
            eprintln!("quote.node is null");
        }

        if format!("{}", quote.node()) != tt.expected {
            eprintln!("not equal. got={}, want={}", quote.node(), tt.expected);
        }
    }

    Ok(())
}

fn test_quote_unquote() -> anyhow::Result<()> {
    struct Test {
        input: String,
        expected: String,
    }

    let tests = vec![
        Test {
            input: "quote(unquote(4))".to_string(),
            expected: "4".to_string(),
        },
        Test {
            input: "quote(unquote(4 + 4))".to_string(),
            expected: "8".to_string(),
        },
        Test {
            input: "quote(8 + unquote(4 + 4))".to_string(),
            expected: "(8 + 8)".to_string(),
        },
        Test {
            input: "quote(unquote(4 + 4) + 8)".to_string(),
            expected: "(8 + 8)".to_string(),
        },
    ];

    for tt in tests {
        let evaluated = test_eval(tt.input)?;
        let quote = Quote::try_from(evaluated)?;

        if format!("{}", quote.node()) == "null" {
            eprintln!("quote.node is null");
        }

        if format!("{}", quote.node()) != tt.expected {
            eprintln!("no equal. got={}, want={}", quote.node(), tt.expected);
        }
    }

    Ok(())
}

#[derive(Debug)]
pub enum Interface {
    Isize(isize),
    Bool(bool),
    Null(Null),
    String(String),
    StaticStr(&'static str),
}

#[test]
fn test_test_eval_integer_expression() {
    let ret = test_eval_integer_expression();
    println!("test_eval_integer_expression : ret = {:?}", ret);
}

#[test]
fn test_test_eval_boolean_expression() {
    let ret = test_eval_boolean_expression();
    println!("test_eval_boolean_expression : ret = {:?}", ret);
}

#[test]
fn test_test_bang_operator() {
    let ret = test_bang_operator();
    println!("test_bang_operator : ret = {:?}", ret);
}

#[test]
fn test_test_if_else_expressions() {
    let ret = test_if_else_expressions();
    println!("test_if_else_expressions : ret = {:?}", ret);
}

#[test]
fn test_test_return_statements() {
    let ret = test_return_statements();
    println!("test_test_return_statements: ret = {:?}", ret);
}

#[test]
fn test_test_error_handling() {
    let ret = test_error_handling();
    println!("test_error_handling: ret = {:?}", ret);
}

#[test]
fn test_test_let_statements() {
    let ret = test_let_statements();
    println!("test_let_statements: ret = {:?}", ret);
}

#[test]
fn test_test_function_object() {
    let ret = test_function_object();
    println!("test_function_object: ret = {:?}", ret);
}

#[test]
fn test_test_function_application() {
    let ret = test_function_application();
    println!("test_function_application: ret = {:?}", ret);
}

#[test]
fn test_test_closures() {
    let ret = test_closures();
    println!("test_closures : ret = {:?}", ret);
}

#[test]
fn test_test_string_literal() {
    let ret = test_string_literal();
    println!("test_string_literal: ret = {:?}", ret);
}

#[test]
fn test_test_string_concatenation() {
    let ret = test_string_concatenation();
    println!("test_string_concatenation: ret = {:?}", ret);
}

#[test]
fn test_test_string_not_equal() {
    let ret = test_string_not_equal();
    println!("test_string_not_equal: ret = {:?}", ret);
}

#[test]
fn test_test_string_equal() {
    let ret = test_string_equal();
    println!("test_string_equal: ret = {:?}", ret);
}

#[test]
fn test_test_builtin_functions() {
    let ret = test_builtin_functions();
    println!("test_builtin_functions: ret = {:?}", ret);
}

#[test]
fn test_test_array_literals() {
    let ret = test_array_literals();
    println!("test_array_literals: ret = {:?}", ret);
}

#[test]
fn test_test_array_index_expressions() {
    let ret = test_array_index_expressions();
    println!("test_array_index_expressions: ret = {:?}", ret);
}

#[test]
fn test_test_hash_literals() {
    let ret = test_hash_literals();
    println!("test_hash_literals: ret = {:?}", ret);
}

#[test]
fn test_test_hash_index_expressions() {
    let ret = test_hash_index_expressions();
    println!("test_hash_index_expressions: ret = {:?}", ret);
}

#[test]
fn test_test_quote() {
    let ret = test_quote();
    println!("test_quote: ret = {:?}", ret);
}

#[test]
fn test_test_quote_unquote() {
    let ret = test_quote_unquote();
    println!("test_quote_unquote: ret = {:?}", ret);
}
