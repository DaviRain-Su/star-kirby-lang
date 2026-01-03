const std = @import("std");
const repl_mod = @import("repl.zig");
const lexer_mod = @import("lexer.zig");

const VERSION = "0.7.0";

fn printBanner() void {
    std.debug.print(
        \\================================================================================
        \\                    Star Kirby Lang - Monkey Language in Zig
        \\================================================================================
        \\  Version: {s}
        \\  A complete implementation of the Monkey programming language
        \\
        \\  Features:
        \\    - Integers, Booleans, Strings, Arrays, Hash Maps
        \\    - First-class functions and closures
        \\    - Higher-order functions (map, filter, reduce)
        \\    - Control flow: if/else, while, for-in, break, continue
        \\    - Operators: +, -, *, /, %, <, >, <=, >=, ==, !=, &&, ||
        \\    - File I/O: readFile, writeFile, appendFile, fileExists
        \\
        \\  Built-in Functions:
        \\    len, first, last, rest, push, puts, println, print, type
        \\    str, int, keys, values, range, map, filter, reduce
        \\    split, join, trim, upper, lower, contains, replace
        \\    charAt, substring, indexOf, readFile, writeFile, appendFile, fileExists
        \\================================================================================
        \\
    , .{VERSION});
}

const HELP_TEXT =
    \\Usage:
    \\  zig build run                         Run interactive examples
    \\  zig build run -- "<code>"             Evaluate Monkey code
    \\  zig build run -- <file.monkey>        Execute a Monkey script file
    \\  zig build run -- --help               Show this help message
    \\  zig build run -- --examples           Show language examples
    \\  zig build run -- --version            Show version information
    \\
    \\Examples:
    \\  zig build run -- "1 + 2 * 3"
    \\  zig build run -- "let add = fn(a, b) { a + b }; add(2, 3)"
    \\  zig build run -- "map([1, 2, 3], fn(x) { x * 2 })"
    \\  zig build run -- script.monkey
    \\
;

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
        // Handle special commands
        if (std.mem.eql(u8, input, "--help") or std.mem.eql(u8, input, "-h")) {
            printBanner();
            std.debug.print("{s}", .{HELP_TEXT});
            return;
        }

        if (std.mem.eql(u8, input, "--version") or std.mem.eql(u8, input, "-v")) {
            std.debug.print("Star Kirby Lang v{s}\n", .{VERSION});
            std.debug.print("Monkey Language Interpreter in Zig\n", .{});
            return;
        }

        if (std.mem.eql(u8, input, "--examples") or std.mem.eql(u8, input, "-e")) {
            try showExamples(allocator, &repl);
            return;
        }

        // Check if input is a file path
        if (std.mem.endsWith(u8, input, ".monkey") or std.mem.endsWith(u8, input, ".mk")) {
            try executeScript(allocator, &repl, input);
            return;
        }

        // Evaluate the provided input as code
        std.debug.print("Input: '{s}' (len={})\n", .{ input, input.len });
        const result = try repl.eval(input);
        defer allocator.free(result);
        std.debug.print("{s}\n", .{result});
    } else {
        // No arguments - show banner and run interactive examples
        printBanner();
        try showInteractiveDemo(allocator, &repl);
    }
}

fn executeScript(allocator: std.mem.Allocator, repl: *repl_mod.REPL, path: []const u8) !void {
    std.debug.print("Executing script: {s}\n", .{path});

    // Read the script file
    const file = std.fs.cwd().openFile(path, .{}) catch |err| {
        std.debug.print("Error: Could not open file '{s}': {}\n", .{ path, err });
        return;
    };
    defer file.close();

    const content = file.readToEndAlloc(allocator, 10 * 1024 * 1024) catch |err| {
        std.debug.print("Error: Could not read file '{s}': {}\n", .{ path, err });
        return;
    };
    defer allocator.free(content);

    // Execute the script
    const result = repl.eval(content) catch |err| {
        std.debug.print("Error executing script: {}\n", .{err});
        return;
    };
    defer allocator.free(result);

    // Only print the result if it's not null
    if (!std.mem.eql(u8, result, "null")) {
        std.debug.print("{s}\n", .{result});
    }
}

