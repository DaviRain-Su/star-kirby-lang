const std = @import("std");
const repl_mod = @import("repl.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var repl = repl_mod.REPL.init(allocator);
    defer repl.deinit();

    std.debug.print("Hello! This is the Monkey programming language in Zig!\n", .{});

    // Test some expressions
    const test_cases = [_][]const u8{
        "42",
        "true",
        "let x = 5; x",
    };

    for (test_cases) |test_case| {
        std.debug.print(">> {s}\n", .{test_case});
        const result = try repl.eval(test_case);
        defer allocator.free(result);
        std.debug.print("{s}\n\n", .{result});
    }
}

test "simple test" {
    const gpa = std.testing.allocator;
    var list: std.ArrayList(i32) = .empty;
    defer list.deinit(gpa); // Try commenting this out and see if zig detects the memory leak!
    try list.append(gpa, 42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}

test "fuzz example" {
    const Context = struct {
        fn testOne(context: @This(), input: []const u8) anyerror!void {
            _ = context;
            // Try passing `--fuzz` to `zig build test` and see if it manages to fail this test case!
            try std.testing.expect(!std.mem.eql(u8, "canyoufindme", input));
        }
    };
    try std.testing.fuzz(Context{}, Context.testOne, .{});
}
