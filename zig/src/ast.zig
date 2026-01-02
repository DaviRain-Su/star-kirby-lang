const std = @import("std");
const token_mod = @import("token.zig");

/// Forward declarations
pub const Expression = union(enum) {
    identifier: Identifier,
    integer_literal: IntegerLiteral,
    boolean: Boolean,
};

pub const Statement = union(enum) {
    let: LetStatement,
    return_stmt: ReturnStatement,
    expression: ExpressionStatement,
};

pub const Program = struct {
    statements: []Statement,

    pub fn init(statements: []Statement) Program {
        return Program{
            .statements = statements,
        };
    }

    pub fn deinit(self: *Program, allocator: std.mem.Allocator) void {
        allocator.free(self.statements);
    }
};

pub const Identifier = struct {
    token: token_mod.Token,
    value: []const u8,
};

pub const IntegerLiteral = struct {
    token: token_mod.Token,
    value: i64,
};

pub const Boolean = struct {
    token: token_mod.Token,
    value: bool,
};

pub const LetStatement = struct {
    token: token_mod.Token,
    name: Identifier,
    value: Expression,
};

pub const ReturnStatement = struct {
    token: token_mod.Token,
    return_value: Expression,
};

pub const ExpressionStatement = struct {
    token: token_mod.Token,
    expression: Expression,
};
