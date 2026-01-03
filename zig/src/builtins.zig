const std = @import("std");
const object_mod = @import("object.zig");
const ast_mod = @import("ast.zig");
const lexer_mod = @import("lexer.zig");
const parser_mod = @import("parser.zig");
const Object = object_mod.Object;
const Environment = object_mod.Environment;

// Global storage for script arguments (v0.10.0)
var script_args: ?[]const []const u8 = null;
var script_args_allocator: ?std.mem.Allocator = null;

/// Set the script arguments (called from main.zig)
pub fn setScriptArgs(allocator: std.mem.Allocator, args: []const []const u8) !void {
    // Clean up previous args if any
    if (script_args) |old_args| {
        if (script_args_allocator) |alloc| {
            for (old_args) |arg| {
                alloc.free(arg);
            }
            alloc.free(old_args);
        }
    }

    // Duplicate the args
    var new_args = try allocator.alloc([]const u8, args.len);
    for (args, 0..) |arg, i| {
        new_args[i] = try allocator.dupe(u8, arg);
    }
    script_args = new_args;
    script_args_allocator = allocator;
}

/// Clear script arguments (for cleanup)
pub fn clearScriptArgs() void {
    if (script_args) |args| {
        if (script_args_allocator) |allocator| {
            for (args) |arg| {
                allocator.free(arg);
            }
            allocator.free(args);
        }
    }
    script_args = null;
    script_args_allocator = null;
}

/// Builtin function: len
/// Returns the length of a string or array
fn builtinLen(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `len`. want=1");
    }

    return switch (args[0]) {
        .string => |s| object_mod.makeInteger(@intCast(s.value.len)),
        .array => |a| object_mod.makeInteger(@intCast(a.elements.len)),
        else => try object_mod.makeError(allocator, "argument to `len` not supported, got non-string/array"),
    };
}

/// Builtin function: first
/// Returns the first element of an array
fn builtinFirst(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `first`. want=1");
    }

    if (args[0].objectType() != .array) {
        return try object_mod.makeError(allocator, "argument to `first` must be ARRAY");
    }

    const arr = args[0].array;
    if (arr.elements.len == 0) {
        return object_mod.makeNull();
    }

    return arr.elements[0];
}

/// Builtin function: last
/// Returns the last element of an array
fn builtinLast(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `last`. want=1");
    }

    if (args[0].objectType() != .array) {
        return try object_mod.makeError(allocator, "argument to `last` must be ARRAY");
    }

    const arr = args[0].array;
    if (arr.elements.len == 0) {
        return object_mod.makeNull();
    }

    return arr.elements[arr.elements.len - 1];
}

/// Builtin function: rest
/// Returns a new array with all elements except the first
fn builtinRest(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `rest`. want=1");
    }

    if (args[0].objectType() != .array) {
        return try object_mod.makeError(allocator, "argument to `rest` must be ARRAY");
    }

    const arr = args[0].array;
    if (arr.elements.len == 0) {
        return object_mod.makeNull();
    }

    const new_elements = try allocator.dupe(Object, arr.elements[1..]);
    return object_mod.makeArray(allocator, new_elements);
}

/// Builtin function: push
/// Returns a new array with the element appended
fn builtinPush(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 2) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `push`. want=2");
    }

    if (args[0].objectType() != .array) {
        return try object_mod.makeError(allocator, "first argument to `push` must be ARRAY");
    }

    const arr = args[0].array;
    var new_elements = try allocator.alloc(Object, arr.elements.len + 1);
    @memcpy(new_elements[0..arr.elements.len], arr.elements);
    new_elements[arr.elements.len] = args[1];

    return object_mod.makeArray(allocator, new_elements);
}

/// Builtin function: puts
/// Prints arguments to stdout
fn builtinPuts(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    for (args) |arg| {
        const str = try arg.inspect(allocator);
        defer allocator.free(str);
        std.debug.print("{s}\n", .{str});
    }
    return object_mod.makeNull();
}

/// Builtin function: type
/// Returns the type of an object as a string
fn builtinType(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `type`. want=1");
    }

    const type_str = switch (args[0]) {
        .integer => "INTEGER",
        .boolean => "BOOLEAN",
        .null => "NULL",
        .string => "STRING",
        .array => "ARRAY",
        .hash => "HASH",
        .function => "FUNCTION",
        .builtin => "BUILTIN",
        .return_value => "RETURN_VALUE",
        .@"error" => "ERROR",
        .loop_control => "LOOP_CONTROL",
    };

    return object_mod.makeString(allocator, type_str);
}

/// Builtin function: print
/// Prints arguments to stdout without newline
fn builtinPrint(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    for (args) |arg| {
        const str = try arg.inspect(allocator);
        defer allocator.free(str);
        std.debug.print("{s}", .{str});
    }
    return object_mod.makeNull();
}

/// Builtin function: str
/// Converts any value to its string representation
fn builtinStr(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `str`. want=1");
    }

    const str = try args[0].inspect(allocator);
    return object_mod.makeStringOwned(allocator, str);
}

/// Builtin function: int
/// Parses a string to an integer
fn builtinInt(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `int`. want=1");
    }

    switch (args[0]) {
        .string => |s| {
            const value = std.fmt.parseInt(i64, s.value, 10) catch {
                return try object_mod.makeError(allocator, "cannot parse string to integer");
            };
            return object_mod.makeInteger(value);
        },
        .integer => |i| {
            return object_mod.makeInteger(i.value);
        },
        .boolean => |b| {
            return object_mod.makeInteger(if (b.value) 1 else 0);
        },
        else => {
            return try object_mod.makeError(allocator, "argument to `int` not supported");
        },
    }
}

/// Builtin function: keys
/// Returns an array of all keys in a hash
fn builtinKeys(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `keys`. want=1");
    }

    if (args[0].objectType() != .hash) {
        return try object_mod.makeError(allocator, "argument to `keys` must be HASH");
    }

    const hash = args[0].hash;
    var keys_list = try std.ArrayList(Object).initCapacity(allocator, hash.pairs.count());

    var it = hash.pairs.iterator();
    while (it.next()) |entry| {
        // Get the key from the HashPair
        try keys_list.append(allocator, entry.value_ptr.*.key);
    }

    const elements = try keys_list.toOwnedSlice(allocator);
    return object_mod.makeArray(allocator, elements);
}

/// Builtin function: values
/// Returns an array of all values in a hash
fn builtinValues(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `values`. want=1");
    }

    if (args[0].objectType() != .hash) {
        return try object_mod.makeError(allocator, "argument to `values` must be HASH");
    }

    const hash = args[0].hash;
    var values_list = try std.ArrayList(Object).initCapacity(allocator, hash.pairs.count());

    var it = hash.pairs.iterator();
    while (it.next()) |entry| {
        // Get the value from the HashPair
        try values_list.append(allocator, entry.value_ptr.*.value);
    }

    const elements = try values_list.toOwnedSlice(allocator);
    return object_mod.makeArray(allocator, elements);
}

/// Helper to apply a function object to arguments
/// This handles both builtin and user-defined functions
fn applyFunction(allocator: std.mem.Allocator, function: Object, args: []Object) anyerror!Object {
    // Handle builtin functions
    if (function.objectType() == .builtin) {
        const builtin = function.builtin;
        return builtin.func(allocator, args);
    }

    // Handle user-defined functions
    if (function.objectType() != .function) {
        return try object_mod.makeError(allocator, "not a function");
    }

    const fn_obj = function.function;

    if (args.len != fn_obj.parameters.items.len) {
        return try object_mod.makeError(allocator, "wrong number of arguments");
    }

    // Create new environment for function call
    const fn_env = fn_obj.env;
    const extended_env_ptr = try allocator.create(Environment);
    errdefer allocator.destroy(extended_env_ptr);

    if (fn_env) |env| {
        extended_env_ptr.* = try Environment.initEnclosed(allocator, env);
    } else {
        extended_env_ptr.* = Environment.init(allocator);
    }

    // Bind parameters to arguments
    for (fn_obj.parameters.items, 0..) |param, i| {
        try extended_env_ptr.set(param.value, args[i]);
    }

    // Evaluate function body
    var result = object_mod.makeNull();
    for (fn_obj.body) |stmt| {
        result = try evalStatementForBuiltin(allocator, stmt, extended_env_ptr);
        if (result.objectType() == .return_value) {
            result = result.return_value.value.*;
            break;
        }
    }

    // Cleanup if not a closure
    if (result.objectType() != .function) {
        extended_env_ptr.deinit();
        allocator.destroy(extended_env_ptr);
    }

    return result;
}

/// Simple statement evaluator for builtins (handles expressions only)
/// Note: This duplicates variable names to ensure they outlive the source code
fn evalStatementForBuiltin(allocator: std.mem.Allocator, stmt: ast_mod.Statement, env: *Environment) anyerror!Object {
    return switch (stmt) {
        .expression => |expr_stmt| try evalExpressionForBuiltin(allocator, expr_stmt.expression, env),
        .return_stmt => |ret_stmt| blk: {
            const val = try evalExpressionForBuiltin(allocator, ret_stmt.return_value, env);
            const return_obj_ptr = try allocator.create(Object);
            return_obj_ptr.* = val;
            break :blk object_mod.makeReturnValue(return_obj_ptr);
        },
        .let => |let_stmt| blk: {
            const val = try evalExpressionForBuiltin(allocator, let_stmt.value, env);
            // Duplicate the name to ensure it outlives the source file
            const name_copy = try allocator.dupe(u8, let_stmt.name.value);
            try env.set(name_copy, val);
            break :blk val;
        },
        else => object_mod.makeNull(),
    };
}

