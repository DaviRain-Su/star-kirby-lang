const std = @import("std");
const ast_mod = @import("ast.zig");

/// 字节码缓存系统，用于加速重复执行
pub const BytecodeCache = struct {
    allocator: std.mem.Allocator,
    cache_dir: std.fs.Dir,
    enabled: bool,

    const CacheEntry = struct {
        source_hash: [32]u8, // SHA256 hash of source
        source_mtime: i128, // Source file modification time
        bytecode: []u8, // Serialized AST/program
    };

    pub fn init(allocator: std.mem.Allocator, cache_dir_path: []const u8, enabled: bool) !BytecodeCache {
        // Create cache directory if it doesn't exist
        std.fs.cwd().makeDir(cache_dir_path) catch |err| {
            if (err != error.PathAlreadyExists) {
                return err;
            }
        };

        const cache_dir = try std.fs.cwd().openDir(cache_dir_path, .{});

        return BytecodeCache{
            .allocator = allocator,
            .cache_dir = cache_dir,
            .enabled = enabled,
        };
    }

    pub fn deinit(self: *BytecodeCache) void {
        self.cache_dir.close();
    }

    /// 计算源代码的 SHA256 哈希
    fn computeSourceHash(source: []const u8) [32]u8 {
        var hash: [32]u8 = undefined;
        std.crypto.hash.sha2.Sha256.hash(source, &hash, .{});
        return hash;
    }

    /// 生成缓存文件名（基于源文件路径的哈希）
    fn getCacheFilename(self: *BytecodeCache, source_path: []const u8) ![]u8 {
        var path_hash: [32]u8 = undefined;
        std.crypto.hash.sha2.Sha256.hash(source_path, &path_hash, .{});

        // Use first 8 bytes as filename (16 hex chars)
        var hex_buf: [16]u8 = undefined;
        for (0..8) |i| {
            _ = std.fmt.bufPrint(hex_buf[i * 2 .. i * 2 + 2], "{x:0>2}", .{path_hash[i]}) catch unreachable;
        }

        return try std.fmt.allocPrint(self.allocator, "{s}.cache", .{hex_buf});
    }

    /// 检查缓存是否有效（简化版本，只检查哈希和时间戳）
    pub fn isValid(self: *BytecodeCache, source_path: []const u8, source: []const u8, source_mtime: i128) !bool {
        if (!self.enabled) return false;

        const cache_filename = try self.getCacheFilename(source_path);
        defer self.allocator.free(cache_filename);

        // Try to open cache file
        const cache_file = self.cache_dir.openFile(cache_filename, .{}) catch |err| {
            if (err == error.FileNotFound) {
                return false; // Cache miss
            }
            return err;
        };
        defer cache_file.close();

        // Read cached metadata
        var cached_hash: [32]u8 = undefined;
        _ = try cache_file.read(&cached_hash);

        var mtime_buf: [16]u8 = undefined;
        _ = try cache_file.read(&mtime_buf);
        const cached_mtime = std.mem.readInt(i128, &mtime_buf, .little);

        // Verify cache validity
        const current_hash = computeSourceHash(source);
        if (!std.mem.eql(u8, &cached_hash, &current_hash)) {
            return false; // Source changed
        }

        if (cached_mtime != source_mtime) {
            return false; // File modified
        }

        return true; // Cache hit
    }

    /// 标记缓存为有效（存储元数据）
    pub fn markValid(self: *BytecodeCache, source_path: []const u8, source: []const u8, source_mtime: i128) !void {
        if (!self.enabled) return;

        const cache_filename = try self.getCacheFilename(source_path);
        defer self.allocator.free(cache_filename);

        // Create/update cache file
        const cache_file = try self.cache_dir.createFile(cache_filename, .{ .truncate = true });
        defer cache_file.close();

        const source_hash = computeSourceHash(source);

        // Write cache metadata
        _ = try cache_file.write(&source_hash);

        var mtime_buf: [16]u8 = undefined;
        std.mem.writeInt(i128, &mtime_buf, source_mtime, .little);
        _ = try cache_file.write(&mtime_buf);
    }

    /// 清理所有缓存文件
    pub fn clear(self: *BytecodeCache) !void {
        var it = self.cache_dir.iterate();
        while (try it.next()) |entry| {
            if (entry.kind == .file and std.mem.endsWith(u8, entry.name, ".cache")) {
                try self.cache_dir.deleteFile(entry.name);
            }
        }
    }

    /// 获取缓存统计信息
    pub fn getStats(self: *BytecodeCache) !struct { files: usize, total_size: usize } {
        var files: usize = 0;
        var total_size: usize = 0;

        var it = self.cache_dir.iterate();
        while (try it.next()) |entry| {
            if (entry.kind == .file and std.mem.endsWith(u8, entry.name, ".cache")) {
                files += 1;
                const stat = try self.cache_dir.statFile(entry.name);
                total_size += stat.size;
            }
        }

        return .{ .files = files, .total_size = total_size };
    }
};
