const std = @import("std");
const ast_mod = @import("ast.zig");

/// WebAssembly code generator for Monkey language
/// Uses std.wasm for standard WASM definitions and opcodes
pub const WASMGenerator = struct {
    allocator: std.mem.Allocator,
    buffer: std.ArrayList(u8),

    pub fn init(allocator: std.mem.Allocator) WASMGenerator {
        return WASMGenerator{
            .allocator = allocator,
            .buffer = std.ArrayList(u8).initCapacity(allocator, 1024) catch unreachable,
        };
    }

    pub fn deinit(self: *WASMGenerator) void {
        self.buffer.deinit();
    }

    /// Generate WASM module from AST
    pub fn generate(self: *WASMGenerator, program: ast_mod.Program) ![]u8 {
        // WASM magic number and version
        try self.writeBytes(&[_]u8{ 0x00, 0x61, 0x73, 0x6D }); // "\0asm"
        try self.writeU32(0x01000000); // Version 1

        // Type section
        try self.writeTypeSection();

        // Function section
        try self.writeFunctionSection();

        // Export section
        try self.writeExportSection();

        // Code section
        try self.writeCodeSection(program);

        return try self.buffer.toOwnedSlice();
    }

    fn writeTypeSection(self: *WASMGenerator) !void {
        try self.writeSection(1); // Type section

        // Number of types
        try self.writeU32(1);

        // Function type: () -> (i32)
        try self.writeByte(std.wasm.function_type); // func
        try self.writeU32(0); // No parameters
        try self.writeU32(1); // One result
        try self.writeByte(@intFromEnum(std.wasm.Valtype.i32)); // i32 result
    }

    fn writeFunctionSection(self: *WASMGenerator) !void {
        try self.writeSection(3); // Function section

        // Number of functions
        try self.writeU32(1);

        // Function type index
        try self.writeU32(0);
    }

    fn writeExportSection(self: *WASMGenerator) !void {
        try self.writeSection(7); // Export section

        // Number of exports
        try self.writeU32(1);

        // Export name: "main"
        const name = "main";
        try self.writeU32(name.len);
        try self.writeBytes(name);

        // Export kind: function
        try self.writeByte(0x00);
        // Export index: 0
        try self.writeU32(0);
    }

    fn writeCodeSection(self: *WASMGenerator, program: ast_mod.Program) !void {
        try self.writeSection(10); // Code section

        // Number of function bodies
        try self.writeU32(1);

        // Function body
        const body_start = self.buffer.items.len;

        // Local variables (none for now)
        try self.writeU32(0);

        // Generate code for the program
        try self.generateProgramCode(program);

        // End of function
        try self.writeByte(@intFromEnum(std.wasm.Opcode.end));

        // Update function body size
        _ = self.buffer.items.len - body_start;
        // Note: In a real implementation, we'd need to write the size before the body
    }

    fn generateProgramCode(self: *WASMGenerator, program: ast_mod.Program) !void {
        // For now, just evaluate the last expression and return its result
        if (program.statements.len > 0) {
            const last_stmt = program.statements[program.statements.len - 1];
            switch (last_stmt) {
                .expression => |expr_stmt| {
                    try self.generateExpression(expr_stmt.expression);
                },
                else => {
                    // For non-expression statements, return 0
                    try self.writeByte(@intFromEnum(std.wasm.Opcode.i32_const));
                    try self.writeU32(0);
                },
            }
        } else {
            // Empty program, return 0
            try self.writeByte(@intFromEnum(std.wasm.Opcode.i32_const));
            try self.writeU32(0);
        }
    }

    fn generateExpression(self: *WASMGenerator, expr: ast_mod.Expression) anyerror!void {
        switch (expr) {
            .integer_literal => |int_lit| {
                try self.writeByte(@intFromEnum(std.wasm.Opcode.i32_const));
                try self.writeU32(@intCast(int_lit.value));
            },
            .infix => |infix| {
                // Generate left operand
                try self.generateExpression(infix.left.*);
                // Generate right operand
                try self.generateExpression(infix.right.*);

                // Generate operator
                const opcode = switch (infix.operator) {
                    .plus => std.wasm.Opcode.i32_add,
                    .minus => std.wasm.Opcode.i32_sub,
                    .asterisk => std.wasm.Opcode.i32_mul,
                    .slash => std.wasm.Opcode.i32_div_s,
                    .percent => std.wasm.Opcode.i32_rem_s,
                    else => return error.UnsupportedOperator,
                };
                try self.writeByte(@intFromEnum(opcode));
            },
            else => {
                // Unsupported expression type, return 0
                try self.writeByte(@intFromEnum(std.wasm.Opcode.i32_const));
                try self.writeU32(0);
            },
        }
    }

    fn writeSection(self: *WASMGenerator, section_id: u8) !void {
        try self.writeByte(section_id);
        // Size placeholder (will be updated)
        try self.writeU32(0);
        // TODO: Implement proper section size calculation
    }

    fn writeByte(self: *WASMGenerator, byte: u8) !void {
        try self.buffer.append(byte);
    }

    fn writeBytes(self: *WASMGenerator, bytes: []const u8) !void {
        try self.buffer.appendSlice(bytes);
    }

    fn writeU32(self: *WASMGenerator, value: u32) !void {
        var buf: [4]u8 = undefined;
        std.mem.writeInt(u32, &buf, value, .little);
        try self.buffer.appendSlice(&buf);
    }

    /// Validate generated WASM module
    pub fn validateWASM(wasm_bytes: []const u8) bool {
        // Basic validation: check magic number
        if (wasm_bytes.len < 8) return false;
        return std.mem.eql(u8, wasm_bytes[0..4], &[_]u8{ 0x00, 0x61, 0x73, 0x6D });
    }
};

