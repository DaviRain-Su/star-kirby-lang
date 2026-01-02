const std = @import("std");
const zigfp = @import("zigfp");
const ast_mod = @import("ast.zig");
const object_mod = @import("object.zig");

const Object = object_mod.Object;
const Environment = object_mod.Environment;
const Result = zigfp.Result;

/// Evaluator errors
pub const EvalError = error{
    TypeMismatch,
    UnknownOperator,
    IdentifierNotFound,
    NotAFunction,
    WrongNumberOfArguments,
    OutOfMemory,
};

/// Result type for evaluator operations
pub const EvalResult = Result(Object, EvalError);

/// Evaluate a program
pub fn evalProgram(allocator: std.mem.Allocator, program: ast_mod.Program, env: *Environment) EvalResult {
    var result = object_mod.makeNull();

    for (program.statements) |stmt| {
        const stmt_result = evalStatement(allocator, stmt, env);
        if (stmt_result.isErr()) {
            return stmt_result;
        }
        result = stmt_result.unwrap();

        // Handle return values
        if (result.objectType() == .return_value) {
            return zigfp.ok(Object, EvalError, result.return_value.value.*);
        }
    }

    return zigfp.ok(Object, EvalError, result);
}

/// Evaluate a statement
pub fn evalStatement(allocator: std.mem.Allocator, stmt: ast_mod.Statement, env: *Environment) EvalResult {
    return switch (stmt) {
        .expression => |expr_stmt| evalExpression(allocator, expr_stmt.expression, env),
        .let => |let_stmt| evalLetStatement(allocator, let_stmt, env),
        .return_stmt => |ret_stmt| evalReturnStatement(allocator, ret_stmt, env),
        .block => |block_stmt| {
            const result = evalBlockStatement(allocator, block_stmt, env);
            return if (result) |obj| zigfp.ok(Object, EvalError, obj) else |err| zigfp.err(Object, EvalError, err);
        },
    };
}

/// Evaluate a let statement
pub fn evalLetStatement(allocator: std.mem.Allocator, stmt: ast_mod.LetStatement, env: *Environment) EvalResult {
    const val_result = evalExpression(allocator, stmt.value, env);
    if (val_result.isErr()) {
        return val_result;
    }
    const val = val_result.unwrap();
    env.set(stmt.name.value, val) catch unreachable;
    return zigfp.ok(Object, EvalError, val);
}

/// Evaluate a return statement
pub fn evalReturnStatement(allocator: std.mem.Allocator, stmt: ast_mod.ReturnStatement, env: *Environment) EvalResult {
    const val_result = evalExpression(allocator, stmt.return_value, env);
    if (val_result.isErr()) {
        return val_result;
    }
    // TODO: Properly wrap the return value
    return zigfp.ok(Object, EvalError, object_mod.makeNull());
}

/// Evaluate an expression
pub fn evalExpression(allocator: std.mem.Allocator, expr: ast_mod.Expression, env: *Environment) EvalResult {
    return switch (expr) {
        .integer_literal => |int_lit| zigfp.ok(Object, EvalError, object_mod.makeInteger(int_lit.value)),
        .boolean => |bool_expr| zigfp.ok(Object, EvalError, object_mod.makeBoolean(bool_expr.value)),
        .identifier => |ident| evalIdentifier(ident, env),
        .prefix => |prefix_expr| {
            const result = evalPrefixExpression(allocator, prefix_expr, env);
            return if (result) |obj| zigfp.ok(Object, EvalError, obj) else |err| zigfp.err(Object, EvalError, err);
        },
        .infix => |infix_expr| {
            const result = evalInfixExpression(allocator, infix_expr, env);
            return if (result) |obj| zigfp.ok(Object, EvalError, obj) else |err| zigfp.err(Object, EvalError, err);
        },
        .if_expression => |if_expr| {
            const result = evalIfExpression(allocator, if_expr, env);
            return if (result) |obj| zigfp.ok(Object, EvalError, obj) else |err| zigfp.err(Object, EvalError, err);
        },
        .function_literal => |fn_lit| {
            const result = evalFunctionLiteral(allocator, fn_lit, env);
            return if (result) |obj| zigfp.ok(Object, EvalError, obj) else |err| zigfp.err(Object, EvalError, err);
        },
    };
}

/// Evaluate a prefix expression
pub fn evalPrefixExpression(allocator: std.mem.Allocator, expr: ast_mod.Prefix, env: *Environment) !Object {
    const right_result = evalExpression(allocator, expr.right.*, env);
    if (right_result.isErr()) return right_result.unwrapErr();
    const right = right_result.unwrap();

    if (std.mem.eql(u8, expr.operator, "!")) {
        return evalBangOperator(right);
    } else if (std.mem.eql(u8, expr.operator, "-")) {
        return evalMinusPrefixOperator(right);
    } else {
        return EvalError.UnknownOperator;
    }
}

/// Evaluate bang operator
pub fn evalBangOperator(right: Object) Object {
    return switch (right) {
        .boolean => |bool_obj| object_mod.makeBoolean(!bool_obj.value),
        .null => object_mod.makeBoolean(true),
        else => object_mod.makeBoolean(false),
    };
}