/// Simple expression evaluator for builtins
fn evalExpressionForBuiltin(allocator: std.mem.Allocator, expr: ast_mod.Expression, env: *Environment) anyerror!Object {
    return switch (expr) {
        .integer_literal => |int_lit| object_mod.makeInteger(int_lit.value),
        .boolean => |bool_expr| object_mod.makeBoolean(bool_expr.value),
        .string_literal => |str_lit| try object_mod.makeString(allocator, str_lit.value),
        .identifier => |ident| blk: {
            if (env.get(ident.value)) |val| {
                break :blk val;
            }
            if (getBuiltin(ident.value)) |builtin| {
                break :blk builtin;
            }
            break :blk try object_mod.makeError(allocator, "identifier not found");
        },
        .prefix => |prefix_expr| blk: {
            const right = try evalExpressionForBuiltin(allocator, prefix_expr.right.*, env);
            if (std.mem.eql(u8, prefix_expr.operator, "!")) {
                break :blk switch (right) {
                    .boolean => |bool_obj| object_mod.makeBoolean(!bool_obj.value),
                    .null => object_mod.makeBoolean(true),
                    else => object_mod.makeBoolean(false),
                };
            } else if (std.mem.eql(u8, prefix_expr.operator, "-")) {
                if (right.objectType() != .integer) {
                    break :blk try object_mod.makeError(allocator, "type mismatch");
                }
                break :blk object_mod.makeInteger(-right.integer.value);
            }
            break :blk object_mod.makeNull();
        },
        .infix => |infix_expr| blk: {
            const left = try evalExpressionForBuiltin(allocator, infix_expr.left.*, env);
            const right = try evalExpressionForBuiltin(allocator, infix_expr.right.*, env);

            if (left.objectType() == .integer and right.objectType() == .integer) {
                const l = left.integer.value;
                const r = right.integer.value;
                if (std.mem.eql(u8, infix_expr.operator, "+")) {
                    break :blk object_mod.makeInteger(l + r);
                } else if (std.mem.eql(u8, infix_expr.operator, "-")) {
                    break :blk object_mod.makeInteger(l - r);
                } else if (std.mem.eql(u8, infix_expr.operator, "*")) {
                    break :blk object_mod.makeInteger(l * r);
                } else if (std.mem.eql(u8, infix_expr.operator, "/")) {
                    break :blk object_mod.makeInteger(@divTrunc(l, r));
                } else if (std.mem.eql(u8, infix_expr.operator, "%")) {
                    break :blk object_mod.makeInteger(@rem(l, r));
                } else if (std.mem.eql(u8, infix_expr.operator, "<")) {
                    break :blk object_mod.makeBoolean(l < r);
                } else if (std.mem.eql(u8, infix_expr.operator, ">")) {
                    break :blk object_mod.makeBoolean(l > r);
                } else if (std.mem.eql(u8, infix_expr.operator, "<=")) {
                    break :blk object_mod.makeBoolean(l <= r);
                } else if (std.mem.eql(u8, infix_expr.operator, ">=")) {
                    break :blk object_mod.makeBoolean(l >= r);
                } else if (std.mem.eql(u8, infix_expr.operator, "==")) {
                    break :blk object_mod.makeBoolean(l == r);
                } else if (std.mem.eql(u8, infix_expr.operator, "!=")) {
                    break :blk object_mod.makeBoolean(l != r);
                }
            }
            if (left.objectType() == .boolean and right.objectType() == .boolean) {
                const l = left.boolean.value;
                const r = right.boolean.value;
                if (std.mem.eql(u8, infix_expr.operator, "==")) {
                    break :blk object_mod.makeBoolean(l == r);
                } else if (std.mem.eql(u8, infix_expr.operator, "!=")) {
                    break :blk object_mod.makeBoolean(l != r);
                } else if (std.mem.eql(u8, infix_expr.operator, "&&")) {
                    break :blk object_mod.makeBoolean(l and r);
                } else if (std.mem.eql(u8, infix_expr.operator, "||")) {
                    break :blk object_mod.makeBoolean(l or r);
                }
            }
            break :blk object_mod.makeNull();
        },
        .call => |call_expr| blk: {
            const function = try evalExpressionForBuiltin(allocator, call_expr.function.*, env);
            var args_list = try std.ArrayList(Object).initCapacity(allocator, call_expr.arguments.len);
            defer args_list.deinit(allocator);
            for (call_expr.arguments) |arg| {
                const evaluated = try evalExpressionForBuiltin(allocator, arg, env);
                try args_list.append(allocator, evaluated);
            }
            break :blk try applyFunction(allocator, function, args_list.items);
        },
        .function_literal => |fn_lit| blk: {
            // Create function object
            var params_list = try std.ArrayList(ast_mod.Identifier).initCapacity(allocator, fn_lit.parameters.len);
            for (fn_lit.parameters) |param| {
                try params_list.append(allocator, param);
            }
            break :blk object_mod.makeFunction(allocator, params_list, fn_lit.body.*.statements, env);
        },
        else => object_mod.makeNull(),
    };
}

/// Builtin function: range
/// Generates an array of integers
/// range(n) -> [0, 1, ..., n-1]
/// range(start, end) -> [start, start+1, ..., end-1]
/// range(start, end, step) -> [start, start+step, ...]
fn builtinRange(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len == 0 or args.len > 3) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `range`. want=1, 2, or 3");
    }

    // Validate all arguments are integers
    for (args) |arg| {
        if (arg.objectType() != .integer) {
            return try object_mod.makeError(allocator, "arguments to `range` must be integers");
        }
    }

    var start: i64 = 0;
    var end: i64 = 0;
    var step: i64 = 1;

    if (args.len == 1) {
        // range(n) -> [0, 1, ..., n-1]
        end = args[0].integer.value;
    } else if (args.len == 2) {
        // range(start, end) -> [start, start+1, ..., end-1]
        start = args[0].integer.value;
        end = args[1].integer.value;
    } else {
        // range(start, end, step)
        start = args[0].integer.value;
        end = args[1].integer.value;
        step = args[2].integer.value;
        if (step == 0) {
            return try object_mod.makeError(allocator, "step argument to `range` cannot be zero");
        }
    }

    // Calculate array size
    var count: usize = 0;
    if (step > 0 and start < end) {
        count = @intCast(@divTrunc(end - start + step - 1, step));
    } else if (step < 0 and start > end) {
        count = @intCast(@divTrunc(start - end - step - 1, -step));
    }

    var elements = try std.ArrayList(Object).initCapacity(allocator, count);
    defer elements.deinit(allocator);

    var current = start;
    if (step > 0) {
        while (current < end) : (current += step) {
            try elements.append(allocator, object_mod.makeInteger(current));
        }
    } else {
        while (current > end) : (current += step) {
            try elements.append(allocator, object_mod.makeInteger(current));
        }
    }

    const elements_slice = try elements.toOwnedSlice(allocator);
    return object_mod.makeArray(allocator, elements_slice);
}

/// Builtin function: map
/// Applies a function to each element of an array
/// map(array, fn) -> [fn(e1), fn(e2), ...]
fn builtinMap(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 2) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `map`. want=2");
    }

    if (args[0].objectType() != .array) {
        return try object_mod.makeError(allocator, "first argument to `map` must be ARRAY");
    }

    if (args[1].objectType() != .function and args[1].objectType() != .builtin) {
        return try object_mod.makeError(allocator, "second argument to `map` must be FUNCTION");
    }

    const arr = args[0].array;
    const func = args[1];

    var results = try std.ArrayList(Object).initCapacity(allocator, arr.elements.len);
    defer results.deinit(allocator);

    for (arr.elements) |element| {
        var fn_args = [_]Object{element};
        const result = try applyFunction(allocator, func, &fn_args);
        if (result.objectType() == .@"error") {
            return result;
        }
        try results.append(allocator, result);
    }

    const elements = try results.toOwnedSlice(allocator);
    return object_mod.makeArray(allocator, elements);
}

/// Builtin function: filter
/// Returns elements for which the function returns true
/// filter(array, fn) -> [e | fn(e) is truthy]
fn builtinFilter(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 2) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `filter`. want=2");
    }

    if (args[0].objectType() != .array) {
        return try object_mod.makeError(allocator, "first argument to `filter` must be ARRAY");
    }

    if (args[1].objectType() != .function and args[1].objectType() != .builtin) {
        return try object_mod.makeError(allocator, "second argument to `filter` must be FUNCTION");
    }

    const arr = args[0].array;
    const func = args[1];

    var results = try std.ArrayList(Object).initCapacity(allocator, arr.elements.len);
    defer results.deinit(allocator);

    for (arr.elements) |element| {
        var fn_args = [_]Object{element};
        const result = try applyFunction(allocator, func, &fn_args);
        if (result.objectType() == .@"error") {
            return result;
        }
        // Check if result is truthy
        const truthy = switch (result) {
            .null => false,
            .boolean => |b| b.value,
            else => true,
        };
        if (truthy) {
            try results.append(allocator, element);
        }
    }

    const elements = try results.toOwnedSlice(allocator);
    return object_mod.makeArray(allocator, elements);
}

