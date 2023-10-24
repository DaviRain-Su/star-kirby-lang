use crate::ast::expression::array::ArrayLiteral;
use crate::ast::expression::boolean::Boolean;
use crate::ast::expression::call::Call;
use crate::ast::expression::function::FunctionLiteral;
use crate::ast::expression::hash::HashLiteral;
use crate::ast::expression::if_expression::If;
use crate::ast::expression::index::Index;
use crate::ast::expression::infix::Infix;
use crate::ast::expression::integer::IntegerLiteral;
use crate::ast::expression::prefix::Prefix;
use crate::ast::expression::string::StringLiteral;
use crate::ast::expression::Expression;
use crate::ast::statement::expression::ExpressionStatement;
use crate::ast::statement::let_statement::LetStatement;
use crate::ast::statement::return_statement::ReturnStatement;
use crate::ast::statement::Statement;
use crate::ast::Identifier;
use crate::ast::NodeInterface;
use crate::lexer::lexer;
use crate::object::hash::Hash;
use crate::object::string::StringObj;
use crate::object::Object;
use crate::parser::Parser;
use std::collections::{BTreeMap, HashMap};

fn test_let_statements() -> anyhow::Result<()> {
    struct LetStatementTest<'a> {
        input: &'a str,
        expected_identifier: &'a str,
        expected_value: Interface,
    }

    let tests = vec![
        LetStatementTest {
            input: "let x = 5;",
            expected_identifier: "x",
            expected_value: 5.into(),
        },
        LetStatementTest {
            input: "let y = true;",
            expected_identifier: "y",
            expected_value: true.into(),
        },
        LetStatementTest {
            input: "let foobar = y;",
            expected_identifier: "foobar",
            expected_value: "y".to_string().into(),
        },
    ];

    for tt in tests.iter() {
        let lexer = lexer(tt.input)?.1;

        let mut parser = Parser::new(lexer)?;

        let program = parser.parse_program()?;

        if program.statements.len() != 1 {
            eprintln!(
                "program statements does not contain 1 statements. got = {}",
                program.statements.len()
            );
        }

        let stmt = program.statements.get(0).unwrap();

        if !test_let_statement(stmt, tt.expected_identifier) {
            eprintln!("test let statement error");
        }

        let val = LetStatement::try_from(stmt).unwrap().value().clone();

        if !tt.expected_value.test_literal_expression(val.clone())? {
            eprintln!("test literal expression error");
        }
    }

    Ok(())
}

fn test_let_statement(s: &Statement, name: &str) -> bool {
    if s.token_literal() != "let" {
        eprint!(
            "Statement token_literal not 'let'. got = {}",
            s.token_literal()
        );
        return false;
    }

    // HOW TODO this convert from box to concept type
    let let_stmt = LetStatement::try_from(s).unwrap();

    if let_stmt.name().value != name {
        eprint!(
            "let_stmt.name.value not `{name}`. got = {}",
            let_stmt.name().value
        );
        return false;
    }

    if let_stmt.name().token_literal() != name {
        eprint!(
            "let_stmt.name.token_literal() not `{name}`. got = {}",
            let_stmt.name().token_literal()
        );
        return false;
    }

    true
}
fn test_return_statements() -> anyhow::Result<()> {
    struct Test<'a> {
        input: &'a str,
        expected_value: Interface,
    }
    let tests = vec![
        Test {
            input: "return 5;",
            expected_value: 5.into(),
        },
        Test {
            input: "return true;",
            expected_value: true.into(),
        },
        Test {
            input: "return foobar;",
            expected_value: "foobar".to_string().into(),
        },
    ];

    for tt in tests {
        let lexer = lexer(tt.input)?.1;
        let mut parser = Parser::new(lexer)?;

        let program = parser.parse_program()?;

        let stmt = program.statements.get(0).unwrap();
        let return_stmt = ReturnStatement::try_from(stmt.clone()).unwrap();

        if return_stmt.token_literal() != "return" {
            eprintln!(
                "return statement not 'return', got = {}",
                return_stmt.token_literal()
            );
        }

        if !tt
            .expected_value
            .test_literal_expression(*return_stmt.return_value.clone())?
        {
            eprintln!("test_literal_expression error");
        }
    }

    Ok(())
}

