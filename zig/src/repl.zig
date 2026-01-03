const std = @import("std");
const lexer_mod = @import("lexer.zig");
const parser_mod = @import("parser.zig");
const Parser = parser_mod.Parser;
const ast_mod = @import("ast.zig");
const evaluator_mod = @import("evaluator.zig");
const object_mod = @import("object.zig");

pub const EvalOptions = struct {
    debug: bool = false,
    print_ast: bool = false,
    print_tokens: bool = false,
};

/// Simple AST printing for debugging
fn printProgram(program: ast_mod.Program) void {
    std.debug.print("Program({d} statements):\n", .{program.statements.len});
    for (program.statements) |stmt| {
        switch (stmt) {
            .let => |ls| std.debug.print("  LetStatement: {s} = ...\n", .{ls.name.value}),
            .return_stmt => std.debug.print("  ReturnStatement\n", .{}),
            .expression => std.debug.print("  ExpressionStatement\n", .{}),
            .block => std.debug.print("  BlockStatement\n", .{}),
            else => std.debug.print("  Other statement\n", .{}),
        }
    }
}

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
        return self.evalWithOptions(input, .{});
    }

    /// Evaluate input with options and return result as string
    pub fn evalWithOptions(self: *REPL, input: []const u8, options: EvalOptions) anyerror![]u8 {
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

        // Print tokens if requested
        if (options.print_tokens) {
            std.debug.print("Tokens:\n", .{});
            for (tokens) |token| {
                std.debug.print("  {f}\n", .{token});
            }
            std.debug.print("\n", .{});
        }

        // Parse
        var parser = parser_mod.Parser.init(arena_allocator, tokens);
        const program = try parser.parseProgram();

        // Print AST if requested
        if (options.print_ast) {
            std.debug.print("AST:\n", .{});
            printProgram(program);
            std.debug.print("\n", .{});
        }

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

    // Test function call - user-defined functions are tested above with identity function
}

// =============================================================================
// Memory Leak Tests
// =============================================================================
// These tests use std.testing.allocator which automatically detects memory leaks.
// If any test leaks memory, the test will fail with "memory leak detected".

test "memory: complex expression evaluation" {
    const allocator = std.testing.allocator;
    var repl = REPL.init(allocator);
    defer repl.deinit();

    // Complex nested expressions
    const result = try repl.eval("1 + 2 * 3 + 4 / 2");
    defer allocator.free(result);
    try std.testing.expectEqualStrings("9", result);
}

test "memory: recursive function" {
    const allocator = std.testing.allocator;
    var repl = REPL.init(allocator);
    defer repl.deinit();

    // Define and call a recursive factorial function
    const result = try repl.eval(
        \\let factorial = fn(n) {
        \\  if (n < 2) {
        \\    return 1;
        \\  } else {
        \\    return n * factorial(n - 1);
        \\  }
        \\};
        \\factorial(5)
    );
    defer allocator.free(result);
    try std.testing.expectEqualStrings("120", result);
}

test "memory: hash literal creation and access" {
    const allocator = std.testing.allocator;
    var repl = REPL.init(allocator);
    defer repl.deinit();

    // Create hash and access it
    const result = try repl.eval(
        \\let h = {"one": 1, "two": 2, "three": 3};
        \\h["two"]
    );
    defer allocator.free(result);
    try std.testing.expectEqualStrings("2", result);
}

test "memory: array with nested operations" {
    const allocator = std.testing.allocator;
    var repl = REPL.init(allocator);
    defer repl.deinit();

    // Array with nested expressions
    const result = try repl.eval(
        \\let arr = [1 + 1, 2 * 2, 3 + 3];
        \\arr[0] + arr[1] + arr[2]
    );
    defer allocator.free(result);
    try std.testing.expectEqualStrings("12", result);
}

test "memory: multiple function calls" {
    const allocator = std.testing.allocator;
    var repl = REPL.init(allocator);
    defer repl.deinit();

    // Define and call multiple functions
    const result = try repl.eval(
        \\let add = fn(a, b) { a + b };
        \\let mul = fn(a, b) { a * b };
        \\mul(add(1, 2), add(3, 4))
    );
    defer allocator.free(result);
    try std.testing.expectEqualStrings("21", result);
}