/// Builtin function: reduce
/// Reduces an array to a single value
/// reduce(array, fn, initial) -> fn(fn(fn(initial, e1), e2), e3)...
fn builtinReduce(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 3) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `reduce`. want=3");
    }

    if (args[0].objectType() != .array) {
        return try object_mod.makeError(allocator, "first argument to `reduce` must be ARRAY");
    }

    if (args[1].objectType() != .function and args[1].objectType() != .builtin) {
        return try object_mod.makeError(allocator, "second argument to `reduce` must be FUNCTION");
    }

    const arr = args[0].array;
    const func = args[1];
    var accumulator = args[2];

    for (arr.elements) |element| {
        var fn_args = [_]Object{ accumulator, element };
        const result = try applyFunction(allocator, func, &fn_args);
        if (result.objectType() == .@"error") {
            return result;
        }
        accumulator = result;
    }

    return accumulator;
}

// =============================================================================
// File I/O Functions
// =============================================================================

/// Builtin function: readFile
/// Reads the contents of a file as a string
fn builtinReadFile(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `readFile`. want=1");
    }

    if (args[0].objectType() != .string) {
        return try object_mod.makeError(allocator, "argument to `readFile` must be STRING");
    }

    const path = args[0].string.value;
    const file = std.fs.cwd().openFile(path, .{}) catch {
        return try object_mod.makeError(allocator, "could not open file");
    };
    defer file.close();

    const content = file.readToEndAlloc(allocator, 10 * 1024 * 1024) catch {
        return try object_mod.makeError(allocator, "could not read file");
    };

    return object_mod.makeStringOwned(allocator, content);
}

/// Builtin function: writeFile
/// Writes a string to a file (creates or overwrites)
fn builtinWriteFile(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 2) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `writeFile`. want=2");
    }

    if (args[0].objectType() != .string) {
        return try object_mod.makeError(allocator, "first argument to `writeFile` must be STRING (path)");
    }

    if (args[1].objectType() != .string) {
        return try object_mod.makeError(allocator, "second argument to `writeFile` must be STRING (content)");
    }

    const path = args[0].string.value;
    const content = args[1].string.value;

    const file = std.fs.cwd().createFile(path, .{}) catch {
        return try object_mod.makeError(allocator, "could not create file");
    };
    defer file.close();

    file.writeAll(content) catch {
        return try object_mod.makeError(allocator, "could not write to file");
    };

    return object_mod.makeBoolean(true);
}

/// Builtin function: appendFile
/// Appends a string to a file
fn builtinAppendFile(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 2) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `appendFile`. want=2");
    }

    if (args[0].objectType() != .string) {
        return try object_mod.makeError(allocator, "first argument to `appendFile` must be STRING (path)");
    }

    if (args[1].objectType() != .string) {
        return try object_mod.makeError(allocator, "second argument to `appendFile` must be STRING (content)");
    }

    const path = args[0].string.value;
    const content = args[1].string.value;

    const file = std.fs.cwd().openFile(path, .{ .mode = .write_only }) catch {
        // If file doesn't exist, create it
        const new_file = std.fs.cwd().createFile(path, .{}) catch {
            return try object_mod.makeError(allocator, "could not create file");
        };
        defer new_file.close();
        new_file.writeAll(content) catch {
            return try object_mod.makeError(allocator, "could not write to file");
        };
        return object_mod.makeBoolean(true);
    };
    defer file.close();

    file.seekFromEnd(0) catch {
        return try object_mod.makeError(allocator, "could not seek to end of file");
    };

    file.writeAll(content) catch {
        return try object_mod.makeError(allocator, "could not append to file");
    };

    return object_mod.makeBoolean(true);
}

/// Builtin function: fileExists
/// Checks if a file exists
fn builtinFileExists(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `fileExists`. want=1");
    }

    if (args[0].objectType() != .string) {
        return try object_mod.makeError(allocator, "argument to `fileExists` must be STRING");
    }

    const path = args[0].string.value;
    const exists = std.fs.cwd().access(path, .{}) != error.FileNotFound;

    return object_mod.makeBoolean(exists);
}

// =============================================================================
// String Manipulation Functions
// =============================================================================

/// Builtin function: split
/// Splits a string by a delimiter
fn builtinSplit(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 2) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `split`. want=2");
    }

    if (args[0].objectType() != .string or args[1].objectType() != .string) {
        return try object_mod.makeError(allocator, "arguments to `split` must be STRING");
    }

    const str = args[0].string.value;
    const delimiter = args[1].string.value;

    var results = try std.ArrayList(Object).initCapacity(allocator, 8);
    defer results.deinit(allocator);

    if (delimiter.len == 0) {
        // Split into individual characters
        for (str) |c| {
            var char_str = try allocator.alloc(u8, 1);
            char_str[0] = c;
            try results.append(allocator, try object_mod.makeStringOwned(allocator, char_str));
        }
    } else {
        var it = std.mem.splitSequence(u8, str, delimiter);
        while (it.next()) |part| {
            try results.append(allocator, try object_mod.makeString(allocator, part));
        }
    }

    const elements = try results.toOwnedSlice(allocator);
    return object_mod.makeArray(allocator, elements);
}

/// Builtin function: join
/// Joins array elements into a string with a delimiter
fn builtinJoin(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 2) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `join`. want=2");
    }

    if (args[0].objectType() != .array) {
        return try object_mod.makeError(allocator, "first argument to `join` must be ARRAY");
    }

    if (args[1].objectType() != .string) {
        return try object_mod.makeError(allocator, "second argument to `join` must be STRING");
    }

    const arr = args[0].array;
    const delimiter = args[1].string.value;

    var buffer = try std.ArrayList(u8).initCapacity(allocator, 64);
    defer buffer.deinit(allocator);

    for (arr.elements, 0..) |elem, i| {
        if (i > 0) try buffer.appendSlice(allocator, delimiter);
        const elem_str = try elem.inspect(allocator);
        defer allocator.free(elem_str);
        try buffer.appendSlice(allocator, elem_str);
    }

    const result = try buffer.toOwnedSlice(allocator);
    return object_mod.makeStringOwned(allocator, result);
}

/// Builtin function: trim
/// Removes leading and trailing whitespace
fn builtinTrim(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `trim`. want=1");
    }

    if (args[0].objectType() != .string) {
        return try object_mod.makeError(allocator, "argument to `trim` must be STRING");
    }

    const str = args[0].string.value;
    const trimmed = std.mem.trim(u8, str, " \t\n\r");
    return object_mod.makeString(allocator, trimmed);
}

/// Builtin function: upper
/// Converts string to uppercase
fn builtinUpper(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `upper`. want=1");
    }

    if (args[0].objectType() != .string) {
        return try object_mod.makeError(allocator, "argument to `upper` must be STRING");
    }

    const str = args[0].string.value;
    var result = try allocator.alloc(u8, str.len);
    for (str, 0..) |c, i| {
        result[i] = std.ascii.toUpper(c);
    }
    return object_mod.makeStringOwned(allocator, result);
}

/// Builtin function: lower
/// Converts string to lowercase
fn builtinLower(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `lower`. want=1");
    }

    if (args[0].objectType() != .string) {
        return try object_mod.makeError(allocator, "argument to `lower` must be STRING");
    }

    const str = args[0].string.value;
    var result = try allocator.alloc(u8, str.len);
    for (str, 0..) |c, i| {
        result[i] = std.ascii.toLower(c);
    }
    return object_mod.makeStringOwned(allocator, result);
}

/// Builtin function: contains
/// Checks if a string contains a substring
fn builtinContains(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 2) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `contains`. want=2");
    }

    if (args[0].objectType() != .string or args[1].objectType() != .string) {
        return try object_mod.makeError(allocator, "arguments to `contains` must be STRING");
    }

    const str = args[0].string.value;
    const substr = args[1].string.value;

    const found = std.mem.indexOf(u8, str, substr) != null;
    return object_mod.makeBoolean(found);
}

/// Builtin function: replace
/// Replaces all occurrences of a substring
fn builtinReplace(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 3) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `replace`. want=3");
    }

    if (args[0].objectType() != .string or args[1].objectType() != .string or args[2].objectType() != .string) {
        return try object_mod.makeError(allocator, "arguments to `replace` must be STRING");
    }

    const str = args[0].string.value;
    const old = args[1].string.value;
    const new = args[2].string.value;

    if (old.len == 0) {
        return object_mod.makeString(allocator, str);
    }

    const result = try std.mem.replaceOwned(u8, allocator, str, old, new);
    return object_mod.makeStringOwned(allocator, result);
}