fn test_identifier_expression() -> anyhow::Result<()> {
    let input = "foobar;";

    let lexer = lexer(input)?.1;

    let mut parser = Parser::new(lexer)?;

    let program = parser.parse_program()?;

    println!("program: {program}");

    if program.statements.len() != 1 {
        eprintln!(
            "program has not enough statements. got = {}",
            program.statements.len()
        );
    }

    let stmt: Option<Result<ExpressionStatement, anyhow::Error>> =
        program.statements.get(0).map(|value| value.try_into());

    println!("expression statement: {stmt:?}");

    if stmt.is_none() {
        eprintln!("program statement[0] is None");
    }

    let identifier: Identifier = Identifier::try_from(stmt.unwrap().unwrap().expression)?;

    if identifier.value != "foobar" {
        eprintln!("ident.value not foobar. got = {}", identifier.value);
    }

    if identifier.token_literal() != "foobar" {
        eprintln!(
            "ident.token_literal not foobar. got = {}",
            identifier.token_literal()
        );
    }

    Ok(())
}

fn test_integer_literal_expression() -> anyhow::Result<()> {
    let input = "5;";

    let lexer = lexer(input)?.1;

    let mut parser = Parser::new(lexer)?;

    let program = parser.parse_program()?;

    println!("program: {program}");

    if program.statements.len() != 1 {
        eprintln!(
            "program has not enough statements. got = {}",
            program.statements.len()
        );
    }

    let stmt: Option<Result<ExpressionStatement, anyhow::Error>> =
        program.statements.get(0).map(|value| value.try_into());

    println!("expression statement: {stmt:?}");

    if stmt.is_none() {
        eprintln!("program statement[0] is None");
    }

    let literal = IntegerLiteral::try_from(stmt.unwrap().unwrap()).unwrap();

    if literal.value() != 5 {
        eprintln!("ident.value not foobar. got = {}", literal.value());
    }

    if literal.token_literal() != "5" {
        eprintln!(
            "ident.token_literal not foobar. got = {}",
            literal.token_literal()
        );
    }

    Ok(())
}

fn test_parsing_prefix_expression() -> anyhow::Result<()> {
    struct PrefixTest<'a> {
        input: &'a str,
        operator: &'a str,
        integer_value: Interface,
    }

    impl<'a> PrefixTest<'a> {
        fn new(input: &'a str, operator: &'a str, integer_value: Interface) -> Self {
            Self {
                input,
                operator,
                integer_value,
            }
        }
    }

    let prefix_tests = vec![
        PrefixTest::new("!5;", "!", 5.into()),
        PrefixTest::new("-15;", "-", 15.into()),
        // PrefixTest::new("!foobar;", "!", 15),
        // PrefixTest::new("-foobar;", "-", 15),
        PrefixTest::new("!true;", "!", true.into()),
        PrefixTest::new("!false;", "!", false.into()),
    ];

    for tt in prefix_tests.iter() {
        let lexer = lexer(tt.input)?.1;
        let mut parser = Parser::new(lexer)?;
        let program = parser.parse_program()?;

        println!("Program = {program}");

        let program_statements_len = program.statements.len();
        if program_statements_len != 1 {
            eprintln!(
                "program statements does not contain {} statements. got = {}",
                1, program_statements_len
            );
        }

        let stmt: Option<Result<ExpressionStatement, anyhow::Error>> =
            program.statements.get(0).map(|value| value.try_into());
        if stmt.is_none() {
            eprintln!("program statements[0] is not expression statement. got = {stmt:?}");
        }

        let exp = Prefix::try_from(stmt.unwrap().unwrap())?;

        println!("PrefixExpression = {}", exp);

        if exp.operator() != tt.operator {
            eprintln!(
                "exp.operator is no '{}'. got = {}",
                tt.operator,
                exp.operator()
            );
        }

        let ret = tt.integer_value.test_literal_expression(exp.into())?;

        if !ret {
            eprintln!("test_integer_literal error!");
        }
    }

    Ok(())
}