/// Evaluate minus prefix operator
pub fn evalMinusPrefixOperator(right: Object) !Object {
    if (right.objectType() != .integer) {
        return EvalError.TypeMismatch;
    }

    const value = right.integer.value;
    return object_mod.makeInteger(-value);
}

/// Evaluate an infix expression
pub fn evalInfixExpression(allocator: std.mem.Allocator, expr: ast_mod.Infix, env: *Environment) !Object {
    const left_result = evalExpression(allocator, expr.left.*, env);
    if (left_result.isErr()) return left_result.unwrapErr();
    const left = left_result.unwrap();

    const right_result = evalExpression(allocator, expr.right.*, env);
    if (right_result.isErr()) return right_result.unwrapErr();
    const right = right_result.unwrap();

    if (left.objectType() == .integer and right.objectType() == .integer) {
        return evalIntegerInfixExpression(expr.operator, left.integer.value, right.integer.value);
    }

    if (left.objectType() == .boolean and right.objectType() == .boolean) {
        return evalBooleanInfixExpression(expr.operator, left.boolean.value, right.boolean.value);
    }

    return EvalError.TypeMismatch;
}

/// Evaluate integer infix expression
pub fn evalIntegerInfixExpression(operator: []const u8, left: i64, right: i64) Object {
    if (std.mem.eql(u8, operator, "+")) {
        return object_mod.makeInteger(left + right);
    } else if (std.mem.eql(u8, operator, "-")) {
        return object_mod.makeInteger(left - right);
    } else if (std.mem.eql(u8, operator, "*")) {
        return object_mod.makeInteger(left * right);
    } else if (std.mem.eql(u8, operator, "/")) {
        return object_mod.makeInteger(@divTrunc(left, right));
    } else if (std.mem.eql(u8, operator, "<")) {
        return object_mod.makeBoolean(left < right);
    } else if (std.mem.eql(u8, operator, ">")) {
        return object_mod.makeBoolean(left > right);
    } else if (std.mem.eql(u8, operator, "==")) {
        return object_mod.makeBoolean(left == right);
    } else if (std.mem.eql(u8, operator, "!=")) {
        return object_mod.makeBoolean(left != right);
    } else {
        return object_mod.makeNull();
    }
}

/// Evaluate boolean infix expression
pub fn evalBooleanInfixExpression(operator: []const u8, left: bool, right: bool) Object {
    if (std.mem.eql(u8, operator, "==")) {
        return object_mod.makeBoolean(left == right);
    } else if (std.mem.eql(u8, operator, "!=")) {
        return object_mod.makeBoolean(left != right);
    } else {
        return object_mod.makeNull();
    }
}

/// Evaluate an if expression
pub fn evalIfExpression(allocator: std.mem.Allocator, expr: ast_mod.IfExpression, env: *Environment) !Object {
    const condition_result = evalExpression(allocator, expr.condition.*, env);
    if (condition_result.isErr()) return condition_result.unwrapErr();
    const condition = condition_result.unwrap();

    if (isTruthy(condition)) {
        return evalBlockStatement(allocator, expr.consequence.*, env);
    } else if (expr.alternative) |alt| {
        return evalBlockStatement(allocator, alt.*, env);
    } else {
        return object_mod.makeNull();
    }
}

/// Check if an object is truthy
pub fn isTruthy(obj: Object) bool {
    return switch (obj) {
        .null => false,
        .boolean => |bool_obj| bool_obj.value,
        else => true,
    };
}

/// Evaluate a block statement
pub fn evalBlockStatement(allocator: std.mem.Allocator, block: ast_mod.BlockStatement, env: *Environment) !Object {
    var result = object_mod.makeNull();

    for (block.statements) |stmt| {
        const stmt_result = evalStatement(allocator, stmt, env);
        if (stmt_result.isErr()) return stmt_result.unwrapErr();
        result = stmt_result.unwrap();

        if (result.objectType() == .return_value) {
            return result;
        }
    }

    return result;
}

/// Evaluate an identifier
pub fn evalIdentifier(ident: ast_mod.Identifier, env: *Environment) EvalResult {
    const val = env.get(ident.value);
    if (val) |v| {
        return zigfp.ok(Object, EvalError, v);
    } else {
        return zigfp.err(Object, EvalError, EvalError.IdentifierNotFound);
    }
}

/// Evaluate a function literal
pub fn evalFunctionLiteral(allocator: std.mem.Allocator, fn_lit: ast_mod.FunctionLiteral, env: *Environment) !Object {
    var params_list = try std.ArrayList(ast_mod.Identifier).initCapacity(allocator, fn_lit.parameters.len);
    for (fn_lit.parameters) |param| {
        try params_list.append(allocator, param);
    }
    return object_mod.makeFunction(allocator, params_list, fn_lit.body.*.statements, env);
}

