const std = @import("std");
const object_mod = @import("object.zig");
const Object = object_mod.Object;

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
    };

    return object_mod.makeString(allocator, type_str);
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
    } else if (std.mem.eql(u8, name, "type")) {
        return object_mod.makeBuiltin("type", builtinType);
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
