const std = @import("std");
const repl_mod = @import("repl.zig");
const lexer_mod = @import("lexer.zig");
const builtins_mod = @import("builtins.zig");
const cache_mod = @import("cache.zig");

const VERSION = "0.11.0";

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
        \\    - Higher-order functions (map, filter, reduce, find, some, every)
        \\    - Control flow: if/else, while, for-in, break, continue
        \\    - Operators: +, -, *, /, %, <, >, <=, >=, ==, !=, &&, ||
        \\    - File I/O: readFile, writeFile, appendFile, fileExists
        \\    - Math: abs, min, max, pow, sqrt, sum, sign, clamp, gcd, lcm, avg, product
        \\    - Random: rand, shuffle
        \\    - System: getenv, time, sleep
        \\
        \\  Built-in Functions (70+):
        \\    Core: len, first, last, rest, push, puts, println, print, type, typeof
        \\    Convert: str, int, bool, array
        \\    Hash: keys, values
        \\    Functional: range, map, filter, reduce, find, some, every
        \\    Array: reverse, sort, slice, concat, flatten, shuffle
        \\    String: split, join, trim, upper, lower, contains, replace
        \\            charAt, substring, indexOf, startsWith, endsWith
        \\            repeat, padLeft, padRight
        \\    Math: abs, min, max, pow, sqrt, sum, sign, clamp, gcd, lcm, avg, product
        \\    Random: rand, shuffle
        \\    Type: isInt, isStr, isBool, isArray, isHash, isFunc, isNull
        \\    Utility: assert, default, args, import
        \\    File: readFile, writeFile, appendFile, fileExists
        \\    System: getenv, time, sleep, args
        \\    Module: import
        \\================================================================================
        \\
    , .{VERSION});
}

const HELP_TEXT =
    \\Usage:
    \\  zig build run                         Run interactive examples
    \\  zig build run -- "<code>"             Evaluate Monkey code
    \\  zig build run -- file.monkey          Execute Monkey script
    \\
    \\Options:
    \\  --debug                               Enable debug output
    \\  --ast                                 Print AST after parsing
    \\  --tokens                              Print tokens after lexing
    \\  --help, -h                            Show this help
    \\  --version, -v                         Show version
    \\  --examples, -e                        Show examples
    \\  --repl, -r                            Start interactive REPL
    \\  zig build run -- <file.monkey>        Execute a Monkey script file
    \\  zig build run -- --repl               Start interactive REPL
    \\  zig build run -- --help               Show this help message
    \\  zig build run -- --examples           Show language examples
    \\  zig build run -- --version            Show version information
    \\
    \\Examples:
    \\  zig build run -- "1 + 2 * 3"
    \\  zig build run -- "let add = fn(a, b) { a + b }; add(2, 3)"
    \\  zig build run -- "map([1, 2, 3], fn(x) { x * 2 })"
    \\  zig build run -- "sort([3, 1, 4, 1, 5])"
    \\  zig build run -- "sum(range(1, 101))"
    \\  zig build run -- script.monkey
    \\
;

