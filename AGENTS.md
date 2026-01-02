# AGENTS.md - AI 编码代理规范 (Zig 项目模板)

> 本模板定义了 AI 编码代理在 Zig 项目中的行为准则。
> 使用时请根据具体项目进行配置。

**Zig 版本**: 0.15.x (最低要求)

---

## 项目配置（使用前必填）

```yaml
# 项目基本信息
project_name: "Your Project Name"
description: "项目简短描述"

# 文档语言
doc_language: "中文"  # 或 "English"

# 项目结构
src_dir: "src/"
docs_dir: "docs/"
examples_dir: "examples/"

# 可选：项目特定规则
sensitive_data_wrapper: "Secret"  # 敏感数据包装类型名
decimal_type: "Decimal"           # 精确计算类型名（如金融项目）
```

---

## 文档语言规范

**强制规则**: 项目中所有文档必须使用配置的语言编写。

- README.md、ROADMAP.md、所有 Story 文件使用配置的文档语言
- 代码注释可以使用英文（遵循 Zig 惯例）
- 变量名、函数名使用英文（编程规范）

---

## 开发流程规范（文档驱动开发）

**核心原则**: 文档先行，代码跟随，测试验证，文档收尾。

### 开发周期

每个功能/修改都必须遵循以下流程：

```
┌─────────────────────────────────────────────────────────────────┐
│  1. 文档准备阶段                                                   │
│     ├── 更新/创建设计文档 (docs/design/)                          │
│     ├── 更新 ROADMAP.md (如果是新功能)                            │
│     └── 更新 Story 文件 (stories/)                                │
├─────────────────────────────────────────────────────────────────┤
│  2. 编码阶段                                                       │
│     ├── 实现功能代码                                               │
│     ├── 添加代码注释                                               │
│     ├── 同步更新 docs/ 对应文档（必须！）                          │
│     └── 更新模块文档                                               │
├─────────────────────────────────────────────────────────────────┤
│  3. 测试阶段                                                       │
│     ├── 单元测试 (zig test src/xxx.zig)                           │
│     ├── 集成测试 (zig build test)                                 │
│     └── 示例测试 (examples/*.zig)                                 │
├─────────────────────────────────────────────────────────────────┤
│  4. 文档收尾阶段                                                   │
│     ├── 更新 CHANGELOG.md                                         │
│     ├── 更新 API 文档 (如有变化)                                   │
│     └── 更新 README.md (如有用户可见变化)                          │
└─────────────────────────────────────────────────────────────────┘
```

### 强制规则

**每次代码修改必须通过测试验证**

所有代码修改完成后必须立即运行测试验证：

```bash
# 运行完整测试套件
zig build test

# 或运行特定模块测试
zig test src/module.zig
```

**测试失败处理原则**:
- ❌ **禁止**: 提交测试失败的代码
- ❌ **禁止**: 为了"暂时修复"而禁用测试或添加不正确的代码
- ✅ **必须**: 修复所有编译错误和测试失败，直到完全通过
- ✅ **必须**: 如果遇到无法立即解决的错误，撤销修改或寻求帮助解决
- ✅ **必须**: 确保 `zig build test` 完全通过后才算完成修改

**代码实现要求**:
- ✅ **必须**: 所有代码实现必须是真实功能实现，不能使用模拟(placeholder/mock)实现
- ✅ **必须**: 功能必须实际可用，能够执行预期的操作
- ❌ **禁止**: 使用 `undefined`、`unreachable` 或其他placeholder值
- ❌ **禁止**: 实现返回硬编码值或模拟数据的函数，除非是测试辅助函数

**验证流程**:
1. 修改代码后立即运行 `zig build test`
2. 如果测试失败，立即修复所有错误
3. 重复步骤1-2直到测试完全通过
4. 只有在测试通过后，才能进行下一步开发

---

### 阶段详解

#### 1. 文档准备阶段

在写任何代码之前，必须先准备文档：