fn showExamples(allocator: std.mem.Allocator, repl: *repl_mod.REPL) !void {
    printBanner();
    std.debug.print("\n=== Language Examples ===\n\n", .{});

    const categories = [_]struct {
        name: []const u8,
        examples: []const struct { code: []const u8, desc: []const u8 },
    }{
        .{
            .name = "Basic Types",
            .examples = &.{
                .{ .code = "42", .desc = "Integer" },
                .{ .code = "true", .desc = "Boolean" },
                .{ .code = "\"hello world\"", .desc = "String" },
                .{ .code = "[1, 2, 3, 4, 5]", .desc = "Array" },
                .{ .code = "{\"name\": \"Monkey\", \"version\": 1}", .desc = "Hash Map" },
            },
        },
        .{
            .name = "Arithmetic & Comparison",
            .examples = &.{
                .{ .code = "1 + 2 * 3", .desc = "Operator precedence" },
                .{ .code = "10 % 3", .desc = "Modulo" },
                .{ .code = "5 <= 5", .desc = "Less than or equal" },
                .{ .code = "10 >= 5", .desc = "Greater than or equal" },
                .{ .code = "true && false", .desc = "Logical AND" },
                .{ .code = "true || false", .desc = "Logical OR" },
            },
        },
        .{
            .name = "Variables & Functions",
            .examples = &.{
                .{ .code = "let x = 10; x * 2", .desc = "Variable binding" },
                .{ .code = "let add = fn(a, b) { a + b }; add(2, 3)", .desc = "Function definition" },
                .{ .code = "let factorial = fn(n) { if (n <= 1) { 1 } else { n * factorial(n - 1) } }; factorial(5)", .desc = "Recursive function" },
            },
        },
        .{
            .name = "Closures",
            .examples = &.{
                .{ .code = "let makeAdder = fn(x) { fn(y) { x + y } }; let add5 = makeAdder(5); add5(10)", .desc = "Closure" },
            },
        },
        .{
            .name = "Control Flow",
            .examples = &.{
                .{ .code = "if (10 > 5) { \"yes\" } else { \"no\" }", .desc = "If-else expression" },
                .{ .code = "let sum = 0; let i = 0; while (i < 5) { let sum = sum + i; let i = i + 1; }; sum", .desc = "While loop" },
                .{ .code = "let sum = 0; for (x in [1, 2, 3, 4, 5]) { let sum = sum + x; }; sum", .desc = "For-in loop" },
                .{ .code = "let i = 0; while (i < 10) { if (i == 5) { break; }; let i = i + 1; }; i", .desc = "Break statement" },
            },
        },
        .{
            .name = "Built-in Functions",
            .examples = &.{
                .{ .code = "len(\"hello\")", .desc = "String length" },
                .{ .code = "len([1, 2, 3])", .desc = "Array length" },
                .{ .code = "first([1, 2, 3])", .desc = "First element" },
                .{ .code = "last([1, 2, 3])", .desc = "Last element" },
                .{ .code = "rest([1, 2, 3])", .desc = "Rest of array" },
                .{ .code = "push([1, 2], 3)", .desc = "Push element" },
                .{ .code = "str(42)", .desc = "Convert to string" },
                .{ .code = "int(\"123\")", .desc = "Parse integer" },
                .{ .code = "type(42)", .desc = "Get type" },
            },
        },
        .{
            .name = "Range Function",
            .examples = &.{
                .{ .code = "let sum = 0; for (i in range(5)) { let sum = sum + i; }; sum", .desc = "range(n)" },
                .{ .code = "let sum = 0; for (i in range(1, 6)) { let sum = sum + i; }; sum", .desc = "range(start, end)" },
                .{ .code = "let sum = 0; for (i in range(0, 10, 2)) { let sum = sum + i; }; sum", .desc = "range(start, end, step)" },
            },
        },
        .{
            .name = "Functional Programming",
            .examples = &.{
                .{ .code = "let doubled = map([1, 2, 3], fn(x) { x * 2 }); first(doubled)", .desc = "Map" },
                .{ .code = "let evens = filter([1, 2, 3, 4, 5], fn(x) { x % 2 == 0 }); len(evens)", .desc = "Filter" },
                .{ .code = "reduce([1, 2, 3, 4, 5], fn(acc, x) { acc + x }, 0)", .desc = "Reduce" },
            },
        },
        .{
            .name = "Hash Map Operations",
            .examples = &.{
                .{ .code = "let h = {\"a\": 1, \"b\": 2}; h[\"a\"]", .desc = "Hash access" },
                .{ .code = "let h = {\"a\": 1, \"b\": 2}; keys(h)", .desc = "Get keys" },
                .{ .code = "let h = {\"a\": 1, \"b\": 2}; values(h)", .desc = "Get values" },
            },
        },
        .{
            .name = "Array Operations",
            .examples = &.{
                .{ .code = "let arr = [1, 2, 3]; arr[0] = 10; arr[0]", .desc = "Index assignment" },
                .{ .code = "[1, 2, 3][1]", .desc = "Index access" },
            },
        },
    };

    for (categories) |category| {
        std.debug.print("--- {s} ---\n\n", .{category.name});
        for (category.examples) |example| {
            std.debug.print("// {s}\n", .{example.desc});
            std.debug.print(">> {s}\n", .{example.code});
            const result = repl.eval(example.code) catch |err| {
                std.debug.print("Error: {}\n\n", .{err});
                continue;
            };
            defer allocator.free(result);
            std.debug.print("{s}\n\n", .{result});
        }
    }
}

fn showInteractiveDemo(allocator: std.mem.Allocator, repl: *repl_mod.REPL) !void {
    std.debug.print("Running quick demo...\n\n", .{});

    const quick_examples = [_]struct { code: []const u8, desc: []const u8 }{
        .{ .code = "42", .desc = "Integer literal" },
        .{ .code = "1 + 2 * 3", .desc = "Arithmetic with precedence" },
        .{ .code = "\"hello\" + \" world\"", .desc = "String concatenation" },
        .{ .code = "[1, 2, 3]", .desc = "Array literal" },
        .{ .code = "let add = fn(a, b) { a + b }; add(2, 3)", .desc = "Function definition and call" },
        .{ .code = "if (10 > 5) { \"yes\" } else { \"no\" }", .desc = "Conditional expression" },
        .{ .code = "let sum = 0; for (x in [1, 2, 3]) { let sum = sum + x; }; sum", .desc = "For loop" },
        .{ .code = "reduce([1, 2, 3, 4, 5], fn(acc, x) { acc + x }, 0)", .desc = "Reduce function" },
    };

    for (quick_examples) |example| {
        std.debug.print("// {s}\n", .{example.desc});
        std.debug.print(">> {s}\n", .{example.code});
        const result = try repl.eval(example.code);
        defer allocator.free(result);
        std.debug.print("{s}\n\n", .{result});
    }

    std.debug.print(
        \\================================================================================
        \\To run your own code:
        \\  zig build run -- "your_code_here"
        \\
        \\For more examples:
        \\  zig build run -- --examples
        \\
        \\For help:
        \\  zig build run -- --help
        \\================================================================================
        \\
    , .{});
}
