const std = @import("std");
const ast_mod = @import("ast.zig");

/// Object types in the Monkey language runtime
pub const ObjectType = enum {
    integer,
    boolean,
    null,
    return_value,
    @"error",
    function,
    string,
};

/// Runtime objects in Monkey
pub const Object = union(ObjectType) {
    integer: Integer,
    boolean: Boolean,
    null: Null,
    return_value: ReturnValue,
    @"error": Error,
    function: Function,
    string: StringObj,

    pub fn objectType(self: *const Object) ObjectType {
        return std.meta.activeTag(self.*);
    }

    /// Inspect returns a string representation of the object
    pub fn inspect(self: *const Object, allocator: std.mem.Allocator) ![]u8 {
        return switch (self.*) {
            .integer => |int| try std.fmt.allocPrint(allocator, "{}", .{int.value}),
            .boolean => |bool_val| try std.fmt.allocPrint(allocator, "{}", .{bool_val.value}),
            .null => try allocator.dupe(u8, "null"),
            .return_value => |ret| ret.value.inspect(allocator),
            .@"error" => |err| try std.fmt.allocPrint(allocator, "ERROR: {s}", .{err.message}),
            .function => |func| try std.fmt.allocPrint(allocator, "fn({d} params) {{\n{s}\n}}", .{ func.parameters.items.len, "body" }),
            .string => |str| try allocator.dupe(u8, str.value),
        };
    }

    /// Clone the object
    pub fn clone(self: *const Object, allocator: std.mem.Allocator) !Object {
        return switch (self.*) {
            .integer => |int| Object{ .integer = int },
            .boolean => |bool_val| Object{ .boolean = bool_val },
            .null => Object{ .null = Null{} },
            .return_value => |_| Object{ .return_value = ReturnValue{ .value = try allocator.create(Object) } },
            .@"error" => |err| Object{ .@"error" = Error{ .message = try allocator.dupe(u8, err.message) } },
            .function => |func| {
                var params = std.ArrayList(ast_mod.Identifier).init(allocator);
                for (func.parameters.items) |param| {
                    try params.append(param);
                }
                return Object{
                    .function = Function{
                        .parameters = params,
                        .body = try allocator.create(ast_mod.BlockStatement),
                        .env = func.env, // TODO: proper environment cloning
                    },
                };
            },
            .string => |str| Object{ .string = StringObj{ .value = try allocator.dupe(u8, str.value) } },
        };
    }

    /// Deallocate the object and its contents
    pub fn deinit(self: *Object, allocator: std.mem.Allocator) void {
        switch (self.*) {
            .@"error" => |err| allocator.free(err.message),
            .function => |func| {
                func.parameters.deinit();
                // TODO: deinit body and env
            },
            .string => |str| allocator.free(str.value),
            .return_value => |ret| {
                ret.value.deinit(allocator);
                allocator.destroy(ret.value);
            },
            else => {},
        }
    }
};

/// Integer object
pub const Integer = struct {
    value: i64,
};

/// Boolean object
pub const Boolean = struct {
    value: bool,
};

/// Null object
pub const Null = struct {};

/// Return value wrapper
pub const ReturnValue = struct {
    value: *Object,
};

/// Error object
pub const Error = struct {
    message: []const u8,
};

/// Function object
pub const Function = struct {
    parameters: std.ArrayList(ast_mod.Identifier),
    body: *ast_mod.BlockStatement,
    env: ?*Environment, // Forward declaration needed
};

/// String object
pub const StringObj = struct {
    value: []const u8,
};

/// Forward declaration for Environment
pub const Environment = struct {
    store: std.StringHashMap(Object),
    outer: ?*Environment,

    pub fn init(allocator: std.mem.Allocator) Environment {
        return Environment{
            .store = std.StringHashMap(Object).init(allocator),
            .outer = null,
        };
    }

    pub fn initEnclosed(allocator: std.mem.Allocator, outer: *Environment) !Environment {
        return Environment{
            .store = std.StringHashMap(Object).init(allocator),
            .outer = outer,
        };
    }

    pub fn deinit(self: *Environment) void {
        self.store.deinit();
    }

    pub fn get(self: *Environment, name: []const u8) ?Object {
        if (self.store.get(name)) |obj| {
            return obj;
        }
        if (self.outer) |outer| {
            return outer.get(name);
        }
        return null;
    }

    pub fn set(self: *Environment, name: []const u8, value: Object) !void {
        try self.store.put(name, value);
    }
};

