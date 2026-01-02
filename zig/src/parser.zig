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

    pub fn parseProgram(self: *Parser) !Program {
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

    fn parseStatement(self: *Parser) !Statement {
        const token = self.currentToken();
        return switch (token.token_type) {
            .LET => Statement{ .let = try self.parseLetStatement() },
            .RETURN => Statement{ .return_stmt = try self.parseReturnStatement() },
            else => Statement{ .expression = try self.parseExpressionStatement() },
        };
    }

    fn parseLetStatement(self: *Parser) !ast_mod.LetStatement {
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

    fn parseReturnStatement(self: *Parser) !ast_mod.ReturnStatement {
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

    fn parseExpressionStatement(self: *Parser) !ast_mod.ExpressionStatement {
        const expression = try self.parseExpression(.lowest);

        if (self.currentToken().token_type == .SEMICOLON) {
            self.advance();
        }

        return ast_mod.ExpressionStatement{
            .token = token_mod.Token{ .token_type = .ILLEGAL, .literal = "" },
            .expression = expression,
        };
    }

    fn parseExpression(self: *Parser, prec: Precedence) !Expression {
        var left_exp = try self.parsePrefix();

        while (!self.peekTokenIs(.SEMICOLON) and @intFromEnum(prec) < @intFromEnum(self.peekPrecedence())) {
            const infix = try self.parseInfix(left_exp);
            left_exp = infix;
        }

        return left_exp;
    }

    fn parsePrefix(self: *Parser) !Expression {
        const token = self.currentToken();

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
                break :blk Expression{ .integer_literal = ast_mod.IntegerLiteral{
                    .token = token,
                    .value = std.fmt.parseInt(i64, token.literal, 10) catch 0,
                } };
            },
            .TRUE, .FALSE => blk: {
                self.advance();
                break :blk Expression{ .boolean = ast_mod.Boolean{
                    .token = token,
                    .value = token.token_type == .TRUE,
                } };
            },
            .BANG, .MINUS => try self.parsePrefixExpression(),
            .LPAREN => try self.parseGroupedExpression(),
            .IF => try self.parseIfExpression(),
            .FUNCTION => try self.parseFunctionLiteral(),
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
        const operator = token.literal;
        const prec = self.currentPrecedence();

        self.advance();
        const right = try self.parseExpression(prec);

        _ = left; // TODO: Use left expression
        return Expression{ .infix = ast_mod.Infix{
            .token = token,
            .left = try self.allocator.create(Expression),
            .operator = operator,
            .right = try self.allocator.create(Expression),
        } };
        // Note: Memory management needs proper implementation
    }

    fn peekPrecedence(self: *const Parser) Precedence {
        const peek_tok = self.peekToken();
        return precedence(peek_tok.token_type);
    }

    fn currentPrecedence(self: *const Parser) Precedence {
        const curr_tok = self.currentToken();
        return precedence(curr_tok.token_type);
    }

    fn parsePrefixExpression(self: *Parser) !Expression {
        const token = self.currentToken();
        const operator = token.literal;

        self.advance();
        const right = try self.parseExpression(.prefix);

        return Expression{ .prefix = ast_mod.Prefix{
            .token = token,
            .operator = operator,
            .right = try self.allocator.create(Expression),
        } };
    }

    fn parseGroupedExpression(self: *Parser) !Expression {
        self.advance();
        const exp = try self.parseExpression(.lowest);
        try self.expectPeek(.RPAREN);
        return exp;
    }
};
