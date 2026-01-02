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
        // Tokenize
        var lexer = lexer_mod.Lexer.init(input);

        var tokens_list = try std.ArrayList(lexer_mod.Token).initCapacity(self.allocator, 16);
        defer tokens_list.deinit(self.allocator);

        while (true) {
            const tok = lexer.nextToken();
            try tokens_list.append(self.allocator, tok);
            if (tok.token_type == .EOF) break;
        }

        const tokens = try tokens_list.toOwnedSlice(self.allocator);
        defer self.allocator.free(tokens);

        // Parse
        var parser = parser_mod.Parser.init(self.allocator, tokens);
        var program = try parser.parseProgram();

        // Evaluate
        const eval_result = evaluator_mod.evalProgram(program, &self.env);

        // Clean up program
        program.deinit(self.allocator);

        // Handle result
        if (eval_result.isErr()) {
            return std.fmt.allocPrint(self.allocator, "ERROR: {}", .{eval_result.unwrapErr()});
        }

        const result = eval_result.unwrap();
        // Return result as string
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
}