// Convenience functions for creating objects
pub fn makeInteger(value: i64) Object {
    return Object{ .integer = Integer{ .value = value } };
}

pub fn makeBoolean(value: bool) Object {
    return Object{ .boolean = Boolean{ .value = value } };
}

pub fn makeNull() Object {
    return Object{ .null = Null{} };
}

pub fn makeReturnValue(value: *Object) Object {
    return Object{ .return_value = ReturnValue{ .value = value } };
}

pub fn makeError(allocator: std.mem.Allocator, message: []const u8) !Object {
    return Object{ .@"error" = Error{ .message = try allocator.dupe(u8, message) } };
}

pub fn makeString(allocator: std.mem.Allocator, value: []const u8) !Object {
    return Object{ .string = StringObj{ .value = try allocator.dupe(u8, value) } };
}

pub fn makeFunction(
    allocator: std.mem.Allocator,
    parameters: std.ArrayList(ast_mod.Identifier),
    body: *ast_mod.BlockStatement,
    env: ?*Environment,
) Object {
    _ = allocator; // TODO: Use allocator for proper memory management
    return Object{ .function = Function{
        .parameters = parameters,
        .body = body,
        .env = env,
    } };
}

test "integer object" {
    const obj = makeInteger(42);
    try std.testing.expectEqual(ObjectType.integer, obj.objectType());
    try std.testing.expectEqual(@as(i64, 42), obj.integer.value);
}

test "boolean object" {
    const true_obj = makeBoolean(true);
    const false_obj = makeBoolean(false);

    try std.testing.expectEqual(ObjectType.boolean, true_obj.objectType());
    try std.testing.expectEqual(ObjectType.boolean, false_obj.objectType());
    try std.testing.expectEqual(true, true_obj.boolean.value);
    try std.testing.expectEqual(false, false_obj.boolean.value);
}

test "null object" {
    const obj = makeNull();
    try std.testing.expectEqual(ObjectType.null, obj.objectType());
}

test "string object" {
    const allocator = std.testing.allocator;
    var obj = try makeString(allocator, "hello");
    defer obj.deinit(allocator);

    try std.testing.expectEqual(ObjectType.string, obj.objectType());
    try std.testing.expectEqualStrings("hello", obj.string.value);
}

test "error object" {
    const allocator = std.testing.allocator;
    var obj = try makeError(allocator, "test error");
    defer obj.deinit(allocator);

    try std.testing.expectEqual(ObjectType.@"error", obj.objectType());
    try std.testing.expectEqualStrings("test error", obj.@"error".message);
}

test "inspect integer" {
    const allocator = std.testing.allocator;
    const obj = makeInteger(123);

    const str = try obj.inspect(allocator);
    defer allocator.free(str);

    try std.testing.expectEqualStrings("123", str);
}

test "inspect boolean" {
    const allocator = std.testing.allocator;

    const true_obj = makeBoolean(true);
    const false_obj = makeBoolean(false);

    const true_str = try true_obj.inspect(allocator);
    defer allocator.free(true_str);
    try std.testing.expectEqualStrings("true", true_str);

    const false_str = try false_obj.inspect(allocator);
    defer allocator.free(false_str);
    try std.testing.expectEqualStrings("false", false_str);
}

test "inspect null" {
    const allocator = std.testing.allocator;
    const obj = makeNull();

    const str = try obj.inspect(allocator);
    defer allocator.free(str);

    try std.testing.expectEqualStrings("null", str);
}

test "environment basic" {
    var env = Environment.init(std.testing.allocator);
    defer env.deinit();

    const obj = makeInteger(42);
    try env.set("x", obj);

    const retrieved = env.get("x");
    try std.testing.expect(retrieved != null);
    try std.testing.expectEqual(@as(i64, 42), retrieved.?.integer.value);
}

test "environment not found" {
    const allocator = std.testing.allocator;
    var env = Environment.init(allocator);
    defer env.deinit();

    const retrieved = env.get("nonexistent");
    try std.testing.expect(retrieved == null);
}
