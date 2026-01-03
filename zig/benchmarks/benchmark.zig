const std = @import("std");
const zig = @import("zig");
const lexer_mod = zig.lexer;
const parser_mod = zig.parser;
const evaluator_mod = zig.evaluator;
const object_mod = zig.object;
const builtins_mod = zig.builtins;

/// Benchmark result for a single test
pub const BenchmarkResult = struct {
    name: []const u8,
    iterations: u64,
    total_time_ns: u64,
    avg_time_ns: u64,
    min_time_ns: u64,
    max_time_ns: u64,

    pub fn print(self: BenchmarkResult) void {
        std.debug.print(
            \\{s}:
            \\  iterations: {d}
            \\  total time: {d:.3} ms
            \\  avg time:   {d:.3} us
            \\  min time:   {d:.3} us
            \\  max time:   {d:.3} us
            \\
        , .{
            self.name,
            self.iterations,
            @as(f64, @floatFromInt(self.total_time_ns)) / 1_000_000.0,
            @as(f64, @floatFromInt(self.avg_time_ns)) / 1_000.0,
            @as(f64, @floatFromInt(self.min_time_ns)) / 1_000.0,
            @as(f64, @floatFromInt(self.max_time_ns)) / 1_000.0,
        });
    }
};

/// Benchmark runner for Monkey language code
pub const Benchmark = struct {
    allocator: std.mem.Allocator,
    warmup_iterations: u64,
    benchmark_iterations: u64,

    pub fn init(allocator: std.mem.Allocator) Benchmark {
        return Benchmark{
            .allocator = allocator,
            .warmup_iterations = 10,
            .benchmark_iterations = 100,
        };
    }

    /// Run a benchmark on a piece of Monkey code
    pub fn run(self: *Benchmark, name: []const u8, code: []const u8) !BenchmarkResult {
        // Warmup phase - run without timing
        var warmup_count: u64 = 0;
        while (warmup_count < self.warmup_iterations) : (warmup_count += 1) {
            _ = self.evalOnce(code) catch |err| {
                // std.debug.print("Warmup {d} failed for {s}: {}\n", .{ warmup_count, name, err });
                return err;
            };
        }

        // Benchmark phase
        var min_time: u64 = std.math.maxInt(u64);
        var max_time: u64 = 0;
        var total_time: u64 = 0;

        var i: u64 = 0;
        while (i < self.benchmark_iterations) : (i += 1) {
            const start = std.time.nanoTimestamp();
            _ = try self.evalOnce(code);
            const end = std.time.nanoTimestamp();

            const elapsed: u64 = @intCast(end - start);
            total_time += elapsed;
            if (elapsed < min_time) min_time = elapsed;
            if (elapsed > max_time) max_time = elapsed;
        }

        return BenchmarkResult{
            .name = name,
            .iterations = self.benchmark_iterations,
            .total_time_ns = total_time,
            .avg_time_ns = total_time / self.benchmark_iterations,
            .min_time_ns = min_time,
            .max_time_ns = max_time,
        };
    }

    /// Evaluate code once and return result
    fn evalOnce(self: *Benchmark, code: []const u8) !object_mod.Object {
        var arena = std.heap.ArenaAllocator.init(self.allocator);
        defer arena.deinit();
        const arena_allocator = arena.allocator();

        // Tokenize
        var lexer = lexer_mod.Lexer.init(code);
        var tokens_list = try std.ArrayList(lexer_mod.Token).initCapacity(arena_allocator, 64);

        while (true) {
            const tok = lexer.nextToken();
            try tokens_list.append(arena_allocator, tok);
            if (tok.token_type == .EOF) break;
        }

        const tokens = try tokens_list.toOwnedSlice(arena_allocator);

        // Parse
        var parser = parser_mod.Parser.init(arena_allocator, tokens);
        const program = try parser.parseProgram();

        // Evaluate - use main allocator for environment to support closures
        var env = object_mod.Environment.init(self.allocator);
        defer env.deinit();

        const eval_result = evaluator_mod.evalProgram(arena_allocator, program, &env);

        if (eval_result.isErr()) {
            // std.debug.print("Evaluation error: {}\n", .{eval_result.unwrapErr()});
            return error.EvaluationError;
        }

        return eval_result.unwrap();
    }
};

// =============================================================================
// Benchmark Tests
// =============================================================================

/// Arithmetic benchmark: simple integer operations
const arithmetic_code =
    \\let a = 5;
    \\let b = 10;
    \\let c = a + b * 2;
    \\let d = c - a / 1;
    \\d
;

/// Function call benchmark: simple function calls
const function_call_code =
    \\let add = fn(x, y) { x + y };
    \\let mul = fn(x, y) { x * y };
    \\let result = add(mul(2, 3), mul(4, 5));
    \\result
;

/// Recursive benchmark: fibonacci
const fibonacci_code =
    \\let fib = fn(n) {
    \\    if (n < 2) {
    \\        return n;
    \\    }
    \\    return fib(n - 1) + fib(n - 2);
    \\};
    \\fib(10)
;