test "memory: closure" {
    const allocator = std.testing.allocator;
    var repl = REPL.init(allocator);
    defer repl.deinit();

    // Closure that captures outer variable
    const result = try repl.eval(
        \\let makeAdder = fn(x) {
        \\  fn(y) { x + y }
        \\};
        \\let addFive = makeAdder(5);
        \\addFive(10)
    );
    defer allocator.free(result);
    try std.testing.expectEqualStrings("15", result);
}

test "memory: string operations" {
    const allocator = std.testing.allocator;
    var repl = REPL.init(allocator);
    defer repl.deinit();

    // String concatenation
    const result = try repl.eval(
        \\let first = "Hello";
        \\let second = "World";
        \\first + " " + second
    );
    defer allocator.free(result);
    try std.testing.expectEqualStrings("Hello World", result);
}

test "memory: builtin functions" {
    const allocator = std.testing.allocator;
    var repl = REPL.init(allocator);
    defer repl.deinit();

    // Test len builtin
    {
        const result = try repl.eval("len(\"hello\")");
        defer allocator.free(result);
        try std.testing.expectEqualStrings("5", result);
    }

    // Test first builtin
    {
        const result = try repl.eval("first([1, 2, 3])");
        defer allocator.free(result);
        try std.testing.expectEqualStrings("1", result);
    }

    // Test last builtin
    {
        const result = try repl.eval("last([1, 2, 3])");
        defer allocator.free(result);
        try std.testing.expectEqualStrings("3", result);
    }
}

test "memory: if-else chains" {
    const allocator = std.testing.allocator;
    var repl = REPL.init(allocator);
    defer repl.deinit();

    // Nested if-else
    const result = try repl.eval(
        \\let x = 10;
        \\if (x > 15) {
        \\  "big"
        \\} else {
        \\  if (x > 5) {
        \\    "medium"
        \\  } else {
        \\    "small"
        \\  }
        \\}
    );
    defer allocator.free(result);
    try std.testing.expectEqualStrings("medium", result);
}

test "memory: repeated evaluations" {
    const allocator = std.testing.allocator;
    var repl = REPL.init(allocator);
    defer repl.deinit();

    // Run many evaluations in sequence to test for cumulative leaks
    var i: usize = 0;
    while (i < 100) : (i += 1) {
        const result = try repl.eval("1 + 2 + 3");
        defer allocator.free(result);
        try std.testing.expectEqualStrings("6", result);
    }
}

// =============================================================================
// Index Assignment Tests
// =============================================================================

test "index assignment: array element" {
    const allocator = std.testing.allocator;
    var repl = REPL.init(allocator);
    defer repl.deinit();

    // Create array and modify element
    const result = try repl.eval(
        \\let arr = [1, 2, 3];
        \\arr[0] = 10;
        \\arr[0]
    );
    defer allocator.free(result);
    try std.testing.expectEqualStrings("10", result);
}

test "index assignment: hash key" {
    const allocator = std.testing.allocator;
    var repl = REPL.init(allocator);
    defer repl.deinit();

    // Create hash and modify/add key
    const result = try repl.eval(
        \\let h = {"a": 1};
        \\h["b"] = 2;
        \\h["b"]
    );
    defer allocator.free(result);
    try std.testing.expectEqualStrings("2", result);
}

test "index assignment: update existing hash key" {
    const allocator = std.testing.allocator;
    var repl = REPL.init(allocator);
    defer repl.deinit();

    // Update existing hash key
    const result = try repl.eval(
        \\let h = {"a": 1};
        \\h["a"] = 100;
        \\h["a"]
    );
    defer allocator.free(result);
    try std.testing.expectEqualStrings("100", result);
}

test "index assignment: with expression value" {
    const allocator = std.testing.allocator;
    var repl = REPL.init(allocator);
    defer repl.deinit();

    // Assign expression result
    const result = try repl.eval(
        \\let arr = [1, 2, 3];
        \\arr[1] = 10 + 20;
        \\arr[1]
    );
    defer allocator.free(result);
    try std.testing.expectEqualStrings("30", result);
}

test "index assignment: multiple assignments" {
    const allocator = std.testing.allocator;
    var repl = REPL.init(allocator);
    defer repl.deinit();

    // Multiple assignments
    const result = try repl.eval(
        \\let arr = [0, 0, 0];
        \\arr[0] = 1;
        \\arr[1] = 2;
        \\arr[2] = 3;
        \\arr[0] + arr[1] + arr[2]
    );
    defer allocator.free(result);
    try std.testing.expectEqualStrings("6", result);
}
