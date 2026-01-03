const std = @import("std");
const zigfp = @import("zigfp");
const ast_mod = @import("ast.zig");
const object_mod = @import("object.zig");
const builtins = @import("builtins.zig");

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
    IndexOutOfBounds,
    KeyNotHashable,
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
        .index_assignment => |idx_assign| {
            const result = evalIndexAssignment(allocator, idx_assign, env);
            return if (result) |obj| zigfp.ok(Object, EvalError, obj) else |err| zigfp.err(Object, EvalError, err);
        },
        .while_stmt => |while_stmt| {
            const result = evalWhileStatement(allocator, while_stmt, env);
            return if (result) |obj| zigfp.ok(Object, EvalError, obj) else |err| zigfp.err(Object, EvalError, err);
        },
        .for_stmt => |for_stmt| {
            const result = evalForStatement(allocator, for_stmt, env);
            return if (result) |obj| zigfp.ok(Object, EvalError, obj) else |err| zigfp.err(Object, EvalError, err);
        },
        .break_stmt => zigfp.ok(Object, EvalError, object_mod.makeBreak()),
        .continue_stmt => zigfp.ok(Object, EvalError, object_mod.makeContinue()),
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
    const val = val_result.unwrap();
    const return_obj_ptr = allocator.create(Object) catch |err| return zigfp.err(Object, EvalError, @errorCast(err));
    return_obj_ptr.* = val;
    const return_value = object_mod.makeReturnValue(return_obj_ptr);
    return zigfp.ok(Object, EvalError, return_value);
}

