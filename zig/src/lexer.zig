const std = @import("std");
const token_mod = @import("token.zig");
pub const Token = token_mod.Token;
const TokenType = token_mod.TokenType;

pub const Lexer = struct {
    input: []const u8,
    position: usize,
    read_position: usize,
    ch: u8,
    line: usize,
    column: usize,

    pub fn init(input: []const u8) Lexer {
        var l = Lexer{
            .input = input,
            .position = 0,
            .read_position = 0,
            .ch = 0,
            .line = 1,
            .column = 0,
        };
        l.readChar();
        return l;
    }

    fn readChar(self: *Lexer) void {
        if (self.read_position >= self.input.len) {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;

        // Track line and column
        if (self.ch == '\n') {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
    }

    fn peekChar(self: *const Lexer) u8 {
        if (self.read_position >= self.input.len) {
            return 0;
        }
        return self.input[self.read_position];
    }

    fn skipWhitespaceAndComments(self: *Lexer) void {
        while (true) {
            // Skip whitespace
            while (self.ch == ' ' or self.ch == '\t' or self.ch == '\n' or self.ch == '\r') {
                self.readChar();
            }

            // Skip single-line comments
            if (self.ch == '/' and self.peekChar() == '/') {
                while (self.ch != '\n' and self.ch != 0) {
                    self.readChar();
                }
                continue;
            }

            // Skip multi-line comments
            if (self.ch == '/' and self.peekChar() == '*') {
                self.readChar(); // skip '/'
                self.readChar(); // skip '*'
                while (!(self.ch == '*' and self.peekChar() == '/') and self.ch != 0) {
                    self.readChar();
                }
                if (self.ch != 0) {
                    self.readChar(); // skip '*'
                    self.readChar(); // skip '/'
                }
                continue;
            }

            break;
        }
    }

    pub fn nextToken(self: *Lexer) Token {
        self.skipWhitespaceAndComments();

        // Save position before reading token
        const start_line = self.line;
        const start_column = self.column;

        const tok = switch (self.ch) {
            '=' => blk: {
                if (self.peekChar() == '=') {
                    self.readChar();
                    break :blk Token{ .token_type = .EQ, .literal = "==", .line = start_line, .column = start_column };
                } else {
                    break :blk Token{ .token_type = .ASSIGN, .literal = "=", .line = start_line, .column = start_column };
                }
            },
            '+' => Token{ .token_type = .PLUS, .literal = "+", .line = start_line, .column = start_column },
            '-' => Token{ .token_type = .MINUS, .literal = "-", .line = start_line, .column = start_column },
            '!' => blk: {
                if (self.peekChar() == '=') {
                    self.readChar();
                    break :blk Token{ .token_type = .NOTEQ, .literal = "!=", .line = start_line, .column = start_column };
                } else {
                    break :blk Token{ .token_type = .BANG, .literal = "!", .line = start_line, .column = start_column };
                }
            },
            '*' => Token{ .token_type = .ASTERISK, .literal = "*", .line = start_line, .column = start_column },
            '/' => Token{ .token_type = .SLASH, .literal = "/", .line = start_line, .column = start_column },
            '%' => Token{ .token_type = .PERCENT, .literal = "%", .line = start_line, .column = start_column },
            '<' => blk: {
                if (self.peekChar() == '=') {
                    self.readChar();
                    break :blk Token{ .token_type = .LTE, .literal = "<=", .line = start_line, .column = start_column };
                } else {
                    break :blk Token{ .token_type = .LT, .literal = "<", .line = start_line, .column = start_column };
                }
            },
            '>' => blk: {
                if (self.peekChar() == '=') {
                    self.readChar();
                    break :blk Token{ .token_type = .GTE, .literal = ">=", .line = start_line, .column = start_column };
                } else {
                    break :blk Token{ .token_type = .GT, .literal = ">", .line = start_line, .column = start_column };
                }
            },
            '&' => blk: {
                if (self.peekChar() == '&') {
                    self.readChar();
                    break :blk Token{ .token_type = .AND, .literal = "&&", .line = start_line, .column = start_column };
                } else {
                    break :blk Token{ .token_type = .ILLEGAL, .literal = "&", .line = start_line, .column = start_column };
                }
            },
            '|' => blk: {
                if (self.peekChar() == '|') {
                    self.readChar();
                    break :blk Token{ .token_type = .OR, .literal = "||", .line = start_line, .column = start_column };
                } else {
                    break :blk Token{ .token_type = .ILLEGAL, .literal = "|", .line = start_line, .column = start_column };
                }
            },
            ';' => Token{ .token_type = .SEMICOLON, .literal = ";", .line = start_line, .column = start_column },
            '(' => Token{ .token_type = .LPAREN, .literal = "(", .line = start_line, .column = start_column },
            ')' => Token{ .token_type = .RPAREN, .literal = ")", .line = start_line, .column = start_column },
            ',' => Token{ .token_type = .COMMA, .literal = ",", .line = start_line, .column = start_column },
            '{' => Token{ .token_type = .LBRACE, .literal = "{", .line = start_line, .column = start_column },
            '}' => Token{ .token_type = .RBRACE, .literal = "}", .line = start_line, .column = start_column },
            '[' => Token{ .token_type = .LBRACKET, .literal = "[", .line = start_line, .column = start_column },
            ']' => Token{ .token_type = .RBRACKET, .literal = "]", .line = start_line, .column = start_column },
            ':' => Token{ .token_type = .COLON, .literal = ":", .line = start_line, .column = start_column },
            '"' => blk: {
                // Read string literal
                const str_start_line = start_line;
                const str_start_column = start_column;
                const start = self.position + 1; // Skip opening quote
                self.readChar(); // Move past opening quote

                while (self.ch != '"' and self.ch != 0) {
                    self.readChar();
                }

                if (self.ch == 0) {
                    // Unterminated string
                    break :blk Token{ .token_type = .ILLEGAL, .literal = "unterminated string", .line = str_start_line, .column = str_start_column };
                }

                const literal = self.input[start..self.position];
                break :blk Token{ .token_type = .STRING, .literal = literal, .line = str_start_line, .column = str_start_column };
            },
            0 => Token{ .token_type = .EOF, .literal = "", .line = start_line, .column = start_column },
            else => {
                if (std.ascii.isDigit(self.ch)) {
                    // Read number
                    const start = self.position;
                    while (std.ascii.isDigit(self.ch)) {
                        self.readChar();
                    }
                    const literal = self.input[start..self.position];
                    return Token{ .token_type = .INT, .literal = literal, .line = start_line, .column = start_column };
                } else if (std.ascii.isAlphabetic(self.ch) or self.ch == '_') {
                    // Read identifier
                    const start = self.position;
                    while (std.ascii.isAlphanumeric(self.ch) or self.ch == '_') {
                        self.readChar();
                    }
                    const literal = self.input[start..self.position];
                    const token_type = token_mod.lookupIdent(literal);
                    return Token{ .token_type = token_type, .literal = literal, .line = start_line, .column = start_column };
                } else {
                    return Token{ .token_type = .ILLEGAL, .literal = &[_]u8{self.ch}, .line = start_line, .column = start_column };
                }
            },
        };

        self.readChar();
        return tok;
    }
};

// Unit tests for Lexer
test "lexer basic tokens" {
    const input = "=+(){},;";
    var lexer = Lexer.init(input);

    const expected = [_]struct { token_type: TokenType, literal: []const u8 }{
        .{ .token_type = .ASSIGN, .literal = "=" },
        .{ .token_type = .PLUS, .literal = "+" },
        .{ .token_type = .LPAREN, .literal = "(" },
        .{ .token_type = .RPAREN, .literal = ")" },
        .{ .token_type = .LBRACE, .literal = "{" },
        .{ .token_type = .RBRACE, .literal = "}" },
        .{ .token_type = .COMMA, .literal = "," },
        .{ .token_type = .SEMICOLON, .literal = ";" },
        .{ .token_type = .EOF, .literal = "" },
    };

    for (expected) |exp| {
        const tok = lexer.nextToken();
        try std.testing.expectEqual(exp.token_type, tok.token_type);
        try std.testing.expectEqualStrings(exp.literal, tok.literal);
    }
}

test "lexer complete program" {
    const input =
        \\let five = 5;
        \\let ten = 10;
        \\let add = fn(x, y) {
        \\  x + y;
        \\};
        \\let result = add(five, ten);
    ;
    var lexer = Lexer.init(input);

    const expected = [_]struct { token_type: TokenType, literal: []const u8 }{
        .{ .token_type = .LET, .literal = "let" },
        .{ .token_type = .IDENT, .literal = "five" },
        .{ .token_type = .ASSIGN, .literal = "=" },
        .{ .token_type = .INT, .literal = "5" },
        .{ .token_type = .SEMICOLON, .literal = ";" },
        .{ .token_type = .LET, .literal = "let" },
        .{ .token_type = .IDENT, .literal = "ten" },
        .{ .token_type = .ASSIGN, .literal = "=" },
        .{ .token_type = .INT, .literal = "10" },
        .{ .token_type = .SEMICOLON, .literal = ";" },
        .{ .token_type = .LET, .literal = "let" },
        .{ .token_type = .IDENT, .literal = "add" },
        .{ .token_type = .ASSIGN, .literal = "=" },
        .{ .token_type = .FUNCTION, .literal = "fn" },
        .{ .token_type = .LPAREN, .literal = "(" },
        .{ .token_type = .IDENT, .literal = "x" },
        .{ .token_type = .COMMA, .literal = "," },
        .{ .token_type = .IDENT, .literal = "y" },
        .{ .token_type = .RPAREN, .literal = ")" },
        .{ .token_type = .LBRACE, .literal = "{" },
        .{ .token_type = .IDENT, .literal = "x" },
        .{ .token_type = .PLUS, .literal = "+" },
        .{ .token_type = .IDENT, .literal = "y" },
        .{ .token_type = .SEMICOLON, .literal = ";" },
        .{ .token_type = .RBRACE, .literal = "}" },
        .{ .token_type = .SEMICOLON, .literal = ";" },
        .{ .token_type = .LET, .literal = "let" },
        .{ .token_type = .IDENT, .literal = "result" },
        .{ .token_type = .ASSIGN, .literal = "=" },
        .{ .token_type = .IDENT, .literal = "add" },
        .{ .token_type = .LPAREN, .literal = "(" },
        .{ .token_type = .IDENT, .literal = "five" },
        .{ .token_type = .COMMA, .literal = "," },
        .{ .token_type = .IDENT, .literal = "ten" },
        .{ .token_type = .RPAREN, .literal = ")" },
        .{ .token_type = .SEMICOLON, .literal = ";" },
        .{ .token_type = .EOF, .literal = "" },
    };

    for (expected) |exp| {
        const tok = lexer.nextToken();
        try std.testing.expectEqual(exp.token_type, tok.token_type);
        try std.testing.expectEqualStrings(exp.literal, tok.literal);
    }
}

test "lexer operators" {
    const input = "!-/+5; 5 < 10 > 5;";
    var lexer = Lexer.init(input);

    const expected = [_]struct { token_type: TokenType, literal: []const u8 }{
        .{ .token_type = .BANG, .literal = "!" },
        .{ .token_type = .MINUS, .literal = "-" },
        .{ .token_type = .SLASH, .literal = "/" },
        .{ .token_type = .PLUS, .literal = "+" },
        .{ .token_type = .INT, .literal = "5" },
        .{ .token_type = .SEMICOLON, .literal = ";" },
        .{ .token_type = .INT, .literal = "5" },
        .{ .token_type = .LT, .literal = "<" },
        .{ .token_type = .INT, .literal = "10" },
        .{ .token_type = .GT, .literal = ">" },
        .{ .token_type = .INT, .literal = "5" },
        .{ .token_type = .SEMICOLON, .literal = ";" },
        .{ .token_type = .EOF, .literal = "" },
    };

    for (expected) |exp| {
        const tok = lexer.nextToken();
        try std.testing.expectEqual(exp.token_type, tok.token_type);
        try std.testing.expectEqualStrings(exp.literal, tok.literal);
    }
}

test "lexer keywords" {
    const input = "if else return true false fn let";
    var lexer = Lexer.init(input);

    const expected = [_]struct { token_type: TokenType, literal: []const u8 }{
        .{ .token_type = .IF, .literal = "if" },
        .{ .token_type = .ELSE, .literal = "else" },
        .{ .token_type = .RETURN, .literal = "return" },
        .{ .token_type = .TRUE, .literal = "true" },
        .{ .token_type = .FALSE, .literal = "false" },
        .{ .token_type = .FUNCTION, .literal = "fn" },
        .{ .token_type = .LET, .literal = "let" },
        .{ .token_type = .EOF, .literal = "" },
    };

    for (expected) |exp| {
        const tok = lexer.nextToken();
        try std.testing.expectEqual(exp.token_type, tok.token_type);
        try std.testing.expectEqualStrings(exp.literal, tok.literal);
    }
}

test "lexer two character operators" {
    const input = "10 == 10; 10 != 9;";
    var lexer = Lexer.init(input);

    const expected = [_]struct { token_type: TokenType, literal: []const u8 }{
        .{ .token_type = .INT, .literal = "10" },
        .{ .token_type = .EQ, .literal = "==" },
        .{ .token_type = .INT, .literal = "10" },
        .{ .token_type = .SEMICOLON, .literal = ";" },
        .{ .token_type = .INT, .literal = "10" },
        .{ .token_type = .NOTEQ, .literal = "!=" },
        .{ .token_type = .INT, .literal = "9" },
        .{ .token_type = .SEMICOLON, .literal = ";" },
        .{ .token_type = .EOF, .literal = "" },
    };

    for (expected) |exp| {
        const tok = lexer.nextToken();
        try std.testing.expectEqual(exp.token_type, tok.token_type);
        try std.testing.expectEqualStrings(exp.literal, tok.literal);
    }
}

test "lexer string literals" {
    const input =
        \\"foobar"
        \\"foo bar"
    ;
    var lexer = Lexer.init(input);

    const tok1 = lexer.nextToken();
    try std.testing.expectEqual(TokenType.STRING, tok1.token_type);
    try std.testing.expectEqualStrings("foobar", tok1.literal);

    const tok2 = lexer.nextToken();
    try std.testing.expectEqual(TokenType.STRING, tok2.token_type);
    try std.testing.expectEqualStrings("foo bar", tok2.literal);
}

test "lexer array literals" {
    const input = "[1, 2];";
    var lexer = Lexer.init(input);

    const expected = [_]struct { token_type: TokenType, literal: []const u8 }{
        .{ .token_type = .LBRACKET, .literal = "[" },
        .{ .token_type = .INT, .literal = "1" },
        .{ .token_type = .COMMA, .literal = "," },
        .{ .token_type = .INT, .literal = "2" },
        .{ .token_type = .RBRACKET, .literal = "]" },
        .{ .token_type = .SEMICOLON, .literal = ";" },
        .{ .token_type = .EOF, .literal = "" },
    };

    for (expected) |exp| {
        const tok = lexer.nextToken();
        try std.testing.expectEqual(exp.token_type, tok.token_type);
        try std.testing.expectEqualStrings(exp.literal, tok.literal);
    }
}

test "lexer hash literals" {
    const input = "{\"foo\": \"bar\"}";
    var lexer = Lexer.init(input);

    const expected = [_]struct { token_type: TokenType, literal: []const u8 }{
        .{ .token_type = .LBRACE, .literal = "{" },
        .{ .token_type = .STRING, .literal = "foo" },
        .{ .token_type = .COLON, .literal = ":" },
        .{ .token_type = .STRING, .literal = "bar" },
        .{ .token_type = .RBRACE, .literal = "}" },
        .{ .token_type = .EOF, .literal = "" },
    };

    for (expected) |exp| {
        const tok = lexer.nextToken();
        try std.testing.expectEqual(exp.token_type, tok.token_type);
        try std.testing.expectEqualStrings(exp.literal, tok.literal);
    }
}

test "lexer new operators" {
    const input = "5 <= 10; 10 >= 5; 10 % 3; true && false; true || false; while";
    var lexer = Lexer.init(input);

    const expected = [_]struct { token_type: TokenType, literal: []const u8 }{
        .{ .token_type = .INT, .literal = "5" },
        .{ .token_type = .LTE, .literal = "<=" },
        .{ .token_type = .INT, .literal = "10" },
        .{ .token_type = .SEMICOLON, .literal = ";" },
        .{ .token_type = .INT, .literal = "10" },
        .{ .token_type = .GTE, .literal = ">=" },
        .{ .token_type = .INT, .literal = "5" },
        .{ .token_type = .SEMICOLON, .literal = ";" },
        .{ .token_type = .INT, .literal = "10" },
        .{ .token_type = .PERCENT, .literal = "%" },
        .{ .token_type = .INT, .literal = "3" },
        .{ .token_type = .SEMICOLON, .literal = ";" },
        .{ .token_type = .TRUE, .literal = "true" },
        .{ .token_type = .AND, .literal = "&&" },
        .{ .token_type = .FALSE, .literal = "false" },
        .{ .token_type = .SEMICOLON, .literal = ";" },
        .{ .token_type = .TRUE, .literal = "true" },
        .{ .token_type = .OR, .literal = "||" },
        .{ .token_type = .FALSE, .literal = "false" },
        .{ .token_type = .SEMICOLON, .literal = ";" },
        .{ .token_type = .WHILE, .literal = "while" },
        .{ .token_type = .EOF, .literal = "" },
    };

    for (expected) |exp| {
        const tok = lexer.nextToken();
        try std.testing.expectEqual(exp.token_type, tok.token_type);
        try std.testing.expectEqualStrings(exp.literal, tok.literal);
    }
}

test "lexer tracks line and column" {
    const input =
        \\let x = 5;
        \\let y = 10;
    ;
    var lexer = Lexer.init(input);

    // First line: "let x = 5;"
    const tok1 = lexer.nextToken(); // let
    try std.testing.expectEqual(@as(usize, 1), tok1.line);
    try std.testing.expectEqual(@as(usize, 1), tok1.column);

    const tok2 = lexer.nextToken(); // x
    try std.testing.expectEqual(@as(usize, 1), tok2.line);
    try std.testing.expectEqual(@as(usize, 5), tok2.column);

    const tok3 = lexer.nextToken(); // =
    try std.testing.expectEqual(@as(usize, 1), tok3.line);
    try std.testing.expectEqual(@as(usize, 7), tok3.column);

    const tok4 = lexer.nextToken(); // 5
    try std.testing.expectEqual(@as(usize, 1), tok4.line);
    try std.testing.expectEqual(@as(usize, 9), tok4.column);

    const tok5 = lexer.nextToken(); // ;
    try std.testing.expectEqual(@as(usize, 1), tok5.line);
    try std.testing.expectEqual(@as(usize, 10), tok5.column);

    // Second line: "let y = 10;"
    const tok6 = lexer.nextToken(); // let
    try std.testing.expectEqual(@as(usize, 2), tok6.line);
    try std.testing.expectEqual(@as(usize, 1), tok6.column);

    const tok7 = lexer.nextToken(); // y
    try std.testing.expectEqual(@as(usize, 2), tok7.line);
    try std.testing.expectEqual(@as(usize, 5), tok7.column);
}