fn test_parsing_infix_expression() -> anyhow::Result<()> {
    struct InfixTest<'a> {
        input: &'a str,
        left_value: Interface,
        operator: &'a str,
        right_value: Interface,
    }

    impl<'a> InfixTest<'a> {
        fn new(
            input: &'a str,
            left_value: Interface,
            operator: &'a str,
            right_value: Interface,
        ) -> Self {
            Self {
                input,
                left_value,
                operator,
                right_value,
            }
        }
    }

    let infix_tests = vec![
        InfixTest::new("5 + 5;", 5.into(), "+", 5.into()),
        InfixTest::new("5 - 5;", 5.into(), "-", 5.into()),
        InfixTest::new("5 * 5;", 5.into(), "*", 5.into()),
        InfixTest::new("5 / 5;", 5.into(), "/", 5.into()),
        InfixTest::new("5 > 5;", 5.into(), ">", 5.into()),
        InfixTest::new("5 < 5;", 5.into(), "<", 5.into()),
        InfixTest::new("5 == 5;", 5.into(), "==", 5.into()),
        InfixTest::new("5 != 5;", 5.into(), "!=", 5.into()),
        InfixTest::new("foobar + barfoo;", "foobar".into(), "+", "barfoo".into()),
        InfixTest::new("foobar - barfoo;", "foobar".into(), "-", "barfoo".into()),
        InfixTest::new("foobar * barfoo;", "foobar".into(), "*", "barfoo".into()),
        InfixTest::new("foobar / barfoo;", "foobar".into(), "/", "barfoo".into()),
        InfixTest::new("foobar < barfoo;", "foobar".into(), "<", "barfoo".into()),
        InfixTest::new("foobar > barfoo;", "foobar".into(), ">", "barfoo".into()),
        InfixTest::new("foobar == barfoo;", "foobar".into(), "==", "barfoo".into()),
        InfixTest::new("foobar != barfoo;", "foobar".into(), "!=", "barfoo".into()),
        InfixTest::new("true == true", true.into(), "==", true.into()),
        InfixTest::new("true != false", true.into(), "!=", false.into()),
        InfixTest::new("false == false", false.into(), "==", false.into()),
    ];

    for tt in infix_tests.iter() {
        let lexer = lexer(tt.input)?.1;

        let mut parser = Parser::new(lexer)?;

        let program = parser.parse_program()?;

        println!("program: {program}");

        if program.statements.len() != 1 {
            eprintln!(
                "program statements does not contain {} statemtns. got = {}",
                1,
                program.statements.len()
            );
        }

        let stmt: Option<Result<ExpressionStatement, anyhow::Error>> =
            program.statements.get(0).map(|value| value.try_into());

        if stmt.is_none() {
            eprintln!("program statements[0] is not ExpressionStatement. got = None");
        }

        if !test_infix_expression(
            &stmt.unwrap().unwrap().expression,
            tt.left_value.clone(),
            tt.operator,
            tt.right_value.clone(),
        )? {
            return Err(anyhow::anyhow!("test_infix_expression error"));
        }
    }
    Ok(())
}

fn test_operator_precedence_parsing() -> anyhow::Result<()> {
    struct TempTest<'a> {
        input: &'a str,
        expected: String,
    }

    let tests = vec![
        TempTest {
            input: "-a * b",
            expected: "((-a) * b)".into(),
        },
        TempTest {
            input: "!-a",
            expected: "(!(-a))".into(),
        },
        TempTest {
            input: "a + b + c",
            expected: "((a + b) + c)".into(),
        },
        TempTest {
            input: "a * b * c",
            expected: "((a * b) * c)".into(),
        },
        TempTest {
            input: "a * b / c",
            expected: "((a * b) / c)".into(),
        },
        TempTest {
            input: "a + b / c",
            expected: "(a + (b / c))".into(),
        },
        TempTest {
            input: "a + b * c + d / e - f",
            expected: "(((a + (b * c)) + (d / e)) - f)".into(),
        },
        TempTest {
            input: "3 + 4; -5 * 5",
            expected: "(3 + 4)((-5) * 5)".into(),
        },
        TempTest {
            input: "5 > 4 == 3 < 4",
            expected: "((5 > 4) == (3 < 4))".into(),
        },
        TempTest {
            input: "3 + 4 * 5 == 3 * 1 + 4 * 5",
            expected: "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))".into(),
        },
        TempTest {
            input: "true",
            expected: "true".into(),
        },
        TempTest {
            input: "3 < 5 == false",
            expected: "((3 < 5) == false)".into(),
        },
        TempTest {
            input: "false",
            expected: "false".into(),
        },
        TempTest {
            input: "3 > 5 == false",
            expected: "((3 > 5) == false)".into(),
        },
        TempTest {
            input: "1 + (2 + 3) + 4",
            expected: "((1 + (2 + 3)) + 4)".into(),
        },
        TempTest {
            input: "(5 + 5) * 2",
            expected: "((5 + 5) * 2)".into(),
        },
        TempTest {
            input: "2 / ( 5 + 5)",
            expected: "(2 / (5 + 5))".into(),
        },
        TempTest {
            input: "-(5 + 5)",
            expected: "(-(5 + 5))".into(),
        },
        TempTest {
            input: "!(true == true)",
            expected: "(!(true == true))".into(),
        },
        TempTest {
            input: "a + add(b * c) + d",
            expected: "((a + add((b * c))) + d)".into(),
        },
        TempTest {
            input: "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
            expected: "add(a,b,1,(2 * 3),(4 + 5),add(6,(7 * 8)))".into(),
        },
        TempTest {
            input: "add(a + b + c * d / f + g)",
            expected: "add((((a + b) + ((c * d) / f)) + g))".into(),
        },
        TempTest {
            input: "a * [1, 2, 3, 4][b * c] * d",
            expected: "((a * ([1,2,3,4][(b * c)])) * d)".into(),
        },
        TempTest {
            input: "add(a * b[2], b[1], 2 * [1, 2][1])",
            expected: "add((a * (b[2])),(b[1]),(2 * ([1,2][1])))".into(),
        },
    ];

    for tt in tests.into_iter() {
        let lexer = lexer(tt.input)?.1;
        let mut parser = Parser::new(lexer)?;
        let program = parser.parse_program()?;

        if format!("{}", program) != tt.expected {
            eprintln!(
                "expected = {}, got = {}",
                tt.expected,
                format_args!("{}", program)
            );
        }
    }

    Ok(())
}