```markdown
# 检查清单

- [ ] 功能是否已在 ROADMAP.md 中规划？
- [ ] 是否需要新的设计文档 (RFC)？
- [ ] Story 文件是否已创建/更新？
```

#### 2. 编码阶段

编码时同步更新相关文档：

```zig
/// 每个公共 API 必须有文档注释
///
/// 示例:
/// ```zig
/// const result = try api.doSomething(.{...});
/// ```
pub fn doSomething(args: Args) !Result {
    // 实现...
}
```

#### 3. 测试阶段

**强制要求**: 所有测试必须完全通过，任何测试失败都必须立即修复（见强制规则）。

测试分三个层次：

```bash
# 1. 单元测试 - 测试单个模块
zig test src/module/file.zig

# 2. 集成测试 - 测试整个项目
zig build test

# 3. 完整测试套件
zig test src/root.zig

# 4. 示例测试 - 验证用户场景
zig build run-example_name
```

### 测试质量要求（强制）

**核心原则**: 所有测试必须通过，且无内存泄漏和段错误。

#### 必须满足的条件

1. **所有测试通过**: `zig build test` 和 `zig test src/root.zig` 必须 100% 通过
2. **无内存泄漏**: 使用 `std.testing.allocator` 会自动检测内存泄漏
3. **无段错误**: 测试不能崩溃或产生未定义行为

#### 内存泄漏检测

Zig 的 `std.testing.allocator` 会自动检测内存泄漏：

```zig
test "no memory leak" {
    const allocator = std.testing.allocator;

    // 如果忘记 free，测试会失败
    const buffer = try allocator.alloc(u8, 100);
    defer allocator.free(buffer);  // ✅ 必须释放

    // 测试代码...
}
```

#### 常见内存问题及解决方案

```zig
// ❌ 错误 - 内存泄漏
test "leaky test" {
    const allocator = std.testing.allocator;
    const data = try allocator.alloc(u8, 100);
    // 忘记 free -> 测试失败: memory leak detected
}

// ✅ 正确 - 使用 defer 释放
test "clean test" {
    const allocator = std.testing.allocator;
    const data = try allocator.alloc(u8, 100);
    defer allocator.free(data);
    // 测试代码...
}

// ❌ 错误 - ArrayList 内存泄漏
test "leaky arraylist" {
    const allocator = std.testing.allocator;
    var list = try std.ArrayList(u8).initCapacity(allocator, 16);
    // 忘记 deinit -> 内存泄漏
}

// ✅ 正确 - ArrayList 正确释放
test "clean arraylist" {
    const allocator = std.testing.allocator;
    var list = try std.ArrayList(u8).initCapacity(allocator, 16);
    defer list.deinit();
    // 测试代码...
}
```

#### 段错误预防

```zig
// ❌ 危险 - 可能段错误
test "dangerous" {
    var ptr: ?*u8 = null;
    _ = ptr.?.*;  // 解引用 null -> 段错误
}

// ✅ 安全 - 检查 null
test "safe" {
    var ptr: ?*u8 = null;
    if (ptr) |p| {
        _ = p.*;
    }
}

// ❌ 危险 - 数组越界
test "out of bounds" {
    const arr = [_]u8{ 1, 2, 3 };
    _ = arr[5];  // 越界 -> 段错误或未定义行为
}

// ✅ 安全 - 边界检查
test "bounds checked" {
    const arr = [_]u8{ 1, 2, 3 };
    if (5 < arr.len) {
        _ = arr[5];
    }
}
```

#### 提交前测试检查清单

```markdown
# 测试检查清单

- [ ] `zig build test` 通过
- [ ] `zig test src/root.zig` 通过
- [ ] 无 "memory leak detected" 错误
- [ ] 无段错误或崩溃
- [ ] 新代码有对应的测试
- [ ] 测试覆盖正常路径和错误路径
```

#### 4. 文档收尾阶段

每次开发完成后必须更新：

```markdown
# CHANGELOG.md 更新模板

### Session YYYY-MM-DD-NNN