/// Builtin function: charAt
/// Gets character at index
fn builtinCharAt(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 2) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `charAt`. want=2");
    }

    if (args[0].objectType() != .string) {
        return try object_mod.makeError(allocator, "first argument to `charAt` must be STRING");
    }

    if (args[1].objectType() != .integer) {
        return try object_mod.makeError(allocator, "second argument to `charAt` must be INTEGER");
    }

    const str = args[0].string.value;
    const idx = args[1].integer.value;

    if (idx < 0 or idx >= str.len) {
        return object_mod.makeNull();
    }

    var result = try allocator.alloc(u8, 1);
    result[0] = str[@intCast(idx)];
    return object_mod.makeStringOwned(allocator, result);
}

/// Builtin function: substring
/// Gets a substring from start to end (exclusive)
fn builtinSubstring(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 3) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `substring`. want=3");
    }

    if (args[0].objectType() != .string) {
        return try object_mod.makeError(allocator, "first argument to `substring` must be STRING");
    }

    if (args[1].objectType() != .integer or args[2].objectType() != .integer) {
        return try object_mod.makeError(allocator, "second and third arguments to `substring` must be INTEGER");
    }

    const str = args[0].string.value;
    var start = args[1].integer.value;
    var end = args[2].integer.value;

    // Handle negative indices
    if (start < 0) start = 0;
    if (end > str.len) end = @intCast(str.len);
    if (start >= end or start >= str.len) {
        return object_mod.makeString(allocator, "");
    }

    const substr = str[@intCast(start)..@intCast(end)];
    return object_mod.makeString(allocator, substr);
}

/// Builtin function: indexOf
/// Finds the index of a substring
fn builtinIndexOf(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 2) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `indexOf`. want=2");
    }

    if (args[0].objectType() != .string or args[1].objectType() != .string) {
        return try object_mod.makeError(allocator, "arguments to `indexOf` must be STRING");
    }

    const str = args[0].string.value;
    const substr = args[1].string.value;

    if (std.mem.indexOf(u8, str, substr)) |idx| {
        return object_mod.makeInteger(@intCast(idx));
    }
    return object_mod.makeInteger(-1);
}

// =============================================================================
// Math Functions (v0.8.0)
// =============================================================================

/// Builtin function: abs
/// Returns the absolute value of an integer
fn builtinAbs(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `abs`. want=1");
    }

    if (args[0].objectType() != .integer) {
        return try object_mod.makeError(allocator, "argument to `abs` must be INTEGER");
    }

    const value = args[0].integer.value;
    return object_mod.makeInteger(if (value < 0) -value else value);
}

/// Builtin function: min
/// Returns the minimum of two integers or the minimum element in an array
fn builtinMin(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len == 2) {
        // min(a, b)
        if (args[0].objectType() != .integer or args[1].objectType() != .integer) {
            return try object_mod.makeError(allocator, "arguments to `min` must be INTEGER");
        }
        const a = args[0].integer.value;
        const b = args[1].integer.value;
        return object_mod.makeInteger(if (a < b) a else b);
    } else if (args.len == 1 and args[0].objectType() == .array) {
        // min(array)
        const elements = args[0].array.elements;
        if (elements.len == 0) {
            return try object_mod.makeError(allocator, "array is empty");
        }
        if (elements[0].objectType() != .integer) {
            return try object_mod.makeError(allocator, "array elements must be INTEGER");
        }
        var min_val = elements[0].integer.value;
        for (elements[1..]) |elem| {
            if (elem.objectType() != .integer) {
                return try object_mod.makeError(allocator, "array elements must be INTEGER");
            }
            if (elem.integer.value < min_val) min_val = elem.integer.value;
        }
        return object_mod.makeInteger(min_val);
    }
    return try object_mod.makeError(allocator, "wrong arguments to `min`. want min(a, b) or min(array)");
}

/// Builtin function: max
/// Returns the maximum of two integers or the maximum element in an array
fn builtinMax(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len == 2) {
        // max(a, b)
        if (args[0].objectType() != .integer or args[1].objectType() != .integer) {
            return try object_mod.makeError(allocator, "arguments to `max` must be INTEGER");
        }
        const a = args[0].integer.value;
        const b = args[1].integer.value;
        return object_mod.makeInteger(if (a > b) a else b);
    } else if (args.len == 1 and args[0].objectType() == .array) {
        // max(array)
        const elements = args[0].array.elements;
        if (elements.len == 0) {
            return try object_mod.makeError(allocator, "array is empty");
        }
        if (elements[0].objectType() != .integer) {
            return try object_mod.makeError(allocator, "array elements must be INTEGER");
        }
        var max_val = elements[0].integer.value;
        for (elements[1..]) |elem| {
            if (elem.objectType() != .integer) {
                return try object_mod.makeError(allocator, "array elements must be INTEGER");
            }
            if (elem.integer.value > max_val) max_val = elem.integer.value;
        }
        return object_mod.makeInteger(max_val);
    }
    return try object_mod.makeError(allocator, "wrong arguments to `max`. want max(a, b) or max(array)");
}

/// Builtin function: pow
/// Returns base raised to the power of exp
fn builtinPow(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 2) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `pow`. want=2");
    }

    if (args[0].objectType() != .integer or args[1].objectType() != .integer) {
        return try object_mod.makeError(allocator, "arguments to `pow` must be INTEGER");
    }

    const base = args[0].integer.value;
    const exp = args[1].integer.value;

    if (exp < 0) {
        return try object_mod.makeError(allocator, "exponent must be non-negative");
    }

    var result: i64 = 1;
    var i: i64 = 0;
    while (i < exp) : (i += 1) {
        result *= base;
    }
    return object_mod.makeInteger(result);
}

/// Builtin function: sqrt
/// Returns the integer square root
fn builtinSqrt(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `sqrt`. want=1");
    }

    if (args[0].objectType() != .integer) {
        return try object_mod.makeError(allocator, "argument to `sqrt` must be INTEGER");
    }

    const value = args[0].integer.value;
    if (value < 0) {
        return try object_mod.makeError(allocator, "cannot take sqrt of negative number");
    }

    // Integer square root using Newton's method
    if (value == 0) return object_mod.makeInteger(0);
    var x: i64 = value;
    var y: i64 = @divTrunc(x + 1, 2);
    while (y < x) {
        x = y;
        y = @divTrunc(x + @divTrunc(value, x), 2);
    }
    return object_mod.makeInteger(x);
}

/// Builtin function: sum
/// Returns the sum of all elements in an array
fn builtinSum(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `sum`. want=1");
    }

    if (args[0].objectType() != .array) {
        return try object_mod.makeError(allocator, "argument to `sum` must be ARRAY");
    }

    const elements = args[0].array.elements;
    var total: i64 = 0;
    for (elements) |elem| {
        if (elem.objectType() != .integer) {
            return try object_mod.makeError(allocator, "array elements must be INTEGER");
        }
        total += elem.integer.value;
    }
    return object_mod.makeInteger(total);
}

// =============================================================================
// Array Operations (v0.8.0)
// =============================================================================

/// Builtin function: reverse
/// Returns a new array with elements in reverse order
fn builtinReverse(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `reverse`. want=1");
    }

    if (args[0].objectType() != .array) {
        return try object_mod.makeError(allocator, "argument to `reverse` must be ARRAY");
    }

    const elements = args[0].array.elements;
    var reversed = try allocator.alloc(Object, elements.len);
    for (elements, 0..) |elem, i| {
        reversed[elements.len - 1 - i] = elem;
    }
    return object_mod.makeArray(allocator, reversed);
}

/// Builtin function: sort
/// Returns a new sorted array (integers only, ascending order)
fn builtinSort(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `sort`. want=1");
    }

    if (args[0].objectType() != .array) {
        return try object_mod.makeError(allocator, "argument to `sort` must be ARRAY");
    }

    const elements = args[0].array.elements;
    if (elements.len == 0) {
        const empty = try allocator.alloc(Object, 0);
        return object_mod.makeArray(allocator, empty);
    }

    // Check if all elements are integers
    for (elements) |elem| {
        if (elem.objectType() != .integer) {
            return try object_mod.makeError(allocator, "sort requires all elements to be INTEGER");
        }
    }

    // Create a copy for sorting
    var sorted = try allocator.alloc(Object, elements.len);
    @memcpy(sorted, elements);

    // Simple bubble sort (for simplicity; could use std.sort for better performance)
    var i: usize = 0;
    while (i < sorted.len) : (i += 1) {
        var j: usize = 0;
        while (j < sorted.len - 1 - i) : (j += 1) {
            if (sorted[j].integer.value > sorted[j + 1].integer.value) {
                const tmp = sorted[j];
                sorted[j] = sorted[j + 1];
                sorted[j + 1] = tmp;
            }
        }
    }

    return object_mod.makeArray(allocator, sorted);
}

/// Builtin function: find
/// Returns the first element for which the function returns true
fn builtinFind(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 2) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `find`. want=2");
    }

    if (args[0].objectType() != .array) {
        return try object_mod.makeError(allocator, "first argument to `find` must be ARRAY");
    }

    if (args[1].objectType() != .function and args[1].objectType() != .builtin) {
        return try object_mod.makeError(allocator, "second argument to `find` must be FUNCTION");
    }

    const arr = args[0].array;
    const func = args[1];

    for (arr.elements) |element| {
        var fn_args = [_]Object{element};
        const result = try applyFunction(allocator, func, &fn_args);
        if (result.objectType() == .@"error") {
            return result;
        }
        const truthy = switch (result) {
            .null => false,
            .boolean => |b| b.value,
            else => true,
        };
        if (truthy) {
            return element;
        }
    }

    return object_mod.makeNull();
}