/// Evaluate a call expression
pub fn evalCallExpression(allocator: std.mem.Allocator, call: ast_mod.Call, env: *Environment) !Object {
    const function = try evalExpression(allocator, call.function.*, env);

    if (function.objectType() != .function) {
        return EvalError.NotAFunction;
    }

    const fn_obj = function.function;

    // Evaluate arguments
    var args = try std.ArrayList(Object).initCapacity(allocator, 16);
    defer args.deinit(allocator);

    for (call.arguments.items) |arg| {
        const evaluated = try evalExpression(allocator, arg, env);
        try args.append(allocator, evaluated);
    }

    if (args.items.len != fn_obj.parameters.items.len) {
        return EvalError.WrongNumberOfArguments;
    }

    // Create new environment for function call
    var extended_env = try Environment.initEnclosed(allocator, env);
    defer extended_env.deinit();

    // Bind parameters to arguments
    for (fn_obj.parameters.items, 0..) |param, i| {
        try extended_env.set(param.value, args.items[i]);
    }

    // Evaluate function body
    const evaluated = try evalBlockStatement(allocator, fn_obj.body.*, &extended_env);

    // Unwrap return value
    if (evaluated.objectType() == .return_value) {
        return evaluated.return_value.value.*;
    }

    return evaluated;
}

test "eval integer literal" {
    const allocator = std.testing.allocator;
    var env = Environment.init(allocator);
    defer env.deinit();

    const expr = ast_mod.Expression{ .integer_literal = ast_mod.IntegerLiteral{
        .token = undefined,
        .value = 42,
    } };

    const result = try evalExpression(allocator, expr, &env);
    try std.testing.expectEqual(@as(i64, 42), result.integer.value);
}

test "eval boolean literal" {
    const allocator = std.testing.allocator;
    var env = Environment.init(allocator);
    defer env.deinit();

    const true_expr = ast_mod.Expression{ .boolean = ast_mod.Boolean{
        .token = undefined,
        .value = true,
    } };

    const false_expr = ast_mod.Expression{ .boolean = ast_mod.Boolean{
        .token = undefined,
        .value = false,
    } };

    const true_result = try evalExpression(allocator, true_expr, &env);
    const false_result = try evalExpression(allocator, false_expr, &env);

    try std.testing.expectEqual(true, true_result.boolean.value);
    try std.testing.expectEqual(false, false_result.boolean.value);
}

test "eval bang operator" {
    const allocator = std.testing.allocator;
    var env = Environment.init(allocator);
    defer env.deinit();

    const true_obj = object_mod.makeBoolean(true);
    const false_obj = object_mod.makeBoolean(false);
    const null_obj = object_mod.makeNull();

    const bang_true = evalBangOperator(true_obj);
    const bang_false = evalBangOperator(false_obj);
    const bang_null = evalBangOperator(null_obj);

    try std.testing.expectEqual(false, bang_true.boolean.value);
    try std.testing.expectEqual(true, bang_false.boolean.value);
    try std.testing.expectEqual(true, bang_null.boolean.value);
}

test "eval minus prefix operator" {
    const allocator = std.testing.allocator;
    var env = Environment.init(allocator);
    defer env.deinit();

    const five = object_mod.makeInteger(5);
    const result = try evalMinusPrefixOperator(five);

    try std.testing.expectEqual(@as(i64, -5), result.integer.value);
}

test "eval integer infix expressions" {
    const add_result = evalIntegerInfixExpression("+", 2, 3);
    const sub_result = evalIntegerInfixExpression("-", 5, 3);
    const mul_result = evalIntegerInfixExpression("*", 4, 5);
    const div_result = evalIntegerInfixExpression("/", 10, 2);
    const lt_result = evalIntegerInfixExpression("<", 3, 5);
    const gt_result = evalIntegerInfixExpression(">", 5, 3);
    const eq_result = evalIntegerInfixExpression("==", 5, 5);
    const neq_result = evalIntegerInfixExpression("!=", 5, 3);

    try std.testing.expectEqual(@as(i64, 5), add_result.integer.value);
    try std.testing.expectEqual(@as(i64, 2), sub_result.integer.value);
    try std.testing.expectEqual(@as(i64, 20), mul_result.integer.value);
    try std.testing.expectEqual(@as(i64, 5), div_result.integer.value);
    try std.testing.expectEqual(true, lt_result.boolean.value);
    try std.testing.expectEqual(true, gt_result.boolean.value);
    try std.testing.expectEqual(true, eq_result.boolean.value);
    try std.testing.expectEqual(true, neq_result.boolean.value);
}

test "eval boolean infix expressions" {
    const eq_result = evalBooleanInfixExpression("==", true, true);
    const neq_result = evalBooleanInfixExpression("!=", true, false);

    try std.testing.expectEqual(true, eq_result.boolean.value);
    try std.testing.expectEqual(true, neq_result.boolean.value);
}

test "is truthy" {
    const true_obj = object_mod.makeBoolean(true);
    const false_obj = object_mod.makeBoolean(false);
    const null_obj = object_mod.makeNull();
    const int_obj = object_mod.makeInteger(5);

    try std.testing.expectEqual(true, isTruthy(true_obj));
    try std.testing.expectEqual(false, isTruthy(false_obj));
    try std.testing.expectEqual(false, isTruthy(null_obj));
    try std.testing.expectEqual(true, isTruthy(int_obj));
}
