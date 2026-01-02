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
        var left_exp = try self.parsePrefix();

        while (!self.currentTokenIs(.SEMICOLON) and !self.currentTokenIs(.EOF)) {
            const curr_prec = self.currentPrecedence();
            if (@intFromEnum(prec) >= @intFromEnum(curr_prec)) {
                break;
            }

            // Handle infix operators
            if (self.currentToken().token_type == .PLUS or
                self.currentToken().token_type == .MINUS or
                self.currentToken().token_type == .ASTERISK or
                self.currentToken().token_type == .SLASH or
                self.currentToken().token_type == .EQ or
                self.currentToken().token_type == .NOTEQ or
                self.currentToken().token_type == .LT or
                self.currentToken().token_type == .GT)
            {
                left_exp = try self.parseInfix(left_exp);
            }
            // Handle function call
            else if (self.currentToken().token_type == .LPAREN) {
                left_exp = try self.parseCallExpression(left_exp);
            }
            // Handle index expression
            else if (self.currentToken().token_type == .LBRACKET) {
                left_exp = try self.parseIndexExpression(left_exp);
            } else {
                break;
            }
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
            .LBRACE => try self.parseHashLiteral(),
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

        self.advance(); // consume the operator
        const right = try self.parseExpression(prec);

        const left_ptr = try self.allocator.create(Expression);
        left_ptr.* = left;
        const right_ptr = try self.allocator.create(Expression);
        right_ptr.* = right;

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

        if (self.currentToken().token_type != .RBRACE) {
            return ParserError.UnexpectedToken;
        }
        self.advance(); // skip '}'

        const owned_statements = try statements.toOwnedSlice(self.allocator);

        return ast_mod.BlockStatement{
            .token = token,
            .statements = owned_statements,
        };
    }

    fn parseIfExpression(self: *Parser) ParserError!Expression {
        const token = self.currentToken();
        self.advance(); // skip 'if'

        // Expect '('
        if (self.currentToken().token_type != .LPAREN) {
            return ParserError.UnexpectedToken;
        }
        self.advance(); // skip '('

        const condition = try self.parseExpression(.lowest);

        // Expect ')'
        if (self.currentToken().token_type != .RPAREN) {
            return ParserError.UnexpectedToken;
        }
        self.advance(); // skip ')'

        // Expect '{'
        if (self.currentToken().token_type != .LBRACE) {
            return ParserError.UnexpectedToken;
        }
        const consequence = try self.parseBlockStatement();

        var alternative: ?*ast_mod.BlockStatement = null;
        if (self.currentToken().token_type == .ELSE) {
            self.advance(); // skip 'else'
            if (self.currentToken().token_type != .LBRACE) {
                return ParserError.UnexpectedToken;
            }
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
        self.advance(); // skip '('

        var arguments = try std.ArrayList(ast_mod.Expression).initCapacity(self.allocator, 4);
        defer arguments.deinit(self.allocator);

        // Parse arguments
        while (self.currentToken().token_type != .RPAREN and self.currentToken().token_type != .EOF) {
            const arg = try self.parseExpression(.lowest);
            try arguments.append(self.allocator, arg);

            if (self.currentToken().token_type == .COMMA) {
                self.advance(); // skip comma
            }
        }

        if (self.currentToken().token_type != .RPAREN) {
            return ParserError.UnexpectedToken;
        }
        self.advance(); // skip ')'

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
        while (self.currentToken().token_type != .RBRACKET and self.currentToken().token_type != .EOF) {
            const element = try self.parseExpression(.lowest);
            try elements.append(self.allocator, element);

            // Check if we need to consume a comma or closing bracket
            if (self.currentToken().token_type == .COMMA) {
                self.advance(); // consume comma
            } else if (self.currentToken().token_type != .RBRACKET) {
                // Check next token
                if (self.peekToken().token_type == .COMMA) {
                    self.advance(); // move to comma
                    self.advance(); // consume comma
                } else if (self.peekToken().token_type == .RBRACKET) {
                    self.advance(); // move to ']'
                    break;
                }
            }
        }

        if (self.currentToken().token_type == .RBRACKET) {
            self.advance(); // consume ']'
        }

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

        // Expect ']'
        if (self.currentToken().token_type != .RBRACKET) {
            return ParserError.UnexpectedToken;
        }
        self.advance(); // skip ']'

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

    fn parseFunctionLiteral(self: *Parser) ParserError!Expression {
        const token = self.currentToken(); // 'fn' token
        self.advance(); // skip 'fn'

        if (self.currentToken().token_type != .LPAREN) {
            return ParserError.UnexpectedToken;
        }
        self.advance(); // skip '('

        // Parse parameters
        var parameters = try std.ArrayList(ast_mod.Identifier).initCapacity(self.allocator, 4);
        defer parameters.deinit(self.allocator);

        while (self.currentToken().token_type != .RPAREN) {
            if (self.currentToken().token_type != .IDENT) {
                return ParserError.UnexpectedToken;
            }
            const param = ast_mod.Identifier{
                .token = self.currentToken(),
                .value = self.currentToken().literal,
            };
            try parameters.append(self.allocator, param);
            self.advance();

            if (self.currentToken().token_type == .COMMA) {
                self.advance(); // skip comma
            }
        }
        self.advance(); // skip ')'

        if (self.currentToken().token_type != .LBRACE) {
            return ParserError.UnexpectedToken;
        }

        const body = try self.parseBlockStatement();

        const params_slice = try parameters.toOwnedSlice(self.allocator);
        const body_ptr = try self.allocator.create(ast_mod.BlockStatement);
        body_ptr.* = body;

        return Expression{ .function_literal = ast_mod.FunctionLiteral{
            .token = token,
            .parameters = params_slice,
            .body = body_ptr,
        } };
    }

    fn parseHashLiteral(self: *Parser) ParserError!Expression {
        const token = self.currentToken(); // '{' token
        self.advance(); // skip '{'

        var pairs = try std.ArrayList(ast_mod.HashPair).initCapacity(self.allocator, 4);
        defer pairs.deinit(self.allocator);

        while (self.currentToken().token_type != .RBRACE) {
            const key = try self.parseExpression(.lowest);

            if (self.currentToken().token_type != .COLON) {
                return ParserError.UnexpectedToken;
            }
            self.advance(); // skip ':'

            const value = try self.parseExpression(.lowest);

            const key_ptr = try self.allocator.create(ast_mod.Expression);
            key_ptr.* = key;
            const value_ptr = try self.allocator.create(ast_mod.Expression);
            value_ptr.* = value;

            try pairs.append(self.allocator, ast_mod.HashPair{
                .key = key_ptr,
                .value = value_ptr,
            });

            if (self.currentToken().token_type == .COMMA) {
                self.advance(); // skip comma
            }
        }
        self.advance(); // skip '}'

        const pairs_slice = try pairs.toOwnedSlice(self.allocator);

        return Expression{ .hash_literal = ast_mod.HashLiteral{
            .token = token,
            .pairs = pairs_slice,
        } };
    }
};
