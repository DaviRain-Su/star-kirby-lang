const std = @import("std");
const zigfp = @import("zigfp");
const token_mod = @import("token.zig");
const ast_mod = @import("ast.zig");

const Token = token_mod.Token;
const TokenType = token_mod.TokenType;
const Expression = ast_mod.Expression;
const Statement = ast_mod.Statement;
const Program = ast_mod.Program;

pub const ParserError = error{
    UnexpectedToken,
    InvalidSyntax,
    OutOfMemory,
};

/// Operator precedence levels
pub const Precedence = enum(u8) {
    lowest = 1,
    equals = 2, // ==
    lessgreater = 3, // < or >
    sum = 4, // +
    product = 5, // *
    prefix = 6, // -X or !X
    call = 7, // myFunction(X)
    index = 8, // array[index]
};

/// Get precedence for a token type
pub fn precedence(token_type: TokenType) Precedence {
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

pub const Parser = struct {
    allocator: std.mem.Allocator,
    tokens: []const Token,
    position: usize,

    pub fn init(allocator: std.mem.Allocator, tokens: []const Token) Parser {
        return Parser{
            .allocator = allocator,
            .tokens = tokens,
            .position = 0,
        };
    }

    fn currentToken(self: *const Parser) Token {
        if (self.position >= self.tokens.len) {
            return Token{ .token_type = .EOF, .literal = "" };
        }
        return self.tokens[self.position];
    }

    fn peekToken(self: *const Parser) Token {
        if (self.position + 1 >= self.tokens.len) {
            return Token{ .token_type = .EOF, .literal = "" };
        }
        return self.tokens[self.position + 1];
    }

    fn advance(self: *Parser) void {
        self.position += 1;
    }

    fn expectPeek(self: *Parser, expected: TokenType) !void {
        if (self.peekToken().token_type == expected) {
            self.advance();
            return;
        }
        return ParserError.UnexpectedToken;
    }

    fn peekTokenIs(self: *const Parser, token_type: TokenType) bool {
        return self.peekToken().token_type == token_type;
    }

    fn currentTokenIs(self: *const Parser, token_type: TokenType) bool {
        return self.currentToken().token_type == token_type;
    }

    pub fn parseProgram(self: *Parser) ParserError!Program {
        var statements = try std.ArrayList(Statement).initCapacity(self.allocator, 16);
        defer statements.deinit(self.allocator);

        while (self.currentToken().token_type != .EOF) {
            const stmt = try self.parseStatement();
            try statements.append(self.allocator, stmt);
        }

        const owned_statements = try statements.toOwnedSlice(self.allocator);
        // Note: statements is now empty, toOwnedSlice transfers ownership
        return Program{ .statements = owned_statements };
    }

    fn parseStatement(self: *Parser) ParserError!Statement {
        const token = self.currentToken();
        return switch (token.token_type) {
            .LET => Statement{ .let = try self.parseLetStatement() },
            .RETURN => Statement{ .return_stmt = try self.parseReturnStatement() },
            else => Statement{ .expression = try self.parseExpressionStatement() },
        };
    }

    fn parseLetStatement(self: *Parser) ParserError!ast_mod.LetStatement {
        // Skip 'let'
        self.advance();

        const ident_token = self.currentToken();
        if (ident_token.token_type != .IDENT) {
            return ParserError.UnexpectedToken;
        }
        self.advance();

        const name = ast_mod.Identifier{
            .token = ident_token,
            .value = ident_token.literal,
        };

        if (self.currentToken().token_type != .ASSIGN) {
            return ParserError.UnexpectedToken;
        }
        self.advance();

        const value = try self.parseExpression(.lowest);
        // Skip semicolon if present
        if (self.currentToken().token_type == .SEMICOLON) {
            self.advance();
        }

        return ast_mod.LetStatement{
            .token = token_mod.Token{ .token_type = .LET, .literal = "let" },
            .name = name,
            .value = value,
        };
    }

    fn parseReturnStatement(self: *Parser) ParserError!ast_mod.ReturnStatement {
        // Skip 'return'
        self.advance();

        const return_value = try self.parseExpression(.lowest);
        // Skip semicolon if present
        if (self.currentToken().token_type == .SEMICOLON) {
            self.advance();
        }

        return ast_mod.ReturnStatement{
            .token = token_mod.Token{ .token_type = .RETURN, .literal = "return" },
            .return_value = return_value,
        };
    }

    fn parseExpressionStatement(self: *Parser) ParserError!ast_mod.ExpressionStatement {
        const expression = try self.parseExpression(.lowest);

        if (self.currentToken().token_type == .SEMICOLON) {
            self.advance();
        }

        return ast_mod.ExpressionStatement{
            .token = token_mod.Token{ .token_type = .ILLEGAL, .literal = "" },
            .expression = expression,
        };
    }

    fn parseExpression(self: *Parser, prec: Precedence) ParserError!Expression {
        std.debug.print("parseExpression: prec={}\n", .{@intFromEnum(prec)});
        var left_exp = try self.parsePrefix();
        std.debug.print("parseExpression: after parsePrefix, left_exp type {}\n", .{@as(std.meta.Tag(ast_mod.Expression), left_exp)});

        const peek_tok = self.peekToken();
        const peek_prec = self.peekPrecedence();
        std.debug.print("parseExpression: peek token {s}, peek prec {}, condition {}\n", .{ @tagName(peek_tok.token_type), @intFromEnum(peek_prec), @intFromEnum(prec) < @intFromEnum(peek_prec) });

        while (!self.currentTokenIs(.SEMICOLON) and @intFromEnum(prec) < @intFromEnum(self.currentPrecedence())) {
            std.debug.print("parseExpression: calling parseInfix, position={}\n", .{self.position});
            const infix = try self.parseInfix(left_exp);
            left_exp = infix;
            std.debug.print("parseExpression: after parseInfix, left_exp type {}, position={}\n", .{ @as(std.meta.Tag(ast_mod.Expression), left_exp), self.position });
        }

        // Check for function call
        if (self.peekToken().token_type == .LPAREN) {
            left_exp = try self.parseCallExpression(left_exp);
        }

        // Check for index expression
        if (self.peekToken().token_type == .LBRACKET) {
            left_exp = try self.parseIndexExpression(left_exp);
        }

        return left_exp;
    }

    fn parsePrefix(self: *Parser) !Expression {
        const token = self.currentToken();
        std.debug.print("parsePrefix: token type {}, literal '{s}'\n", .{ token.token_type, token.literal });

        return switch (token.token_type) {
            .IDENT => blk: {
                self.advance();
                break :blk Expression{ .identifier = ast_mod.Identifier{
                    .token = token,
                    .value = token.literal,
                } };
            },
            .INT => blk: {
                self.advance();
                const value = std.fmt.parseInt(i64, token.literal, 10) catch 0;
                // std.debug.print("Parsed INT: '{s}' -> {}\n", .{token.literal, value});
                break :blk Expression{ .integer_literal = ast_mod.IntegerLiteral{
                    .token = token,
                    .value = value,
                } };
            },
            .TRUE, .FALSE => blk: {
                self.advance();
                break :blk Expression{ .boolean = ast_mod.Boolean{
                    .token = token,
                    .value = token.token_type == .TRUE,
                } };
            },
            .STRING => blk: {
                self.advance();
                break :blk Expression{ .string_literal = ast_mod.StringLiteral{
                    .token = token,
                    .value = token.literal,
                } };
            },
            .LBRACKET => try self.parseArrayLiteral(),
            .BANG, .MINUS => try self.parsePrefixExpression(),
            .LPAREN => try self.parseGroupedExpression(),
            .IF => try self.parseIfExpression(),
            // .FUNCTION => try self.parseFunctionLiteral(), // TODO: Implement function literals
            else => blk: {
                self.advance();
                break :blk Expression{ .identifier = ast_mod.Identifier{
                    .token = token,
                    .value = token.literal,
                } };
            },
        };
    }

    fn parseInfix(self: *Parser, left: Expression) !Expression {
        const token = self.currentToken();
        std.debug.print("parseInfix: operator '{s}'\n", .{token.literal});

        const operator = token.literal;
        const prec = self.currentPrecedence();

        self.advance(); // consume the operator
        const right = try self.parseExpression(prec);
        std.debug.print("parseInfix: right expression type {}\n", .{@as(std.meta.Tag(ast_mod.Expression), right)});

        const left_ptr = try self.allocator.create(Expression);
        left_ptr.* = left;
        const right_ptr = try self.allocator.create(Expression);
        right_ptr.* = right;

        std.debug.print("parseInfix: returning .infix\n", .{});
        return Expression{ .infix = ast_mod.Infix{
            .token = token,
            .left = left_ptr,
            .operator = operator,
            .right = right_ptr,
        } };
    }

    fn peekPrecedence(self: *const Parser) Precedence {
        const peek_tok = self.peekToken();
        return precedence(peek_tok.token_type);
    }

    fn currentPrecedence(self: *const Parser) Precedence {
        const curr_tok = self.currentToken();
        return precedence(curr_tok.token_type);
    }

    fn parsePrefixExpression(self: *Parser) ParserError!Expression {
        const token = self.currentToken();
        const operator = token.literal;

        self.advance();
        const right = try self.parseExpression(.prefix);

        const right_ptr = try self.allocator.create(Expression);
        right_ptr.* = right;

        return Expression{ .prefix = ast_mod.Prefix{
            .token = token,
            .operator = operator,
            .right = right_ptr,
        } };
    }

    fn parseGroupedExpression(self: *Parser) !Expression {
        self.advance();
        const exp = try self.parseExpression(.lowest);
        try self.expectPeek(.RPAREN);
        return exp;
    }

    fn parseBlockStatement(self: *Parser) ParserError!ast_mod.BlockStatement {
        const token = self.currentToken();
        self.advance(); // skip '{'

        var statements = try std.ArrayList(ast_mod.Statement).initCapacity(self.allocator, 8);
        defer statements.deinit(self.allocator);

        while (self.currentToken().token_type != .RBRACE and self.currentToken().token_type != .EOF) {
            const stmt = try self.parseStatement();
            try statements.append(self.allocator, stmt);
        }

        try self.expectPeek(.RBRACE);

        const owned_statements = try statements.toOwnedSlice(self.allocator);

        return ast_mod.BlockStatement{
            .token = token,
            .statements = owned_statements,
        };
    }

    fn parseIfExpression(self: *Parser) ParserError!Expression {
        const token = self.currentToken();
        self.advance(); // skip 'if'

        try self.expectPeek(.LPAREN);
        const condition = try self.parseExpression(.lowest);
        try self.expectPeek(.RPAREN);

        try self.expectPeek(.LBRACE);
        const consequence = try self.parseBlockStatement();

        var alternative: ?*ast_mod.BlockStatement = null;
        if (self.peekToken().token_type == .ELSE) {
            self.advance(); // skip 'else'
            try self.expectPeek(.LBRACE);
            const alt_block = try self.parseBlockStatement();
            const alt_ptr = try self.allocator.create(ast_mod.BlockStatement);
            alt_ptr.* = alt_block;
            alternative = alt_ptr;
        }

        const condition_ptr = try self.allocator.create(ast_mod.Expression);
        condition_ptr.* = condition;

        const consequence_ptr = try self.allocator.create(ast_mod.BlockStatement);
        consequence_ptr.* = consequence;

        return Expression{ .if_expression = ast_mod.IfExpression{
            .token = token,
            .condition = condition_ptr,
            .consequence = consequence_ptr,
            .alternative = alternative,
        } };
    }

    fn parseCallExpression(self: *Parser, function: Expression) ParserError!Expression {
        const token = self.currentToken(); // '(' token

        var arguments = try std.ArrayList(ast_mod.Expression).initCapacity(self.allocator, 4);
        defer arguments.deinit(self.allocator);

        if (self.peekToken().token_type != .RPAREN) {
            self.advance(); // skip '('
            const arg = try self.parseExpression(.lowest);
            try arguments.append(self.allocator, arg);

            while (self.peekToken().token_type == .COMMA) {
                self.advance(); // skip comma
                self.advance(); // skip to next argument
                const next_arg = try self.parseExpression(.lowest);
                try arguments.append(self.allocator, next_arg);
            }
        }

        try self.expectPeek(.RPAREN);

        const args_slice = try arguments.toOwnedSlice(self.allocator);

        const function_ptr = try self.allocator.create(ast_mod.Expression);
        function_ptr.* = function;

        return Expression{ .call = ast_mod.Call{
            .token = token,
            .function = function_ptr,
            .arguments = args_slice,
        } };
    }

    fn parseArrayLiteral(self: *Parser) ParserError!Expression {
        const token = self.currentToken();
        self.advance(); // consume '['

        var elements = try std.ArrayList(ast_mod.Expression).initCapacity(self.allocator, 4);
        defer elements.deinit(self.allocator);

        // Parse elements until we hit ']'
        while (self.currentToken().token_type != .RBRACKET) {
            const element = try self.parseExpression(.lowest);
            try elements.append(self.allocator, element);

            // If next token is comma, consume it and continue
            if (self.peekToken().token_type == .COMMA) {
                self.advance(); // consume comma
            } else if (self.peekToken().token_type != .RBRACKET) {
                // If not comma and not closing bracket, syntax error
                return ParserError.UnexpectedToken;
            }
        }

        self.advance(); // consume ']'

        const elements_slice = try elements.toOwnedSlice(self.allocator);

        return Expression{ .array_literal = ast_mod.ArrayLiteral{
            .token = token,
            .elements = elements_slice,
        } };
    }

    fn parseIndexExpression(self: *Parser, left: Expression) ParserError!Expression {
        const token = self.currentToken(); // '[' token
        self.advance(); // skip '['

        const index = try self.parseExpression(.lowest);
        try self.expectPeek(.RBRACKET);

        const left_ptr = try self.allocator.create(ast_mod.Expression);
        left_ptr.* = left;

        const index_ptr = try self.allocator.create(ast_mod.Expression);
        index_ptr.* = index;

        return Expression{ .index_expression = ast_mod.IndexExpression{
            .token = token,
            .left = left_ptr,
            .index = index_ptr,
        } };
    }
};
