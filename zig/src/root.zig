//! By convention, root.zig is the root source file when making a library.
const std = @import("std");

// Export modules
pub const token = @import("token.zig");
pub const lexer = @import("lexer.zig");
pub const ast = @import("ast.zig");
pub const parser = @import("parser.zig");
pub const object = @import("object.zig");
pub const evaluator = @import("evaluator.zig");
pub const builtins = @import("builtins.zig");
pub const repl = @import("repl.zig");

// v0.12.0 modules
pub const cache = @import("cache.zig");
pub const object_pool = @import("object_pool.zig");
pub const wasm = @import("wasm.zig");

// Tests - include all test modules
test {
    _ = @import("cache.zig");
    _ = @import("object_pool.zig");
    _ = @import("wasm.zig");
}