pub fn main() void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Initialize cache system
    var cache_opt: ?cache_mod.BytecodeCache = blk: {
        break :blk cache_mod.BytecodeCache.init(allocator, ".star-cache", true) catch |err| {
            std.debug.print("Warning: Failed to initialize cache: {}\n", .{err});
            break :blk null;
        };
    };
    defer if (cache_opt) |*c| c.deinit();

    var repl = repl_mod.REPL.init(allocator);
    defer repl.deinit();

    var args = std.process.argsWithAllocator(allocator) catch |err| {
        std.debug.print("Error: Failed to parse command line arguments: {}\n", .{err});
        std.process.exit(1);
    };
    defer args.deinit();
    _ = args.skip();

    // Parse options and input
    var options = repl_mod.EvalOptions{};
    var input: ?[]const u8 = null;
    var script_args = std.ArrayList([]const u8).initCapacity(allocator, 16) catch |err| {
        std.debug.print("Error: Failed to allocate memory: {}\n", .{err});
        std.process.exit(1);
    };
    defer script_args.deinit(allocator);

    while (args.next()) |arg| {
        if (std.mem.eql(u8, arg, "--debug")) {
            options.debug = true;
        } else if (std.mem.eql(u8, arg, "--ast")) {
            options.print_ast = true;
        } else if (std.mem.eql(u8, arg, "--tokens")) {
            options.print_tokens = true;
        } else if (std.mem.eql(u8, arg, "--help") or std.mem.eql(u8, arg, "-h")) {
            printBanner();
            std.debug.print("{s}", .{HELP_TEXT});
            return;
        } else if (std.mem.eql(u8, arg, "--version") or std.mem.eql(u8, arg, "-v")) {
            std.debug.print("Star Kirby Lang v{s}\n", .{VERSION});
            std.debug.print("Monkey Language Interpreter in Zig\n", .{});
            return;
        } else if (std.mem.eql(u8, arg, "--examples") or std.mem.eql(u8, arg, "-e")) {
            showExamples(allocator, &repl) catch |err| {
                std.debug.print("Error: Failed to show examples: {}\n", .{err});
                std.process.exit(1);
            };
            return;
        } else if (std.mem.eql(u8, arg, "--repl") or std.mem.eql(u8, arg, "-r")) {
            runInteractiveRepl(allocator, &repl) catch |err| {
                std.debug.print("Error: Failed to start REPL: {}\n", .{err});
                std.process.exit(1);
            };
            return;
        } else if (input == null) {
            input = arg;
        } else {
            // Additional arguments are treated as script arguments
            script_args.append(allocator, arg) catch |err| {
                std.debug.print("Error: Failed to add argument: {}\n", .{err});
                std.process.exit(1);
            };
        }
    }

    if (input) |inp| {
        // Check if input is a file path
        if (std.mem.endsWith(u8, inp, ".monkey") or std.mem.endsWith(u8, inp, ".mk")) {
            // For script files, we need to read the file content and evaluate it with options
            const file = std.fs.cwd().openFile(inp, .{}) catch |err| {
                std.debug.print("Error: Could not open file '{s}': {}\n", .{ inp, err });
                std.process.exit(1);
            };
            defer file.close();

            const file_size = file.getEndPos() catch |err| {
                std.debug.print("Error: Could not get file size for '{s}': {}\n", .{ inp, err });
                std.process.exit(1);
            };
            const buffer = allocator.alloc(u8, file_size) catch |err| {
                std.debug.print("Error: Failed to allocate memory for file '{s}': {}\n", .{ inp, err });
                std.process.exit(1);
            };
            defer allocator.free(buffer);

            _ = file.readAll(buffer) catch |err| {
                std.debug.print("Error: Could not read file '{s}': {}\n", .{ inp, err });
                std.process.exit(1);
            };

            // Prepare script arguments
            var script_args_list = std.ArrayList([]const u8).initCapacity(allocator, script_args.items.len + 1) catch |err| {
                std.debug.print("Error: Failed to allocate memory for script arguments: {}\n", .{err});
                std.process.exit(1);
            };
            defer script_args_list.deinit(allocator);

            // First arg is the script name
            script_args_list.append(allocator, inp) catch |err| {
                std.debug.print("Error: Failed to add script name to arguments: {}\n", .{err});
                std.process.exit(1);
            };

            // Add remaining args
            for (script_args.items) |arg| {
                script_args_list.append(allocator, arg) catch |err| {
                    std.debug.print("Error: Failed to add argument to script arguments: {}\n", .{err});
                    std.process.exit(1);
                };
            }

            // Set script args in builtins module
            builtins_mod.setScriptArgs(allocator, script_args_list.items) catch |err| {
                std.debug.print("Error: Failed to set script arguments: {}\n", .{err});
                std.process.exit(1);
            };
            defer builtins_mod.clearScriptArgs();

            std.debug.print("Executing script: {s}\n", .{inp});

            // Check cache
            const file_stat = file.stat() catch |err| {
                std.debug.print("Error: Could not get file stats: {}\n", .{err});
                std.process.exit(1);
            };

            const cache_hit = if (cache_opt) |*cache| blk: {
                break :blk cache.isValid(inp, buffer, file_stat.mtime) catch false;
            } else false;

            if (cache_hit) {
                std.debug.print("Using cached bytecode\n", .{});
            }

            const result = repl.evalWithOptions(buffer, options) catch {
                // Error messages are already printed by the parser/evaluator
                std.process.exit(1);
            };
            defer allocator.free(result);

            // Mark cache as valid after successful execution
            if (cache_opt) |*cache| {
                cache.markValid(inp, buffer, file_stat.mtime) catch |err| {
                    std.debug.print("Warning: Failed to update cache: {}\n", .{err});
                };
            }

            std.debug.print("{s}\n", .{result});
            return;
        }

        // Evaluate the provided input as code
        std.debug.print("Input: '{s}' (len={})\n", .{ inp, inp.len });
        const result = repl.evalWithOptions(inp, options) catch {
            // Error messages are already printed by the parser/evaluator
            std.process.exit(1);
        };
        defer allocator.free(result);
        std.debug.print("{s}\n", .{result});
    } else {
        // No arguments - show banner and run interactive examples
        printBanner();
        showInteractiveDemo(allocator, &repl) catch |err| {
            std.debug.print("Error: Failed to show interactive demo: {}\n", .{err});
            std.process.exit(1);
        };
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

fn runInteractiveRepl(allocator: std.mem.Allocator, repl: *repl_mod.REPL) !void {
    std.debug.print(
        \\================================================================================
        \\              Star Kirby Lang Interactive REPL v0.10.0
        \\================================================================================
        \\  Type Monkey code and press Enter to evaluate.
        \\  Type 'exit' or 'quit' to exit. Press Ctrl+D to exit.
        \\
        \\  Examples:
        \\    1 + 2 * 3
        \\    let x = 10; x * 2
        \\    map([1, 2, 3], fn(x) {{ x * 2 }})
        \\================================================================================
        \\
    , .{});

    var line_buf: [4096]u8 = undefined;

    // Use posix stdin for reading
    const stdin_file = std.posix.STDIN_FILENO;

    while (true) {
        std.debug.print(">> ", .{});

        // Read line from stdin using posix
        var len: usize = 0;
        while (len < line_buf.len - 1) {
            var buf: [1]u8 = undefined;
            const n = std.posix.read(stdin_file, &buf) catch break;
            if (n == 0) {
                // EOF
                if (len == 0) {
                    std.debug.print("\nGoodbye!\n", .{});
                    return;
                }
                break;
            }
            if (buf[0] == '\n') break;
            line_buf[len] = buf[0];
            len += 1;
        }

        const input = std.mem.trim(u8, line_buf[0..len], " \t\r");
        if (input.len == 0) continue;

        if (std.mem.eql(u8, input, "exit") or std.mem.eql(u8, input, "quit")) {
            break;
        }

        const result = repl.eval(input) catch |err| {
            std.debug.print("Error: {}\n", .{err});
            continue;
        };
        defer allocator.free(result);

        std.debug.print("{s}\n", .{result});
    }

    std.debug.print("\nGoodbye!\n", .{});
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
