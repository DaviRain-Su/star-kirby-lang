const std = @import("std");
const token_mod = @import("token.zig");

/// Forward declarations
pub const Expression = union(enum) {
    identifier: Identifier,
    integer_literal: IntegerLiteral,
    boolean: Boolean,
    string_literal: StringLiteral,
    array_literal: ArrayLiteral,
    hash_literal: HashLiteral,
    index_expression: IndexExpression,
    prefix: Prefix,
    infix: Infix,
    if_expression: IfExpression,
    function_literal: FunctionLiteral,
    call: Call,
};

pub const Statement = union(enum) {
    let: LetStatement,
    return_stmt: ReturnStatement,
    expression: ExpressionStatement,
    block: BlockStatement,
    index_assignment: IndexAssignment,
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

pub const StringLiteral = struct {
    token: token_mod.Token,
    value: []const u8,
};

pub const ArrayLiteral = struct {
    token: token_mod.Token,
    elements: []Expression,
};

pub const IndexExpression = struct {
    token: token_mod.Token,
    left: *Expression,
    index: *Expression,
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

pub const Prefix = struct {
    token: token_mod.Token,
    operator: []const u8,
    right: *Expression,
};

pub const Infix = struct {
    token: token_mod.Token,
    left: *Expression,
    operator: []const u8,
    right: *Expression,
};

pub const IfExpression = struct {
    token: token_mod.Token,
    condition: *Expression,
    consequence: *BlockStatement,
    alternative: ?*BlockStatement,
};

pub const FunctionLiteral = struct {
    token: token_mod.Token,
    parameters: []Identifier,
    body: *BlockStatement,
};

pub const BlockStatement = struct {
    token: token_mod.Token,
    statements: []Statement,
};

pub const Call = struct {
    token: token_mod.Token,
    function: *Expression,
    arguments: []Expression,
};

pub const HashPair = struct {
    key: *Expression,
    value: *Expression,
};

pub const HashLiteral = struct {
    token: token_mod.Token,
    pairs: []HashPair,
};

/// Index assignment statement: arr[index] = value
pub const IndexAssignment = struct {
    token: token_mod.Token, // The '=' token
    left: *Expression, // The expression being indexed (e.g., arr, hash)
    index: *Expression, // The index expression
    value: *Expression, // The value being assigned
};