/// Builtin function: some
/// Returns true if at least one element satisfies the predicate
fn builtinSome(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 2) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `some`. want=2");
    }

    if (args[0].objectType() != .array) {
        return try object_mod.makeError(allocator, "first argument to `some` must be ARRAY");
    }

    if (args[1].objectType() != .function and args[1].objectType() != .builtin) {
        return try object_mod.makeError(allocator, "second argument to `some` must be FUNCTION");
    }

    const arr = args[0].array;
    const func = args[1];

    for (arr.elements) |element| {
        var fn_args = [_]Object{element};
        const result = try applyFunction(allocator, func, &fn_args);
        if (result.objectType() == .@"error") {
            return result;
        }
        const truthy = switch (result) {
            .null => false,
            .boolean => |b| b.value,
            else => true,
        };
        if (truthy) {
            return object_mod.makeBoolean(true);
        }
    }

    return object_mod.makeBoolean(false);
}

/// Builtin function: every
/// Returns true if all elements satisfy the predicate
fn builtinEvery(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 2) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `every`. want=2");
    }

    if (args[0].objectType() != .array) {
        return try object_mod.makeError(allocator, "first argument to `every` must be ARRAY");
    }

    if (args[1].objectType() != .function and args[1].objectType() != .builtin) {
        return try object_mod.makeError(allocator, "second argument to `every` must be FUNCTION");
    }

    const arr = args[0].array;
    const func = args[1];

    for (arr.elements) |element| {
        var fn_args = [_]Object{element};
        const result = try applyFunction(allocator, func, &fn_args);
        if (result.objectType() == .@"error") {
            return result;
        }
        const truthy = switch (result) {
            .null => false,
            .boolean => |b| b.value,
            else => true,
        };
        if (!truthy) {
            return object_mod.makeBoolean(false);
        }
    }

    return object_mod.makeBoolean(true);
}

/// Builtin function: slice
/// Returns a slice of an array from start to end (exclusive)
fn builtinSlice(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 3) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `slice`. want=3");
    }

    if (args[0].objectType() != .array) {
        return try object_mod.makeError(allocator, "first argument to `slice` must be ARRAY");
    }

    if (args[1].objectType() != .integer or args[2].objectType() != .integer) {
        return try object_mod.makeError(allocator, "second and third arguments to `slice` must be INTEGER");
    }

    const elements = args[0].array.elements;
    var start = args[1].integer.value;
    var end = args[2].integer.value;

    // Handle negative indices and bounds
    if (start < 0) start = 0;
    if (end > @as(i64, @intCast(elements.len))) end = @intCast(elements.len);
    if (start >= end or start >= @as(i64, @intCast(elements.len))) {
        const empty = try allocator.alloc(Object, 0);
        return object_mod.makeArray(allocator, empty);
    }

    const start_usize: usize = @intCast(start);
    const end_usize: usize = @intCast(end);
    const sliced = try allocator.dupe(Object, elements[start_usize..end_usize]);
    return object_mod.makeArray(allocator, sliced);
}

/// Builtin function: concat
/// Concatenates two arrays
fn builtinConcat(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 2) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `concat`. want=2");
    }

    if (args[0].objectType() != .array or args[1].objectType() != .array) {
        return try object_mod.makeError(allocator, "arguments to `concat` must be ARRAY");
    }

    const arr1 = args[0].array.elements;
    const arr2 = args[1].array.elements;

    var result = try allocator.alloc(Object, arr1.len + arr2.len);
    @memcpy(result[0..arr1.len], arr1);
    @memcpy(result[arr1.len..], arr2);

    return object_mod.makeArray(allocator, result);
}

/// Builtin function: flatten
/// Flattens nested arrays by one level
fn builtinFlatten(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `flatten`. want=1");
    }

    if (args[0].objectType() != .array) {
        return try object_mod.makeError(allocator, "argument to `flatten` must be ARRAY");
    }

    const elements = args[0].array.elements;
    var result = try std.ArrayList(Object).initCapacity(allocator, elements.len * 2);
    defer result.deinit(allocator);

    for (elements) |elem| {
        if (elem.objectType() == .array) {
            for (elem.array.elements) |inner| {
                try result.append(allocator, inner);
            }
        } else {
            try result.append(allocator, elem);
        }
    }

    const final = try result.toOwnedSlice(allocator);
    return object_mod.makeArray(allocator, final);
}

// =============================================================================
// System Interaction (v0.8.0)
// =============================================================================

/// Builtin function: getenv
/// Gets an environment variable value
fn builtinGetenv(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `getenv`. want=1");
    }

    if (args[0].objectType() != .string) {
        return try object_mod.makeError(allocator, "argument to `getenv` must be STRING");
    }

    const name = args[0].string.value;
    const value = std.posix.getenv(name);

    if (value) |v| {
        return object_mod.makeString(allocator, v);
    }
    return object_mod.makeNull();
}

/// Builtin function: time
/// Returns the current Unix timestamp in milliseconds
fn builtinTime(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    _ = allocator;
    if (args.len != 0) {
        return object_mod.makeInteger(-1); // Error case, but we can't call makeError here
    }

    const now = std.time.milliTimestamp();
    return object_mod.makeInteger(now);
}

/// Builtin function: sleep
/// Pauses execution for the specified number of milliseconds
fn builtinSleep(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `sleep`. want=1");
    }

    if (args[0].objectType() != .integer) {
        return try object_mod.makeError(allocator, "argument to `sleep` must be INTEGER");
    }

    const ms = args[0].integer.value;
    if (ms < 0) {
        return try object_mod.makeError(allocator, "sleep duration must be non-negative");
    }

    std.Thread.sleep(@intCast(ms * 1_000_000)); // Convert ms to ns
    return object_mod.makeNull();
}

// =============================================================================
// Type Conversion (v0.8.0)
// =============================================================================

/// Builtin function: bool
/// Converts a value to boolean
fn builtinBool(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `bool`. want=1");
    }

    const truthy = switch (args[0]) {
        .null => false,
        .boolean => |b| b.value,
        .integer => |i| i.value != 0,
        .string => |s| s.value.len > 0,
        .array => |a| a.elements.len > 0,
        else => true,
    };

    return object_mod.makeBoolean(truthy);
}

/// Builtin function: array
/// Converts a string to an array of characters
fn builtinArray(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `array`. want=1");
    }

    if (args[0].objectType() == .array) {
        // Already an array, return copy
        const elements = try allocator.dupe(Object, args[0].array.elements);
        return object_mod.makeArray(allocator, elements);
    }

    if (args[0].objectType() != .string) {
        return try object_mod.makeError(allocator, "argument to `array` must be STRING or ARRAY");
    }

    const str = args[0].string.value;
    var chars = try std.ArrayList(Object).initCapacity(allocator, str.len);
    defer chars.deinit(allocator);

    for (str) |c| {
        var char_str = try allocator.alloc(u8, 1);
        char_str[0] = c;
        try chars.append(allocator, try object_mod.makeStringOwned(allocator, char_str));
    }

    const elements = try chars.toOwnedSlice(allocator);
    return object_mod.makeArray(allocator, elements);
}

// =============================================================================
// Random Functions (v0.9.0)
// =============================================================================

/// Global random state - initialized on first use
var random_state: ?std.Random.DefaultPrng = null;

fn getRandomGenerator() std.Random {
    if (random_state == null) {
        random_state = std.Random.DefaultPrng.init(@intCast(std.time.milliTimestamp()));
    }
    return random_state.?.random();
}

/// Builtin function: rand
/// Returns a random integer
/// rand() - random integer
/// rand(n) - random 0 to n-1
/// rand(min, max) - random min to max-1
fn builtinRand(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    const random = getRandomGenerator();

    if (args.len == 0) {
        // rand() - return random i64
        const value = random.int(i63);
        return object_mod.makeInteger(@intCast(value));
    } else if (args.len == 1) {
        // rand(n) - return 0 to n-1
        if (args[0].objectType() != .integer) {
            return try object_mod.makeError(allocator, "argument to `rand` must be INTEGER");
        }
        const n = args[0].integer.value;
        if (n <= 0) {
            return try object_mod.makeError(allocator, "argument to `rand` must be positive");
        }
        const value = @mod(random.int(i63), n);
        return object_mod.makeInteger(if (value < 0) -value else value);
    } else if (args.len == 2) {
        // rand(min, max) - return min to max-1
        if (args[0].objectType() != .integer or args[1].objectType() != .integer) {
            return try object_mod.makeError(allocator, "arguments to `rand` must be INTEGER");
        }
        const min_val = args[0].integer.value;
        const max_val = args[1].integer.value;
        if (min_val >= max_val) {
            return try object_mod.makeError(allocator, "min must be less than max");
        }
        const range_size = max_val - min_val;
        const value = @mod(random.int(i63), range_size);
        return object_mod.makeInteger(min_val + (if (value < 0) -value else value));
    }
    return try object_mod.makeError(allocator, "wrong number of arguments to `rand`. want=0, 1, or 2");
}

