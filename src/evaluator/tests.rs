use crate::ast::Node;
use crate::lexer::lexer;
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

use std::collections::BTreeMap;

fn test_eval_integer_expression() -> anyhow::Result<()> {
    struct Test<'a> {
        input: &'a str,
        expected: isize,
    }

    let tests = vec![
        Test {
            input: "5",
            expected: 5,
        },
        Test {
            input: "10",
            expected: 10,
        },
        Test {
            input: "-5",
            expected: -5,
        },
        Test {
            input: "-10",
            expected: -10,
        },
        Test {
            input: "5 + 5 + 5 + 5 - 10",
            expected: 10,
        },
        Test {
            input: "2 * 2 * 2 * 2 * 2",
            expected: 32,
        },
        Test {
            input: "-50 + 100 + -50",
            expected: 0,
        },
        Test {
            input: "5 * 2 + 10",
            expected: 20,
        },
        Test {
            input: "5 + 2 * 10",
            expected: 25,
        },
        Test {
            input: "20 + 2 * -10",
            expected: 0,
        },
        Test {
            input: "50 / 2 * 2 + 10",
            expected: 60,
        },
        Test {
            input: "2 * (5 + 10)",
            expected: 30,
        },
        Test {
            input: "3 * 3 * 3 + 10",
            expected: 37,
        },
        Test {
            input: "3 * (3 * 3) + 10",
            expected: 37,
        },
        Test {
            input: "(5 + 10 * 2 + 15 /3) * 2 + -10",
            expected: 50,
        },
    ];

    for tt in tests {
        let evaluated = test_eval(tt.input)?;

        test_integer_object(evaluated, tt.expected)?;
    }

    Ok(())
}

fn test_eval(input: &str) -> anyhow::Result<Object> {
    let lexer = lexer(input).unwrap().1;

    let mut parser = Parser::new(lexer)?;

    let program = parser.parse_program()?;
    println!("[test_eval] program: {program:#?}");

    let mut env = Environment::new();

    let program_node: Node = program.into();
    program_node.eval(&mut env)
}