**日期**: YYYY-MM-DD
**目标**: 简要描述

#### 完成的工作
1. ...
2. ...

#### 测试结果
- 单元测试: X tests passed
- 集成测试: passed/failed

#### 下一步
- [ ] ...
```

---

## Stories 文件规范（强制）

**核心原则**: 每个版本必须有对应的 Story 文件，且必须与实现状态保持同步。

### Story 文件结构

```
stories/
├── v0.1.0-core-types.md      # v0.1.0 核心功能
├── v0.2.0-extensions.md      # v0.2.0 扩展功能
└── v0.3.0-advanced.md        # v0.3.0 高级功能
```

### Story 文件模板

```markdown
# Story: vX.Y.Z 功能名称

> 简短描述

## 目标

实现的功能列表...

## 验收标准

### 模块名 (module.zig)

- [ ] 功能 1
- [ ] 功能 2
- [ ] 单元测试

### 集成

- [ ] root.zig 导出
- [ ] 文档更新
- [ ] 测试通过

## 完成状态

- 开始日期: YYYY-MM-DD
- 完成日期: YYYY-MM-DD
- 状态: ⏳ 进行中 / ✅ 已完成
```

### Story 同步规则（强制）

| 时机 | 必须执行的操作 |
|------|---------------|
| 开始新版本开发 | 创建对应的 Story 文件，列出所有验收标准 |
| 完成单个功能 | 将对应的 `[ ]` 改为 `[x]` |
| 完成整个版本 | **严格检查：只有当所有 `[ ]` 都变为 `[x]` 时，才能更新完成日期和状态为 ✅** |
| 添加新功能 | 在 Story 中添加对应的验收标准 |
| 版本发布前 | 确保所有 `[ ]` 都变为 `[x]` |

### 禁止行为

1. **禁止**: 代码完成但 Story 未更新
2. **禁止**: Story 标记完成但代码未实现
3. **禁止**: 跳过 Story 直接开发
4. **禁止**: 版本发布时 Story 中仍有 `[ ]`
5. **🚨 强制约束**: **绝对禁止在部分功能完成时将整个Story标记为✅完成**。只有当Story中所有验收标准都标记为`[x]`时，才能更新状态为✅。

### Story 完成检查命令

```bash
# 检查 Story 中未完成的任务
grep -rn "\[ \]" stories/

# 检查 Story 状态
grep -rn "状态:" stories/

# 验证 Story 和 ROADMAP 一致性
echo "=== ROADMAP ===" && grep -n "✅\|⏳" ROADMAP.md
echo "=== Stories ===" && grep -rn "状态:" stories/
```

### 禁止行为

1. **禁止**: 代码完成但 Story 未更新
2. **禁止**: Story 标记完成但代码未实现
3. **禁止**: 跳过 Story 直接开发
4. **禁止**: 版本发布时 Story 中仍有 `[ ]`

---

## 文档同步更新规范（强制）

**核心原则**: 代码和文档必须同步更新，不允许代码实现后文档滞后。

### docs/ 目录结构镜像 src/

```
src/                          docs/
├── types/                    ├── types/
│   ├── decimal.zig          │   ├── decimal.md
│   ├── address.zig          │   ├── address.md
│   └── ...                  │   └── ...
├── module_a/                 ├── module_a/
│   └── ...                  │   └── README.md
└── ...                       └── ...
```

### 文档更新触发条件

| 代码变更类型 | 必须更新的文档 |
|-------------|---------------|
| 新增模块 | `docs/<module>/README.md` + 各文件对应的 `.md` |
| 新增类型 | `docs/types/<type>.md` + `docs/design/types.md` |
| 新增公共函数 | 对应模块的 `.md` 文件 |
| 修改函数签名 | 对应模块的 `.md` 文件 |
| 修改行为/逻辑 | 对应模块的 `.md` 文件 |
| 新增错误类型 | `docs/error.md` |
| Story 完成 | ROADMAP.md 对应任务标记 ✅ |

---

## 阶段完成前检查规范（强制）

**核心原则**: 在开始下一个版本阶段之前，必须检查并解决之前版本的遗留问题。

### 文档扫描命令

```bash
# 1. 扫描 ROADMAP.md 中的待办项
grep -n "⏳" ROADMAP.md

