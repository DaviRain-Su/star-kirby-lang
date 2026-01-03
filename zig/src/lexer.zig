const std = @import("std");
const token_mod = @import("token.zig");
pub const Token = token_mod.Token;
const TokenType = token_mod.TokenType;

pub const Lexer = struct {
    input: []const u8,
    position: usize,
    read_position: usize,
    ch: u8,

    pub fn init(input: []const u8) Lexer {
        var l = Lexer{
            .input = input,
            .position = 0,
            .read_position = 0,
            .ch = 0,
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

        const tok = switch (self.ch) {
            '=' => blk: {
                if (self.peekChar() == '=') {
                    self.readChar();
                    break :blk Token{ .token_type = .EQ, .literal = "==" };
                } else {
                    break :blk Token{ .token_type = .ASSIGN, .literal = "=" };
                }
            },
            '+' => Token{ .token_type = .PLUS, .literal = "+" },
            '-' => Token{ .token_type = .MINUS, .literal = "-" },
            '!' => blk: {
                if (self.peekChar() == '=') {
                    self.readChar();
                    break :blk Token{ .token_type = .NOTEQ, .literal = "!=" };
                } else {
                    break :blk Token{ .token_type = .BANG, .literal = "!" };
                }
            },
            '*' => Token{ .token_type = .ASTERISK, .literal = "*" },
            '/' => Token{ .token_type = .SLASH, .literal = "/" },
            '%' => Token{ .token_type = .PERCENT, .literal = "%" },
            '<' => blk: {
                if (self.peekChar() == '=') {
                    self.readChar();
                    break :blk Token{ .token_type = .LTE, .literal = "<=" };
                } else {
                    break :blk Token{ .token_type = .LT, .literal = "<" };
                }
            },
            '>' => blk: {
                if (self.peekChar() == '=') {
                    self.readChar();
                    break :blk Token{ .token_type = .GTE, .literal = ">=" };
                } else {
                    break :blk Token{ .token_type = .GT, .literal = ">" };
                }
            },
            '&' => blk: {
                if (self.peekChar() == '&') {
                    self.readChar();
                    break :blk Token{ .token_type = .AND, .literal = "&&" };
                } else {
                    break :blk Token{ .token_type = .ILLEGAL, .literal = "&" };
                }
            },
            '|' => blk: {
                if (self.peekChar() == '|') {
                    self.readChar();
                    break :blk Token{ .token_type = .OR, .literal = "||" };
                } else {
                    break :blk Token{ .token_type = .ILLEGAL, .literal = "|" };
                }
            },
            ';' => Token{ .token_type = .SEMICOLON, .literal = ";" },
            '(' => Token{ .token_type = .LPAREN, .literal = "(" },
            ')' => Token{ .token_type = .RPAREN, .literal = ")" },
            ',' => Token{ .token_type = .COMMA, .literal = "," },
            '{' => Token{ .token_type = .LBRACE, .literal = "{" },
            '}' => Token{ .token_type = .RBRACE, .literal = "}" },
            '[' => Token{ .token_type = .LBRACKET, .literal = "[" },
            ']' => Token{ .token_type = .RBRACKET, .literal = "]" },
            ':' => Token{ .token_type = .COLON, .literal = ":" },
            '"' => blk: {
                // Read string literal
                const start = self.position + 1; // Skip opening quote
                self.readChar(); // Move past opening quote

                while (self.ch != '"' and self.ch != 0) {
                    self.readChar();
                }

                if (self.ch == 0) {
                    // Unterminated string
                    break :blk Token{ .token_type = .ILLEGAL, .literal = "unterminated string" };
                }

                const literal = self.input[start..self.position];
                break :blk Token{ .token_type = .STRING, .literal = literal };
            },
            0 => Token{ .token_type = .EOF, .literal = "" },
            else => {
                if (std.ascii.isDigit(self.ch)) {
                    // Read number
                    const start = self.position;
                    while (std.ascii.isDigit(self.ch)) {
                        self.readChar();
                    }
                    const literal = self.input[start..self.position];
                    return Token{ .token_type = .INT, .literal = literal };
                } else if (std.ascii.isAlphabetic(self.ch) or self.ch == '_') {
                    // Read identifier
                    const start = self.position;
                    while (std.ascii.isAlphanumeric(self.ch) or self.ch == '_') {
                        self.readChar();
                    }
                    const literal = self.input[start..self.position];
                    const token_type = token_mod.lookupIdent(literal);
                    return Token{ .token_type = token_type, .literal = literal };
                } else {
                    return Token{ .token_type = .ILLEGAL, .literal = &[_]u8{self.ch} };
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
    const input = "!-/*5; 5 < 10 > 5;";
    var lexer = Lexer.init(input);

    const expected = [_]struct { token_type: TokenType, literal: []const u8 }{
        .{ .token_type = .BANG, .literal = "!" },
        .{ .token_type = .MINUS, .literal = "-" },
        .{ .token_type = .SLASH, .literal = "/" },
        .{ .token_type = .ASTERISK, .literal = "*" },
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
