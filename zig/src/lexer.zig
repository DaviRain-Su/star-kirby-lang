const std = @import("std");
const token_mod = @import("token.zig");
const Token = token_mod.Token;
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

    pub fn nextToken(self: *Lexer) Token {
        // Skip whitespace
        while (self.ch == ' ' or self.ch == '\t' or self.ch == '\n' or self.ch == '\r') {
            self.readChar();
        }

        const tok = switch (self.ch) {
            '=' => Token{ .token_type = .ASSIGN, .literal = "=" },
            '+' => Token{ .token_type = .PLUS, .literal = "+" },
            '-' => Token{ .token_type = .MINUS, .literal = "-" },
            '!' => Token{ .token_type = .BANG, .literal = "!" },
            '*' => Token{ .token_type = .ASTERISK, .literal = "*" },
            '/' => Token{ .token_type = .SLASH, .literal = "/" },
            '<' => Token{ .token_type = .LT, .literal = "<" },
            '>' => Token{ .token_type = .GT, .literal = ">" },
            ';' => Token{ .token_type = .SEMICOLON, .literal = ";" },
            '(' => Token{ .token_type = .LPAREN, .literal = "(" },
            ')' => Token{ .token_type = .RPAREN, .literal = ")" },
            ',' => Token{ .token_type = .COMMA, .literal = "," },
            '{' => Token{ .token_type = .LBRACE, .literal = "{" },
            '}' => Token{ .token_type = .RBRACE, .literal = "}" },
            '[' => Token{ .token_type = .LBRACKET, .literal = "[" },
            ']' => Token{ .token_type = .RBRACKET, .literal = "]" },
            ':' => Token{ .token_type = .COLON, .literal = ":" },
            0 => Token{ .token_type = .EOF, .literal = "" },
            else => Token{ .token_type = .ILLEGAL, .literal = [_]u8{self.ch} },
        };

        self.readChar();
        return tok;
    }
};

test "lexer basic" {
    const input = "=+(){},;";
    var lexer = Lexer.init(input);

    const expected_tokens = [_]TokenType{ .ASSIGN, .PLUS, .LPAREN, .RPAREN, .LBRACE, .RBRACE, .COMMA, .SEMICOLON, .EOF };

    for (expected_tokens) |expected| {
        const tok = lexer.nextToken();
        try std.testing.expectEqual(expected, tok.token_type);
    }
}