fn test_integer_object(obj: Object, expected: isize) -> anyhow::Result<bool> {
    let value = Integer::try_from(obj);
    match value {
        Ok(integer) => {
            if integer.value() != expected {
                eprintln!(
                    "object has wrong value. got = {:?}, want = {expected:?}",
                    integer.value()
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
    struct Test<'a> {
        input: &'a str,
        expected: bool,
    }

    let tests = vec![
        Test {
            input: "true",
            expected: true,
        },
        Test {
            input: "false",
            expected: false,
        },
        Test {
            input: "1 < 2",
            expected: true,
        },
        Test {
            input: "1 > 2",
            expected: false,
        },
        Test {
            input: "1 < 1",
            expected: false,
        },
        Test {
            input: "1 > 1",
            expected: false,
        },
        Test {
            input: "1 == 1",
            expected: true,
        },
        Test {
            input: "1 != 1",
            expected: false,
        },
        Test {
            input: "1 == 2",
            expected: false,
        },
        Test {
            input: "1 != 2",
            expected: true,
        },
        Test {
            input: "true == true",
            expected: true,
        },
        Test {
            input: "false == false",
            expected: true,
        },
        Test {
            input: "true == false",
            expected: false,
        },
        Test {
            input: "true != false",
            expected: true,
        },
        Test {
            input: "false != true",
            expected: true,
        },
        Test {
            input: "(1 < 2) == true",
            expected: true,
        },
        Test {
            input: "(1 < 2) == false",
            expected: false,
        },
        Test {
            input: "(1 > 2) == true",
            expected: false,
        },
        Test {
            input: "(1 > 2) == false",
            expected: true,
        },
    ];

    for tt in tests.iter() {
        let evaluated = test_eval(tt.input)?;

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
                    "object has wrong value. got = {:?}, want = {expected:?}",
                    boolean.value()
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
    struct Test<'a> {
        input: &'a str,
        expected: bool,
    }

    let tests = vec![
        Test {
            input: "!true",
            expected: false,
        },
        Test {
            input: "!false",
            expected: true,
        },
        Test {
            input: "!5",
            expected: false,
        },
        Test {
            input: "!!true",
            expected: true,
        },
        Test {
            input: "!!false",
            expected: false,
        },
        Test {
            input: "!!5",
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
    struct Test<'a> {
        input: &'a str,
        expected: Interface,
    }

    let tests = vec![
        Test {
            input: "if (true) { 10 }",
            expected: 10.into(),
        },
        Test {
            input: "if (false) { 10 }",
            expected: Null.into(),
        },
        Test {
            input: "if (1) { 10 }",
            expected: 10.into(),
        },
        Test {
            input: "if (1 < 2) { 10 }",
            expected: 10.into(),
        },
        Test {
            input: "if (1 > 2) { 10 }",
            expected: Null.into(),
        },
        Test {
            input: "if (1 > 2) { 10 } else { 20 }",
            expected: 20.into(),
        },
        Test {
            input: "if (1 < 2) { 10 } else { 20 }",
            expected: 10.into(),
        },
    ];

    for tt in tests.into_iter() {
        let evaluated = test_eval(tt.input)?;
        tt.expected.handler(evaluated)?;
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
    struct Test<'a> {
        input: &'a str,
        expected: isize,
    }

    let tests = vec![
        Test {
            input: "return 10;",
            expected: 10,
        },
        Test {
            input: "return 10; 9;",
            expected: 10,
        },
        Test {
            input: "return 2 * 5; 9;",
            expected: 10,
        },
        Test {
            input: "9; return 2 * 5; 9;",
            expected: 10,
        },
        Test {
            input: r#"
if (10 > 1) {
    if (10 > 1) {
        return 10;
    }
    return 1;
}"#,
            expected: 10,
        },
    ];

    for tt in tests.into_iter() {
        println!("test_return_statements = {tt:?}");
        let evaluated = test_eval(tt.input)?;

        let ret = test_integer_object(evaluated, tt.expected)?;
        if !ret {
            eprintln!("test return statement failed");
        }
    }

    Ok(())
}

fn test_error_handling() -> anyhow::Result<()> {
    struct Test<'a> {
        input: &'a str,
        expected_message: String,
    }

    let tests = vec![
        Test {
            input: "5 + true;",
            expected_message: "type mismatch: INTEGER + BOOLEAN".into(),
        },
        Test {
            input: "5 + true; 5;",
            expected_message: "type mismatch: INTEGER + BOOLEAN".into(),
        },
        Test {
            input: "-true",
            expected_message: "unknown operator: -BOOLEAN".into(),
        },
        Test {
            input: "true + false;",
            expected_message: "unknown operator: BOOLEAN + BOOLEAN".into(),
        },
        Test {
            input: "5; true + false; 5",
            expected_message: "unknown operator: BOOLEAN + BOOLEAN".into(),
        },
        Test {
            input: "if (10 > 1) { true + false; }",
            expected_message: "unknown operator: BOOLEAN + BOOLEAN".into(),
        },
        Test {
            input: r#"
if (10 > 1) {
    if (10 > 1) {
        return true + false;
    }

    return 1;
}
"#,
            expected_message: "unknown operator: BOOLEAN + BOOLEAN".into(),
        },
        Test {
            input: "foobar",
            expected_message: "identifier not found: foobar".into(),
        },
        Test {
            input: r#""Hello" - "World""#,
            expected_message: "unknown operator: STRING - STRING".into(),
        },
    ];

    for tt in tests {
        let evaluated = test_eval(tt.input);

        match evaluated {
            Ok(value) => {
                eprintln!("no error object returned. got = {value:?}");
                continue;
            }
            Err(err) => {
                if format!("{}", err) != tt.expected_message {
                    eprintln!(
                        "wrong error message. expected = {}, got = {err}",
                        tt.expected_message
                    )
                }
            }
        }
    }
    Ok(())
}

fn test_let_statements() -> anyhow::Result<()> {
    struct Test<'a> {
        input: &'a str,
        expected: isize,
    }

    let tests = vec![
        Test {
            input: "let a = 5; a;",
            expected: 5,
        },
        Test {
            input: "let a = 5 * 5; a;",
            expected: 25,
        },
        Test {
            input: "let a = 5; let b = a; b;",
            expected: 5,
        },
        Test {
            input: "let a = 5; let b = a; let c = a + b + 5; c;",
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

    let evaluated = test_eval(input)?;
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
        eprintln!("body is not {expected_body}. got = {}", value.body());
    }

    Ok(())
}

fn test_function_application() -> anyhow::Result<()> {
    struct Test<'a> {
        input: &'a str,
        expected: isize,
    }

    let tests = vec![
        Test {
            input: "let identity = fn(x) { x; }; identity(5);",
            expected: 5,
        },
        Test {
            input: "let identity = fn(x) { return x; }; identity(5);",
            expected: 5,
        },
        Test {
            input: "let double = fn(x) { return x * 2; }; double(5);",
            expected: 10,
        },
        Test {
            input: "let add = fn(x, y) { return x + y; }; add(5, 5);",
            expected: 10,
        },
        Test {
            input: "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));",
            expected: 20,
        },
        Test {
            input: "fn(x) { x; }(5)",
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
addTwo(2);"#;

    let ret = test_integer_object(test_eval(input)?, 4)?;
    if !ret {
        eprintln!("test integer object failed");
    }

    Ok(())
}

fn test_string_literal() -> anyhow::Result<()> {
    let input = r#""Hello World!""#;
    let evaluated = test_eval(input)?;
    let str_lit = StringObj::try_from(evaluated)?;
    println!("test string literal = {str_lit:?}");

    if str_lit.value() != "Hello World!" {
        eprintln!("String has wrong value. got = {}", str_lit.value());
    }
    Ok(())
}

fn test_string_concatenation() -> anyhow::Result<()> {
    let input = r#""Hello" + " " + "World!""#;

    let evaluated = test_eval(input)?;
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

    let evaluated = test_eval(input)?;
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

    let evaluated = test_eval(input)?;
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
    struct Test<'a> {
        input: &'a str,
        expected: Interface,
    }

    let tests = vec![
        Test {
            input: r#"len("")"#,
            expected: 0.into(),
        },
        Test {
            input: r#"len("four")"#,
            expected: 4.into(),
        },
        Test {
            input: r#"len("hello world")"#,
            expected: 11.into(),
        },
        Test {
            input: r#"len(1)"#,
            expected: "argument to `len` not supported, got INTEGER".into(),
        },
        Test {
            input: r#"len("one", "two")"#,
            expected: "wrong number of arguments. got=2, want=1"
                .to_string()
                .into(),
        },
    ];

    for tt in tests {
        let evaluated = test_eval(tt.input)?;
        tt.expected.handler(evaluated)?;
    }

    Ok(())
}

fn test_array_literals() -> anyhow::Result<()> {
    let input = "[1, 2 * 2, 3 + 3]";

    let evaluated = test_eval(input)?;
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
    struct Test<'a> {
        input: &'a str,
        expected: Interface,
    }

    let tests = vec![
        Test {
            input: "[1, 2, 3][0]",
            expected: 1.into(),
        },
        Test {
            input: "[1, 2, 3][1]",
            expected: 2.into(),
        },
        Test {
            input: "[1, 2, 3][2]",
            expected: 3.into(),
        },
        Test {
            input: "let i = 0; [1][i];",
            expected: 1.into(),
        },
        Test {
            input: "[1, 2, 3][1 + 1]",
            expected: 3.into(),
        },
        Test {
            input: "let myArray = [1, 2, 3]; myArray[2]",
            expected: 3.into(),
        },
        Test {
            input: "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2]",
            expected: 6.into(),
        },
        Test {
            input: "let myArray = [1, 2, 3]; let i =  myArray[0]; myArray[i]",
            expected: 2.into(),
        },
        Test {
            input: "[1, 2, 3][3]",
            expected: Null.into(),
        },
        Test {
            input: "[1, 2, 3][-1]",
            expected: Null.into(),
        },
    ];

    for tt in tests {
        let evaluated = test_eval(tt.input)?;
        tt.expected.handler(evaluated)?;
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

    let evaluated = test_eval(input)?;
    let result = Hash::try_from(evaluated)?;

    let mut expected = BTreeMap::<Object, isize>::new();
    expected.insert(Object::String("one".into()), 1);
    expected.insert(Object::String("two".into()), 2);
    expected.insert(Object::String("three".into()), 3);
    expected.insert(Object::Integer(4.into()), 4);
    expected.insert(Object::Boolean(true.into()), 5);
    expected.insert(Object::Boolean(false.into()), 6);

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
    struct Test<'a> {
        input: &'a str,
        expected: Interface,
    }

    let tests = vec![
        Test {
            input: r#"{"foo": 5}["foo"]"#,
            expected: 5.into(),
        },
        Test {
            input: r#"{"foo": 5}["bar"]"#,
            expected: Null.into(),
        },
        Test {
            input: r#"let key = "foo"; {"foo": 5}[key]"#,
            expected: 5.into(),
        },
        Test {
            input: r#"{}["foo"]"#,
            expected: Null.into(),
        },
        Test {
            input: r#"{5: 5}[5]"#,
            expected: 5.into(),
        },
        Test {
            input: r#"{true: 5}[true]"#,
            expected: 5.into(),
        },
        Test {
            input: r#"{false: 5}[false]"#,
            expected: 5.into(),
        },
    ];

    for tt in tests {
        let evaluated = test_eval(tt.input)?;
        tt.expected.handler(evaluated)?;
    }

    Ok(())
}

fn test_quote() -> anyhow::Result<()> {
    struct Test<'a> {
        input: &'a str,
        expected: String,
    }

    let tests = vec![
        Test {
            input: "quote(5)",
            expected: "5".into(),
        },
        Test {
            input: "quote(5 + 8)",
            expected: "(5 + 8)".into(),
        },
        Test {
            input: "quote(foobar)",
            expected: "foobar".into(),
        },
        Test {
            input: "quote(foobar + barfoo)",
            expected: "(foobar + barfoo)".into(),
        },
    ];

    for tt in tests {
        let evaluated = test_eval(tt.input)?;
        let quote = Quote::try_from(evaluated)?;
        println!("evaluated: {quote}");

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
    struct Test<'a> {
        input: &'a str,
        expected: String,
    }

    let tests = vec![
        Test {
            input: "quote(unquote(4))",
            expected: "4".into(),
        },
        Test {
            input: "quote(unquote(4 + 4))",
            expected: "8".into(),
        },
        Test {
            input: "quote(8 + unquote(4 + 4))",
            expected: "(8 + 8)".into(),
        },
        Test {
            input: "quote(unquote(4 + 4) + 8)",
            expected: "(8 + 8)".into(),
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
    Null(Null),
    String(String),
    StaticStr(&'static str),
}

impl From<isize> for Interface {
    fn from(value: isize) -> Self {
        Self::Isize(value)
    }
}

impl From<Null> for Interface {
    fn from(value: Null) -> Self {
        Self::Null(value)
    }
}

impl From<String> for Interface {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&'static str> for Interface {
    fn from(value: &'static str) -> Self {
        Self::StaticStr(value)
    }
}

impl Interface {
    pub fn handler(&self, evaluated: Object) -> anyhow::Result<()> {
        match self {
            Interface::Isize(integer) => {
                let ret = test_integer_object(evaluated, *integer)?;
                if !ret {
                    eprintln!("test integer object error")
                }
            }
            Interface::String(value) => {
                eprintln!("object is not Error. got = {value}");
            }
            Interface::StaticStr(value) => {
                eprintln!("object is not Error. got = {value}");
            }
            Interface::Null(_) => {
                let ret = test_null_object(evaluated)?;
                if !ret {
                    eprintln!("test Null object error")
                }
            }
        }
        Ok(())
    }
}

#[test]
fn test_test_eval_integer_expression() {
    let ret = test_eval_integer_expression();
    println!("test_eval_integer_expression : ret = {ret:?}");
}

#[test]
fn test_test_eval_boolean_expression() {
    let ret = test_eval_boolean_expression();
    println!("test_eval_boolean_expression : ret = {ret:?}");
}

#[test]
fn test_test_bang_operator() {
    let ret = test_bang_operator();
    println!("test_bang_operator : ret = {ret:?}");
}

#[test]
fn test_test_if_else_expressions() {
    let ret = test_if_else_expressions();
    println!("test_if_else_expressions : ret = {ret:?}");
}

#[test]
fn test_test_return_statements() {
    let ret = test_return_statements();
    println!("test_test_return_statements: ret = {ret:?}");
}

#[test]
fn test_test_error_handling() {
    let ret = test_error_handling();
    println!("test_error_handling: ret = {ret:?}");
}

#[test]
fn test_test_let_statements() {
    let ret = test_let_statements();
    println!("test_let_statements: ret = {ret:?}");
}

#[test]
fn test_test_function_object() {
    let ret = test_function_object();
    println!("test_function_object: ret = {ret:?}");
}

#[test]
fn test_test_function_application() {
    let ret = test_function_application();
    println!("test_function_application: ret = {ret:?}");
}

#[test]
fn test_test_closures() {
    let ret = test_closures();
    println!("test_closures : ret = {ret:?}");
}

#[test]
fn test_test_string_literal() {
    let ret = test_string_literal();
    println!("test_string_literal: ret = {ret:?}");
}

#[test]
fn test_test_string_concatenation() {
    let ret = test_string_concatenation();
    println!("test_string_concatenation: ret = {ret:?}");
}

#[test]
fn test_test_string_not_equal() {
    let ret = test_string_not_equal();
    println!("test_string_not_equal: ret = {ret:?}");
}

#[test]
fn test_test_string_equal() {
    let ret = test_string_equal();
    println!("test_string_equal: ret = {ret:?}");
}

#[test]
fn test_test_builtin_functions() {
    let ret = test_builtin_functions();
    println!("test_builtin_functions: ret = {ret:?}");
}

#[test]
fn test_test_array_literals() {
    let ret = test_array_literals();
    println!("test_array_literals: ret = {ret:?}");
}

#[test]
fn test_test_array_index_expressions() {
    let ret = test_array_index_expressions();
    println!("test_array_index_expressions: ret = {ret:?}");
}

#[test]
fn test_test_hash_literals() {
    let ret = test_hash_literals();
    println!("test_hash_literals: ret = {ret:?}");
}

#[test]
fn test_test_hash_index_expressions() {
    let ret = test_hash_index_expressions();
    println!("test_hash_index_expressions: ret = {ret:?}");
}

#[test]
fn test_test_quote() {
    let ret = test_quote();
    println!("test_quote: ret = {ret:?}");
}

#[test]
fn test_test_quote_unquote() {
    let ret = test_quote_unquote();
    println!("test_quote_unquote: ret = {ret:?}");
}