# 2. 扫描 stories/ 中未完成的任务
grep -rn "\[ \]" stories/
grep -rn "⏳" stories/

# 3. 扫描 docs/ 中的 TODO 和未完成标记
grep -rn "TODO\|FIXME\|⏳\|\[ \]" docs/

# 4. 扫描代码中的 TODO 和 FIXME
grep -rn "TODO\|FIXME\|XXX" src/ --include="*.zig"

# 5. 一键扫描所有
echo "=== ROADMAP.md ===" && grep -n "⏳" ROADMAP.md && \
echo "=== stories/ ===" && grep -rn "\[ \]\|⏳" stories/ && \
echo "=== docs/ ===" && grep -rn "TODO\|FIXME\|⏳\|\[ \]" docs/ && \
echo "=== src/ ===" && grep -rn "TODO\|FIXME\|XXX" src/ --include="*.zig"
```

### 未完成标记说明

| 标记 | 位置 | 含义 |
|------|------|------|
| `⏳` | ROADMAP.md, stories/, docs/ | 待开始或进行中 |
| `🔨` | ROADMAP.md, stories/ | 正在进行中 |
| `[ ]` | stories/, docs/ | 未完成的检查项/任务 |
| `TODO` | 代码注释, docs/ | 待实现的功能 |
| `FIXME` | 代码注释, docs/ | 需要修复的问题 |
| `XXX` | 代码注释 | 需要注意或重构的代码 |

### 阶段完成标准

只有满足以下所有条件，才能标记版本为"已完成"：

1. **当前版本核心功能 100% 完成**
2. **所有测试通过**，无内存泄漏
3. **文档状态同步**
4. **遗留问题已评估并记录**

---

## 构建命令

```bash
# 构建项目
zig build

# 运行可执行文件
zig build run

# 运行所有测试
zig build test

# 运行单个文件测试
zig test src/module/file.zig

# 使用优化构建
zig build -Doptimize=ReleaseFast

# 清理构建缓存
rm -rf .zig-cache zig-out
```

---

## 代码风格规范

### 命名约定

```zig
// 类型名: PascalCase
const MyStruct = struct {};
const ClientState = enum {};

// 函数和变量: camelCase
fn processOrder() void {}
var orderCount: u32 = 0;

// 常量: snake_case 或 SCREAMING_SNAKE_CASE
const max_retries = 3;
const DEFAULT_TIMEOUT: u64 = 30000;

// 文件名: snake_case.zig
// order_builder.zig, http_client.zig
```

### 导入顺序

```zig
const std = @import("std");

// 导入分组：先 std，再项目模块
const types = @import("types/mod.zig");
const Decimal = types.Decimal;
```

### 文档注释

所有公共 API 必须有文档注释：

```zig
/// 创建新的资源
///
/// 参数:
///   - allocator: 内存分配器
///   - config: 配置选项
///
/// 返回: 资源对象
/// 错误: OutOfMemory, InvalidConfig
pub fn create(allocator: std.mem.Allocator, config: Config) !Resource {
    // ...
}
```

### 类型定义统一规范

**强制规则**: 所有基础类型必须在统一位置定义，避免重复定义。

#### 禁止行为
- ❌ 不要在多个文件中重复定义相同的类型
- ❌ 不要在模块内部重新定义已存在的类型（如 Result、Option）

#### 正确做法
- ✅ 在 `result.zig` 中定义 `Result` 类型，并在其他地方通过导入使用
- ✅ 在 `option.zig` 中定义 `Option` 类型，并在其他地方通过导入使用
- ✅ 在 `root.zig` 中统一导出所有基础类型

#### 类型引用示例
```zig
// ✅ 正确 - 通过导入使用统一定义的类型
const Result = @import("result.zig").Result;
const Option = @import("option.zig").Option;

