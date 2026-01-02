const std = @import("std");
const zigfp = @import("zigfp");
const token_mod = @import("token.zig");
const ast_mod = @import("ast.zig");

const Token = token_mod.Token;
const TokenType = token_mod.TokenType;
const Expression = ast_mod.Expression;
const Statement = ast_mod.Statement;
const Program = ast_mod.Program;

// Import zigfp types
const Result = zigfp.result.Result;
const Error = zigfp.result.Error;

pub const ParserError = error{
    NoPrefixParseFunction,
    UnexpectedToken,
    InvalidInteger,
};

pub const OperatorPrecedence = enum(u8) {
    lowest = 1,
    equals = 2, // ==
    lessgreater = 3, // < or >
    sum = 4, // +
    product = 5, // *
    prefix = 6, // -X or !x
    call = 7, // myFunction(x)
    index = 8, // array[index]
};

pub const Parser = struct {
    allocator: std.mem.Allocator,
    tokens: []const Token,
    current_position: usize,
    current_token: Token,
    peek_token: Token,

    pub fn init(allocator: std.mem.Allocator, tokens: []const Token) !Parser {
        if (tokens.len == 0) {
            return error.EmptyTokens;
        }

        var parser = Parser{
            .allocator = allocator,
            .tokens = tokens,
            .current_position = 0,
            .current_token = tokens[0],
            .peek_token = if (tokens.len > 1) tokens[1] else Token{ .token_type = .EOF, .literal = "" },
        };

        return parser;
    }

    fn nextToken(self: *Parser) void {
        self.current_position += 1;
        if (self.current_position >= self.tokens.len) {
            self.current_token = Token{ .token_type = .EOF, .literal = "" };
        } else {
            self.current_token = self.tokens[self.current_position];
        }

        if (self.current_position + 1 >= self.tokens.len) {
            self.peek_token = Token{ .token_type = .EOF, .literal = "" };
        } else {
            self.peek_token = self.tokens[self.current_position + 1];
        }
    }

    fn currentTokenIs(self: *const Parser, token_type: TokenType) bool {
        return self.current_token.token_type == token_type;
    }

    fn peekTokenIs(self: *const Parser, token_type: TokenType) bool {
        return self.peek_token.token_type == token_type;
    }

    fn expectPeek(self: *Parser, token_type: TokenType) !void {
        if (self.peekTokenIs(token_type)) {
            self.nextToken();
            return;
        }
        return ParserError.UnexpectedToken;
    }

    fn peekPrecedence(self: *const Parser) OperatorPrecedence {
        return precedence(self.peek_token.token_type);
    }

    fn currentPrecedence(self: *const Parser) OperatorPrecedence {
        return precedence(self.current_token.token_type);
    }

    pub fn parseProgram(self: *Parser) !Program {
        var statements = std.ArrayList(Statement).init(self.allocator);
        defer statements.deinit();

        while (!self.currentTokenIs(.EOF)) {
            const stmt = try self.parseStatement();
            try statements.append(stmt);
            self.nextToken();
        }

        return Program{
            .statements = statements,
        };
    }

    fn parseStatement(self: *Parser) !Statement {
        return switch (self.current_token.token_type) {
            .LET => Statement{ .let = try self.parseLetStatement() },
            .RETURN => Statement{ .return_stmt = try self.parseReturnStatement() },
            else => Statement{ .expression = try self.parseExpressionStatement() },
        };
    }

    fn parseLetStatement(self: *Parser) !ast_mod.LetStatement {
        const token = self.current_token;

        try self.expectPeek(.IDENT);
        const name = ast_mod.Identifier{
            .token = self.current_token,
            .value = self.current_token.literal,
        };

        try self.expectPeek(.ASSIGN);

        self.nextToken();
        const value = try self.parseExpression(.lowest);

        if (self.peekTokenIs(.SEMICOLON)) {
            self.nextToken();
        }

        return ast_mod.LetStatement{
            .token = token,
            .name = name,
            .value = value,
        };
    }

    fn parseReturnStatement(self: *Parser) !ast_mod.ReturnStatement {
        const token = self.current_token;

        self.nextToken();
        const return_value = try self.parseExpression(.lowest);

        if (self.peekTokenIs(.SEMICOLON)) {
            self.nextToken();
        }

        return ast_mod.ReturnStatement{
            .token = token,
            .return_value = return_value,
        };
    }

    fn parseExpressionStatement(self: *Parser) !ast_mod.ExpressionStatement {
        const token = self.current_token;
        const expression = try self.parseExpression(.lowest);

        if (self.peekTokenIs(.SEMICOLON)) {
            self.nextToken();
        }

        return ast_mod.ExpressionStatement{
            .token = token,
            .expression = expression,
        };
    }

    fn parseExpression(self: *Parser, precedence: OperatorPrecedence) !Expression {
        var left = try self.parsePrefix();

        while (!self.peekTokenIs(.SEMICOLON) and @intFromEnum(precedence) < @intFromEnum(self.peekPrecedence())) {
            const infix = try self.parseInfix(left);
            left = infix;
        }

        return left;
    }

    fn parsePrefix(self: *Parser) !Expression {
        return switch (self.current_token.token_type) {
            .IDENT => Expression{ .identifier = ast_mod.Identifier{
                .token = self.current_token,
                .value = self.current_token.literal,
            } },
            .INT => Expression{ .integer_literal = try self.parseIntegerLiteral() },
            .TRUE, .FALSE => Expression{ .boolean = ast_mod.Boolean{
                .token = self.current_token,
                .value = self.current_token.token_type == .TRUE,
            } },
            .BANG, .MINUS => try self.parsePrefixExpression(),
            .LPAREN => try self.parseGroupedExpression(),
            .IF => try self.parseIfExpression(),
            .FUNCTION => try self.parseFunctionLiteral(),
            else => ParserError.NoPrefixParseFunction,
        };
    }

    fn parseInfix(self: *Parser, left: Expression) !Expression {
        const token = self.current_token;
        const operator = self.current_token.literal;
        const precedence = self.currentPrecedence();

        self.nextToken();
        const right = try self.parseExpression(precedence);

        return Expression{ .infix = ast_mod.Infix{
            .token = token,
            .left = try self.allocator.create(Expression),
            .operator = operator,
            .right = try self.allocator.create(Expression),
        } };
        // Note: In a real implementation, we'd need to handle memory management properly
        // This is simplified for now
    }

    fn parseIntegerLiteral(self: *Parser) !ast_mod.IntegerLiteral {
        const token = self.current_token;
        const value = std.fmt.parseInt(i64, self.current_token.literal, 10) catch {
            return ParserError.InvalidInteger;
        };

        return ast_mod.IntegerLiteral{
            .token = token,
            .value = value,
        };
    }

    fn parsePrefixExpression(self: *Parser) !Expression {
        const token = self.current_token;
        const operator = self.current_token.literal;

        self.nextToken();
        const right = try self.parseExpression(.prefix);

        return Expression{ .prefix = ast_mod.Prefix{
            .token = token,
            .operator = operator,
            .right = try self.allocator.create(Expression),
        } };
        // Note: Memory management simplification
    }

    fn parseGroupedExpression(self: *Parser) !Expression {
        self.nextToken();
        const exp = try self.parseExpression(.lowest);
        try self.expectPeek(.RPAREN);
        return exp;
    }

    fn parseIfExpression(self: *Parser) !Expression {
        const token = self.current_token;

        try self.expectPeek(.LPAREN);
        self.nextToken();
        const condition = try self.parseExpression(.lowest);
        try self.expectPeek(.RPAREN);
        try self.expectPeek(.LBRACE);

        const consequence = try self.parseBlockStatement();

        var alternative: ?ast_mod.BlockStatement = null;
        if (self.peekTokenIs(.ELSE)) {
            self.nextToken();
            try self.expectPeek(.LBRACE);
            alternative = try self.parseBlockStatement();
        }

        return Expression{ .if_expression = ast_mod.If{
            .token = token,
            .condition = try self.allocator.create(Expression),
            .consequence = try self.allocator.create(ast_mod.BlockStatement),
            .alternative = if (alternative) |alt| try self.allocator.create(ast_mod.BlockStatement) else null,
        } };
        // Note: Memory management simplification
    }

    fn parseBlockStatement(self: *Parser) !ast_mod.BlockStatement {
        const token = self.current_token;
        var statements = std.ArrayList(Statement).init(self.allocator);
        defer statements.deinit();

        self.nextToken();

        while (!self.currentTokenIs(.RBRACE) and !self.currentTokenIs(.EOF)) {
            const stmt = try self.parseStatement();
            try statements.append(stmt);
            self.nextToken();
        }

        return ast_mod.BlockStatement{
            .token = token,
            .statements = statements,
        };
    }

    fn parseFunctionLiteral(self: *Parser) !Expression {
        const token = self.current_token;

        try self.expectPeek(.LPAREN);
        const parameters = try self.parseFunctionParameters();
        try self.expectPeek(.LBRACE);

        const body = try self.parseBlockStatement();

        return Expression{ .function_literal = ast_mod.FunctionLiteral{
            .token = token,
            .parameters = parameters,
            .body = try self.allocator.create(ast_mod.BlockStatement),
        } };
        // Note: Memory management simplification
    }

    fn parseFunctionParameters(self: *Parser) !std.ArrayList(ast_mod.Identifier) {
        var identifiers = std.ArrayList(ast_mod.Identifier).init(self.allocator);
        defer identifiers.deinit();

        if (self.peekTokenIs(.RPAREN)) {
            self.nextToken();
            return identifiers;
        }

        self.nextToken();
        var ident = ast_mod.Identifier{
            .token = self.current_token,
            .value = self.current_token.literal,
        };
        try identifiers.append(ident);

        while (self.peekTokenIs(.COMMA)) {
            self.nextToken();
            self.nextToken();
            ident = ast_mod.Identifier{
                .token = self.current_token,
                .value = self.current_token.literal,
            };
            try identifiers.append(ident);
        }

        try self.expectPeek(.RPAREN);

        return identifiers;
    }
};

fn precedence(token_type: TokenType) OperatorPrecedence {
    return switch (token_type) {
        .EQ, .NOTEQ => .equals,
        .LT, .GT => .lessgreater,
        .PLUS, .MINUS => .sum,
        .SLASH, .ASTERISK => .product,
        .LPAREN => .call,
        .LBRACKET => .index,
        else => .lowest,
    };
}

test "parser basic" {
    const allocator = std.testing.allocator;

    const tokens = [_]Token{
        Token{ .token_type = .LET, .literal = "let" },
        Token{ .token_type = .IDENT, .literal = "x" },
        Token{ .token_type = .ASSIGN, .literal = "=" },
        Token{ .token_type = .INT, .literal = "5" },
        Token{ .token_type = .SEMICOLON, .literal = ";" },
        Token{ .token_type = .EOF, .literal = "" },
    };

    var parser = try Parser.init(allocator, &tokens);
    const program = try parser.parseProgram();
    defer program.deinit();

    try std.testing.expectEqual(@as(usize, 1), program.statements.items.len);
}
