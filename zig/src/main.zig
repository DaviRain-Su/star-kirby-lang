const std = @import("std");
const repl_mod = @import("repl.zig");
const lexer_mod = @import("lexer.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var repl = repl_mod.REPL.init(allocator);
    defer repl.deinit();

    // Check command line arguments
    var args = try std.process.argsWithAllocator(allocator);
    defer args.deinit();

    // Skip program name
    _ = args.skip();

    if (args.next()) |input| {
        // Evaluate the provided input
        std.debug.print("Input: '{s}' (len={})\n", .{ input, input.len });
        const result = try repl.eval(input);
        defer allocator.free(result);
        std.debug.print("{s}\n", .{result});
    } else {
        // Test parsing directly
        const test_input = "1 + 2";
        std.debug.print("Testing parsing of: {s}\n", .{test_input});

        // Tokenize
        var lexer = lexer_mod.Lexer.init(test_input);
        var tokens_list = try std.ArrayList(lexer_mod.Token).initCapacity(allocator, 16);
        defer tokens_list.deinit(allocator);

        while (true) {
            const tok = lexer.nextToken();
            // std.debug.print("Token: {s} '{s}'\n", .{@tagName(tok.token_type), tok.literal});
            try tokens_list.append(allocator, tok);
            if (tok.token_type == .EOF) break;
        }

        const tokens = try tokens_list.toOwnedSlice(allocator);
        defer allocator.free(tokens);

        std.debug.print("Tokens:\n", .{});
        for (tokens) |token| {
            std.debug.print("  {s}: '{s}'\n", .{ @tagName(token.token_type), token.literal });
        }

        std.debug.print("\nRunning examples...\n", .{});

        // Run interactive REPL
        std.debug.print("Hello! This is the Monkey programming language in Zig!\n", .{});
        std.debug.print("Feel free to type in commands\n", .{});
        std.debug.print("Type 'exit' or 'quit' to exit\n", .{});

        // For now, just show some examples since interactive input is complex in this environment
        const examples = [_][]const u8{
            "42",
            "1 + 2",
            "1 + 2 * 3",
            "\"hello\" + \" world\"",
            "[1, 2, 3]",
            "let arr = [1, 2, 3]; arr[1]",
            "let add = fn(a, b) { return a + b; }; add(2, 3)",
            "if (true) { 42 } else { 24 }",
        };

        for (examples) |example| {
            std.debug.print(">> {s}\n", .{example});
            const result = try repl.eval(example);
            defer allocator.free(result);
            std.debug.print("{s}\n\n", .{result});
        }

        std.debug.print("To run your own code: zig build run -- your_code_here\n", .{});
        std.debug.print("Example: zig build run -- \"1 + 2\"\n", .{});
    }
}