// ✅ 正确 - 从 root.zig 导入
const { Result, Option } = @import("root.zig");

// ❌ 错误 - 重复定义
pub fn Result(comptime T: type, comptime E: type) type {
    return union(enum) {
        ok: T,
        err: E,
    };
}
```

#### 审查要点
- 检查所有 `.zig` 文件，确保没有重复的类型定义
- 验证所有类型都通过正确的导入路径引用
- 确保 root.zig 包含所有必要的类型导出

---

## Zig 0.15 API 要求（关键）

### ArrayList（需要 allocator 参数）

```zig
// 初始化 - 始终使用 initCapacity
var list = try std.ArrayList(T).initCapacity(allocator, 16);
defer list.deinit();

// ❌ 错误 - Zig 0.15 的 append 需要 allocator
list.append(item);
try list.append(item);

// ✅ 正确 - 传入 allocator 参数
try list.append(allocator, item);
try list.appendSlice(allocator, items);
const ptr = try list.addOne(allocator);
try list.ensureTotalCapacity(allocator, n);
const owned = try list.toOwnedSlice(allocator);

// AssumeCapacity 系列不需要 allocator
list.appendAssumeCapacity(item);
```

### ArrayList API 速查表（Zig 0.15+）

| 方法 | 需要 allocator | 说明 |
|------|---------------|------|
| `initCapacity(allocator, n)` | 是 | 初始化并预分配容量 |
| `deinit()` | 否 | 释放内存 |
| `append(allocator, item)` | 是 | 添加单个元素 |
| `appendSlice(allocator, items)` | 是 | 添加多个元素 |
| `addOne(allocator)` | 是 | 获取新元素指针 |
| `ensureTotalCapacity(allocator, n)` | 是 | 确保容量 |
| `toOwnedSlice(allocator)` | 是 | 转换为拥有的切片 |
| `appendAssumeCapacity(item)` | 否 | 假设容量足够 |
| `items` 字段 | 否 | 只读访问 |

### HashMap

```zig
// Managed（StringHashMap, AutoHashMap）- 存储 allocator
var map = std.StringHashMap(V).init(allocator);
defer map.deinit();
try map.put(key, value);  // 不需要 allocator

// Unmanaged（StringHashMapUnmanaged）- 需要 allocator
var umap = std.StringHashMapUnmanaged(V){};
defer umap.deinit(allocator);
try umap.put(allocator, key, value);  // 需要 allocator

// 使用 getOrPut 避免重复查找
const result = try map.getOrPut(key);
if (!result.found_existing) {
    result.value_ptr.* = new_value;
}
```

### HTTP Client（Zig 0.15+ request/response API）

**注意**: Zig 0.15 完全重构了 HTTP Client API，移除了 `fetch()` 方法。

```zig
var client: std.http.Client = .{ .allocator = allocator };
defer client.deinit();

// 解析 URI
const uri = std.Uri.parse(url) catch return error.BadRequest;

// 创建请求
var req = client.request(.GET, uri, .{
    .extra_headers = &.{
        .{ .name = "Accept", .value = "application/json" },
    },
}) catch return error.ConnectionFailed;
defer req.deinit();

// 发送 GET 请求（无 body）
req.sendBodiless() catch return error.ConnectionFailed;

// 或发送 POST 请求（带 body）
// req.transfer_encoding = .{ .content_length = body.len };
// var body_writer = req.sendBodyUnflushed(&.{}) catch return error.ConnectionFailed;
// body_writer.writer.writeAll(body) catch return error.ConnectionFailed;
// body_writer.end() catch return error.ConnectionFailed;
// if (req.connection) |conn| {
//     conn.flush() catch return error.ConnectionFailed;
// }

// 接收响应头
var response = req.receiveHead(&.{}) catch return error.ConnectionFailed;

// 检查状态码
if (response.head.status != .ok) {
    return error.HttpError;
}

