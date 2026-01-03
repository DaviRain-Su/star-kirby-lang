const std = @import("std");
const object_mod = @import("object.zig");
const ast_mod = @import("ast.zig");
const Object = object_mod.Object;
const Environment = object_mod.Environment;

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
            try env.set(let_stmt.name.value, val);
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
