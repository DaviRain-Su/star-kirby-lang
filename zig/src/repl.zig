const std = @import("std");
const lexer_mod = @import("lexer.zig");
const parser_mod = @import("parser.zig");
const Parser = parser_mod.Parser;
const evaluator_mod = @import("evaluator.zig");
const object_mod = @import("object.zig");

/// Simple REPL for Monkey language
pub const REPL = struct {
    allocator: std.mem.Allocator,
    env: object_mod.Environment,

    pub fn init(allocator: std.mem.Allocator) REPL {
        return REPL{
            .allocator = allocator,
            .env = object_mod.Environment.init(allocator),
        };
    }

    pub fn deinit(self: *REPL) void {
        self.env.deinit();
    }

    /// Evaluate input and return result as string
    pub fn eval(self: *REPL, input: []const u8) ![]u8 {
        // Use arena allocator for temporary allocations during parsing/evaluation
        var arena = std.heap.ArenaAllocator.init(self.allocator);
        defer arena.deinit();
        const arena_allocator = arena.allocator();

        // Tokenize
        var lexer = lexer_mod.Lexer.init(input);

        var tokens_list = try std.ArrayList(lexer_mod.Token).initCapacity(arena_allocator, 16);

        while (true) {
            const tok = lexer.nextToken();
            try tokens_list.append(arena_allocator, tok);
            if (tok.token_type == .EOF) break;
        }

        const tokens = try tokens_list.toOwnedSlice(arena_allocator);

        // Parse
        var parser = parser_mod.Parser.init(arena_allocator, tokens);
        const program = try parser.parseProgram();

        // Evaluate
        const eval_result = evaluator_mod.evalProgram(arena_allocator, program, &self.env);

        // Handle result - use main allocator for result string so it persists
        if (eval_result.isErr()) {
            return std.fmt.allocPrint(self.allocator, "ERROR: {}", .{eval_result.unwrapErr()});
        }

        const result = eval_result.unwrap();
        // Return result as string using main allocator
        return result.inspect(self.allocator);
    }
};

test "repl evaluation" {
    const allocator = std.testing.allocator;
    var repl = REPL.init(allocator);
    defer repl.deinit();

    // Test integer literal
    const result1 = try repl.eval("42");
    defer allocator.free(result1);
    try std.testing.expectEqualStrings("42", result1);

    // Test boolean literal
    const result2 = try repl.eval("true");
    defer allocator.free(result2);
    try std.testing.expectEqualStrings("true", result2);

    // Test let statement
    const result3 = try repl.eval("let x = 5; x");
    defer allocator.free(result3);
    try std.testing.expectEqualStrings("5", result3);

    // Test function literal
    const result4 = try repl.eval("fn(x) { x }");
    defer allocator.free(result4);
    // Function objects don't have a string representation yet, just check it doesn't error

    // Test function call with return statement
    const result6 = try repl.eval("let identity = fn(x) { return x; }; identity(42)");
    defer allocator.free(result6);
    try std.testing.expectEqualStrings("42", result6);

    // Test string literals and operations
    const result7 = try repl.eval("\"hello\" + \" \" + \"world\"");
    defer allocator.free(result7);
    try std.testing.expectEqualStrings("hello world", result7);

    // Test string comparison
    const result8 = try repl.eval("\"hello\" == \"hello\"");
    defer allocator.free(result8);
    try std.testing.expectEqualStrings("true", result8);

    // Test array literals
    const result9 = try repl.eval("[1, 2, 3]");
    defer allocator.free(result9);
    // Array should be created

    // Test array indexing
    const result10 = try repl.eval("let arr = [1, 2, 3]; arr[1]");
    defer allocator.free(result10);
    try std.testing.expectEqualStrings("2", result10);

    // Test if expression
    const result5 = try repl.eval("if (true) { 10 } else { 20 }");
    defer allocator.free(result5);
    try std.testing.expectEqualStrings("10", result5);

    // Test function call (using built-in functions for now)
    // TODO: Add proper function call tests when we implement user-defined functions
}
