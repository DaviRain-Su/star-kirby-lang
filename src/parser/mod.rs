pub mod operator_priority;
pub mod parser_tracing;
#[cfg(test)]
mod tests;

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
use crate::ast::statement::block::BlockStatement;
use crate::ast::statement::expression::ExpressionStatement;
use crate::ast::statement::let_statement::LetStatement;
use crate::ast::statement::return_statement::ReturnStatement;
use crate::ast::statement::Statement;
use crate::ast::{Identifier, Program};
use crate::error::Error;
use crate::parser::operator_priority::OperatorPriority;
use crate::parser::operator_priority::OperatorPriority::{LOWEST, PREFIX};
use crate::token::token_type::TokenType;
use crate::token::token_type::TokenType::{COLON, COMMA, RBRACE, RBRACKET};
use crate::token::Token;
use std::collections::HashMap;

/// 前缀解析函数
/// 前缀运算符左侧为空。
/// 在前缀位置遇到关联的词法单元类型时会调用 prefixParseFn
type PrefixParseFn<'a> = fn(&mut Parser<'a>) -> anyhow::Result<Expression>;

/// 中缀解析函数
/// infixParseFn 接受另一个 ast.Expression 作为参数。该参数是所解析的中缀运算符
/// 左侧的内容。
/// 在中缀位置遇到词法单元类型时会调用 infixParseFn
type InferParseFn<'a> = fn(&mut Parser<'a>, Expression) -> anyhow::Result<Expression>;

