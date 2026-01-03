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
    or_op = 2, // ||
    and_op = 3, // &&
    equals = 4, // ==
    lessgreater = 5, // < or > or <= or >=
    sum = 6, // +
    product = 7, // * / %
    prefix = 8, // -X or !X
    call = 9, // myFunction(X)
    index = 10, // array[index]
};

/// Get precedence for a token type
pub fn precedence(token_type: TokenType) Precedence {
    return switch (token_type) {
        .OR => .or_op,
        .AND => .and_op,
        .EQ, .NOTEQ => .equals,
        .LT, .GT, .LTE, .GTE => .lessgreater,
        .PLUS, .MINUS => .sum,
        .SLASH, .ASTERISK, .PERCENT => .product,
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
            .WHILE => Statement{ .while_stmt = try self.parseWhileStatement() },
            .FOR => Statement{ .for_stmt = try self.parseForStatement() },
            .BREAK => Statement{ .break_stmt = try self.parseBreakStatement() },
            .CONTINUE => Statement{ .continue_stmt = try self.parseContinueStatement() },
            else => try self.parseExpressionOrIndexAssignment(),
        };
    }

    /// Parse either an expression statement or an index assignment
    /// This handles: expr; OR arr[index] = value;
    fn parseExpressionOrIndexAssignment(self: *Parser) ParserError!Statement {
        const expression = try self.parseExpression(.lowest);

        // Check if this is an index assignment: expr[index] = value
        if (expression == .index_expression and self.currentToken().token_type == .ASSIGN) {
            const idx_expr = expression.index_expression;
            self.advance(); // skip '='

            const value = try self.parseExpression(.lowest);

            if (self.currentToken().token_type == .SEMICOLON) {
                self.advance();
            }

            const value_ptr = try self.allocator.create(Expression);
            value_ptr.* = value;

            return Statement{ .index_assignment = ast_mod.IndexAssignment{
                .token = token_mod.Token{ .token_type = .ASSIGN, .literal = "=" },
                .left = idx_expr.left,
                .index = idx_expr.index,
                .value = value_ptr,
            } };
        }

        // Regular expression statement
        if (self.currentToken().token_type == .SEMICOLON) {
            self.advance();
        }

        return Statement{ .expression = ast_mod.ExpressionStatement{
            .token = token_mod.Token{ .token_type = .ILLEGAL, .literal = "" },
            .expression = expression,
        } };
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

    fn parseWhileStatement(self: *Parser) ParserError!ast_mod.WhileStatement {
        const token = self.currentToken();

        // Skip 'while'
        self.advance();

        // Expect '('
        if (self.currentToken().token_type != .LPAREN) {
            return ParserError.UnexpectedToken;
        }
        self.advance();

        // Parse condition
        const condition = try self.parseExpression(.lowest);
        const condition_ptr = try self.allocator.create(Expression);
        condition_ptr.* = condition;

        // Expect ')'
        if (self.currentToken().token_type != .RPAREN) {
            return ParserError.UnexpectedToken;
        }
        self.advance();

        // Expect '{'
        if (self.currentToken().token_type != .LBRACE) {
            return ParserError.UnexpectedToken;
        }

        // Parse body block
        const body = try self.parseBlockStatement();
        const body_ptr = try self.allocator.create(ast_mod.BlockStatement);
        body_ptr.* = body;

        // Skip semicolon if present
        if (self.currentToken().token_type == .SEMICOLON) {
            self.advance();
        }

        return ast_mod.WhileStatement{
            .token = token,
            .condition = condition_ptr,
            .body = body_ptr,
        };
    }

    fn parseForStatement(self: *Parser) ParserError!ast_mod.ForStatement {
        const token = self.currentToken();

        // Skip 'for'
        self.advance();

        // Expect '('
        if (self.currentToken().token_type != .LPAREN) {
            return ParserError.UnexpectedToken;
        }
        self.advance();

        // Parse loop variable (identifier)
        if (self.currentToken().token_type != .IDENT) {
            return ParserError.UnexpectedToken;
        }
        const variable = ast_mod.Identifier{
            .token = self.currentToken(),
            .value = self.currentToken().literal,
        };
        self.advance();

        // Expect 'in'
        if (self.currentToken().token_type != .IN) {
            return ParserError.UnexpectedToken;
        }
        self.advance();

        // Parse iterable expression
        const iterable = try self.parseExpression(.lowest);
        const iterable_ptr = try self.allocator.create(Expression);
        iterable_ptr.* = iterable;

        // Expect ')'
        if (self.currentToken().token_type != .RPAREN) {
            return ParserError.UnexpectedToken;
        }
        self.advance();

        // Expect '{'
        if (self.currentToken().token_type != .LBRACE) {
            return ParserError.UnexpectedToken;
        }

        // Parse body block
        const body = try self.parseBlockStatement();
        const body_ptr = try self.allocator.create(ast_mod.BlockStatement);
        body_ptr.* = body;

        // Skip semicolon if present
        if (self.currentToken().token_type == .SEMICOLON) {
            self.advance();
        }

        return ast_mod.ForStatement{
            .token = token,
            .variable = variable,
            .iterable = iterable_ptr,
            .body = body_ptr,
        };
    }

    fn parseBreakStatement(self: *Parser) ParserError!ast_mod.BreakStatement {
        const token = self.currentToken();

        // Skip 'break'
        self.advance();

        // Skip semicolon if present
        if (self.currentToken().token_type == .SEMICOLON) {
            self.advance();
        }

        return ast_mod.BreakStatement{
            .token = token,
        };
    }

    fn parseContinueStatement(self: *Parser) ParserError!ast_mod.ContinueStatement {
        const token = self.currentToken();

        // Skip 'continue'
        self.advance();

        // Skip semicolon if present
        if (self.currentToken().token_type == .SEMICOLON) {
            self.advance();
        }

        return ast_mod.ContinueStatement{
            .token = token,
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
                self.currentToken().token_type == .PERCENT or
                self.currentToken().token_type == .EQ or
                self.currentToken().token_type == .NOTEQ or
                self.currentToken().token_type == .LT or
                self.currentToken().token_type == .GT or
                self.currentToken().token_type == .LTE or
                self.currentToken().token_type == .GTE or
                self.currentToken().token_type == .AND or
                self.currentToken().token_type == .OR)
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

// =============================================================================
// Parser Tests
// =============================================================================

const Lexer = @import("lexer.zig").Lexer;

fn tokenize(allocator: std.mem.Allocator, input: []const u8) ![]Token {
    var lexer = Lexer.init(input);
    var tokens = try std.ArrayList(Token).initCapacity(allocator, 32);
    defer tokens.deinit(allocator);

    while (true) {
        const tok = lexer.nextToken();
        try tokens.append(allocator, tok);
        if (tok.token_type == .EOF) break;
    }

    return try tokens.toOwnedSlice(allocator);
}

test "parser: integer literal" {
    const allocator = std.testing.allocator;

    const tokens = try tokenize(allocator, "5;");
    defer allocator.free(tokens);

    var parser = Parser.init(allocator, tokens);
    const program = try parser.parseProgram();
    defer allocator.free(program.statements);

    try std.testing.expectEqual(@as(usize, 1), program.statements.len);

    const stmt = program.statements[0];
    const expr = stmt.expression.expression;
    try std.testing.expectEqual(@as(i64, 5), expr.integer_literal.value);
}

test "parser: boolean literals" {
    const allocator = std.testing.allocator;

    // Test true
    {
        const tokens = try tokenize(allocator, "true;");
        defer allocator.free(tokens);

        var parser = Parser.init(allocator, tokens);
        const program = try parser.parseProgram();
        defer allocator.free(program.statements);

        try std.testing.expectEqual(@as(usize, 1), program.statements.len);
        const expr = program.statements[0].expression.expression;
        try std.testing.expect(expr.boolean.value);
    }

    // Test false
    {
        const tokens = try tokenize(allocator, "false;");
        defer allocator.free(tokens);

        var parser = Parser.init(allocator, tokens);
        const program = try parser.parseProgram();
        defer allocator.free(program.statements);

        try std.testing.expectEqual(@as(usize, 1), program.statements.len);
        const expr = program.statements[0].expression.expression;
        try std.testing.expect(!expr.boolean.value);
    }
}

test "parser: identifier expression" {
    const allocator = std.testing.allocator;

    const tokens = try tokenize(allocator, "foobar;");
    defer allocator.free(tokens);

    var parser = Parser.init(allocator, tokens);
    const program = try parser.parseProgram();
    defer allocator.free(program.statements);

    try std.testing.expectEqual(@as(usize, 1), program.statements.len);

    const stmt = program.statements[0];
    const expr = stmt.expression.expression;
    try std.testing.expectEqualStrings("foobar", expr.identifier.value);
}

test "parser: prefix expressions" {
    const allocator = std.testing.allocator;

    // Test !5
    {
        const tokens = try tokenize(allocator, "!5;");
        defer allocator.free(tokens);

        var parser = Parser.init(allocator, tokens);
        const program = try parser.parseProgram();
        defer {
            allocator.destroy(program.statements[0].expression.expression.prefix.right);
            allocator.free(program.statements);
        }

        try std.testing.expectEqual(@as(usize, 1), program.statements.len);
        const expr = program.statements[0].expression.expression;
        try std.testing.expectEqualStrings("!", expr.prefix.operator);
        try std.testing.expectEqual(@as(i64, 5), expr.prefix.right.integer_literal.value);
    }

    // Test -15
    {
        const tokens = try tokenize(allocator, "-15;");
        defer allocator.free(tokens);

        var parser = Parser.init(allocator, tokens);
        const program = try parser.parseProgram();
        defer {
            allocator.destroy(program.statements[0].expression.expression.prefix.right);
            allocator.free(program.statements);
        }

        try std.testing.expectEqual(@as(usize, 1), program.statements.len);
        const expr = program.statements[0].expression.expression;
        try std.testing.expectEqualStrings("-", expr.prefix.operator);
        try std.testing.expectEqual(@as(i64, 15), expr.prefix.right.integer_literal.value);
    }
}

test "parser: infix expressions" {
    const allocator = std.testing.allocator;

    const TestCase = struct {
        input: []const u8,
        left_value: i64,
        operator: []const u8,
        right_value: i64,
    };

    const test_cases = [_]TestCase{
        .{ .input = "5 + 5;", .left_value = 5, .operator = "+", .right_value = 5 },
        .{ .input = "5 - 5;", .left_value = 5, .operator = "-", .right_value = 5 },
        .{ .input = "5 * 5;", .left_value = 5, .operator = "*", .right_value = 5 },
        .{ .input = "5 / 5;", .left_value = 5, .operator = "/", .right_value = 5 },
        .{ .input = "5 > 5;", .left_value = 5, .operator = ">", .right_value = 5 },
        .{ .input = "5 < 5;", .left_value = 5, .operator = "<", .right_value = 5 },
        .{ .input = "5 == 5;", .left_value = 5, .operator = "==", .right_value = 5 },
        .{ .input = "5 != 5;", .left_value = 5, .operator = "!=", .right_value = 5 },
    };

    for (test_cases) |tc| {
        const tokens = try tokenize(allocator, tc.input);
        defer allocator.free(tokens);

        var parser = Parser.init(allocator, tokens);
        const program = try parser.parseProgram();
        defer {
            const infix = program.statements[0].expression.expression.infix;
            allocator.destroy(infix.left);
            allocator.destroy(infix.right);
            allocator.free(program.statements);
        }

        try std.testing.expectEqual(@as(usize, 1), program.statements.len);

        const expr = program.statements[0].expression.expression;
        try std.testing.expectEqual(tc.left_value, expr.infix.left.integer_literal.value);
        try std.testing.expectEqualStrings(tc.operator, expr.infix.operator);
        try std.testing.expectEqual(tc.right_value, expr.infix.right.integer_literal.value);
    }
}

test "parser: operator precedence" {
    const allocator = std.testing.allocator;

    // Test: 1 + 2 * 3 should parse as 1 + (2 * 3)
    {
        const tokens = try tokenize(allocator, "1 + 2 * 3;");
        defer allocator.free(tokens);

        var parser = Parser.init(allocator, tokens);
        const program = try parser.parseProgram();
        defer {
            // Free the complex nested structure
            const outer_infix = program.statements[0].expression.expression.infix;
            const inner_infix = outer_infix.right.infix;
            allocator.destroy(inner_infix.left);
            allocator.destroy(inner_infix.right);
            allocator.destroy(outer_infix.left);
            allocator.destroy(outer_infix.right);
            allocator.free(program.statements);
        }

        try std.testing.expectEqual(@as(usize, 1), program.statements.len);

        const expr = program.statements[0].expression.expression;
        // Should be: (1) + ((2) * (3))
        try std.testing.expectEqualStrings("+", expr.infix.operator);
        try std.testing.expectEqual(@as(i64, 1), expr.infix.left.integer_literal.value);

        const right = expr.infix.right;
        try std.testing.expectEqualStrings("*", right.infix.operator);
        try std.testing.expectEqual(@as(i64, 2), right.infix.left.integer_literal.value);
        try std.testing.expectEqual(@as(i64, 3), right.infix.right.integer_literal.value);
    }
}

test "parser: let statement" {
    const allocator = std.testing.allocator;

    const tokens = try tokenize(allocator, "let x = 5;");
    defer allocator.free(tokens);

    var parser = Parser.init(allocator, tokens);
    const program = try parser.parseProgram();
    defer allocator.free(program.statements);

    try std.testing.expectEqual(@as(usize, 1), program.statements.len);

    const stmt = program.statements[0].let;
    try std.testing.expectEqualStrings("x", stmt.name.value);
    try std.testing.expectEqual(@as(i64, 5), stmt.value.integer_literal.value);
}

test "parser: return statement" {
    const allocator = std.testing.allocator;

    const tokens = try tokenize(allocator, "return 10;");
    defer allocator.free(tokens);

    var parser = Parser.init(allocator, tokens);
    const program = try parser.parseProgram();
    defer allocator.free(program.statements);

    try std.testing.expectEqual(@as(usize, 1), program.statements.len);

    const stmt = program.statements[0].return_stmt;
    try std.testing.expectEqual(@as(i64, 10), stmt.return_value.integer_literal.value);
}

test "parser: if expression" {
    const allocator = std.testing.allocator;

    const tokens = try tokenize(allocator, "if (x < y) { x }");
    defer allocator.free(tokens);

    var parser = Parser.init(allocator, tokens);
    const program = try parser.parseProgram();
    defer {
        const if_expr = program.statements[0].expression.expression.if_expression;
        const cond_infix = if_expr.condition.infix;
        allocator.destroy(cond_infix.left);
        allocator.destroy(cond_infix.right);
        allocator.destroy(if_expr.condition);
        allocator.free(if_expr.consequence.statements);
        allocator.destroy(if_expr.consequence);
        allocator.free(program.statements);
    }

    try std.testing.expectEqual(@as(usize, 1), program.statements.len);

    const expr = program.statements[0].expression.expression;
    const if_expr = expr.if_expression;

    // Condition: x < y
    try std.testing.expectEqualStrings("<", if_expr.condition.infix.operator);

    // Consequence has 1 statement
    try std.testing.expectEqual(@as(usize, 1), if_expr.consequence.statements.len);

    // No alternative
    try std.testing.expect(if_expr.alternative == null);
}

test "parser: if-else expression" {
    const allocator = std.testing.allocator;

    const tokens = try tokenize(allocator, "if (x < y) { x } else { y }");
    defer allocator.free(tokens);

    var parser = Parser.init(allocator, tokens);
    const program = try parser.parseProgram();
    defer {
        const if_expr = program.statements[0].expression.expression.if_expression;
        const cond_infix = if_expr.condition.infix;
        allocator.destroy(cond_infix.left);
        allocator.destroy(cond_infix.right);
        allocator.destroy(if_expr.condition);
        allocator.free(if_expr.consequence.statements);
        allocator.destroy(if_expr.consequence);
        if (if_expr.alternative) |alt| {
            allocator.free(alt.statements);
            allocator.destroy(alt);
        }
        allocator.free(program.statements);
    }

    try std.testing.expectEqual(@as(usize, 1), program.statements.len);

    const expr = program.statements[0].expression.expression;
    const if_expr = expr.if_expression;

    // Has alternative
    try std.testing.expect(if_expr.alternative != null);
    try std.testing.expectEqual(@as(usize, 1), if_expr.alternative.?.statements.len);
}

test "parser: function literal" {
    const allocator = std.testing.allocator;

    const tokens = try tokenize(allocator, "fn(x, y) { x + y; }");
    defer allocator.free(tokens);

    var parser = Parser.init(allocator, tokens);
    const program = try parser.parseProgram();
    defer {
        const fn_lit = program.statements[0].expression.expression.function_literal;
        allocator.free(fn_lit.parameters);
        const stmt = fn_lit.body.statements[0];
        const infix = stmt.expression.expression.infix;
        allocator.destroy(infix.left);
        allocator.destroy(infix.right);
        allocator.free(fn_lit.body.statements);
        allocator.destroy(fn_lit.body);
        allocator.free(program.statements);
    }

    try std.testing.expectEqual(@as(usize, 1), program.statements.len);

    const fn_lit = program.statements[0].expression.expression.function_literal;

    // Check parameters
    try std.testing.expectEqual(@as(usize, 2), fn_lit.parameters.len);
    try std.testing.expectEqualStrings("x", fn_lit.parameters[0].value);
    try std.testing.expectEqualStrings("y", fn_lit.parameters[1].value);

    // Check body has 1 statement
    try std.testing.expectEqual(@as(usize, 1), fn_lit.body.statements.len);
}

test "parser: call expression" {
    const allocator = std.testing.allocator;

    const tokens = try tokenize(allocator, "add(1, 2 * 3);");
    defer allocator.free(tokens);

    var parser = Parser.init(allocator, tokens);
    const program = try parser.parseProgram();
    defer {
        const call = program.statements[0].expression.expression.call;
        allocator.destroy(call.function);
        const arg2_infix = call.arguments[1].infix;
        allocator.destroy(arg2_infix.left);
        allocator.destroy(arg2_infix.right);
        allocator.free(call.arguments);
        allocator.free(program.statements);
    }

    try std.testing.expectEqual(@as(usize, 1), program.statements.len);

    const call = program.statements[0].expression.expression.call;

    // Check function name
    try std.testing.expectEqualStrings("add", call.function.identifier.value);

    // Check arguments
    try std.testing.expectEqual(@as(usize, 2), call.arguments.len);
    try std.testing.expectEqual(@as(i64, 1), call.arguments[0].integer_literal.value);
    try std.testing.expectEqualStrings("*", call.arguments[1].infix.operator);
}

test "parser: string literal" {
    const allocator = std.testing.allocator;

    const tokens = try tokenize(allocator, "\"hello world\";");
    defer allocator.free(tokens);

    var parser = Parser.init(allocator, tokens);
    const program = try parser.parseProgram();
    defer allocator.free(program.statements);

    try std.testing.expectEqual(@as(usize, 1), program.statements.len);

    const expr = program.statements[0].expression.expression;
    try std.testing.expectEqualStrings("hello world", expr.string_literal.value);
}

test "parser: array literal" {
    const allocator = std.testing.allocator;

    const tokens = try tokenize(allocator, "[1, 2, 3];");
    defer allocator.free(tokens);

    var parser = Parser.init(allocator, tokens);
    const program = try parser.parseProgram();
    defer {
        const arr = program.statements[0].expression.expression.array_literal;
        allocator.free(arr.elements);
        allocator.free(program.statements);
    }

    try std.testing.expectEqual(@as(usize, 1), program.statements.len);

    const arr = program.statements[0].expression.expression.array_literal;
    try std.testing.expectEqual(@as(usize, 3), arr.elements.len);
    try std.testing.expectEqual(@as(i64, 1), arr.elements[0].integer_literal.value);
    try std.testing.expectEqual(@as(i64, 2), arr.elements[1].integer_literal.value);
    try std.testing.expectEqual(@as(i64, 3), arr.elements[2].integer_literal.value);
}

test "parser: index expression" {
    const allocator = std.testing.allocator;

    const tokens = try tokenize(allocator, "myArray[1 + 1];");
    defer allocator.free(tokens);

    var parser = Parser.init(allocator, tokens);
    const program = try parser.parseProgram();
    defer {
        const idx = program.statements[0].expression.expression.index_expression;
        allocator.destroy(idx.left);
        const idx_infix = idx.index.infix;
        allocator.destroy(idx_infix.left);
        allocator.destroy(idx_infix.right);
        allocator.destroy(idx.index);
        allocator.free(program.statements);
    }

    try std.testing.expectEqual(@as(usize, 1), program.statements.len);

    const idx = program.statements[0].expression.expression.index_expression;
    try std.testing.expectEqualStrings("myArray", idx.left.identifier.value);
    try std.testing.expectEqualStrings("+", idx.index.infix.operator);
}

test "parser: hash literal with string keys" {
    const allocator = std.testing.allocator;

    const tokens = try tokenize(allocator, "{\"one\": 1, \"two\": 2};");
    defer allocator.free(tokens);

    var parser = Parser.init(allocator, tokens);
    const program = try parser.parseProgram();
    defer {
        const hash = program.statements[0].expression.expression.hash_literal;
        for (hash.pairs) |pair| {
            allocator.destroy(pair.key);
            allocator.destroy(pair.value);
        }
        allocator.free(hash.pairs);
        allocator.free(program.statements);
    }

    try std.testing.expectEqual(@as(usize, 1), program.statements.len);

    const hash = program.statements[0].expression.expression.hash_literal;
    try std.testing.expectEqual(@as(usize, 2), hash.pairs.len);

    try std.testing.expectEqualStrings("one", hash.pairs[0].key.string_literal.value);
    try std.testing.expectEqual(@as(i64, 1), hash.pairs[0].value.integer_literal.value);
    try std.testing.expectEqualStrings("two", hash.pairs[1].key.string_literal.value);
    try std.testing.expectEqual(@as(i64, 2), hash.pairs[1].value.integer_literal.value);
}

test "parser: empty hash literal" {
    const allocator = std.testing.allocator;

    const tokens = try tokenize(allocator, "{};");
    defer allocator.free(tokens);

    var parser = Parser.init(allocator, tokens);
    const program = try parser.parseProgram();
    defer {
        const hash = program.statements[0].expression.expression.hash_literal;
        allocator.free(hash.pairs);
        allocator.free(program.statements);
    }

    try std.testing.expectEqual(@as(usize, 1), program.statements.len);

    const hash = program.statements[0].expression.expression.hash_literal;
    try std.testing.expectEqual(@as(usize, 0), hash.pairs.len);
}

test "parser: grouped expression" {
    const allocator = std.testing.allocator;

    // Test: (5 + 5) * 2 should parse with correct precedence
    {
        const tokens = try tokenize(allocator, "(5 + 5) * 2;");
        defer allocator.free(tokens);

        var parser = Parser.init(allocator, tokens);
        const program = try parser.parseProgram();
        defer {
            const outer_infix = program.statements[0].expression.expression.infix;
            const inner_infix = outer_infix.left.infix;
            allocator.destroy(inner_infix.left);
            allocator.destroy(inner_infix.right);
            allocator.destroy(outer_infix.left);
            allocator.destroy(outer_infix.right);
            allocator.free(program.statements);
        }

        try std.testing.expectEqual(@as(usize, 1), program.statements.len);

        const expr = program.statements[0].expression.expression;
        // Should be: ((5) + (5)) * (2)
        try std.testing.expectEqualStrings("*", expr.infix.operator);

        const left = expr.infix.left;
        try std.testing.expectEqualStrings("+", left.infix.operator);
        try std.testing.expectEqual(@as(i64, 2), expr.infix.right.integer_literal.value);
    }
}

test "parser: multiple statements" {
    const allocator = std.testing.allocator;

    const tokens = try tokenize(allocator, "let x = 5; let y = 10; let z = x + y;");
    defer allocator.free(tokens);

    var parser = Parser.init(allocator, tokens);
    const program = try parser.parseProgram();
    defer {
        // Free the infix expression in the third statement
        const z_infix = program.statements[2].let.value.infix;
        allocator.destroy(z_infix.left);
        allocator.destroy(z_infix.right);
        allocator.free(program.statements);
    }

    try std.testing.expectEqual(@as(usize, 3), program.statements.len);

    try std.testing.expectEqualStrings("x", program.statements[0].let.name.value);
    try std.testing.expectEqualStrings("y", program.statements[1].let.name.value);
    try std.testing.expectEqualStrings("z", program.statements[2].let.name.value);
}

// =============================================================================
// Index Assignment Tests
// =============================================================================

test "parser: index assignment - array" {
    const allocator = std.testing.allocator;

    const tokens = try tokenize(allocator, "arr[0] = 10;");
    defer allocator.free(tokens);

    var parser = Parser.init(allocator, tokens);
    const program = try parser.parseProgram();
    defer {
        const idx_assign = program.statements[0].index_assignment;
        allocator.destroy(idx_assign.left);
        allocator.destroy(idx_assign.index);
        allocator.destroy(idx_assign.value);
        allocator.free(program.statements);
    }

    try std.testing.expectEqual(@as(usize, 1), program.statements.len);

    const idx_assign = program.statements[0].index_assignment;
    try std.testing.expectEqualStrings("arr", idx_assign.left.identifier.value);
    try std.testing.expectEqual(@as(i64, 0), idx_assign.index.integer_literal.value);
    try std.testing.expectEqual(@as(i64, 10), idx_assign.value.integer_literal.value);
}

test "parser: index assignment - hash with string key" {
    const allocator = std.testing.allocator;

    const tokens = try tokenize(allocator, "hash[\"key\"] = \"value\";");
    defer allocator.free(tokens);

    var parser = Parser.init(allocator, tokens);
    const program = try parser.parseProgram();
    defer {
        const idx_assign = program.statements[0].index_assignment;
        allocator.destroy(idx_assign.left);
        allocator.destroy(idx_assign.index);
        allocator.destroy(idx_assign.value);
        allocator.free(program.statements);
    }

    try std.testing.expectEqual(@as(usize, 1), program.statements.len);

    const idx_assign = program.statements[0].index_assignment;
    try std.testing.expectEqualStrings("hash", idx_assign.left.identifier.value);
    try std.testing.expectEqualStrings("key", idx_assign.index.string_literal.value);
    try std.testing.expectEqualStrings("value", idx_assign.value.string_literal.value);
}

test "parser: index assignment with expression value" {
    const allocator = std.testing.allocator;

    const tokens = try tokenize(allocator, "arr[0] = 1 + 2;");
    defer allocator.free(tokens);

    var parser = Parser.init(allocator, tokens);
    const program = try parser.parseProgram();
    defer {
        const idx_assign = program.statements[0].index_assignment;
        allocator.destroy(idx_assign.left);
        allocator.destroy(idx_assign.index);
        const val_infix = idx_assign.value.infix;
        allocator.destroy(val_infix.left);
        allocator.destroy(val_infix.right);
        allocator.destroy(idx_assign.value);
        allocator.free(program.statements);
    }

    try std.testing.expectEqual(@as(usize, 1), program.statements.len);

    const idx_assign = program.statements[0].index_assignment;
    try std.testing.expectEqualStrings("arr", idx_assign.left.identifier.value);
    try std.testing.expectEqual(@as(i64, 0), idx_assign.index.integer_literal.value);
    try std.testing.expectEqualStrings("+", idx_assign.value.infix.operator);
}
