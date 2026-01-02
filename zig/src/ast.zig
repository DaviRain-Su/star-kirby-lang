const std = @import("std");
const token_mod = @import("token.zig");
const Token = token_mod.Token;

// Forward declarations
pub const Expression = union(enum) {
    identifier: Identifier,
    integer_literal: IntegerLiteral,
    boolean: Boolean,
    prefix: Prefix,
    infix: Infix,
    if_expression: If,
    function_literal: FunctionLiteral,
    call: Call,
};

pub const Statement = union(enum) {
    let: LetStatement,
    return_stmt: ReturnStatement,
    expression: ExpressionStatement,
};

pub const Program = struct {
    statements: std.ArrayList(Statement),

    pub fn init(allocator: std.mem.Allocator) Program {
        return Program{
            .statements = std.ArrayList(Statement).init(allocator),
        };
    }

    pub fn deinit(self: *Program) void {
        self.statements.deinit();
    }

    pub fn tokenLiteral(self: *const Program) []const u8 {
        if (self.statements.items.len == 0) {
            return "";
        }
        return ""; // TODO: implement
    }
};

pub const Identifier = struct {
    token: Token,
    value: []const u8,
};

pub const IntegerLiteral = struct {
    token: Token,
    value: i64,
};

pub const Boolean = struct {
    token: Token,
    value: bool,
};

pub const Prefix = struct {
    token: Token,
    operator: []const u8,
    right: *Expression,
};

pub const Infix = struct {
    token: Token,
    left: *Expression,
    operator: []const u8,
    right: *Expression,
};

pub const If = struct {
    token: Token,
    condition: *Expression,
    consequence: *BlockStatement,
    alternative: ?*BlockStatement,
};

pub const FunctionLiteral = struct {
    token: Token,
    parameters: std.ArrayList(Identifier),
    body: *BlockStatement,
};

pub const Call = struct {
    token: Token,
    function: *Expression,
    arguments: std.ArrayList(Expression),
};

pub const BlockStatement = struct {
    token: Token,
    statements: std.ArrayList(Statement),
};

pub const LetStatement = struct {
    token: Token,
    name: Identifier,
    value: Expression,
};

pub const ReturnStatement = struct {
    token: Token,
    return_value: Expression,
};

pub const ExpressionStatement = struct {
    token: Token,
    expression: Expression,
};