#[derive(Clone)]
pub struct Parser<'a> {
    /// lexer 是指向词法分析器实例的指针，在该实例上重复调用NextToken()能不断获取输入中的下一个词法单元
    lexer: Vec<Token>,
    /// curToken和 peekToken 的行为与词法分析器中的两个“指针”position 和 readPosition 完全
    /// 相同，但它们分别指向输入中的当前词法单元和下一个词法单元，而不是输入中的字
    /// 符。查看 curToken（当前正在检查的词法单元）是为了决定下
    /// 一步该怎么做，如果 curToken 没有提供足够的信息，还需要根据 peekToken 来做决
    /// 策。
    current_token: Token,
    peek_token: Token,
    current_position: usize, // 添加一个字段来追踪当前位置
    prefix_parse_fns: HashMap<TokenType, PrefixParseFn<'a>>,
    infix_parse_fns: HashMap<TokenType, InferParseFn<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Vec<Token>) -> anyhow::Result<Self> {
        let mut lexer = lexer;
        lexer.push(Token::new(TokenType::EOF, '\0')); // 在末尾添加EOF标记，这里假设 '\0' 表示EOF

        let mut parser = Parser {
            lexer,
            current_token: Token::default(),
            peek_token: Token::default(),
            current_position: 0,
            prefix_parse_fns: HashMap::default(),
            infix_parse_fns: HashMap::default(),
        };

        parser.register_prefix(TokenType::IDENT, Self::parse_identifier);
        parser.register_prefix(TokenType::INT, Self::parser_integer_literal);
        parser.register_prefix(TokenType::BANG, Self::parse_prefix_expression);
        parser.register_prefix(TokenType::MINUS, Self::parse_prefix_expression);

        parser.register_prefix(TokenType::TRUE, Self::parse_boolean);
        parser.register_prefix(TokenType::FALSE, Self::parse_boolean);
        parser.register_prefix(TokenType::LPAREN, Self::parse_grouped_expression);
        parser.register_prefix(TokenType::IF, Self::parse_if_expression);
        parser.register_prefix(TokenType::FUNCTION, Self::parse_function_literal);
        parser.register_prefix(TokenType::STRING, Self::parse_string);
        parser.register_prefix(TokenType::LBRACKET, Self::parse_array_literal);
        parser.register_prefix(TokenType::LBRACE, Self::parse_hash_literal);

        parser.register_infix(TokenType::PLUS, Self::parse_infix_expression);
        parser.register_infix(TokenType::MINUS, Self::parse_infix_expression);
        parser.register_infix(TokenType::SLASH, Self::parse_infix_expression);
        parser.register_infix(TokenType::ASTERISK, Self::parse_infix_expression);
        parser.register_infix(TokenType::EQ, Self::parse_infix_expression);
        parser.register_infix(TokenType::NOTEQ, Self::parse_infix_expression);
        parser.register_infix(TokenType::LT, Self::parse_infix_expression);
        parser.register_infix(TokenType::GT, Self::parse_infix_expression);
        parser.register_infix(TokenType::LPAREN, Self::parser_call_expression);
        parser.register_infix(TokenType::LBRACKET, Self::parse_index_expression);

        // 读取两个词法单元，以设置 curToken 和 peekToken
        parser.next_token()?;
        parser.next_token()?;

        Ok(parser)
    }

    // TODO 因为使用 PrefixParseFn 和InferParseFn 的原因，其中的第一个参数是parser
    #[tracing::instrument(name = "parse_program", skip(self, parse), level = "debug")]
    fn update_parser(&mut self, parse: Parser<'a>) {
        self.lexer = parse.lexer;
        self.current_token = parse.current_token;
        self.peek_token = parse.peek_token;
        self.prefix_parse_fns = parse.prefix_parse_fns;
        self.infix_parse_fns = parse.infix_parse_fns;
    }

    #[tracing::instrument(name = "parse_program", skip(self), level = "debug")]
    fn next_token(&mut self) -> anyhow::Result<()> {
        self.current_token = self.peek_token.clone();
        if self.current_position < self.lexer.len() {
            self.peek_token = self.lexer[self.current_position].clone();
            self.current_position += 1;
        } else {
            self.peek_token = Token::new(TokenType::EOF, '\0'); // 返回一个EOF标记，而不是错误
        }
        Ok(())
    }

    #[tracing::instrument(name = "parse_identifier", skip(self), level = "debug")]
    pub fn parse_program(&mut self) -> anyhow::Result<Program> {
        tracing::trace!("[parse_program] current_token = {:?}", self.current_token);
        let mut program = Program::new();

        // Now fix this to EOF
        while !self.cur_token_is(TokenType::EOF) {
            let stmt = self.parse_statement()?;
            program.statements.push(stmt);
            self.next_token()?;
        }

        Ok(program)
    }

    #[tracing::instrument(name = "parse_statement", skip(self), level = "debug")]
    fn parse_statement(&mut self) -> anyhow::Result<Statement> {
        tracing::trace!("[parse_statement] current_token = {:?}", self.current_token);
        match self.current_token.token_type() {
            TokenType::LET => Ok(self.parse_let_statement()?.into()),
            TokenType::RETURN => Ok(self.parse_return_statement()?.into()),
            _ => {
                // default parse expression statement
                Ok(self.parse_expression_statement()?.into())
            }
        }
    }

    /// 先来看 parseLetStatement。这里使用当前所在的词法单元（token.LET）构造
    /// 了一个*ast.LetStatement 节点，然后调用 expectPeek 来判断下一个是不是期望的
    /// 词法单元，如果是，则前移词法单元指针。第一次期望的是一个 token.IDENT 词法单
    /// 元，用于构造一个*ast.Identifier 节点。然后期望下一个词法单元是等号。之后跳
    /// 过了一些表达式，直到遇见分号为止。目前的代码跳过了表达式的处理，后面介绍完
    /// 如何解析表达式后会返回来替换这里的代码。
    ///
    /// # 解析let 语句
    #[tracing::instrument(name = "parse_let_statement", skip(self), level = "debug")]
    fn parse_let_statement(&mut self) -> anyhow::Result<LetStatement> {
        tracing::trace!(
            "[parse_let_statement] current_token = {:?}",
            self.current_token
        );
        let mut stmt = LetStatement::new(self.current_token.clone());
        tracing::trace!("[parse_let_statement] stmt = {stmt}");
        if self.expect_peek(TokenType::IDENT).is_err() {
            return Err(Error::CannotFindTokenType { ty: "IDENT".into() }.into());
        }
        *stmt.name_mut() = Identifier::new(
            self.current_token.clone(),
            self.current_token.literal().into(),
        );
        tracing::trace!("[parse_let_statement] stmt = {stmt}");
        if self.expect_peek(TokenType::ASSIGN).is_err() {
            return Err(Error::CannotFindTokenType {
                ty: "ASSIGN".into(),
            }
            .into());
        }
        self.next_token()?;
        *stmt.value_mut() = self.parse_expression(LOWEST)?;
        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token()?;
        }
        tracing::trace!("stmt = {stmt}");
        Ok(stmt)
    }

    /// 解析return 语句
    #[tracing::instrument(name = "parse_return_statement", skip(self), level = "debug")]
    fn parse_return_statement(&mut self) -> anyhow::Result<ReturnStatement> {
        tracing::trace!(
            "[parse_return_statement] current_token = {:?}",
            self.current_token
        );
        let mut stmt = ReturnStatement::new(self.current_token.clone());
        self.next_token()?;
        // add equal expression
        *stmt.return_value_mut() = self.parse_expression(LOWEST)?.into();
        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token()?;
        }
        Ok(stmt)
    }

    /// 解析表达式语句
    /// 这是因为表达式语句不是真正的语句，而是仅由表达式构成的语句，相当于一层封装
    #[tracing::instrument(name = "parse_expression_statement", skip(self), level = "debug")]
    fn parse_expression_statement(&mut self) -> anyhow::Result<ExpressionStatement> {
        // un_trace(trace("parseExpressionStatement".into()));
        tracing::trace!(
            "[parse_expression_statement] current_token = {:?}",
            self.current_token
        );
        let mut stmt = ExpressionStatement::new(self.current_token.clone());
        tracing::trace!("[parse_expression_statement] >> before ExpressionStatement = {stmt}");
        *stmt.expression_mut() = self.parse_expression(LOWEST)?;
        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token()?;
        }
        tracing::trace!("[parse_expression_statement] >> after ExpressionStatement = {stmt}");
        Ok(stmt)
    }

    /// parse expression
    #[tracing::instrument(name = "parse_expression", skip(self, precedence), level = "debug")]
    fn parse_expression(&mut self, precedence: OperatorPriority) -> anyhow::Result<Expression> {
        tracing::trace!(
            "[parse_expression] current_token = {:?}",
            self.current_token
        );
        // TODO clone evn to temp value
        // TODO 因为使用 PrefixParseFn 和InferParseFn 的原因，其中的第一个参数是parser
        let mut parser = self.clone();

        let prefix = self.prefix_parse_fns.get(self.current_token.token_type());

        // create temp infix parse fns for immutable checks
        let temp_infix_parse_fns = self.infix_parse_fns.clone();

        if prefix.is_none() {
            return Err(Error::NoPrefixParseFunctionFound(
                self.current_token.token_type().to_string(),
            )
            .into());
        }
        // FIXME: THIS IS OK
        let prefix = prefix.unwrap();

        let mut left_exp = prefix(&mut parser)?;
        // TODO 因为使用 PrefixParseFn 和InferParseFn 的原因，其中的第一个参数是parser
        self.update_parser(parser);
        // TODO 因为使用 PrefixParseFn 和InferParseFn 的原因，其中的第一个参数是parser
        tracing::trace!("[parse_expression] left expression = {left_exp:?}");

        while !self.peek_token_is(TokenType::SEMICOLON) && precedence < self.peek_precedence() {
            tracing::trace!("[parse_expression] peek_token = {:?}", self.peek_token);
            let infix = temp_infix_parse_fns.get(self.peek_token.token_type());
            if infix.is_none() {
                return Ok(left_exp);
            }

            self.next_token()?;
            // TODO
            // 第二次分析道这里的bug
            // then update parser, because there update self useed by next_token
            //  因为使用 PrefixParseFn 和InferParseFn 的原因，其中的第一个参数是parser
            parser = self.clone();

            let infix = infix.unwrap();
            left_exp = infix(&mut parser, left_exp)?;

            // TODO又是这个错误
            // TODO 因为使用 PrefixParseFn 和InferParseFn 的原因，其中的第一个参数是parser
            // update env with temp value
            self.update_parser(parser);
        }

        tracing::trace!(
            "[parse_expression] end current_token = {:?}",
            self.current_token
        );
        // 总结只要有变更Self的地方，都需要更新self
        Ok(left_exp)
    }

    /// parse string
    #[tracing::instrument(name = "parse_string", skip(self), level = "debug")]
    fn parse_string(&mut self) -> anyhow::Result<Expression> {
        Ok(StringLiteral::new(
            self.current_token.clone(),
            self.current_token.literal().into(),
        )
        .into())
    }

    /// parse identifier
    #[tracing::instrument(name = "parse_identifier", skip(self), level = "debug")]
    fn parse_identifier(&mut self) -> anyhow::Result<Expression> {
        Ok(Identifier::new(
            self.current_token.clone(),
            self.current_token.literal().into(),
        )
        .into())
    }

    #[tracing::instrument(name = "parse_boolean", skip(self), level = "debug")]
    fn parse_boolean(&mut self) -> anyhow::Result<Expression> {
        Ok(Boolean::new(
            self.current_token.clone(),
            self.cur_token_is(TokenType::TRUE),
        )
        .into())
    }

    /// parse integer literal
    #[tracing::instrument(name = "parse_integer_literal", skip(self), level = "debug")]
    fn parser_integer_literal(&mut self) -> anyhow::Result<Expression> {
        // un_trace(trace("parseIntegerLiteral".into()));

        let mut literal = IntegerLiteral::new(self.current_token.clone());
        let value = self.current_token.literal().parse::<isize>()?;

        *literal.value_mut() = value;
        Ok(literal.into())
    }

    /// parse prefix expression
    #[tracing::instrument(name = "parse_prefix_expression", skip(self), level = "debug")]
    fn parse_prefix_expression(&mut self) -> anyhow::Result<Expression> {
        let mut expression = Prefix::new(
            self.current_token.clone(),
            self.current_token.literal().into(),
        );
        self.next_token()?;
        *expression.right_mut() = Box::new(self.parse_expression(PREFIX)?);
        Ok(expression.into())
    }

    /// parse infix expression
    #[tracing::instrument(name = "parse_infix_expression", skip(self, left_exp), level = "debug")]
    fn parse_infix_expression(&mut self, left_exp: Expression) -> anyhow::Result<Expression> {
        let mut expression = Infix::new(
            self.current_token.clone(),
            left_exp,
            self.current_token.literal().into(),
        );

        tracing::trace!("[parse_infix_expression] before InfixExpression = {expression}");

        let precedence = self.cur_precedence();

        self.next_token()?;

        *expression.right_mut() = Box::new(self.parse_expression(precedence)?);

        tracing::trace!("[parse_infix_expression] after InfixExpression = {expression}");

        Ok(expression.into())
    }

    /// parse ground expression
    #[tracing::instrument(name = "parse_grouped_expression", skip(self), level = "debug")]
    fn parse_grouped_expression(&mut self) -> anyhow::Result<Expression> {
        self.next_token()?;

        let exp = self.parse_expression(LOWEST)?;

        if self.expect_peek(TokenType::RPAREN).is_err() {
            return Err(Error::CannotFindTokenType {
                ty: "RPAREN".into(),
            }
            .into());
        }

        Ok(exp)
    }

    /// parse if expression
    #[tracing::instrument(name = "parse_if_expression", skip(self), level = "debug")]
    fn parse_if_expression(&mut self) -> anyhow::Result<Expression> {
        let mut expression = If::new(self.current_token.clone());

        if self.expect_peek(TokenType::LPAREN).is_err() {
            return Err(Error::CannotFindTokenType {
                ty: TokenType::LPAREN.to_string(),
            }
            .into());
        }

        self.next_token()?;

        *expression.condition_mut() = Box::new(self.parse_expression(LOWEST)?);

        if self.expect_peek(TokenType::RPAREN).is_err() {
            return Err(Error::CannotFindTokenType {
                ty: TokenType::RPAREN.to_string(),
            }
            .into());
        }

        if self.expect_peek(TokenType::LBRACE).is_err() {
            return Err(Error::CannotFindTokenType {
                ty: TokenType::LBRACE.to_string(),
            }
            .into());
        }

        *expression.consequence_mut() = Some(self.parse_block_statement()?);

        if self.peek_token_is(TokenType::ELSE) {
            self.next_token()?;

            if self.expect_peek(TokenType::LBRACE).is_err() {
                return Err(Error::CannotFindTokenType {
                    ty: TokenType::LBRACE.to_string(),
                }
                .into());
            }

            *expression.alternative_mut() = Some(self.parse_block_statement()?);
        }

        Ok(Expression::If(expression))
    }

    /// parse block statement
    #[tracing::instrument(name = "parse_block_statement", skip(self), level = "debug")]
    fn parse_block_statement(&mut self) -> anyhow::Result<BlockStatement> {
        let mut block = BlockStatement::new(self.current_token.clone());

        self.next_token()?;

        // TODO this should be EOF, but this is ILLEGAL
        while !self.cur_token_is(TokenType::RBRACE) && !self.cur_token_is(TokenType::ILLEGAL) {
            let stmt = self.parse_statement()?;
            block.push_statement(stmt);
            self.next_token()?;
        }

        Ok(block)
    }

    /// parse function literals
    #[tracing::instrument(name = "parse_function_literal", skip(self), level = "debug")]
    fn parse_function_literal(&mut self) -> anyhow::Result<Expression> {
        let mut lit = FunctionLiteral::new(self.current_token.clone());

        self.next_token()?; // skip `fn`

        lit.update_parameters(self.parse_function_parameters()?);

        if self.expect_peek(TokenType::LBRACE).is_err() {
            return Err(Error::CannotFindTokenType {
                ty: "LBRACE".into(),
            }
            .into());
        }

        *lit.body_mut() = self.parse_block_statement()?;

        Ok(Expression::FunctionLiteral(lit))
    }

    #[tracing::instrument(name = "parse_function_parameters", skip(self), level = "debug")]
    fn parse_function_parameters(&mut self) -> anyhow::Result<Vec<Identifier>> {
        let mut identifiers = Vec::<Identifier>::new();

        if self.peek_token_is(TokenType::RPAREN) {
            self.next_token()?;
            return Ok(identifiers);
        }
        tracing::trace!(
            "[parser function parameters ] current_token {:?}",
            self.current_token
        );

        self.next_token()?; // skip `(`

        let ident = Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal().into(),
        };

        identifiers.push(ident);

        tracing::trace!(
            "[parser function parameters ] current_token {:?}",
            self.current_token
        );
        while self.peek_token_is(TokenType::COMMA) {
            tracing::trace!(
                "[parser function parameters ] current_token {:?}",
                self.current_token
            );
            self.next_token()?; // skip one ident
            tracing::trace!(
                "[parser function parameters ] current_token {:?}",
                self.current_token
            );
            self.next_token()?; // skip one `,`
            tracing::trace!(
                "[parser function parameters ] current_token {:?}",
                self.current_token
            );
            let ident = Identifier {
                token: self.current_token.clone(),
                value: self.current_token.literal().into(),
            };

            identifiers.push(ident);
        }
        tracing::trace!(
            "[parser function parameters ] current_token {:?}",
            self.current_token
        );

        if self.expect_peek(TokenType::RPAREN).is_err() {
            tracing::trace!(
                "[parser function parameters ] expect_peek {}",
                self.peek_token.token_type()
            );
            return Err(Error::CannotFindTokenType {
                ty: "RPAREN".into(),
            }
            .into());
        }

        Ok(identifiers)
    }

    #[tracing::instrument(name = "parser_call_expression", skip(self), level = "debug")]
    fn parser_call_expression(&mut self, function: Expression) -> anyhow::Result<Expression> {
        let mut exp = Call::new(self.current_token.clone(), function);

        exp.update_arguments(self.parse_expression_list(TokenType::RPAREN)?);

        Ok(Expression::Call(exp))
    }

    #[tracing::instrument(name = "parse_index_expression", skip(self), level = "debug")]
    fn parse_index_expression(&mut self, left: Expression) -> anyhow::Result<Expression> {
        let mut exp = Index::new(self.current_token.clone(), left);

        self.next_token()?;

        *exp.index_mut() = Box::new(self.parse_expression(LOWEST)?);

        if self.expect_peek(RBRACKET).is_err() {
            return Err(Error::CannotFindTokenType {
                ty: "RBRACKET".into(),
            }
            .into());
        }

        Ok(exp.into())
    }

    #[tracing::instrument(name = "parse_array_literal", skip(self), level = "debug")]
    fn parse_array_literal(&mut self) -> anyhow::Result<Expression> {
        let mut array = ArrayLiteral::new(self.current_token.clone());

        array.update_elements(self.parse_expression_list(RBRACKET)?);

        Ok(array.into())
    }

    #[tracing::instrument(name = "parse_expression_list", skip(self), level = "debug")]
    fn parse_expression_list(&mut self, end: TokenType) -> anyhow::Result<Vec<Expression>> {
        let mut args: Vec<Expression> = vec![];

        if self.peek_token_is(end.clone()) {
            self.next_token()?;
            return Ok(args);
        }

        self.next_token()?;
        args.push(self.parse_expression(LOWEST)?);

        while self.peek_token_is(TokenType::COMMA) {
            self.next_token()?;
            self.next_token()?;
            args.push(self.parse_expression(LOWEST)?);
        }

        if self.expect_peek(end.clone()).is_err() {
            return Err(Error::CannotFindTokenType {
                ty: end.to_string(),
            }
            .into());
        }

        Ok(args)
    }

    #[tracing::instrument(name = "parse_hash_literal", skip(self), level = "debug")]
    fn parse_hash_literal(&mut self) -> anyhow::Result<Expression> {
        let mut hash = HashLiteral::new(self.current_token.clone());

        while !self.peek_token_is(RBRACE) {
            self.next_token()?;
            let key = self.parse_expression(LOWEST)?;
            if self.expect_peek(COLON).is_err() {
                return Err(Error::ExpectColonError.into());
            }

            self.next_token()?;

            let value = self.parse_expression(LOWEST)?;

            hash.pair_mut().insert(key, value);

            if !self.peek_token_is(RBRACE) && self.expect_peek(COMMA).is_err() {
                return Err(Error::ExpectBraceAndCommaError.into());
            }
        }

        if self.expect_peek(RBRACE).is_err() {
            return Err(Error::ExpectRbraceError.into());
        }

        Ok(hash.into())
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.current_token.token_type() == &t
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.token_type() == &t
    }

    /// 断言函数的主要目的是通过检查下一个词法单元的
    /// 类型，确保词法单元顺序的正确性。这里的 expectPeek 会检查 peekToken 的类型，
    /// 并且只有在类型正确的情况下，它才会调用 nextToken 前移词法单元。
    fn expect_peek(&mut self, t: TokenType) -> anyhow::Result<()> {
        if self.peek_token_is(t.clone()) {
            self.next_token()?;
            Ok(())
        } else {
            Err(Error::ExpectNextToken {
                expected: t.to_string(),
                got: self.peek_token.token_type().to_string(),
            }
            .into())
        }
    }

    /// peekPrecedence 方法根据 p.peekToken 中的词法单元类型，返回所关联的优先
    /// 级。如果在 p.peekToken 中没有存储对应的优先级，则使用默认值 LOWEST，这是所
    /// 有运算符都可能具有的最低优先级。
    fn peek_precedence(&self) -> OperatorPriority {
        operator_priority::precedence(self.peek_token.token_type().clone())
    }

    /// same peek precedence
    fn cur_precedence(&self) -> OperatorPriority {
        operator_priority::precedence(self.current_token.token_type().clone())
    }

    /// register prefix
    fn register_prefix(&mut self, token_type: TokenType, prefix_parse_fn: PrefixParseFn<'a>) {
        self.prefix_parse_fns.insert(token_type, prefix_parse_fn);
    }

    /// register infix
    fn register_infix(&mut self, token_type: TokenType, infix_parse_fn: InferParseFn<'a>) {
        self.infix_parse_fns.insert(token_type, infix_parse_fn);
    }
}