fn test_integer_literal(il: Expression, value: isize) -> anyhow::Result<bool> {
    let integ = IntegerLiteral::try_from(il)?;
    if integ.value() != value {
        eprintln!("integ value not {value}. got = {}", integ.value());
        return Ok(false);
    }

    if integ.token_literal() != format!("{}", value) {
        eprintln!(
            "integ token_literal not {value}. got = {}",
            integ.token_literal()
        );
        return Ok(false);
    }

    Ok(true)
}

fn test_identifier(exp: Expression, value: &str) -> anyhow::Result<bool> {
    let ident = Identifier::try_from(exp)?;

    if ident.value != value {
        eprintln!("identifier value not {value}. got = {}", ident.value);
        return Ok(false);
    }

    if ident.token_literal() != value {
        eprintln!(
            "identifier token_literal not {value}. got = {}",
            ident.token_literal()
        );
        return Ok(false);
    }
    Ok(true)
}

fn test_boolean_literal(exp: Expression, value: bool) -> anyhow::Result<bool> {
    let boolean = Boolean::try_from(exp)?;

    if boolean.value() != value {
        eprintln!("boolean value not {value}. got = {}", boolean.value());
        return Ok(false);
    }

    if boolean.token_literal() != format!("{}", value) {
        eprintln!(
            "boolean token_literal not {value}. got = {}",
            boolean.token_literal()
        );
        return Ok(false);
    }
    Ok(true)
}