// 读取响应体
var reader = response.reader(&.{});
const body = reader.allocRemaining(allocator, std.Io.Limit.limited(10 * 1024 * 1024)) catch return error.ReadFailed;
defer allocator.free(body);
```

### std.json

```zig
// 解析并释放
const parsed = try std.json.parseFromSlice(MyStruct, allocator, json_string, .{});
defer parsed.deinit();
const data = parsed.value;

// 序列化
const json_output = try std.json.stringifyAlloc(allocator, data, .{});
defer allocator.free(json_output);
```

### std.fmt

```zig
// 分配式格式化
const formatted = try std.fmt.allocPrint(allocator, "value: {d}", .{42});
defer allocator.free(formatted);

// 非分配式格式化（使用缓冲区）
var buffer: [256]u8 = undefined;
const result = try std.fmt.bufPrint(&buffer, "value: {d}", .{42});
```

### 自定义 format 函数（Zig 0.15+）

```zig
// ❌ 旧版本
pub fn format(
    self: Self,
    comptime fmt: []const u8,
    options: std.fmt.FormatOptions,
    writer: anytype,
) !void {
    _ = fmt;
    _ = options;
    try writer.writeAll("...");
}
// 使用: std.fmt.bufPrint(&buf, "{}", .{value});

// ✅ Zig 0.15+ (使用 {f} 格式)
pub fn format(self: Self, writer: anytype) !void {
    _ = self;
    try writer.writeAll("...");
}
// 使用: std.fmt.bufPrint(&buf, "{f}", .{value});
```

### 类型信息枚举（Zig 0.15+）

```zig
// ❌ 旧版本
if (@typeInfo(T) == .Slice) { ... }
if (info.pointer.size == .Slice) { ... }

// ✅ Zig 0.15+
if (@typeInfo(T) == .slice) { ... }
if (info.pointer.size == .slice) { ... }
```

**影响的枚举**:
- `.Slice` → `.slice`
- `.Pointer` → `.pointer`
- `.Struct` → `.@"struct"`
- `.Enum` → `.@"enum"`
- `.Union` → `.@"union"`
- `.Array` → `.array`
- `.Optional` → `.optional`

---

## 内存管理

### 资源清理

```zig
// 始终使用 defer 清理
const buffer = try allocator.alloc(u8, size);
defer allocator.free(buffer);

// 使用 errdefer 处理错误路径清理
fn createResource(allocator: Allocator) !*Resource {
    const res = try allocator.create(Resource);
    errdefer allocator.destroy(res);

    res.data = try allocator.alloc(u8, 100);
    errdefer allocator.free(res.data);

    try res.initialize();
    return res;
}
```

### Arena Allocator 用于临时分配

```zig
var arena = std.heap.ArenaAllocator.init(allocator);
defer arena.deinit();
const temp = arena.allocator();
// arena.deinit() 会一次性释放所有分配
```

### 字符串所有权

```zig
// 借用 - 不要释放
fn process(borrowed: []const u8) void {
    // 只读，不能释放
}

// 拥有 - 调用者必须释放
fn createString(allocator: std.mem.Allocator) ![]u8 {
    return try allocator.dupe(u8, "owned string");
}

const owned = try createString(allocator);
defer allocator.free(owned);
```

---

## 错误处理

```zig
// ❌ 错误 - 静默忽略错误
const result = doSomething() catch null;

// ✅ 正确 - 传播错误
const result = try doSomething();

// ✅ 正确 - 有意义的错误处理
const result = doSomething() catch |err| {
    std.log.err("Failed: {}", .{err});
    return err;
};
```

---

## 项目特定规则（可选配置）

### 敏感数据处理

```zig
// ❌ 错误 - 原始敏感数据
const Credentials = struct {
    secret: []const u8,      // 可能被意外打印
};

// ✅ 正确 - 使用 Secret 包装
const Credentials = struct {
    secret: Secret([]const u8),
    passphrase: Secret([]const u8),
};

