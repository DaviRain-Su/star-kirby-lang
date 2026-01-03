const std = @import("std");
const object_mod = @import("object.zig");

/// Inline cache for property access optimization
pub const InlineCache = struct {
    /// Cache entry for property access
    pub const PropertyCacheEntry = struct {
        object_type: object_mod.ObjectType,
        property_name: []const u8,
        cached_result: ?*object_mod.Object,
        access_count: usize,
    };

    cache_entries: std.StringHashMap(PropertyCacheEntry),
    max_entries: usize,

    pub fn init(allocator: std.mem.Allocator, max_entries: usize) InlineCache {
        return InlineCache{
            .cache_entries = std.StringHashMap(PropertyCacheEntry).init(allocator),
            .max_entries = max_entries,
        };
    }

    pub fn deinit(self: *InlineCache) void {
        self.cache_entries.deinit();
    }

    /// Try to get cached property access result
    pub fn getCachedProperty(self: *InlineCache, object: *object_mod.Object, property: []const u8) ?*object_mod.Object {
        const key = std.fmt.allocPrint(self.cache_entries.allocator, "{s}:{s}", .{ @tagName(object.objectType()), property }) catch return null;
        defer self.cache_entries.allocator.free(key);

        if (self.cache_entries.get(key)) |entry| {
            if (entry.object_type == object.objectType() and std.mem.eql(u8, entry.property_name, property)) {
                // Update access count
                var updated_entry = entry;
                updated_entry.access_count += 1;
                self.cache_entries.put(key, updated_entry) catch {};
                return entry.cached_result;
            }
        }
        return null;
    }

    /// Cache a property access result
    pub fn cacheProperty(self: *InlineCache, object: *object_mod.Object, property: []const u8, result: ?*object_mod.Object) !void {
        if (self.cache_entries.count() >= self.max_entries) {
            // Simple eviction: remove oldest entries
            // In a real implementation, we'd use LRU or similar
            return;
        }

        const key = try std.fmt.allocPrint(self.cache_entries.allocator, "{s}:{s}", .{ @tagName(object.objectType()), property });

        const entry = PropertyCacheEntry{
            .object_type = object.objectType(),
            .property_name = try self.cache_entries.allocator.dupe(u8, property),
            .cached_result = result,
            .access_count = 1,
        };

        try self.cache_entries.put(key, entry);
    }

    /// Get cache statistics
    pub fn getStats(self: *InlineCache) struct {
        entries: usize,
        total_accesses: usize,
        hit_rate: f64,
    } {
        var total_accesses: usize = 0;
        var hits: usize = 0;

        var it = self.cache_entries.iterator();
        while (it.next()) |entry| {
            total_accesses += entry.value_ptr.access_count;
            if (entry.value_ptr.cached_result != null) {
                hits += entry.value_ptr.access_count;
            }
        }

        const hit_rate = if (total_accesses > 0) @as(f64, @floatFromInt(hits)) / @as(f64, @floatFromInt(total_accesses)) else 0.0;

        return .{
            .entries = self.cache_entries.count(),
            .total_accesses = total_accesses,
            .hit_rate = hit_rate,
        };
    }

    /// Function call cache for pure functions
    pub const FunctionCallCache = struct {
        pub const CacheKey = struct {
            function_name: []const u8,
            args_hash: u64,
        };

        pub const FunctionCacheEntry = struct {
            key: CacheKey,
            result: *object_mod.Object,
            access_count: usize,
            last_accessed: i64,
        };

        cache_entries: std.AutoHashMap(u64, FunctionCacheEntry),
        max_entries: usize,
        allocator: std.mem.Allocator,

        pub fn init(allocator: std.mem.Allocator, max_entries: usize) FunctionCallCache {
            return FunctionCallCache{
                .cache_entries = std.AutoHashMap(u64, FunctionCacheEntry).init(allocator),
                .max_entries = max_entries,
                .allocator = allocator,
            };
        }

        pub fn deinit(self: *FunctionCallCache) void {
            self.cache_entries.deinit();
        }

        /// Compute hash for function call
        fn computeCallHash(function_name: []const u8, args: []const *object_mod.Object) u64 {
            var hasher = std.hash.Wyhash.init(0);
            hasher.update(function_name);

            for (args) |arg| {
                switch (arg.*) {
                    .integer => |int| hasher.update(std.mem.asBytes(&int.value)),
                    .boolean => |bool_val| hasher.update(std.mem.asBytes(&bool_val.value)),
                    .string => |str| hasher.update(str.value),
                    else => {
                        // For complex objects, don't cache
                        return 0;
                    },
                }
            }

            return hasher.final();
        }

        /// Try to get cached function result
        pub fn getCachedResult(self: *FunctionCallCache, function_name: []const u8, args: []const *object_mod.Object) ?*object_mod.Object {
            const hash = computeCallHash(function_name, args);
            if (hash == 0) return null; // Don't cache complex calls

            if (self.cache_entries.get(hash)) |entry| {
                if (std.mem.eql(u8, entry.key.function_name, function_name)) {
                    // Update access info
                    var updated_entry = entry;
                    updated_entry.access_count += 1;
                    updated_entry.last_accessed = std.time.timestamp() catch 0;
                    self.cache_entries.put(hash, updated_entry) catch {};
                    return entry.result;
                }
            }
            return null;
        }

        /// Cache a function call result
        pub fn cacheResult(self: *FunctionCallCache, function_name: []const u8, args: []const *object_mod.Object, result: *object_mod.Object) !void {
            if (self.cache_entries.count() >= self.max_entries) {
                // Simple cleanup: remove entries with low access count
                // In production, would use more sophisticated eviction
                return;
            }

            const hash = computeCallHash(function_name, args);
            if (hash == 0) return; // Don't cache complex calls

            const key = CacheKey{
                .function_name = try self.allocator.dupe(u8, function_name),
                .args_hash = hash,
            };

            const entry = FunctionCacheEntry{
                .key = key,
                .result = result,
                .access_count = 1,
                .last_accessed = std.time.timestamp() catch 0,
            };

            try self.cache_entries.put(hash, entry);
        }

        /// Get cache statistics
        pub fn getStats(self: *FunctionCallCache) struct {
            entries: usize,
            total_accesses: usize,
        } {
            var total_accesses: usize = 0;

            var it = self.cache_entries.iterator();
            while (it.next()) |entry| {
                total_accesses += entry.value_ptr.access_count;
            }

            return .{
                .entries = self.cache_entries.count(),
                .total_accesses = total_accesses,
            };
        }
    };
};