#[derive(Debug, Clone)]
enum Interface {
    Isize(isize),
    String(String),
    StaticStr(&'static str),
    Bool(bool),
}

impl From<isize> for Interface {
    fn from(value: isize) -> Self {
        Self::Isize(value)
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

impl From<bool> for Interface {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl Interface {
    pub fn test_literal_expression(&self, exp: Expression) -> anyhow::Result<bool> {
        match self {
            Interface::Isize(value) => test_integer_literal(exp, *value),
            Interface::String(value) => test_identifier(exp, value),
            Interface::StaticStr(value) => test_identifier(exp, value),
            Interface::Bool(value) => test_boolean_literal(exp, *value),
        }
    }
}

fn test_infix_expression(
    exp: &Expression,
    left: Interface,
    operator: &str,
    right: Interface,
) -> anyhow::Result<bool> {
    let op_exp = Infix::try_from(exp)?;

    if !left.test_literal_expression(op_exp.left().clone())? {
        return Ok(false);
    }

    if op_exp.operator() != operator {
        eprintln!(
            "exp.operator is not '{operator}'. got = {}",
            op_exp.operator()
        );
        return Ok(false);
    }

    if !right.test_literal_expression(op_exp.right().clone())? {
        return Ok(false);
    }

    Ok(true)
}

fn test_if_expression() -> anyhow::Result<()> {
    let input = "if (x < y) { x }";

    let lexer = lexer(input)?.1;
    let mut parser = Parser::new(lexer)?;

    let program = parser.parse_program()?;

    if program.statements.len() != 1 {
        eprintln!(
            "program statements does not contain {} statements. got = {}",
            1,
            program.statements.len()
        );
    }

    let stmt = program.statements.get(0).map(ExpressionStatement::try_from);

    if stmt.is_none() {
        eprintln!("program statements[0] is not ExpressionStatement. got = None");
    }

    let exp = If::try_from(stmt.unwrap().unwrap().expression)?;
    println!("IfExpression Debug is = {exp:?}",);
    println!("IfExpression Display is = {exp}");

    if !test_infix_expression(exp.condition(), "x".into(), "<", "y".into())? {
        eprintln!("test_infix_expression error");
    }

    if exp.consequence().is_none() {
        eprintln!(
            "exp consequence statements was not nil. got = {:?}",
            exp.consequence()
        );
    }

    if exp.consequence().as_ref().unwrap().statements_len() != 1 {
        eprintln!(
            "consequence is not 1 statements. got = {}",
            exp.consequence().as_ref().unwrap().statements_len()
        );
    }

    let consequence = exp
        .consequence()
        .clone()
        .unwrap()
        .statements()
        .get(0)
        .map(ExpressionStatement::try_from);

    if consequence.is_none() {
        eprintln!("statements[0] is not ExpressionStatement. got = None");
    }

    if !test_identifier(consequence.unwrap().unwrap().expression.clone(), "x")? {
        eprintln!("test identifier error");
    }

    if exp.alternative().is_some() {
        eprintln!(
            "exp alternative statements was not nil. got = {:?}",
            exp.alternative()
        );
    }

    Ok(())
}

fn test_if_else_expression() -> anyhow::Result<()> {
    let input = "if (x < y) { x } else { y }";

    let lexer = lexer(input)?.1;
    let mut parser = Parser::new(lexer)?;

    let program = parser.parse_program()?;

    if program.statements.len() != 1 {
        eprintln!(
            "program statements does not contain {} statements. got = {}",
            1,
            program.statements.len()
        );
    }

    let stmt = program.statements.get(0).map(ExpressionStatement::try_from);

    if stmt.is_none() {
        eprintln!("program statements[0] is not ExpressionStatement. got = None");
    }

    let exp = If::try_from(stmt.unwrap().unwrap().expression)?;

    if !test_infix_expression(exp.condition(), "x".into(), "<", "y".into())? {
        eprintln!("test infix expression error");
    }

    if exp.consequence().is_none() {
        eprintln!(
            "exp consequence statements was not nil. got = {:?}",
            exp.consequence()
        );
    }

    if exp.consequence().as_ref().unwrap().statements_len() != 1 {
        eprintln!(
            "consequence is not 1 statements. got = {}",
            exp.consequence().as_ref().unwrap().statements_len()
        );
    }

    let alternative = exp
        .alternative()
        .clone()
        .unwrap()
        .statements()
        .get(0)
        .map(ExpressionStatement::try_from);

    if alternative.is_none() {
        eprintln!("statements[0] is not ExpressionStatement. got = None");
    }

    if !test_identifier(alternative.unwrap().unwrap().expression.clone(), "y")? {
        eprintln!("test identifier error");
    }

    Ok(())
}

fn test_function_literal_parsing() -> anyhow::Result<()> {
    let input = "fn(x, y) { x + y; }";

    let lexer = lexer(input)?.1;

    let mut parser = Parser::new(lexer)?;

    let program = parser.parse_program()?;

    if program.statements.len() != 1 {
        eprintln!(
            "program statements does not contain {} statments. got = {}",
            1,
            program.statements.len()
        );
    }

    let stmt = program.statements.get(0).map(ExpressionStatement::try_from);
    if stmt.is_none() {
        eprintln!("program statements[0] is not  expression statement. got = None");
    }

    let function = FunctionLiteral::try_from(stmt.unwrap().unwrap().expression)?;

    if function.parameters().len() != 2 {
        eprintln!(
            "function literals parameters wrong. want 2, got = {}",
            function.parameters().len()
        );
    }

    let x_interface = Interface::from("x");
    let y_interface = Interface::from("y");

    x_interface
        .test_literal_expression(function.parameters()[0].clone().into())
        .expect("test literals expression error");
    y_interface
        .test_literal_expression(function.parameters()[1].clone().into())
        .expect("test literals expression error");

    if function.body().statements_len() != 1 {
        eprintln!(
            "function body statements wrong. want 1, got = {}",
            function.body().statements_len()
        );
    }

    let body_stmt = function
        .body()
        .statements()
        .get(0)
        .map(ExpressionStatement::try_from);
    if body_stmt.is_none() {
        eprintln!("function body stmt is not ExpressionStatement. got = None");
    }

    test_infix_expression(
        &body_stmt.unwrap().unwrap().expression,
        "x".into(),
        "+",
        "y".into(),
    )
    .expect("test infix expression error");

    Ok(())
}

fn test_function_parameter_parsing() -> anyhow::Result<()> {
    struct Test<'a> {
        input: &'a str,
        expected_params: Vec<&'a str>,
    }

    let tests = vec![
        Test {
            input: "fn() {};",
            expected_params: vec![],
        },
        Test {
            input: "fn(x) {};",
            expected_params: vec!["x"],
        },
        Test {
            input: "fn(x, y, z) {};",
            expected_params: vec!["x", "y", "z"],
        },
    ];

    for tt in tests.into_iter() {
        let lexer = lexer(tt.input)?.1;
        let mut parser = Parser::new(lexer)?;

        let program = parser.parse_program()?;

        let stmt = program.statements.get(0).map(ExpressionStatement::try_from);
        let function = FunctionLiteral::try_from(stmt.unwrap().unwrap().expression)?;

        if function.parameters().len() != tt.expected_params.len() {
            eprintln!(
                "length parameters wrong. want {}. got = {}",
                tt.expected_params.len(),
                function.parameters().len()
            );
        }

        for (i, ident) in tt.expected_params.into_iter().enumerate() {
            let ident_interface = Interface::from(ident);
            ident_interface.test_literal_expression(function.parameters()[i].clone().into())?;
        }
    }
    Ok(())
}

fn test_call_expression_parsing() -> anyhow::Result<()> {
    let input = "add(1, 2*3, 4 + 5);";
    let lexer = lexer(input)?.1;
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;

    if program.statements.len() != 1 {
        eprintln!(
            "program statements does not contain 1 statement. got = {}",
            program.statements.len()
        );
    }

    let stmt = program.statements.get(0).map(ExpressionStatement::try_from);

    if stmt.is_none() {
        eprintln!("stmt is not ExpressionStatement. got = None");
    }

    let exp = Call::try_from(stmt.unwrap().unwrap().expression)?;

    if !test_identifier(exp.function().clone(), "add")? {
        eprintln!("test identifier error");
    }

    if exp.arguments().len() != 3 {
        eprint!("wrong length of arguments. got = {}", exp.arguments().len());
    }

    let one_interface = Interface::from(1);
    one_interface.test_literal_expression(exp.arguments()[0].clone())?;
    test_infix_expression(&exp.arguments()[1].clone(), 2.into(), "*", 3.into())?;
    test_infix_expression(&exp.arguments()[2].clone(), 4.into(), "+", 5.into())?;

    Ok(())
}

fn test_call_expression_parameter_parsing() -> anyhow::Result<()> {
    struct Test<'a> {
        input: &'a str,
        expected_ident: &'a str,
        expected_args: Vec<&'a str>,
    }

    let tests = vec![
        Test {
            input: "add();",
            expected_ident: "add",
            expected_args: vec![],
        },
        Test {
            input: "add(1);",
            expected_ident: "add",
            expected_args: vec!["1"],
        },
        Test {
            input: "add(1, 2 * 3, 4 + 5);",
            expected_ident: "add",
            expected_args: vec!["1", "(2 * 3)", "(4 + 5)"],
        },
    ];

    for tt in tests {
        let lexer = lexer(tt.input)?.1;
        let mut parser = Parser::new(lexer)?;
        let program = parser.parse_program()?;

        let stmt = program.statements.get(0).map(ExpressionStatement::try_from);
        let exp = Call::try_from(stmt.unwrap().unwrap().expression)?;

        if !test_identifier(exp.function().clone(), tt.expected_ident)? {
            eprintln!("test identifier error");
        }

        if exp.arguments().len() != tt.expected_args.len() {
            eprintln!(
                "wrong number of arguments. want = {}, got = {}",
                tt.expected_args.len(),
                exp.arguments().len()
            );
        }

        for (i, arg) in tt.expected_args.into_iter().enumerate() {
            if exp.arguments()[i].to_string() != arg {
                eprintln!(
                    "arguments {i} wrong. want = {arg}, got = {}",
                    exp.arguments()[i]
                );
            }
        }
    }

    Ok(())
}

fn test_string_literal_expression() -> anyhow::Result<()> {
    let input = r#""hello world""#;

    let lexer = lexer(input)?.1;

    let mut parser = Parser::new(lexer)?;

    let program = parser.parse_program()?;

    let stmt = program.statements.get(0).map(ExpressionStatement::try_from);

    let literal = StringLiteral::try_from(stmt.unwrap().unwrap().expression)?;

    if literal.value() != "hello world" {
        eprintln!("literal.value not hello world. got = {}", literal.value());
    }

    Ok(())
}

fn test_parsing_array_literals() -> anyhow::Result<()> {
    let input = "[1, 2 * 2, 3 + 3]";

    let lexer = lexer(input)?.1;
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;

    let stmt = program.statements.get(0).map(ExpressionStatement::try_from);

    let array = ArrayLiteral::try_from(stmt.unwrap().unwrap().expression)?;

    if array.elements().len() != 3 {
        eprintln!("len(array.elements) not 3. got={}", array.elements().len());
    }

    test_integer_literal(array.elements()[0].clone(), 1)?;
    test_infix_expression(&array.elements()[1], 2.into(), "*", 2.into())?;
    test_infix_expression(&array.elements()[2], 3.into(), "+", 3.into())?;

    Ok(())
}

fn test_parsing_index_expression() -> anyhow::Result<()> {
    let input = "myArray[1 + 1]";
    let lexer = lexer(input)?.1;
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;
    println!("test_test_parsing_index_expression: program = {program:#?}");

    let stmt = program.statements.get(0).map(ExpressionStatement::try_from);

    println!("test_test_parsing_index_expression: Stmt = {stmt:#?}");
    let index_exp = Index::try_from(stmt.unwrap().unwrap().expression)?;

    if !test_identifier(index_exp.left().clone(), "myArray")? {
        eprintln!("test identifier error");
    }

    if !test_infix_expression(index_exp.index(), 1.into(), "+", 1.into())? {
        eprintln!("test infix expression error");
    }

    Ok(())
}

fn test_parsing_hash_literals_string_keys() -> anyhow::Result<()> {
    let input = r#"{"one": 1, "two": 2, "three": 3}"#;

    let lexer = lexer(input)?.1;
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;
    let stmt = program.statements.get(0).map(ExpressionStatement::try_from);

    let hash = HashLiteral::try_from(stmt.unwrap().unwrap().expression)?;

    if hash.pair().len() != 3 {
        eprintln!("hash.Pair hash wrong length. got={}", hash.pair().len());
    }

    let mut expected = HashMap::new();
    expected.insert("one", 1isize);
    expected.insert("two", 2);
    expected.insert("three", 3);

    for (key, value) in hash.pair() {
        let literal = StringLiteral::try_from(key.clone())?;

        let expected_value = expected.get(literal.value()).unwrap();

        let ret = test_integer_literal(value.clone(), *expected_value)?;
        if !ret {
            eprintln!("test_integer_literal error");
        }
    }
    Ok(())
}

fn test_parsing_empty_hash_literal() -> anyhow::Result<()> {
    let input = "{}";
    let lexer = lexer(input)?.1;
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;
    let stmt = program.statements.get(0).map(ExpressionStatement::try_from);
    let hash = HashLiteral::try_from(stmt.unwrap().unwrap().expression)?;

    if !hash.pair().is_empty() {
        eprintln!("hash.Pairs hash wrong length. got={}", hash.pair().len());
    }

    Ok(())
}

fn test_parsing_hash_literals_with_expressions() -> anyhow::Result<()> {
    let input = r#"{"one": 0 + 1, "two": 10 - 8, "three": 15 / 5}"#;

    let lexer = lexer(input)?.1;
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;
    let stmt = program.statements.get(0).map(ExpressionStatement::try_from);

    let hash = HashLiteral::try_from(stmt.unwrap().unwrap().expression)?;

    if hash.pair().len() != 3 {
        eprintln!("hash.Pair hash wrong length. got={}", hash.pair().len());
    }

    trait FuncCall {
        fn func_call(&self, e: Expression) -> anyhow::Result<()>;
    }

    struct A;

    impl FuncCall for A {
        fn func_call(&self, e: Expression) -> anyhow::Result<()> {
            let ret = test_infix_expression(&e, 0.into(), "+", 1.into())?;
            if !ret {
                eprintln!("test_infix_expression error")
            }
            Ok(())
        }
    }

    struct B;

    impl FuncCall for B {
        fn func_call(&self, e: Expression) -> anyhow::Result<()> {
            let ret = test_infix_expression(&e, 10.into(), "-", 8.into())?;
            if !ret {
                eprintln!("test_infix_expression error")
            }
            Ok(())
        }
    }

    struct C;

    impl FuncCall for C {
        fn func_call(&self, e: Expression) -> anyhow::Result<()> {
            let ret = test_infix_expression(&e, 15.into(), "/", 5.into())?;
            if !ret {
                eprintln!("test_infix_expression error")
            }
            Ok(())
        }
    }

    let mut expected = BTreeMap::<&str, Box<dyn FuncCall>>::new();
    expected.insert("one", Box::new(A));
    expected.insert("two", Box::new(B));
    expected.insert("three", Box::new(C));

    for (key, value) in hash.pair() {
        let literal = StringLiteral::try_from(key.clone())?;
        let test_func = expected.get(literal.value());
        if test_func.is_none() {
            eprintln!("Not test function for key {} found.", literal);
        }

        (test_func.unwrap()).func_call(value.clone())?;
    }
    Ok(())
}

fn test_hash_map_use() {
    let name1 = StringObj::new("name".to_string());

    let monkey = StringObj::new("Monkey".to_string());

    let mut pairs = BTreeMap::<Object, Object>::new();
    pairs.insert(Object::String(name1.clone()), Object::String(monkey));

    let hash_map = Hash::new(pairs.clone());

    println!("hash_map = {pairs:?}");
    println!("hash_map = {hash_map}");

    println!("pairs[name1] = {:?}", pairs.get(&Object::String(name1)));

    let name2 = StringObj::new("name".to_string());

    println!("pairs[name2] = {:?}", pairs.get(&Object::String(name2)));
}

#[test]
fn test_test_let_statements() {
    let ret = test_let_statements();
    println!("test_test_let_statements : Ret = {ret:?}");
}

#[test]
fn test_test_return_statements() {
    let ret = test_return_statements();
    println!("test_test_return_statements : Ret = {ret:?}");
}

#[test]
fn test_test_identifier_expression() {
    let ret = test_identifier_expression();
    println!("test_test_identifier_expression: Ret = {:?}", ret);
}

#[test]
fn test_test_integer_literal_expression() {
    let ret = test_integer_literal_expression();
    println!("test_test_integer_literal_expression : Ret = {ret:?}");
}

#[test]
fn test_test_parsing_prefix_expression() {
    let ret = test_parsing_prefix_expression();
    println!("test_test_parsing_prefix_expression : Ret = {ret:?}");
}

#[test]
fn test_test_parsing_infix_expression() {
    let ret = test_parsing_infix_expression();
    println!("test_parsing_infix_expression: Ret = {ret:?}");
}

#[test]
fn test_test_operator_precedence_parsing() {
    let ret = test_operator_precedence_parsing();
    println!("test_operator_precedence_parsing: Ret = {ret:?}");
}

#[test]
fn test_test_if_expression() {
    let ret = test_if_expression();
    println!("test_if_expression: Ret = {ret:?}");
}

#[test]
fn test_test_if_else_expression() {
    let ret = test_if_else_expression();
    println!("test_if_else_expression: Ret = {ret:?}");
}

#[test]
fn test_test_function_literal_parsing() {
    let ret = test_function_literal_parsing();
    println!("test_function_literal_parsing: ret = {ret:?}");
}

#[test]
fn test_test_function_parameter_parsing() {
    let ret = test_function_parameter_parsing();
    println!("test_function_parameter_parsing: ret = {ret:?}");
}

#[test]
fn test_test_call_expression_parsing() {
    let ret = test_call_expression_parsing();
    println!("test_call_expression_parsing ret = {ret:?}");
}

#[test]
fn test_test_call_expression_parameter_parsing() {
    let ret = test_call_expression_parameter_parsing();
    println!("test_call_expression_parameter_parsing. Ret = {ret:?}");
}

#[test]
fn test_test_string_literal_expression() {
    let ret = test_string_literal_expression();
    println!("test_string_literal_expression: ret = {ret:?}")
}

#[test]
fn test_test_parsing_array_literals() {
    let ret = test_parsing_array_literals();
    println!("test_parsing_array_literals : Ret = {ret:?}");
}

#[test]
fn test_test_parsing_index_expression() {
    let ret = test_parsing_index_expression();
    println!("test_parsing_index_expression: ret = {ret:?}");
}

#[test]
fn test_test_parsing_hash_literals_string_keys() {
    let ret = test_parsing_hash_literals_string_keys();
    println!("test_parsing_hash_literals_string_keys : Ret = {ret:?}");
}

#[test]
fn test_test_parsing_empty_hash_literal() {
    let ret = test_parsing_empty_hash_literal();
    println!("test_parsing_empty_hash_literal: Ret = {ret:?}");
}

#[test]
fn test_test_parsing_hash_literals_with_expressions() {
    let ret = test_parsing_hash_literals_with_expressions();
    println!("test_parsing_hash_literals_with_expressions : Ret  = {ret:?}");
}

#[test]
fn test_test_hash_map_use() {
    test_hash_map_use();
}