/// Array operations benchmark
const array_code =
    \\let arr = [1, 2, 3, 4, 5];
    \\let first_elem = first(arr);
    \\let last_elem = last(arr);
    \\let rest_arr = rest(arr);
    \\let new_arr = push(arr, 6);
    \\len(new_arr)
;

/// Hash operations benchmark
const hash_code =
    \\let h = {"a": 1, "b": 2, "c": 3};
    \\let val_a = h["a"];
    \\let val_b = h["b"];
    \\let val_c = h["c"];
    \\val_a + val_b + val_c
;

/// String operations benchmark
const string_code =
    \\let s1 = "hello";
    \\let s2 = " ";
    \\let s3 = "world";
    \\let combined = s1 + s2 + s3;
    \\len(combined)
;

/// Closure benchmark - functions that return functions capturing outer variables
const closure_code =
    \\let makeAdder = fn(x) {
    \\    fn(y) { x + y }
    \\};
    \\let add5 = makeAdder(5);
    \\let add10 = makeAdder(10);
    \\add5(3) + add10(3)
;

/// Higher-order function benchmark - functions that take and return functions
const higher_order_code =
    \\let compose = fn(f, g) {
    \\    fn(x) { f(g(x)) }
    \\};
    \\let double = fn(x) { x * 2 };
    \\let addTen = fn(x) { x + 10 };
    \\let doubleThenAdd = compose(addTen, double);
    \\doubleThenAdd(5)
;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    std.debug.print(
        \\================================================================================
        \\                     Monkey Language Benchmark Suite
        \\================================================================================
        \\
        \\
    , .{});

    var bench = Benchmark.init(allocator);

    // Configure iterations
    bench.warmup_iterations = 50;
    bench.benchmark_iterations = 500;

    std.debug.print("Configuration:\n", .{});
    std.debug.print("  Warmup iterations:    {d}\n", .{bench.warmup_iterations});
    std.debug.print("  Benchmark iterations: {d}\n\n", .{bench.benchmark_iterations});
    std.debug.print("--------------------------------------------------------------------------------\n\n", .{});

    // Run benchmarks
    const benchmarks = [_]struct { name: []const u8, code: []const u8 }{
        .{ .name = "Arithmetic Operations", .code = arithmetic_code },
        .{ .name = "Function Calls", .code = function_call_code },
        .{ .name = "Fibonacci (n=10)", .code = fibonacci_code },
        .{ .name = "Array Operations", .code = array_code },
        .{ .name = "Hash Operations", .code = hash_code },
        .{ .name = "String Operations", .code = string_code },
        .{ .name = "Closures", .code = closure_code },
        .{ .name = "Higher-Order Functions", .code = higher_order_code },
    };

    for (benchmarks) |b| {
        const result = bench.run(b.name, b.code) catch |err| {
            std.debug.print("{s}: ERROR - {}\n\n", .{ b.name, err });
            continue;
        };
        result.print();
        std.debug.print("\n", .{});
    }

    std.debug.print(
        \\--------------------------------------------------------------------------------
        \\                              Benchmark Complete
        \\================================================================================
        \\
    , .{});
}

// =============================================================================
// Unit Tests for Benchmark Framework
// =============================================================================

test "benchmark arithmetic" {
    const allocator = std.testing.allocator;
    var bench = Benchmark.init(allocator);
    bench.warmup_iterations = 2;
    bench.benchmark_iterations = 5;

    const result = try bench.run("Arithmetic", arithmetic_code);
    try std.testing.expect(result.iterations == 5);
    try std.testing.expect(result.total_time_ns > 0);
    try std.testing.expect(result.avg_time_ns > 0);
}

test "benchmark function calls" {
    const allocator = std.testing.allocator;
    var bench = Benchmark.init(allocator);
    bench.warmup_iterations = 2;
    bench.benchmark_iterations = 5;

    const result = try bench.run("Function Calls", function_call_code);
    try std.testing.expect(result.iterations == 5);
}

test "benchmark fibonacci" {
    const allocator = std.testing.allocator;
    var bench = Benchmark.init(allocator);
    bench.warmup_iterations = 2;
    bench.benchmark_iterations = 5;

    const result = try bench.run("Fibonacci", fibonacci_code);
    try std.testing.expect(result.iterations == 5);
}

test "benchmark arrays" {
    const allocator = std.testing.allocator;
    var bench = Benchmark.init(allocator);
    bench.warmup_iterations = 2;
    bench.benchmark_iterations = 5;

    const result = try bench.run("Arrays", array_code);
    try std.testing.expect(result.iterations == 5);
}

test "benchmark hash" {
    const allocator = std.testing.allocator;
    var bench = Benchmark.init(allocator);
    bench.warmup_iterations = 2;
    bench.benchmark_iterations = 5;

    const result = try bench.run("Hash", hash_code);
    try std.testing.expect(result.iterations == 5);
}

test "benchmark closures" {
    const allocator = std.testing.allocator;
    var bench = Benchmark.init(allocator);
    bench.warmup_iterations = 2;
    bench.benchmark_iterations = 5;

    const result = try bench.run("Closures", closure_code);
    try std.testing.expect(result.iterations == 5);
}