/// Object pool for reusing common objects to reduce allocation overhead
pub const ObjectPool = struct {
    allocator: std.mem.Allocator,

    // Pools for different object types
    integer_pool: std.ArrayList(*object_mod.Object),
    boolean_pool: std.ArrayList(*object_mod.Object),
    string_pool: std.ArrayList(*object_mod.Object),

    // Statistics
    allocations_saved: usize = 0,
    pool_hits: usize = 0,
    pool_misses: usize = 0,

    pub fn init(allocator: std.mem.Allocator) ObjectPool {
        return ObjectPool{
            .allocator = allocator,
            .integer_pool = std.ArrayList(*object_mod.Object).initCapacity(allocator, 32) catch unreachable,
            .boolean_pool = std.ArrayList(*object_mod.Object).initCapacity(allocator, 8) catch unreachable,
            .string_pool = std.ArrayList(*object_mod.Object).initCapacity(allocator, 16) catch unreachable,
        };
    }

    pub fn deinit(self: *ObjectPool) void {
        // Free all pooled objects
        for (self.integer_pool.items) |obj| {
            self.allocator.destroy(obj);
        }
        for (self.boolean_pool.items) |obj| {
            self.allocator.destroy(obj);
        }
        for (self.string_pool.items) |obj| {
            // Free the string value first
            self.allocator.free(obj.string.value);
            self.allocator.destroy(obj);
        }

        self.integer_pool.deinit(self.allocator);
        self.boolean_pool.deinit(self.allocator);
        self.string_pool.deinit(self.allocator);
    }

    /// Get a pooled integer object, or create new one if pool is empty
    pub fn getInteger(self: *ObjectPool, value: i64) !*object_mod.Object {
        if (self.integer_pool.pop()) |obj| {
            obj.integer.value = value;
            self.pool_hits += 1;
            return obj;
        }

        // Create new object
        const obj = try self.allocator.create(object_mod.Object);
        obj.* = object_mod.Object{ .integer = .{ .value = value } };
        self.pool_misses += 1;
        return obj;
    }

    /// Return an integer object to the pool for reuse
    pub fn returnInteger(self: *ObjectPool, obj: *object_mod.Object) !void {
        if (self.integer_pool.items.len < 64) { // Limit pool size
            try self.integer_pool.append(self.allocator, obj);
            self.allocations_saved += 1;
        } else {
            // Pool is full, free the object
            self.allocator.destroy(obj);
        }
    }

    /// Get a pooled boolean object
    pub fn getBoolean(self: *ObjectPool, value: bool) !*object_mod.Object {
        if (self.boolean_pool.pop()) |obj| {
            obj.boolean.value = value;
            self.pool_hits += 1;
            return obj;
        }

        // Create new object
        const obj = try self.allocator.create(object_mod.Object);
        obj.* = object_mod.Object{ .boolean = .{ .value = value } };
        self.pool_misses += 1;
        return obj;
    }

    /// Return a boolean object to the pool
    pub fn returnBoolean(self: *ObjectPool, obj: *object_mod.Object) !void {
        if (self.boolean_pool.items.len < 16) { // Limit pool size
            try self.boolean_pool.append(self.allocator, obj);
            self.allocations_saved += 1;
        } else {
            self.allocator.destroy(obj);
        }
    }

    /// Get a pooled string object
    pub fn getString(self: *ObjectPool, value: []const u8) !*object_mod.Object {
        if (self.string_pool.pop()) |obj| {
            // Free old string and allocate new one
            self.allocator.free(obj.string.value);
            obj.string.value = try self.allocator.dupe(u8, value);
            self.pool_hits += 1;
            return obj;
        }

        // Create new object
        const obj = try self.allocator.create(object_mod.Object);
        obj.* = object_mod.Object{ .string = .{ .value = try self.allocator.dupe(u8, value) } };
        self.pool_misses += 1;
        return obj;
    }

    /// Return a string object to the pool
    pub fn returnString(self: *ObjectPool, obj: *object_mod.Object) !void {
        if (self.string_pool.items.len < 32) { // Limit pool size
            try self.string_pool.append(self.allocator, obj);
            self.allocations_saved += 1;
        } else {
            // Free the string and object
            self.allocator.free(obj.string.value);
            self.allocator.destroy(obj);
        }
    }

    /// Get pool statistics
    pub fn getStats(self: *ObjectPool) struct {
        integer_pool_size: usize,
        boolean_pool_size: usize,
        string_pool_size: usize,
        allocations_saved: usize,
        pool_hits: usize,
        pool_misses: usize,
    } {
        return .{
            .integer_pool_size = self.integer_pool.items.len,
            .boolean_pool_size = self.boolean_pool.items.len,
            .string_pool_size = self.string_pool.items.len,
            .allocations_saved = self.allocations_saved,
            .pool_hits = self.pool_hits,
            .pool_misses = self.pool_misses,
        };
    }

    /// Clear all pools (for cleanup)
    pub fn clear(self: *ObjectPool) void {
        _ = self;
        // Currently a no-op - pools maintain their objects for reuse
    }
};