/// Builtin function: shuffle
/// Returns a new array with elements in random order
fn builtinShuffle(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `shuffle`. want=1");
    }

    if (args[0].objectType() != .array) {
        return try object_mod.makeError(allocator, "argument to `shuffle` must be ARRAY");
    }

    const elements = args[0].array.elements;
    if (elements.len == 0) {
        const empty = try allocator.alloc(Object, 0);
        return object_mod.makeArray(allocator, empty);
    }

    // Create a copy for shuffling
    var shuffled = try allocator.alloc(Object, elements.len);
    @memcpy(shuffled, elements);

    // Fisher-Yates shuffle
    const random = getRandomGenerator();
    var i: usize = shuffled.len - 1;
    while (i > 0) : (i -= 1) {
        const j = @mod(random.int(usize), i + 1);
        const tmp = shuffled[i];
        shuffled[i] = shuffled[j];
        shuffled[j] = tmp;
    }

    return object_mod.makeArray(allocator, shuffled);
}

// =============================================================================
// Type Check Functions (v0.9.0)
// =============================================================================

/// Builtin function: isInt
fn builtinIsInt(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `isInt`. want=1");
    }
    return object_mod.makeBoolean(args[0].objectType() == .integer);
}

/// Builtin function: isStr
fn builtinIsStr(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `isStr`. want=1");
    }
    return object_mod.makeBoolean(args[0].objectType() == .string);
}

/// Builtin function: isBool
fn builtinIsBool(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `isBool`. want=1");
    }
    return object_mod.makeBoolean(args[0].objectType() == .boolean);
}

/// Builtin function: isArray
fn builtinIsArray(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `isArray`. want=1");
    }
    return object_mod.makeBoolean(args[0].objectType() == .array);
}

/// Builtin function: isHash
fn builtinIsHash(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `isHash`. want=1");
    }
    return object_mod.makeBoolean(args[0].objectType() == .hash);
}

/// Builtin function: isFunc
fn builtinIsFunc(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `isFunc`. want=1");
    }
    const obj_type = args[0].objectType();
    return object_mod.makeBoolean(obj_type == .function or obj_type == .builtin);
}

/// Builtin function: isNull
fn builtinIsNull(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `isNull`. want=1");
    }
    return object_mod.makeBoolean(args[0].objectType() == .null);
}

// =============================================================================
// String Functions (v0.9.0)
// =============================================================================

/// Builtin function: startsWith
fn builtinStartsWith(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 2) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `startsWith`. want=2");
    }

    if (args[0].objectType() != .string or args[1].objectType() != .string) {
        return try object_mod.makeError(allocator, "arguments to `startsWith` must be STRING");
    }

    const str = args[0].string.value;
    const prefix = args[1].string.value;

    return object_mod.makeBoolean(std.mem.startsWith(u8, str, prefix));
}

/// Builtin function: endsWith
fn builtinEndsWith(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 2) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `endsWith`. want=2");
    }

    if (args[0].objectType() != .string or args[1].objectType() != .string) {
        return try object_mod.makeError(allocator, "arguments to `endsWith` must be STRING");
    }

    const str = args[0].string.value;
    const suffix = args[1].string.value;

    return object_mod.makeBoolean(std.mem.endsWith(u8, str, suffix));
}

/// Builtin function: repeat
fn builtinRepeat(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 2) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `repeat`. want=2");
    }

    if (args[0].objectType() != .string) {
        return try object_mod.makeError(allocator, "first argument to `repeat` must be STRING");
    }

    if (args[1].objectType() != .integer) {
        return try object_mod.makeError(allocator, "second argument to `repeat` must be INTEGER");
    }

    const str = args[0].string.value;
    const n = args[1].integer.value;

    if (n <= 0) {
        return object_mod.makeString(allocator, "");
    }

    const count: usize = @intCast(n);
    var result = try allocator.alloc(u8, str.len * count);
    for (0..count) |i| {
        @memcpy(result[i * str.len .. (i + 1) * str.len], str);
    }

    return object_mod.makeStringOwned(allocator, result);
}

/// Builtin function: padLeft
fn builtinPadLeft(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 3) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `padLeft`. want=3");
    }

    if (args[0].objectType() != .string) {
        return try object_mod.makeError(allocator, "first argument to `padLeft` must be STRING");
    }
    if (args[1].objectType() != .integer) {
        return try object_mod.makeError(allocator, "second argument to `padLeft` must be INTEGER");
    }
    if (args[2].objectType() != .string) {
        return try object_mod.makeError(allocator, "third argument to `padLeft` must be STRING");
    }

    const str = args[0].string.value;
    const target_len: usize = @intCast(@max(0, args[1].integer.value));
    const pad_char = args[2].string.value;

    if (pad_char.len == 0 or str.len >= target_len) {
        return object_mod.makeString(allocator, str);
    }

    const pad_count = target_len - str.len;
    var result = try allocator.alloc(u8, target_len);

    // Fill with pad character
    for (0..pad_count) |i| {
        result[i] = pad_char[0];
    }
    @memcpy(result[pad_count..], str);

    return object_mod.makeStringOwned(allocator, result);
}

/// Builtin function: padRight
fn builtinPadRight(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 3) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `padRight`. want=3");
    }

    if (args[0].objectType() != .string) {
        return try object_mod.makeError(allocator, "first argument to `padRight` must be STRING");
    }
    if (args[1].objectType() != .integer) {
        return try object_mod.makeError(allocator, "second argument to `padRight` must be INTEGER");
    }
    if (args[2].objectType() != .string) {
        return try object_mod.makeError(allocator, "third argument to `padRight` must be STRING");
    }

    const str = args[0].string.value;
    const target_len: usize = @intCast(@max(0, args[1].integer.value));
    const pad_char = args[2].string.value;

    if (pad_char.len == 0 or str.len >= target_len) {
        return object_mod.makeString(allocator, str);
    }

    const result = try allocator.alloc(u8, target_len);

    @memcpy(result[0..str.len], str);
    // Fill with pad character
    for (str.len..target_len) |i| {
        result[i] = pad_char[0];
    }

    return object_mod.makeStringOwned(allocator, result);
}

// =============================================================================
// Math Functions (v0.9.0)
// =============================================================================

/// Builtin function: sign
fn builtinSign(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `sign`. want=1");
    }

    if (args[0].objectType() != .integer) {
        return try object_mod.makeError(allocator, "argument to `sign` must be INTEGER");
    }

    const value = args[0].integer.value;
    if (value > 0) return object_mod.makeInteger(1);
    if (value < 0) return object_mod.makeInteger(-1);
    return object_mod.makeInteger(0);
}

/// Builtin function: clamp
fn builtinClamp(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 3) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `clamp`. want=3");
    }

    if (args[0].objectType() != .integer or args[1].objectType() != .integer or args[2].objectType() != .integer) {
        return try object_mod.makeError(allocator, "arguments to `clamp` must be INTEGER");
    }

    const value = args[0].integer.value;
    const min_val = args[1].integer.value;
    const max_val = args[2].integer.value;

    if (value < min_val) return object_mod.makeInteger(min_val);
    if (value > max_val) return object_mod.makeInteger(max_val);
    return object_mod.makeInteger(value);
}

/// Builtin function: gcd (greatest common divisor)
fn builtinGcd(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 2) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `gcd`. want=2");
    }

    if (args[0].objectType() != .integer or args[1].objectType() != .integer) {
        return try object_mod.makeError(allocator, "arguments to `gcd` must be INTEGER");
    }

    var a = if (args[0].integer.value < 0) -args[0].integer.value else args[0].integer.value;
    var b = if (args[1].integer.value < 0) -args[1].integer.value else args[1].integer.value;

    while (b != 0) {
        const t = b;
        b = @mod(a, b);
        a = t;
    }

    return object_mod.makeInteger(a);
}

/// Builtin function: lcm (least common multiple)
fn builtinLcm(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 2) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `lcm`. want=2");
    }

    if (args[0].objectType() != .integer or args[1].objectType() != .integer) {
        return try object_mod.makeError(allocator, "arguments to `lcm` must be INTEGER");
    }

    const a = if (args[0].integer.value < 0) -args[0].integer.value else args[0].integer.value;
    const b = if (args[1].integer.value < 0) -args[1].integer.value else args[1].integer.value;

    if (a == 0 or b == 0) {
        return object_mod.makeInteger(0);
    }

    // Calculate GCD first
    var gcd_a = a;
    var gcd_b = b;
    while (gcd_b != 0) {
        const t = gcd_b;
        gcd_b = @mod(gcd_a, gcd_b);
        gcd_a = t;
    }

    // LCM = |a * b| / GCD(a, b)
    return object_mod.makeInteger(@divTrunc(a * b, gcd_a));
}

