const std = @import("std");
const ast_mod = @import("ast.zig");

/// WebAssembly code generator for Monkey language
/// Uses std.wasm for standard WASM definitions and opcodes
pub const WASMGenerator = struct {
    allocator: std.mem.Allocator,
    buffer: std.ArrayList(u8),
    program: ?ast_mod.Program = null,

    pub fn init(allocator: std.mem.Allocator) WASMGenerator {
        return WASMGenerator{
            .allocator = allocator,
            .buffer = std.ArrayList(u8).initCapacity(allocator, 1024) catch unreachable,
        };
    }

    pub fn deinit(self: *WASMGenerator) void {
        self.buffer.deinit(self.allocator);
    }

    const TypeSectionWriter = struct {
        pub fn writeContent(gen: *WASMGenerator) !void {
            try gen.writeTypeSectionContent();
        }
    };

    const FunctionSectionWriter = struct {
        pub fn writeContent(gen: *WASMGenerator) !void {
            try gen.writeFunctionSectionContent();
        }
    };

    const ExportSectionWriter = struct {
        pub fn writeContent(gen: *WASMGenerator) !void {
            try gen.writeExportSectionContent();
        }
    };

    const CodeSectionWriter = struct {
        pub fn writeContent(gen: *WASMGenerator) !void {
            try gen.writeCodeSectionContent();
        }
    };

    /// Generate WASM module from AST
    pub fn generate(self: *WASMGenerator, program: ast_mod.Program) ![]u8 {
        self.program = program;
        // WASM magic number and version
        try self.writeBytes(&[_]u8{ 0x00, 0x61, 0x73, 0x6D }); // "\0asm"
        try self.writeU32(0x01000000); // Version 1

        // Type section
        try self.writeSectionWithContent(1, TypeSectionWriter);

        // Function section
        try self.writeSectionWithContent(3, FunctionSectionWriter);

        // Export section
        try self.writeSectionWithContent(7, ExportSectionWriter);

        // Code section
        try self.writeSectionWithContent(10, CodeSectionWriter);

        return try self.buffer.toOwnedSlice(self.allocator);
    }

    fn writeTypeSectionContent(self: *WASMGenerator) !void {
        // Number of types
        try self.writeU32(1);

        // Function type: () -> (i32)
        try self.writeByte(std.wasm.function_type); // func
        try self.writeU32(0); // No parameters
        try self.writeU32(1); // One result
        try self.writeByte(@intFromEnum(std.wasm.Valtype.i32)); // i32 result
    }

    fn writeFunctionSectionContent(self: *WASMGenerator) !void {
        // Number of functions
        try self.writeU32(1);

        // Function type index
        try self.writeU32(0);
    }

    fn writeExportSectionContent(self: *WASMGenerator) !void {
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

    fn writeCodeSectionContent(self: *WASMGenerator) !void {
        // Number of function bodies
        try self.writeU32(1);

        // Function body size placeholder
        const size_pos = self.buffer.items.len;
        try self.writeU32LEB128(0);

        const body_start = self.buffer.items.len;

        // Local variables (none for now)
        try self.writeU32(0);

        // Generate code for the program
        try self.generateProgramCode(self.program.?);

        // End of function
        try self.writeByte(@intFromEnum(std.wasm.Opcode.end));

        // Calculate and write function body size
        const body_size = self.buffer.items.len - body_start;
        var temp_gen = WASMGenerator.init(self.allocator);
        defer temp_gen.deinit();
        try temp_gen.writeU32LEB128(@intCast(body_size));
        @memcpy(self.buffer.items[size_pos .. size_pos + temp_gen.buffer.items.len], temp_gen.buffer.items);
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
                const opcode = blk: {
                    if (std.mem.eql(u8, infix.operator, "+")) break :blk std.wasm.Opcode.i32_add;
                    if (std.mem.eql(u8, infix.operator, "-")) break :blk std.wasm.Opcode.i32_sub;
                    if (std.mem.eql(u8, infix.operator, "*")) break :blk std.wasm.Opcode.i32_mul;
                    if (std.mem.eql(u8, infix.operator, "/")) break :blk std.wasm.Opcode.i32_div_s;
                    if (std.mem.eql(u8, infix.operator, "%")) break :blk std.wasm.Opcode.i32_rem_s;
                    return error.UnsupportedOperator;
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

    fn writeSectionWithContent(self: *WASMGenerator, section_id: u8, comptime ContentWriter: type) !void {
        try self.writeByte(section_id);
        // Remember position for size
        const size_pos = self.buffer.items.len;
        // Write placeholder size (LEB128)
        try self.writeU32LEB128(0);

        // Write section content
        const content_start = self.buffer.items.len;
        try ContentWriter.writeContent(self);
        const content_end = self.buffer.items.len;

        // Calculate actual size and update
        const size = content_end - content_start;
        // Create temp generator to get LEB128 size
        var temp_gen = WASMGenerator.init(self.allocator);
        defer temp_gen.deinit();
        try temp_gen.writeU32LEB128(@intCast(size));
        // Copy the LEB128 encoded size back (assume it fits in original placeholder space)
        const leb_size = temp_gen.buffer.items.len;
        @memcpy(self.buffer.items[size_pos .. size_pos + leb_size], temp_gen.buffer.items);
    }

    fn writeByte(self: *WASMGenerator, byte: u8) !void {
        try self.buffer.append(self.allocator, byte);
    }

    fn writeBytes(self: *WASMGenerator, bytes: []const u8) !void {
        try self.buffer.appendSlice(self.allocator, bytes);
    }

    fn writeU32LEB128(self: *WASMGenerator, value: u32) !void {
        var val = value;
        while (true) {
            var byte: u8 = @intCast(val & 0x7F);
            val >>= 7;
            if (val != 0) {
                byte |= 0x80;
            }
            try self.buffer.append(self.allocator, byte);
            if (val == 0) break;
        }
    }

    fn writeU32(self: *WASMGenerator, value: u32) !void {
        try self.writeU32LEB128(value);
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
    var statements = [_]ast_mod.Statement{
        .{
            .expression = ast_mod.ExpressionStatement{
                .token = undefined,
                .expression = ast_mod.Expression{
                    .integer_literal = ast_mod.IntegerLiteral{
                        .token = undefined,
                        .value = 42,
                    },
                },
            },
        },
    };

    const program = ast_mod.Program{
        .statements = statements[0..],
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

    var statements = [_]ast_mod.Statement{
        .{
            .expression = ast_mod.ExpressionStatement{
                .token = undefined,
                .expression = ast_mod.Expression{
                    .infix = ast_mod.Infix{
                        .token = undefined,
                        .left = left,
                        .operator = "+",
                        .right = right,
                    },
                },
            },
        },
    };

    const program = ast_mod.Program{
        .statements = statements[0..],
    };

    var generator = WASMGenerator.init(allocator);
    defer generator.deinit();

    const wasm_bytes = try generator.generate(program);
    defer allocator.free(wasm_bytes);

    // Validate the generated WASM
    try std.testing.expect(WASMGenerator.validateWASM(wasm_bytes));
}