// Tests
test "object pool - init and deinit" {
    const allocator = std.testing.allocator;
    var pool = ObjectPool.init(allocator);
    defer pool.deinit();

    const stats = pool.getStats();
    try std.testing.expect(stats.integer_pool_size == 0);
    try std.testing.expect(stats.boolean_pool_size == 0);
    try std.testing.expect(stats.string_pool_size == 0);
}

test "object pool - integer pool" {
    const allocator = std.testing.allocator;
    var pool = ObjectPool.init(allocator);
    defer pool.deinit();

    // Get an integer (should create new)
    const int1 = try pool.getInteger(42);
    try std.testing.expect(int1.integer.value == 42);

    // Return to pool
    try pool.returnInteger(int1);

    // Get another integer (should reuse)
    const int2 = try pool.getInteger(100);
    try std.testing.expect(int2.integer.value == 100);
    try std.testing.expect(int2 == int1); // Same object reused

    // Return for cleanup
    try pool.returnInteger(int2);

    const stats = pool.getStats();
    try std.testing.expect(stats.pool_hits >= 1);
    try std.testing.expect(stats.allocations_saved >= 1);
}

test "object pool - boolean pool" {
    const allocator = std.testing.allocator;
    var pool = ObjectPool.init(allocator);
    defer pool.deinit();

    // Get a boolean
    const bool1 = try pool.getBoolean(true);
    try std.testing.expect(bool1.boolean.value == true);

    // Return to pool
    try pool.returnBoolean(bool1);

    // Get another boolean (should reuse)
    const bool2 = try pool.getBoolean(false);
    try std.testing.expect(bool2.boolean.value == false);
    try std.testing.expect(bool2 == bool1); // Same object reused

    // Return for cleanup
    try pool.returnBoolean(bool2);
}