/// Builtin function: avg (average of array)
fn builtinAvg(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `avg`. want=1");
    }

    if (args[0].objectType() != .array) {
        return try object_mod.makeError(allocator, "argument to `avg` must be ARRAY");
    }

    const elements = args[0].array.elements;
    if (elements.len == 0) {
        return object_mod.makeInteger(0);
    }

    var total: i64 = 0;
    for (elements) |elem| {
        if (elem.objectType() != .integer) {
            return try object_mod.makeError(allocator, "array elements must be INTEGER");
        }
        total += elem.integer.value;
    }

    return object_mod.makeInteger(@divTrunc(total, @as(i64, @intCast(elements.len))));
}

/// Builtin function: product (product of array elements)
fn builtinProduct(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `product`. want=1");
    }

    if (args[0].objectType() != .array) {
        return try object_mod.makeError(allocator, "argument to `product` must be ARRAY");
    }

    const elements = args[0].array.elements;
    if (elements.len == 0) {
        return object_mod.makeInteger(1);
    }

    var result: i64 = 1;
    for (elements) |elem| {
        if (elem.objectType() != .integer) {
            return try object_mod.makeError(allocator, "array elements must be INTEGER");
        }
        result *= elem.integer.value;
    }

    return object_mod.makeInteger(result);
}

// =============================================================================
// Utility Functions (v0.9.0)
// =============================================================================

/// Builtin function: assert
fn builtinAssert(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len < 1 or args.len > 2) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `assert`. want=1 or 2");
    }

    const condition = switch (args[0]) {
        .null => false,
        .boolean => |b| b.value,
        else => true,
    };

    if (!condition) {
        if (args.len == 2 and args[1].objectType() == .string) {
            return try object_mod.makeError(allocator, args[1].string.value);
        }
        return try object_mod.makeError(allocator, "assertion failed");
    }

    return object_mod.makeNull();
}

/// Builtin function: typeof (alias for type with more explicit name)
fn builtinTypeof(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    return builtinType(allocator, args);
}

/// Builtin function: default
/// Returns the first argument if it's not null, otherwise returns the second
fn builtinDefault(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 2) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `default`. want=2");
    }

    if (args[0].objectType() == .null) {
        return args[1];
    }
    return args[0];
}

/// Builtin function: args (v0.10.0)
/// Returns the command-line arguments as an array of strings
fn builtinArgs(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    if (args.len != 0) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `args`. want=0");
    }

    if (script_args) |sa| {
        var elements = try allocator.alloc(Object, sa.len);
        for (sa, 0..) |arg, i| {
            elements[i] = try object_mod.makeString(allocator, arg);
        }
        return object_mod.makeArray(allocator, elements);
    } else {
        // No args set, return empty array
        const elements = try allocator.alloc(Object, 0);
        return object_mod.makeArray(allocator, elements);
    }
}

// Track imported files to prevent circular imports
var imported_files: ?std.StringHashMap(bool) = null;
var imported_files_allocator: ?std.mem.Allocator = null;

/// Initialize the import tracking
fn initImportTracking(allocator: std.mem.Allocator) void {
    if (imported_files == null) {
        imported_files = std.StringHashMap(bool).init(allocator);
        imported_files_allocator = allocator;
    }
}

/// Clear imported files tracking (for cleanup)
pub fn clearImportTracking() void {
    if (imported_files) |*files| {
        files.deinit();
    }
    imported_files = null;
    imported_files_allocator = null;
}

/// Builtin function: import (v0.10.0)
/// Imports and executes another Monkey file, returning the last expression's value
/// The imported code runs in the current environment, so defined variables/functions become available
/// This is the version with environment access (called from evaluator)
pub fn builtinImportWithEnv(allocator: std.mem.Allocator, args: []Object, env: *Environment) anyerror!Object {
    if (args.len != 1) {
        return try object_mod.makeError(allocator, "wrong number of arguments to `import`. want=1");
    }

    if (args[0].objectType() != .string) {
        return try object_mod.makeError(allocator, "argument to `import` must be STRING");
    }

    const path = args[0].string.value;

    // Initialize import tracking if needed
    initImportTracking(allocator);

    // Check for circular imports
    if (imported_files) |*files| {
        if (files.get(path)) |_| {
            // Already imported, return null (prevent circular import)
            return object_mod.makeNull();
        }
        // Mark as imported
        const path_copy = try allocator.dupe(u8, path);
        try files.put(path_copy, true);
    }

    // Read the file
    const file = std.fs.cwd().openFile(path, .{}) catch |err| {
        const error_msg = try std.fmt.allocPrint(allocator, "import error: could not open file '{s}': {}", .{ path, err });
        defer allocator.free(error_msg);
        return try object_mod.makeError(allocator, error_msg);
    };
    defer file.close();

    const content = file.readToEndAlloc(allocator, 10 * 1024 * 1024) catch |err| {
        const error_msg = try std.fmt.allocPrint(allocator, "import error: could not read file '{s}': {}", .{ path, err });
        defer allocator.free(error_msg);
        return try object_mod.makeError(allocator, error_msg);
    };
    defer allocator.free(content);

    // Tokenize
    var lexer = lexer_mod.Lexer.init(content);
    var tokens = try std.ArrayList(lexer_mod.Token).initCapacity(allocator, 256);
    defer tokens.deinit(allocator);

    while (true) {
        const tok = lexer.nextToken();
        try tokens.append(allocator, tok);
        if (tok.token_type == .EOF) break;
    }

    // Parse
    var parser = parser_mod.Parser.init(allocator, tokens.items);

    const program = parser.parseProgram() catch |err| {
        const error_msg = try std.fmt.allocPrint(allocator, "import error: parse error in '{s}': {}", .{ path, err });
        defer allocator.free(error_msg);
        return try object_mod.makeError(allocator, error_msg);
    };

    // Evaluate in the current environment
    var result = object_mod.makeNull();
    for (program.statements) |stmt| {
        result = try evalStatementForBuiltin(allocator, stmt, env);
        if (result.objectType() == .return_value) {
            result = result.return_value.value.*;
            break;
        }
        if (result.objectType() == .@"error") {
            break;
        }
    }

    return result;
}

/// Wrapper for import builtin (the actual implementation uses environment)
/// This is just for registration - the actual call is handled specially in evaluator
fn builtinImport(allocator: std.mem.Allocator, args: []Object) anyerror!Object {
    _ = args;
    // This should never be called directly - evaluator handles import specially
    return try object_mod.makeError(allocator, "import must be called through evaluator");
}

