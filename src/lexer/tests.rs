use crate::lexer::Lexer;
use crate::token::token_type::TokenType;
use crate::token::Token;

fn test_next_token() -> anyhow::Result<()> {
    // example 1
    // let input = "=+(){},;";
    let input = "let five = 5;
let ten = 10;
let add = fn(x, y) {
    x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if ( 5 < 10 ) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
\"foobar\"
\"foo bar\"
";

    let tests = vec![
        Token::from_string(TokenType::LET, "let".into()),
        Token::from_string(TokenType::IDENT, "five".into()),
        Token::from_string(TokenType::ASSIGN, "=".into()),
        Token::from_string(TokenType::INT, "5".into()),
        Token::from_string(TokenType::SEMICOLON, ";".into()),
        Token::from_string(TokenType::LET, "let".into()),
        Token::from_string(TokenType::IDENT, "ten".into()),
        Token::from_string(TokenType::ASSIGN, "=".into()),
        Token::from_string(TokenType::INT, "10".into()),
        Token::from_string(TokenType::SEMICOLON, ";".into()),
        Token::from_string(TokenType::LET, "let".into()),
        Token::from_string(TokenType::IDENT, "add".into()),
        Token::from_string(TokenType::ASSIGN, "=".into()),
        Token::from_string(TokenType::FUNCTION, "fn".into()),
        Token::from_string(TokenType::LPAREN, "(".into()),
        Token::from_string(TokenType::IDENT, "x".into()),
        Token::from_string(TokenType::COMMA, ",".into()),
        Token::from_string(TokenType::IDENT, "y".into()),
        Token::from_string(TokenType::RPAREN, ")".into()),
        Token::from_string(TokenType::LBRACE, "{".into()),
        Token::from_string(TokenType::IDENT, "x".into()),
        Token::from_string(TokenType::PLUS, "+".into()),
        Token::from_string(TokenType::IDENT, "y".into()),
        Token::from_string(TokenType::SEMICOLON, ";".into()),
        Token::from_string(TokenType::RBRACE, "}".into()),
        Token::from_string(TokenType::SEMICOLON, ";".into()),
        Token::from_string(TokenType::LET, "let".into()),
        Token::from_string(TokenType::IDENT, "result".into()),
        Token::from_string(TokenType::ASSIGN, "=".into()),
        Token::from_string(TokenType::IDENT, "add".into()),
        Token::from_string(TokenType::LPAREN, "(".into()),
        Token::from_string(TokenType::IDENT, "five".into()),
        Token::from_string(TokenType::COMMA, ",".into()),
        Token::from_string(TokenType::IDENT, "ten".into()),
        Token::from_string(TokenType::RPAREN, ")".into()),
        Token::from_string(TokenType::SEMICOLON, ";".into()),
        Token::from_string(TokenType::BANG, "!".into()),
        Token::from_string(TokenType::MINUS, "-".into()),
        Token::from_string(TokenType::SLASH, "/".into()),
        Token::from_string(TokenType::ASTERISK, "*".into()),
        Token::from_string(TokenType::INT, "5".into()),
        Token::from_string(TokenType::SEMICOLON, ";".into()),
        Token::from_string(TokenType::INT, "5".into()),
        Token::from_string(TokenType::LT, "<".into()),
        Token::from_string(TokenType::INT, "10".into()),
        Token::from_string(TokenType::GT, ">".into()),
        Token::from_string(TokenType::INT, "5".into()),
        Token::from_string(TokenType::SEMICOLON, ";".into()),
        Token::from_string(TokenType::IF, "if".into()),
        Token::from_string(TokenType::LPAREN, "(".into()),
        Token::from_string(TokenType::INT, "5".into()),
        Token::from_string(TokenType::LT, "<".into()),
        Token::from_string(TokenType::INT, "10".into()),
        Token::from_string(TokenType::RPAREN, ")".into()),
        Token::from_string(TokenType::LBRACE, "{".into()),
        Token::from_string(TokenType::RETURN, "return".into()),
        Token::from_string(TokenType::TRUE, "true".into()),
        Token::from_string(TokenType::SEMICOLON, ";".into()),
        Token::from_string(TokenType::RBRACE, "}".into()),
        Token::from_string(TokenType::ELSE, "else".into()),
        Token::from_string(TokenType::LBRACE, "{".into()),
        Token::from_string(TokenType::RETURN, "return".into()),
        Token::from_string(TokenType::FALSE, "false".into()),
        Token::from_string(TokenType::SEMICOLON, ";".into()),
        Token::from_string(TokenType::RBRACE, "}".into()),
        Token::from_string(TokenType::INT, "10".into()),
        Token::from_string(TokenType::EQ, "==".into()),
        Token::from_string(TokenType::INT, "10".into()),
        Token::from_string(TokenType::SEMICOLON, ";".into()),
        Token::from_string(TokenType::INT, "10".into()),
        Token::from_string(TokenType::NOTEQ, "!=".into()),
        Token::from_string(TokenType::INT, "9".into()),
        Token::from_string(TokenType::SEMICOLON, ";".into()),
        Token::from_string(TokenType::STRING, "foobar".into()),
        Token::from_string(TokenType::STRING, "foo bar".into()),
        Token::from_string(TokenType::EOF, "\0".into()),
    ];

    let mut l = Lexer::new(input)?;
    for (i, tt) in tests.iter().enumerate() {
        let tok = l.next_token()?;

        println!("token = {:?}", tok);

        if tok.r#type != tt.r#type {
            println!(
                "tests[{}] - token type wrong. expected = {:?}, \
                   got = {:?}
                ",
                i, tt.r#type, tok.r#type
            );
        }

        if tok.literal != tt.literal {
            println!(
                "tests[{}] - literal wrong. expected = {:?}, \
                got = {:?}
                ",
                i, tt.literal, tok.literal
            );
        }
    }

    Ok(())
}

#[test]
// #[ignore]
fn test_test_next_token() {
    let ret = test_next_token();
    println!("test_test_next_token: ret = {:?}", ret);
}