test "object pool - string pool" {
    const allocator = std.testing.allocator;
    var pool = ObjectPool.init(allocator);
    defer pool.deinit();

    // Get a string
    const str1 = try pool.getString("hello");
    try std.testing.expectEqualStrings("hello", str1.string.value);

    // Return to pool
    try pool.returnString(str1);

    // Get another string (should reuse object)
    const str2 = try pool.getString("world");
    try std.testing.expectEqualStrings("world", str2.string.value);
    try std.testing.expect(str2 == str1); // Same object reused

    // Return for cleanup
    try pool.returnString(str2);
}

test "object pool - stats tracking" {
    const allocator = std.testing.allocator;
    var pool = ObjectPool.init(allocator);
    defer pool.deinit();

    // Initial stats
    var stats = pool.getStats();
    try std.testing.expect(stats.pool_hits == 0);
    try std.testing.expect(stats.pool_misses == 0);

    // Get new objects (misses)
    const int = try pool.getInteger(1);
    const bl = try pool.getBoolean(true);

    stats = pool.getStats();
    try std.testing.expect(stats.pool_misses == 2);

    // Return and get again (hits)
    try pool.returnInteger(int);
    try pool.returnBoolean(bl);

    const int2 = try pool.getInteger(2);
    const bl2 = try pool.getBoolean(false);

    stats = pool.getStats();
    try std.testing.expect(stats.pool_hits == 2);

    // Cleanup
    try pool.returnInteger(int2);
    try pool.returnBoolean(bl2);
}

test "inline cache - init and deinit" {
    const allocator = std.testing.allocator;
    var cache = InlineCache.init(allocator, 100);
    defer cache.deinit();

    const stats = cache.getStats();
    try std.testing.expect(stats.entries == 0);
}

test "function call cache - init and deinit" {
    const allocator = std.testing.allocator;
    var cache = InlineCache.FunctionCallCache.init(allocator, 100);
    defer cache.deinit();

    const stats = cache.getStats();
    try std.testing.expect(stats.entries == 0);
}
