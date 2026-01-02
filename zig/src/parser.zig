const std = @import("std");
const lexer_mod = @import("lexer.zig");
const parser_mod = @import("parser.zig");

test "integration: parse let statement" {
    const allocator = std.testing.allocator;

    const input = "let x = 5;";
    var lexer = lexer_mod.Lexer.init(input);

    // Tokenize
    var tokens_list = std.ArrayList(lexer_mod.Token).init(allocator);
    defer tokens_list.deinit();

    while (true) {
        const tok = lexer.nextToken();
        try tokens_list.append(tok);
        if (tok.token_type == .EOF) break;
    }

    const tokens = try tokens_list.toOwnedSlice();
    defer allocator.free(tokens);

    // Parse
    var parser = parser_mod.Parser.init(allocator, tokens);
    const program = try parser.parseProgram();
    defer program.deinit();

    try std.testing.expectEqual(@as(usize, 1), program.statements.items.len);
}
