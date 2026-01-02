const std = @import("std");
const lexer_mod = @import("lexer.zig");
const parser_mod = @import("parser.zig");
const evaluator_mod = @import("evaluator.zig");
const object_mod = @import("object.zig");

/// REPL (Read-Eval-Print Loop) for Monkey language
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

    /// Start the REPL
    pub fn start(self: *REPL) !void {
        const stdin = std.io.getStdIn().reader();
        const stdout = std.io.getStdOut().writer();

        try stdout.print("Hello! This is the Monkey programming language!\n", .{});
        try stdout.print("Feel free to type in commands\n", .{});

        var buffer: [1024]u8 = undefined;

        while (true) {
            try stdout.print(">> ", .{});

            const input = try stdin.readUntilDelimiterOrEof(&buffer, '\n');
            if (input) |line| {
                if (std.mem.eql(u8, line, "exit") or std.mem.eql(u8, line, "quit")) {
                    break;
                }

                self.evalAndPrint(line, stdout) catch |err| {
                    stdout.print("Error: {}\n", .{err}) catch {};
                };
            } else {
                break;
            }
        }
    }

    /// Evaluate input and print result
    pub fn evalAndPrint(self: *REPL, input: []const u8, writer: anytype) !void {
        // Tokenize
        var lexer = lexer_mod.Lexer.init(input);

        var tokens_list = std.ArrayList(lexer_mod.Token).init(self.allocator);
        defer tokens_list.deinit();

        while (true) {
            const tok = lexer.nextToken();
            try tokens_list.append(tok);
            if (tok.token_type == .EOF) break;
        }

        const tokens = try tokens_list.toOwnedSlice();
        defer self.allocator.free(tokens);

        // Parse
        var parser = parser_mod.Parser.init(self.allocator, tokens);
        const program = try parser.parseProgram();
        defer program.deinit();

        // Evaluate
        const result = try evaluator_mod.evalProgram(self.allocator, program, &self.env);

        // Print result
        const output = try result.inspect(self.allocator);
        defer self.allocator.free(output);

        try writer.print("{s}\n", .{output});
    }
};

/// Convenience function to start REPL
pub fn startRepl(allocator: std.mem.Allocator) !void {
    var repl = REPL.init(allocator);
    defer repl.deinit();

    try repl.start();
}

test "repl evaluation" {
    const allocator = std.testing.allocator;
    var repl = REPL.init(allocator);
    defer repl.deinit();

    // Test integer literal
    var output_buf = std.ArrayList(u8).init(allocator);
    defer output_buf.deinit();

    const writer = output_buf.writer();

    try repl.evalAndPrint("42", writer);
    try std.testing.expect(std.mem.indexOf(u8, output_buf.items, "42") != null);

    // Test boolean literal
    output_buf.clearRetainingCapacity();
    try repl.evalAndPrint("true", writer);
    try std.testing.expect(std.mem.indexOf(u8, output_buf.items, "true") != null);

    // Test let statement
    output_buf.clearRetainingCapacity();
    try repl.evalAndPrint("let x = 5; x", writer);
    try std.testing.expect(std.mem.indexOf(u8, output_buf.items, "5") != null);
}
