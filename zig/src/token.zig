const std = @import("std");

pub const TokenType = enum {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    STRING,
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    PERCENT, // %
    LT,
    GT,
    LTE, // <=
    GTE, // >=
    EQ,
    NOTEQ,
    AND, // &&
    OR, // ||
    COMMA,
    SEMICOLON,
    COLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
    WHILE, // while 关键字
    FOR, // for 关键字
    IN, // in 关键字
    BREAK, // break 关键字
    CONTINUE, // continue 关键字
};

pub const Token = struct {
    token_type: TokenType,
    literal: []const u8,
    line: usize = 1,
    column: usize = 1,

    pub fn format(self: Token, writer: anytype) !void {
        try writer.print("{s}({s})", .{ @tagName(self.token_type), self.literal });
    }
};

pub fn lookupIdent(ident: []const u8) TokenType {
    if (std.mem.eql(u8, ident, "fn")) return .FUNCTION;
    if (std.mem.eql(u8, ident, "let")) return .LET;
    if (std.mem.eql(u8, ident, "true")) return .TRUE;
    if (std.mem.eql(u8, ident, "false")) return .FALSE;
    if (std.mem.eql(u8, ident, "if")) return .IF;
    if (std.mem.eql(u8, ident, "else")) return .ELSE;
    if (std.mem.eql(u8, ident, "return")) return .RETURN;
    if (std.mem.eql(u8, ident, "while")) return .WHILE;
    if (std.mem.eql(u8, ident, "for")) return .FOR;
    if (std.mem.eql(u8, ident, "in")) return .IN;
    if (std.mem.eql(u8, ident, "break")) return .BREAK;
    if (std.mem.eql(u8, ident, "continue")) return .CONTINUE;
    return .IDENT;
}

pub fn lookupChar(ch: u8) TokenType {
    return switch (ch) {
        '=' => .ASSIGN,
        '+' => .PLUS,
        '-' => .MINUS,
        '!' => .BANG,
        '*' => .ASTERISK,
        '/' => .SLASH,
        '<' => .LT,
        '>' => .GT,
        ',' => .COMMA,
        ';' => .SEMICOLON,
        ':' => .COLON,
        '(' => .LPAREN,
        ')' => .RPAREN,
        '{' => .LBRACE,
        '}' => .RBRACE,
        '[' => .LBRACKET,
        ']' => .RBRACKET,
        else => .ILLEGAL,
    };
}