/// Get a builtin function by name
pub fn getBuiltin(name: []const u8) ?Object {
    if (std.mem.eql(u8, name, "len")) {
        return object_mod.makeBuiltin("len", builtinLen);
    } else if (std.mem.eql(u8, name, "first")) {
        return object_mod.makeBuiltin("first", builtinFirst);
    } else if (std.mem.eql(u8, name, "last")) {
        return object_mod.makeBuiltin("last", builtinLast);
    } else if (std.mem.eql(u8, name, "rest")) {
        return object_mod.makeBuiltin("rest", builtinRest);
    } else if (std.mem.eql(u8, name, "push")) {
        return object_mod.makeBuiltin("push", builtinPush);
    } else if (std.mem.eql(u8, name, "puts")) {
        return object_mod.makeBuiltin("puts", builtinPuts);
    } else if (std.mem.eql(u8, name, "println")) {
        // println is an alias for puts
        return object_mod.makeBuiltin("println", builtinPuts);
    } else if (std.mem.eql(u8, name, "type")) {
        return object_mod.makeBuiltin("type", builtinType);
    } else if (std.mem.eql(u8, name, "print")) {
        return object_mod.makeBuiltin("print", builtinPrint);
    } else if (std.mem.eql(u8, name, "str")) {
        return object_mod.makeBuiltin("str", builtinStr);
    } else if (std.mem.eql(u8, name, "int")) {
        return object_mod.makeBuiltin("int", builtinInt);
    } else if (std.mem.eql(u8, name, "keys")) {
        return object_mod.makeBuiltin("keys", builtinKeys);
    } else if (std.mem.eql(u8, name, "values")) {
        return object_mod.makeBuiltin("values", builtinValues);
    } else if (std.mem.eql(u8, name, "range")) {
        return object_mod.makeBuiltin("range", builtinRange);
    } else if (std.mem.eql(u8, name, "map")) {
        return object_mod.makeBuiltin("map", builtinMap);
    } else if (std.mem.eql(u8, name, "filter")) {
        return object_mod.makeBuiltin("filter", builtinFilter);
    } else if (std.mem.eql(u8, name, "reduce")) {
        return object_mod.makeBuiltin("reduce", builtinReduce);
    }
    // File I/O functions
    else if (std.mem.eql(u8, name, "readFile")) {
        return object_mod.makeBuiltin("readFile", builtinReadFile);
    } else if (std.mem.eql(u8, name, "writeFile")) {
        return object_mod.makeBuiltin("writeFile", builtinWriteFile);
    } else if (std.mem.eql(u8, name, "appendFile")) {
        return object_mod.makeBuiltin("appendFile", builtinAppendFile);
    } else if (std.mem.eql(u8, name, "fileExists")) {
        return object_mod.makeBuiltin("fileExists", builtinFileExists);
    }
    // String manipulation functions
    else if (std.mem.eql(u8, name, "split")) {
        return object_mod.makeBuiltin("split", builtinSplit);
    } else if (std.mem.eql(u8, name, "join")) {
        return object_mod.makeBuiltin("join", builtinJoin);
    } else if (std.mem.eql(u8, name, "trim")) {
        return object_mod.makeBuiltin("trim", builtinTrim);
    } else if (std.mem.eql(u8, name, "upper")) {
        return object_mod.makeBuiltin("upper", builtinUpper);
    } else if (std.mem.eql(u8, name, "lower")) {
        return object_mod.makeBuiltin("lower", builtinLower);
    } else if (std.mem.eql(u8, name, "contains")) {
        return object_mod.makeBuiltin("contains", builtinContains);
    } else if (std.mem.eql(u8, name, "replace")) {
        return object_mod.makeBuiltin("replace", builtinReplace);
    } else if (std.mem.eql(u8, name, "charAt")) {
        return object_mod.makeBuiltin("charAt", builtinCharAt);
    } else if (std.mem.eql(u8, name, "substring")) {
        return object_mod.makeBuiltin("substring", builtinSubstring);
    } else if (std.mem.eql(u8, name, "indexOf")) {
        return object_mod.makeBuiltin("indexOf", builtinIndexOf);
    }
    // Math functions (v0.8.0)
    else if (std.mem.eql(u8, name, "abs")) {
        return object_mod.makeBuiltin("abs", builtinAbs);
    } else if (std.mem.eql(u8, name, "min")) {
        return object_mod.makeBuiltin("min", builtinMin);
    } else if (std.mem.eql(u8, name, "max")) {
        return object_mod.makeBuiltin("max", builtinMax);
    } else if (std.mem.eql(u8, name, "pow")) {
        return object_mod.makeBuiltin("pow", builtinPow);
    } else if (std.mem.eql(u8, name, "sqrt")) {
        return object_mod.makeBuiltin("sqrt", builtinSqrt);
    } else if (std.mem.eql(u8, name, "sum")) {
        return object_mod.makeBuiltin("sum", builtinSum);
    }
    // Array operations (v0.8.0)
    else if (std.mem.eql(u8, name, "reverse")) {
        return object_mod.makeBuiltin("reverse", builtinReverse);
    } else if (std.mem.eql(u8, name, "sort")) {
        return object_mod.makeBuiltin("sort", builtinSort);
    } else if (std.mem.eql(u8, name, "find")) {
        return object_mod.makeBuiltin("find", builtinFind);
    } else if (std.mem.eql(u8, name, "some")) {
        return object_mod.makeBuiltin("some", builtinSome);
    } else if (std.mem.eql(u8, name, "every")) {
        return object_mod.makeBuiltin("every", builtinEvery);
    } else if (std.mem.eql(u8, name, "slice")) {
        return object_mod.makeBuiltin("slice", builtinSlice);
    } else if (std.mem.eql(u8, name, "concat")) {
        return object_mod.makeBuiltin("concat", builtinConcat);
    } else if (std.mem.eql(u8, name, "flatten")) {
        return object_mod.makeBuiltin("flatten", builtinFlatten);
    }
    // System interaction (v0.8.0)
    else if (std.mem.eql(u8, name, "getenv")) {
        return object_mod.makeBuiltin("getenv", builtinGetenv);
    } else if (std.mem.eql(u8, name, "time")) {
        return object_mod.makeBuiltin("time", builtinTime);
    } else if (std.mem.eql(u8, name, "sleep")) {
        return object_mod.makeBuiltin("sleep", builtinSleep);
    }
    // Type conversion (v0.8.0)
    else if (std.mem.eql(u8, name, "bool")) {
        return object_mod.makeBuiltin("bool", builtinBool);
    } else if (std.mem.eql(u8, name, "array")) {
        return object_mod.makeBuiltin("array", builtinArray);
    }
    // Random functions (v0.9.0)
    else if (std.mem.eql(u8, name, "rand")) {
        return object_mod.makeBuiltin("rand", builtinRand);
    } else if (std.mem.eql(u8, name, "shuffle")) {
        return object_mod.makeBuiltin("shuffle", builtinShuffle);
    }
    // Type check functions (v0.9.0)
    else if (std.mem.eql(u8, name, "isInt")) {
        return object_mod.makeBuiltin("isInt", builtinIsInt);
    } else if (std.mem.eql(u8, name, "isStr")) {
        return object_mod.makeBuiltin("isStr", builtinIsStr);
    } else if (std.mem.eql(u8, name, "isBool")) {
        return object_mod.makeBuiltin("isBool", builtinIsBool);
    } else if (std.mem.eql(u8, name, "isArray")) {
        return object_mod.makeBuiltin("isArray", builtinIsArray);
    } else if (std.mem.eql(u8, name, "isHash")) {
        return object_mod.makeBuiltin("isHash", builtinIsHash);
    } else if (std.mem.eql(u8, name, "isFunc")) {
        return object_mod.makeBuiltin("isFunc", builtinIsFunc);
    } else if (std.mem.eql(u8, name, "isNull")) {
        return object_mod.makeBuiltin("isNull", builtinIsNull);
    }
    // String functions (v0.9.0)
    else if (std.mem.eql(u8, name, "startsWith")) {
        return object_mod.makeBuiltin("startsWith", builtinStartsWith);
    } else if (std.mem.eql(u8, name, "endsWith")) {
        return object_mod.makeBuiltin("endsWith", builtinEndsWith);
    } else if (std.mem.eql(u8, name, "repeat")) {
        return object_mod.makeBuiltin("repeat", builtinRepeat);
    } else if (std.mem.eql(u8, name, "padLeft")) {
        return object_mod.makeBuiltin("padLeft", builtinPadLeft);
    } else if (std.mem.eql(u8, name, "padRight")) {
        return object_mod.makeBuiltin("padRight", builtinPadRight);
    }
    // Math functions (v0.9.0)
    else if (std.mem.eql(u8, name, "sign")) {
        return object_mod.makeBuiltin("sign", builtinSign);
    } else if (std.mem.eql(u8, name, "clamp")) {
        return object_mod.makeBuiltin("clamp", builtinClamp);
    } else if (std.mem.eql(u8, name, "gcd")) {
        return object_mod.makeBuiltin("gcd", builtinGcd);
    } else if (std.mem.eql(u8, name, "lcm")) {
        return object_mod.makeBuiltin("lcm", builtinLcm);
    } else if (std.mem.eql(u8, name, "avg")) {
        return object_mod.makeBuiltin("avg", builtinAvg);
    } else if (std.mem.eql(u8, name, "product")) {
        return object_mod.makeBuiltin("product", builtinProduct);
    }
    // Utility functions (v0.9.0)
    else if (std.mem.eql(u8, name, "assert")) {
        return object_mod.makeBuiltin("assert", builtinAssert);
    } else if (std.mem.eql(u8, name, "typeof")) {
        return object_mod.makeBuiltin("typeof", builtinTypeof);
    } else if (std.mem.eql(u8, name, "default")) {
        return object_mod.makeBuiltin("default", builtinDefault);
    }
    // System functions (v0.10.0)
    else if (std.mem.eql(u8, name, "args")) {
        return object_mod.makeBuiltin("args", builtinArgs);
    } else if (std.mem.eql(u8, name, "import")) {
        return object_mod.makeBuiltin("import", builtinImport);
    }
    return null;
}

test "builtin len" {
    const allocator = std.testing.allocator;

    // Test string length
    var str_obj = try object_mod.makeString(allocator, "hello");
    defer str_obj.deinit(allocator);

    var args = [_]Object{str_obj};
    const result = try builtinLen(allocator, &args);
    try std.testing.expectEqual(@as(i64, 5), result.integer.value);
}

test "builtin first" {
    const allocator = std.testing.allocator;

    var elements = try allocator.alloc(Object, 3);
    defer allocator.free(elements);
    elements[0] = object_mod.makeInteger(1);
    elements[1] = object_mod.makeInteger(2);
    elements[2] = object_mod.makeInteger(3);

    var arr_obj = try object_mod.makeArray(allocator, elements);
    defer arr_obj.deinit(allocator);

    var args = [_]Object{arr_obj};
    const result = try builtinFirst(allocator, &args);
    try std.testing.expectEqual(@as(i64, 1), result.integer.value);
}

test "builtin last" {
    const allocator = std.testing.allocator;

    var elements = try allocator.alloc(Object, 3);
    defer allocator.free(elements);
    elements[0] = object_mod.makeInteger(1);
    elements[1] = object_mod.makeInteger(2);
    elements[2] = object_mod.makeInteger(3);

    var arr_obj = try object_mod.makeArray(allocator, elements);
    defer arr_obj.deinit(allocator);

    var args = [_]Object{arr_obj};
    const result = try builtinLast(allocator, &args);
    try std.testing.expectEqual(@as(i64, 3), result.integer.value);
}

test "getBuiltin" {
    const len_builtin = getBuiltin("len");
    try std.testing.expect(len_builtin != null);
    try std.testing.expectEqual(object_mod.ObjectType.builtin, len_builtin.?.objectType());

    const nonexistent = getBuiltin("nonexistent");
    try std.testing.expect(nonexistent == null);
}