// Secret.format() 输出 "[REDACTED]"
```

### 精确计算（金融项目）

```zig
// ❌ 永远不要用 f64 处理金钱
const price: f64 = 0.65;
const total = price * 100.0;  // 可能是 64.99999999...

// ✅ 正确 - 使用 Decimal
const price = try Decimal.fromString("0.65");
const size = try Decimal.fromString("100");
const total = price.mul(size);  // 精确的 65.00
```

### 日志规范

```zig
// ❌ 错误 - 泄露敏感信息
std.log.info("API Key: {s}", .{credentials.key});

// ✅ 正确 - 安全日志
std.log.info("Request to {s}", .{endpoint});
std.log.debug("Order ID: {s}", .{order_id});
```

### 禁止 async/await

Zig 0.11+ 移除了原生 async/await。使用同步代码或线程：

```zig
// 同步（推荐）
const result = try fetchData();

// 并发操作使用线程
const thread = try std.Thread.spawn(.{}, workerFn, .{});
```

---

## 提交前检查清单

### Zig 0.15 API
- [ ] `ArrayList` 使用 `initCapacity` 并向变更方法传入 `allocator`
- [ ] `toOwnedSlice` 传入 `allocator` 参数
- [ ] 区分 Managed（`StringHashMap`）和 Unmanaged（`StringHashMapUnmanaged`）
- [ ] HTTP 请求使用 Zig 0.15 的 `request/response` API（非 fetch）
- [ ] 自定义 format 函数使用 `{f}` 格式说明符
- [ ] @typeInfo 枚举使用小写（如 `.slice` 而非 `.Slice`）

### 内存安全
- [ ] 所有分配都有对应的 `defer`/`errdefer`
- [ ] 使用 `errdefer` 处理错误路径清理
- [ ] 没有使用 `async`/`await`（Zig 0.11+ 已移除）

### 项目规则
- [ ] 敏感数据使用包装类型（如配置）
- [ ] 金融计算使用精确类型（如配置）
- [ ] 日志不包含敏感信息
- [ ] 公共 API 有文档注释
- [ ] **强制**: 测试必须完全通过：`zig build test` (见强制规则)

### 文档规范
- [ ] 相关文档已同步更新
- [ ] ROADMAP.md 状态正确
- [ ] Story 文件已更新（所有 `[ ]` 改为 `[x]`）
- [ ] Story 完成状态已更新（⏳ → ✅）
- [ ] README.md 已更新（如需要）
- [ ] CHANGELOG.md 已更新

### Story 同步检查
- [ ] `grep -rn "\[ \]" stories/` 无输出（所有任务完成）
- [ ] Story 状态与 ROADMAP 一致

---

## Zig 版本兼容性问题速查

### 常见迁移错误消息

| 错误消息 | 原因 | 解决方案 |
|---------|------|---------|
| `expected 2 argument(s), found 1` | ArrayList.append 需要 allocator | 添加 allocator 参数 |
| `ambiguous format string` | 自定义 format 需要 `{f}` | 使用 `{f}` 而非 `{}` |
| `no field named 'response_storage'` | fetch API 已移除 | 使用 request/response 模式 |
| `member access not allowed on type` | 枚举大小写变更 | 使用小写枚举值 |
| `expected type 'i2'` | compare 返回类型 | 返回 -1, 0, 1 而非枚举 |

### 迁移检查清单

- [ ] ArrayList 方法添加 allocator 参数
- [ ] HTTP 请求改用 request/response 模式
- [ ] 自定义 format 使用 `{f}` 格式
- [ ] @typeInfo 枚举使用小写
- [ ] 检查 compare 函数返回 i2
- [ ] 测试所有网络错误处理

---

## 相关文档

- `ROADMAP.md` - 项目路线图（Source of Truth）
- `README.md` - 用户入口文档（必须保持最新）
- `stories/` - 工作单元（Stories）
- `docs/` - 详细设计文档
- `CHANGELOG.md` - 变更日志
