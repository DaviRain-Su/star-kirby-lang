use crate::token::token_type::TokenType;
use crate::token::{token_type, Token};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_till;
use nom::sequence::terminated;
use nom::{character::complete::*, combinator::*, multi::*, IResult};
use std::str;

#[cfg(test)]
mod tests;

fn parse_identifier(input: &str) -> IResult<&str, Token> {
    let (input, ident) = alphanumeric1(input)?;
    let token_type = token_type::lookup_ident(ident);
    Ok((input, Token::from_string(token_type, ident.to_string())))
}

fn parse_double_char_operators(input: &str) -> IResult<&str, Token> {
    alt((
        map(tag("=="), |_| {
            Token::from_string(TokenType::EQ, "==".to_string())
        }),
        map(tag("!="), |_| {
            Token::from_string(TokenType::NOTEQ, "!=".to_string())
        }),
    ))(input)
}

// 解析单字符运算符和其他单字符语法元素
fn parse_single_char_tokens(input: &str) -> IResult<&str, Token> {
    alt((
        map(tag("+"), |_| {
            Token::from_char(token_type::lookup_char('+'), '+')
        }),
        map(tag("-"), |_| {
            Token::from_char(token_type::lookup_char('-'), '-')
        }),
        map(tag("/"), |_| {
            Token::from_char(token_type::lookup_char('/'), '/')
        }),
        map(tag("*"), |_| {
            Token::from_char(token_type::lookup_char('*'), '*')
        }),
        map(tag("<"), |_| {
            Token::from_char(token_type::lookup_char('<'), '<')
        }),
        map(tag(">"), |_| {
            Token::from_char(token_type::lookup_char('>'), '>')
        }),
        map(tag("["), |_| {
            Token::from_char(token_type::lookup_char('['), '[')
        }),
        map(tag("]"), |_| {
            Token::from_char(token_type::lookup_char(']'), ']')
        }),
        map(tag(":"), |_| {
            Token::from_char(token_type::lookup_char(':'), ':')
        }),
        map(tag("="), |_| {
            Token::from_char(token_type::lookup_char('='), '=')
        }),
        map(tag("!"), |_| {
            Token::from_char(token_type::lookup_char('!'), '!')
        }),
        map(tag(";"), |_| {
            Token::from_char(token_type::lookup_char(';'), ';')
        }),
        map(tag("("), |_| {
            Token::from_char(token_type::lookup_char('('), '(')
        }),
        map(tag(")"), |_| {
            Token::from_char(token_type::lookup_char(')'), ')')
        }),
        map(tag(","), |_| {
            Token::from_char(token_type::lookup_char(','), ',')
        }),
        map(tag("{"), |_| {
            Token::from_char(token_type::lookup_char('{'), '{')
        }),
        map(tag("}"), |_| {
            Token::from_char(token_type::lookup_char('}'), '}')
        }),
    ))(input)
}

fn parse_string(input: &str) -> IResult<&str, Token> {
    let (input, _) = char('"')(input)?;
    let (input, str_contents) = take_till(|c| c == '"')(input)?;
    let (input, _) = char('"')(input)?;
    Ok((
        input,
        Token::from_string(TokenType::STRING, str_contents.to_string()),
    ))
}

fn parse_number(input: &str) -> IResult<&str, Token> {
    let (input, num) = digit1(input)?;
    Ok((input, Token::from_string(TokenType::INT, num.to_string())))
}

fn parse_whitespace(input: &str) -> IResult<&str, ()> {
    let (input, _) = multispace0(input)?;
    Ok((input, ()))
}

fn parse_token(input: &str) -> IResult<&str, Token> {
    alt((
        parse_double_char_operators,
        parse_single_char_tokens,
        parse_number,
        parse_string,
        parse_identifier,
    ))(input)
}

pub fn lexer(input: &str) -> IResult<&str, Vec<Token>> {
    let (input, _) = parse_whitespace(input)?;
    many0(terminated(parse_token, parse_whitespace))(input)
}
