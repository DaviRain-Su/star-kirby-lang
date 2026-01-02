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
