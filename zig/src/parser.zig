const std = @import("std");
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

        const value = self.parseExpression();
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

        const return_value = self.parseExpression();
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
        const token = self.currentToken();
        const expression = self.parseExpression();

        if (self.currentToken().token_type == .SEMICOLON) {
            self.advance();
        }

        return ast_mod.ExpressionStatement{
            .token = token,
            .expression = expression,
        };
    }

    fn parseExpression(self: *Parser) Expression {
        const token = self.currentToken();
        self.advance();

        return switch (token.token_type) {
            .IDENT => Expression{ .identifier = ast_mod.Identifier{
                .token = token,
                .value = token.literal,
            } },
            .INT => Expression{ .integer_literal = ast_mod.IntegerLiteral{
                .token = token,
                .value = std.fmt.parseInt(i64, token.literal, 10) catch 0,
            } },
            .TRUE, .FALSE => Expression{ .boolean = ast_mod.Boolean{
                .token = token,
                .value = token.token_type == .TRUE,
            } },
            else => Expression{ .identifier = ast_mod.Identifier{
                .token = token,
                .value = token.literal,
            } },
        };
    }
};