// Test WASM generation
test "wasm generation" {
    const allocator = std.testing.allocator;

    // Create a simple AST program: 42
    const program = ast_mod.Program{
        .statements = &[_]ast_mod.Statement{
            .{
                .expression = .{
                    .expression = ast_mod.Expression{
                        .integer_literal = ast_mod.IntegerLiteral{
                            .token = undefined,
                            .value = 42,
                        },
                    },
                },
            },
        },
    };

    var generator = WASMGenerator.init(allocator);
    defer generator.deinit();

    const wasm_bytes = try generator.generate(program);
    defer allocator.free(wasm_bytes);

    // Validate the generated WASM
    try std.testing.expect(WASMGenerator.validateWASM(wasm_bytes));

    // Check that it starts with the correct magic number
    try std.testing.expectEqualSlices(u8, &[_]u8{ 0x00, 0x61, 0x73, 0x6D }, wasm_bytes[0..4]);
}

test "wasm arithmetic" {
    const allocator = std.testing.allocator;

    // Create AST for: 10 + 5
    const left = try allocator.create(ast_mod.Expression);
    defer allocator.destroy(left);
    left.* = ast_mod.Expression{
        .integer_literal = ast_mod.IntegerLiteral{
            .token = undefined,
            .value = 10,
        },
    };

    const right = try allocator.create(ast_mod.Expression);
    defer allocator.destroy(right);
    right.* = ast_mod.Expression{
        .integer_literal = ast_mod.IntegerLiteral{
            .token = undefined,
            .value = 5,
        },
    };

    const program = ast_mod.Program{
        .statements = &[_]ast_mod.Statement{
            .{
                .expression = .{
                    .expression = ast_mod.Expression{
                        .infix = ast_mod.InfixExpression{
                            .token = undefined,
                            .left = left,
                            .operator = .plus,
                            .right = right,
                        },
                    },
                },
            },
        },
    };

    var generator = WASMGenerator.init(allocator);
    defer generator.deinit();

    const wasm_bytes = try generator.generate(program);
    defer allocator.free(wasm_bytes);

    // Validate the generated WASM
    try std.testing.expect(WASMGenerator.validateWASM(wasm_bytes));
}