/// Evaluate an expression
pub fn evalExpression(allocator: std.mem.Allocator, expr: ast_mod.Expression, env: *Environment) EvalResult {
    return switch (expr) {
        .integer_literal => |int_lit| zigfp.ok(Object, EvalError, object_mod.makeInteger(int_lit.value)),
        .boolean => |bool_expr| zigfp.ok(Object, EvalError, object_mod.makeBoolean(bool_expr.value)),
        .string_literal => |str_lit| zigfp.ok(Object, EvalError, object_mod.makeString(allocator, str_lit.value) catch |err| return zigfp.err(Object, EvalError, @errorCast(err))),
        .array_literal => |arr_lit| blk: {
            var elements = std.ArrayList(Object).initCapacity(allocator, arr_lit.elements.len) catch |err| return zigfp.err(Object, EvalError, @errorCast(err));
            defer elements.deinit(allocator);

            for (arr_lit.elements) |elem| {
                const elem_result = evalExpression(allocator, elem, env);
                if (elem_result.isErr()) return elem_result;
                const evaluated = elem_result.unwrap();
                elements.append(allocator, evaluated) catch |err| return zigfp.err(Object, EvalError, @errorCast(err));
            }

            const elements_slice = elements.toOwnedSlice(allocator) catch |err| return zigfp.err(Object, EvalError, @errorCast(err));
            const arr_obj = object_mod.makeArray(allocator, elements_slice) catch |err| return zigfp.err(Object, EvalError, @errorCast(err));
            break :blk zigfp.ok(Object, EvalError, arr_obj);
        },
        .index_expression => |idx_expr| blk: {
            const left_result = evalExpression(allocator, idx_expr.left.*, env);
            if (left_result.isErr()) return left_result;

            const index_result = evalExpression(allocator, idx_expr.index.*, env);
            if (index_result.isErr()) return index_result;

            const left = left_result.unwrap();
            const index = index_result.unwrap();

            if (left.objectType() == .array) {
                if (index.objectType() != .integer) {
                    return zigfp.err(Object, EvalError, EvalError.TypeMismatch);
                }
                const arr = left.array;
                const idx = index.integer.value;
                if (idx < 0 or idx >= arr.elements.len) {
                    break :blk zigfp.ok(Object, EvalError, object_mod.makeNull());
                }
                break :blk zigfp.ok(Object, EvalError, arr.elements[@intCast(idx)]);
            } else if (left.objectType() == .hash) {
                const hash_key = object_mod.HashKey.fromObject(index);
                if (hash_key == null) {
                    return zigfp.err(Object, EvalError, EvalError.KeyNotHashable);
                }
                const hash = left.hash;
                if (hash.pairs.get(hash_key.?.value)) |pair| {
                    break :blk zigfp.ok(Object, EvalError, pair.value);
                }
                break :blk zigfp.ok(Object, EvalError, object_mod.makeNull());
            } else {
                return zigfp.err(Object, EvalError, EvalError.TypeMismatch);
            }
        },
        .hash_literal => |hash_lit| blk: {
            const result = evalHashLiteral(allocator, hash_lit, env);
            break :blk if (result) |obj| zigfp.ok(Object, EvalError, obj) else |err| zigfp.err(Object, EvalError, err);
        },
        .infix => |infix_expr| blk: {
            const result = evalInfixExpression(allocator, infix_expr, env);
            break :blk if (result) |obj| zigfp.ok(Object, EvalError, obj) else |err| zigfp.err(Object, EvalError, err);
        },
        .identifier => |ident| evalIdentifier(ident, env),
        .prefix => |prefix_expr| {
            const result = evalPrefixExpression(allocator, prefix_expr, env);
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
        .call => |call_expr| {
            const result = evalCallExpression(allocator, call_expr, env);
            return if (result) |obj| zigfp.ok(Object, EvalError, obj) else |_| zigfp.err(Object, EvalError, EvalError.NotAFunction);
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
    // Short-circuit evaluation for && and ||
    if (std.mem.eql(u8, expr.operator, "&&")) {
        const left_result = evalExpression(allocator, expr.left.*, env);
        if (left_result.isErr()) return left_result.unwrapErr();
        const left = left_result.unwrap();

        // Short-circuit: if left is false, return false without evaluating right
        if (!isTruthy(left)) {
            return object_mod.makeBoolean(false);
        }

        // Evaluate right and return its truthiness
        const right_result = evalExpression(allocator, expr.right.*, env);
        if (right_result.isErr()) return right_result.unwrapErr();
        const right = right_result.unwrap();
        return object_mod.makeBoolean(isTruthy(right));
    }

    if (std.mem.eql(u8, expr.operator, "||")) {
        const left_result = evalExpression(allocator, expr.left.*, env);
        if (left_result.isErr()) return left_result.unwrapErr();
        const left = left_result.unwrap();

        // Short-circuit: if left is true, return true without evaluating right
        if (isTruthy(left)) {
            return object_mod.makeBoolean(true);
        }

        // Evaluate right and return its truthiness
        const right_result = evalExpression(allocator, expr.right.*, env);
        if (right_result.isErr()) return right_result.unwrapErr();
        const right = right_result.unwrap();
        return object_mod.makeBoolean(isTruthy(right));
    }

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

    if (left.objectType() == .string and right.objectType() == .string) {
        const result = try evalStringInfixExpression(allocator, expr.operator, left.string.value, right.string.value);
        return result;
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
    } else if (std.mem.eql(u8, operator, "%")) {
        return object_mod.makeInteger(@rem(left, right));
    } else if (std.mem.eql(u8, operator, "<")) {
        return object_mod.makeBoolean(left < right);
    } else if (std.mem.eql(u8, operator, ">")) {
        return object_mod.makeBoolean(left > right);
    } else if (std.mem.eql(u8, operator, "<=")) {
        return object_mod.makeBoolean(left <= right);
    } else if (std.mem.eql(u8, operator, ">=")) {
        return object_mod.makeBoolean(left >= right);
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
    } else if (std.mem.eql(u8, operator, "&&")) {
        return object_mod.makeBoolean(left and right);
    } else if (std.mem.eql(u8, operator, "||")) {
        return object_mod.makeBoolean(left or right);
    } else {
        return object_mod.makeNull();
    }
}

/// Evaluate string infix expression
pub fn evalStringInfixExpression(allocator: std.mem.Allocator, operator: []const u8, left: []const u8, right: []const u8) !Object {
    if (std.mem.eql(u8, operator, "+")) {
        // String concatenation
        const result_len = left.len + right.len;
        const result = try allocator.alloc(u8, result_len);
        std.mem.copyForwards(u8, result[0..left.len], left);
        std.mem.copyForwards(u8, result[left.len..], right);
        return object_mod.makeString(allocator, result);
    } else if (std.mem.eql(u8, operator, "==")) {
        return object_mod.makeBoolean(std.mem.eql(u8, left, right));
    } else if (std.mem.eql(u8, operator, "!=")) {
        return object_mod.makeBoolean(!std.mem.eql(u8, left, right));
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

        // Propagate return values and loop control signals
        if (result.objectType() == .return_value or result.objectType() == .loop_control) {
            return result;
        }
    }

    return result;
}

/// Evaluate a while statement
pub fn evalWhileStatement(allocator: std.mem.Allocator, stmt: ast_mod.WhileStatement, env: *Environment) !Object {
    var result = object_mod.makeNull();
    const max_iterations: u32 = 1000000; // Prevent infinite loops in REPL
    var iterations: u32 = 0;

    while (iterations < max_iterations) : (iterations += 1) {
        // Evaluate condition
        const condition_result = evalExpression(allocator, stmt.condition.*, env);
        if (condition_result.isErr()) return condition_result.unwrapErr();
        const condition = condition_result.unwrap();

        // Check if condition is truthy
        if (!isTruthy(condition)) {
            break;
        }

        // Evaluate body
        result = try evalBlockStatement(allocator, stmt.body.*, env);

        // Handle return inside loop
        if (result.objectType() == .return_value) {
            return result;
        }

        // Handle loop control signals
        if (result.objectType() == .loop_control) {
            const control = result.loop_control.control;
            if (control == .break_signal) {
                // Break exits the loop
                result = object_mod.makeNull();
                break;
            } else if (control == .continue_signal) {
                // Continue skips to next iteration
                result = object_mod.makeNull();
                continue;
            }
        }
    }

    return result;
}

/// Evaluate a for statement: for (variable in iterable) { body }
pub fn evalForStatement(allocator: std.mem.Allocator, stmt: ast_mod.ForStatement, env: *Environment) !Object {
    // Evaluate the iterable
    const iterable_result = evalExpression(allocator, stmt.iterable.*, env);
    if (iterable_result.isErr()) return iterable_result.unwrapErr();
    const iterable = iterable_result.unwrap();

    // Only arrays are iterable for now
    if (iterable.objectType() != .array) {
        return EvalError.TypeMismatch;
    }

    var result = object_mod.makeNull();
    const elements = iterable.array.elements;

    for (elements) |element| {
        // Bind the loop variable to the current element
        try env.set(stmt.variable.value, element);

        // Evaluate body
        result = try evalBlockStatement(allocator, stmt.body.*, env);

        // Handle return inside loop
        if (result.objectType() == .return_value) {
            return result;
        }

        // Handle loop control signals
        if (result.objectType() == .loop_control) {
            const control = result.loop_control.control;
            if (control == .break_signal) {
                // Break exits the loop
                result = object_mod.makeNull();
                break;
            } else if (control == .continue_signal) {
                // Continue skips to next iteration
                result = object_mod.makeNull();
                continue;
            }
        }
    }

    return result;
}

/// Evaluate an identifier
pub fn evalIdentifier(ident: ast_mod.Identifier, env: *Environment) EvalResult {
    // First check environment
    const val = env.get(ident.value);
    if (val) |v| {
        return zigfp.ok(Object, EvalError, v);
    }

    // Then check builtins
    if (builtins.getBuiltin(ident.value)) |builtin| {
        return zigfp.ok(Object, EvalError, builtin);
    }

    return zigfp.err(Object, EvalError, EvalError.IdentifierNotFound);
}

/// Evaluate a hash literal
pub fn evalHashLiteral(allocator: std.mem.Allocator, hash_lit: ast_mod.HashLiteral, env: *Environment) !Object {
    var hash_obj = object_mod.makeHash(allocator);

    for (hash_lit.pairs) |pair| {
        const key_result = evalExpression(allocator, pair.key.*, env);
        if (key_result.isErr()) return key_result.unwrapErr();
        const key = key_result.unwrap();

        const value_result = evalExpression(allocator, pair.value.*, env);
        if (value_result.isErr()) return value_result.unwrapErr();
        const value = value_result.unwrap();

        const hash_key = object_mod.HashKey.fromObject(key);
        if (hash_key == null) {
            return EvalError.KeyNotHashable;
        }

        try hash_obj.hash.pairs.put(hash_key.?.value, object_mod.HashPair{
            .key = key,
            .value = value,
        });
    }

    return hash_obj;
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
    const function_result = evalExpression(allocator, call.function.*, env);
    if (function_result.isErr()) return function_result.unwrapErr();
    const function = function_result.unwrap();

    // Evaluate arguments first
    var args = try std.ArrayList(Object).initCapacity(allocator, 16);
    defer args.deinit(allocator);

    for (call.arguments) |arg| {
        const arg_result = evalExpression(allocator, arg, env);
        if (arg_result.isErr()) return arg_result.unwrapErr();
        const evaluated = arg_result.unwrap();
        try args.append(allocator, evaluated);
    }

    // Handle builtin functions
    if (function.objectType() == .builtin) {
        const builtin = function.builtin;
        return builtin.func(allocator, args.items);
    }

    // Handle user-defined functions
    if (function.objectType() != .function) {
        return EvalError.NotAFunction;
    }

    const fn_obj = function.function;

    if (args.items.len != fn_obj.parameters.items.len) {
        return EvalError.WrongNumberOfArguments;
    }

    // Create new environment for function call
    // Use the function's captured environment (for closures) or the current env
    const fn_env = fn_obj.env orelse env;

    // Allocate extended_env on the heap so it can outlive this function call
    // This is necessary for closures that capture the environment
    const extended_env_ptr = try allocator.create(Environment);
    errdefer allocator.destroy(extended_env_ptr);
    extended_env_ptr.* = try Environment.initEnclosed(allocator, fn_env);

    // Bind parameters to arguments
    for (fn_obj.parameters.items, 0..) |param, i| {
        try extended_env_ptr.set(param.value, args.items[i]);
    }

    // Evaluate function body
    const block_stmt = ast_mod.BlockStatement{
        .token = undefined, // Not needed for evaluation
        .statements = fn_obj.body,
    };
    const evaluated = try evalBlockStatement(allocator, block_stmt, extended_env_ptr);

    // Unwrap return value if present
    var result = evaluated;
    if (evaluated.objectType() == .return_value) {
        result = evaluated.return_value.value.*;
    }

    // Only cleanup the environment if the result is NOT a function
    // If it's a function (closure), it needs the environment to persist
    if (result.objectType() != .function) {
        extended_env_ptr.deinit();
        allocator.destroy(extended_env_ptr);
    }
    // Note: If the result is a function, we intentionally "leak" the environment
    // because the closure needs it. In a production system, we would use
    // reference counting or a garbage collector to manage this properly.

    return result;
}

/// Evaluate an index assignment: arr[index] = value
pub fn evalIndexAssignment(allocator: std.mem.Allocator, idx_assign: ast_mod.IndexAssignment, env: *Environment) !Object {
    // Evaluate the left expression (the container - array or hash)
    const left_result = evalExpression(allocator, idx_assign.left.*, env);
    if (left_result.isErr()) return left_result.unwrapErr();
    var left = left_result.unwrap();

    // Evaluate the index
    const index_result = evalExpression(allocator, idx_assign.index.*, env);
    if (index_result.isErr()) return index_result.unwrapErr();
    const index = index_result.unwrap();

    // Evaluate the value to assign
    const value_result = evalExpression(allocator, idx_assign.value.*, env);
    if (value_result.isErr()) return value_result.unwrapErr();
    const value = value_result.unwrap();

    // Handle array index assignment
    if (left.objectType() == .array) {
        if (index.objectType() != .integer) {
            return EvalError.TypeMismatch;
        }
        const idx = index.integer.value;
        if (idx < 0 or idx >= left.array.elements.len) {
            return EvalError.IndexOutOfBounds;
        }
        // Modify the array element
        left.array.elements[@intCast(idx)] = value;

        // Update the variable in environment if it's an identifier
        // The left expression should point to the same array object in memory
        return value;
    }

    // Handle hash key assignment
    if (left.objectType() == .hash) {
        const hash_key = object_mod.HashKey.fromObject(index);
        if (hash_key == null) {
            return EvalError.KeyNotHashable;
        }

        // Add or update the key-value pair
        try left.hash.pairs.put(hash_key.?.value, object_mod.HashPair{
            .key = index,
            .value = value,
        });

        return value;
    }

    return EvalError.TypeMismatch;
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

// =============================================================================
// Error Situation Tests
// =============================================================================

test "error: identifier not found" {
    const allocator = std.testing.allocator;
    var env = Environment.init(allocator);
    defer env.deinit();

    const ident = ast_mod.Identifier{
        .token = undefined,
        .value = "unknown_var",
    };

    const result = evalIdentifier(ident, &env);
    try std.testing.expect(result.isErr());
    try std.testing.expectEqual(EvalError.IdentifierNotFound, result.unwrapErr());
}

test "error: type mismatch in minus prefix" {
    // Applying minus to a boolean should return TypeMismatch error
    const bool_obj = object_mod.makeBoolean(true);
    const result = evalMinusPrefixOperator(bool_obj);

    try std.testing.expectError(EvalError.TypeMismatch, result);
}

test "error: wrong number of arguments" {
    const allocator = std.testing.allocator;
    var env = Environment.init(allocator);
    defer env.deinit();

    // Create a function that expects 2 parameters
    var params = try std.ArrayList(ast_mod.Identifier).initCapacity(allocator, 2);
    defer params.deinit(allocator);
    try params.append(allocator, ast_mod.Identifier{ .token = undefined, .value = "x" });
    try params.append(allocator, ast_mod.Identifier{ .token = undefined, .value = "y" });

    const fn_obj = try object_mod.makeFunction(allocator, params, &[_]ast_mod.Statement{}, &env);
    try env.set("myFunc", fn_obj);

    // Try to call with wrong number of arguments (1 instead of 2)
    const call = ast_mod.Call{
        .token = undefined,
        .function = &ast_mod.Expression{ .identifier = ast_mod.Identifier{ .token = undefined, .value = "myFunc" } },
        .arguments = &[_]ast_mod.Expression{
            ast_mod.Expression{ .integer_literal = ast_mod.IntegerLiteral{ .token = undefined, .value = 1 } },
        },
    };

    const result = evalCallExpression(allocator, call, &env);
    try std.testing.expectError(EvalError.WrongNumberOfArguments, result);
}

test "error: not a function" {
    const allocator = std.testing.allocator;
    var env = Environment.init(allocator);
    defer env.deinit();

    // Set an integer as "notAFunc"
    try env.set("notAFunc", object_mod.makeInteger(42));

    // Try to call it as a function
    const call = ast_mod.Call{
        .token = undefined,
        .function = &ast_mod.Expression{ .identifier = ast_mod.Identifier{ .token = undefined, .value = "notAFunc" } },
        .arguments = &[_]ast_mod.Expression{},
    };

    const result = evalCallExpression(allocator, call, &env);
    try std.testing.expectError(EvalError.NotAFunction, result);
}

test "error: key not hashable (function as key)" {
    const allocator = std.testing.allocator;
    var env = Environment.init(allocator);
    defer env.deinit();

    // Try to use a function as a hash key (not hashable)
    var params = try std.ArrayList(ast_mod.Identifier).initCapacity(allocator, 0);
    defer params.deinit(allocator);

    const fn_obj = try object_mod.makeFunction(allocator, params, &[_]ast_mod.Statement{}, &env);

    // HashKey.fromObject should return null for non-hashable types
    const hash_key = object_mod.HashKey.fromObject(fn_obj);
    try std.testing.expect(hash_key == null);
}

test "error: infix type mismatch (integer + boolean)" {
    const allocator = std.testing.allocator;
    var env = Environment.init(allocator);
    defer env.deinit();

    // Create an infix expression: 5 + true
    const left_ptr = try allocator.create(ast_mod.Expression);
    left_ptr.* = ast_mod.Expression{ .integer_literal = ast_mod.IntegerLiteral{ .token = undefined, .value = 5 } };
    defer allocator.destroy(left_ptr);

    const right_ptr = try allocator.create(ast_mod.Expression);
    right_ptr.* = ast_mod.Expression{ .boolean = ast_mod.Boolean{ .token = undefined, .value = true } };
    defer allocator.destroy(right_ptr);

    const expr = ast_mod.Expression{ .infix = ast_mod.Infix{
        .token = undefined,
        .left = left_ptr,
        .operator = "+",
        .right = right_ptr,
    } };

    const result = evalExpression(allocator, expr, &env);
    try std.testing.expect(result.isErr());
    try std.testing.expectEqual(EvalError.TypeMismatch, result.unwrapErr());
}

test "error: unknown operator for booleans" {
    const allocator = std.testing.allocator;
    var env = Environment.init(allocator);
    defer env.deinit();

    // Create an infix expression: true + false (+ not valid for booleans)
    const left_ptr = try allocator.create(ast_mod.Expression);
    left_ptr.* = ast_mod.Expression{ .boolean = ast_mod.Boolean{ .token = undefined, .value = true } };
    defer allocator.destroy(left_ptr);

    const right_ptr = try allocator.create(ast_mod.Expression);
    right_ptr.* = ast_mod.Expression{ .boolean = ast_mod.Boolean{ .token = undefined, .value = false } };
    defer allocator.destroy(right_ptr);

    const expr = ast_mod.Expression{ .infix = ast_mod.Infix{
        .token = undefined,
        .left = left_ptr,
        .operator = "+",
        .right = right_ptr,
    } };

    const result = evalExpression(allocator, expr, &env);
    try std.testing.expect(result.isErr());
    try std.testing.expectEqual(EvalError.UnknownOperator, result.unwrapErr());
}

test "error: index expression with non-integer index on array" {
    const allocator = std.testing.allocator;
    var env = Environment.init(allocator);
    defer env.deinit();

    // Create index expression: [1, 2, 3]["string"] - should fail
    const arr_elements = &[_]ast_mod.Expression{
        ast_mod.Expression{ .integer_literal = ast_mod.IntegerLiteral{ .token = undefined, .value = 1 } },
    };

    const left_ptr = try allocator.create(ast_mod.Expression);
    left_ptr.* = ast_mod.Expression{ .array_literal = ast_mod.ArrayLiteral{
        .token = undefined,
        .elements = arr_elements,
    } };
    defer allocator.destroy(left_ptr);

    const index_ptr = try allocator.create(ast_mod.Expression);
    index_ptr.* = ast_mod.Expression{ .string_literal = ast_mod.StringLiteral{
        .token = undefined,
        .value = "invalid",
    } };
    defer allocator.destroy(index_ptr);

    const expr = ast_mod.Expression{ .index_expression = ast_mod.IndexExpression{
        .token = undefined,
        .left = left_ptr,
        .index = index_ptr,
    } };

    const result = evalExpression(allocator, expr, &env);
    try std.testing.expect(result.isErr());
    try std.testing.expectEqual(EvalError.TypeMismatch, result.unwrapErr());
}

test "error: index expression on non-indexable type" {
    const allocator = std.testing.allocator;
    var env = Environment.init(allocator);
    defer env.deinit();

    // Create index expression: 42[0] - integer is not indexable
    const left_ptr = try allocator.create(ast_mod.Expression);
    left_ptr.* = ast_mod.Expression{ .integer_literal = ast_mod.IntegerLiteral{
        .token = undefined,
        .value = 42,
    } };
    defer allocator.destroy(left_ptr);

    const index_ptr = try allocator.create(ast_mod.Expression);
    index_ptr.* = ast_mod.Expression{ .integer_literal = ast_mod.IntegerLiteral{
        .token = undefined,
        .value = 0,
    } };
    defer allocator.destroy(index_ptr);

    const expr = ast_mod.Expression{ .index_expression = ast_mod.IndexExpression{
        .token = undefined,
        .left = left_ptr,
        .index = index_ptr,
    } };

    const result = evalExpression(allocator, expr, &env);
    try std.testing.expect(result.isErr());
    try std.testing.expectEqual(EvalError.TypeMismatch, result.unwrapErr());
}
