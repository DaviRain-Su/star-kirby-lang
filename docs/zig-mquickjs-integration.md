# zig-mquickjs 集成评估

> 评估将 zig-mquickjs (MicroQuickJS) 集成到 star-kirby-lang 项目的可行性

**评估日期**: 2026-01-03
**项目地址**: https://github.com/vExcess/zig-mquickjs
**原始项目**: https://github.com/bellard/mquickjs (Fabrice Bellard)

---

## 项目概述

### MicroQuickJS 简介

MicroQuickJS (mquickjs) 是由 Fabrice Bellard 开发的轻量级 JavaScript 引擎，专为嵌入式系统设计：

- **极低内存占用**: 只需 10KB RAM 即可运行 JavaScript 程序
- **ROM 占用**: 约 100KB（ARM Thumb-2 代码，包含 C 库）
- **JavaScript 支持**: ES5 子集 + 部分 ES6+ 特性
- **严格模式**: 禁用一些容易出错的 JavaScript 特性

### zig-mquickjs 状态

vExcess 的 fork 正在将 mquickjs 从 C 移植到 Zig：

**已移植的 Zig 文件**:
- `src/cutils.zig` - 工具函数
- `src/libm.zig` - 数学库
- `src/list.zig` - 链表实现
- `src/softfp_template.zig` - 软浮点模板
- `src/softfp_template_icvt.zig` - 软浮点转换

**仍为 C 代码**:
- `mquickjs.c` - 核心 JS 引擎 (~20000+ 行)
- `mqjs.c` - REPL
- `dtoa.c` - 双精度转字符串
- 其他支持文件

---

## 技术特点

### 内存管理
- 紧凑追踪垃圾收集器（非引用计数）
- 自带内存分配器，不依赖 malloc/free
- 对象地址可能在每次分配时移动
- 需要使用 `JSGCRef` 处理 JSValue 指针

### 值表示
- 32位 CPU 上值大小 = 1 个 CPU 字
- 支持 31 位整数、单个 Unicode 码点、指针
- 字符串内部使用 WTF-8 (UTF-8 + 不成对代理项)

### 字节码
- 基于栈的字节码（类似 QuickJS）
- 可从 ROM 执行预编译字节码
- 调试信息使用指数哥伦布编码压缩

---

## 与 star-kirby-lang 的关系

### 当前项目栈
```
star-kirby-lang
├── src/      # Rust 实现的 Monkey 解释器
└── zig/      # Zig 实现的 Monkey 解释器
    └── src/
        ├── lexer.zig
        ├── parser.zig
        ├── evaluator.zig
        └── ...
```

### 可能的集成方式

#### 方案 1: JavaScript 互操作层
在 Monkey 代码中调用 JavaScript：
```monkey
let js = require("mquickjs");
let result = js.eval("1 + 2 * 3");
puts(result);  // 7
```

**优点**: 扩展 Monkey 的能力
**缺点**: 增加复杂性，目标不明确

#### 方案 2: 学习/参考
参考 mquickjs 的实现技术：
- 紧凑 GC 算法
- 低内存优化技术
- 字节码设计

**优点**: 改进我们自己的实现
**缺点**: 需要深入理解 C 代码

#### 方案 3: 替代后端
将 Monkey 编译为 mquickjs 字节码执行。

**优点**: 利用成熟的 JS 运行时
**缺点**: 语义差异大，工作量大

---

## 集成挑战

### 技术挑战

1. **C 代码占主导**
   - 核心引擎 `mquickjs.c` 仍是 C 代码
   - Zig 移植进度约 10-15%

2. **依赖未定义行为**
   ```zig
   if (optimize == .Debug or optimize == .ReleaseSafe) {
       // C 版本依赖未定义行为
       // 只能用 ReleaseFast 或 ReleaseSmall
   }
   ```

3. **需要链接 libc**
   ```zig
   .link_libc = true
   ```

4. **GC 复杂性**
   - 对象地址会移动
   - 需要使用 `JSGCRef` 保护 JSValue
   - 与我们当前的内存模型不兼容

### 语义差异

| 特性 | Monkey | mquickjs JavaScript |
|------|--------|---------------------|
| 类型系统 | 动态 | 动态 |
| 数组 | 可变长 | 无空洞 |
| 闭包 | 支持 | 支持 |
| 类 | 无 | 有 (ES5) |
| 原型链 | 无 | 有 |
| 正则 | 无 | 有 |

---

## 建议

### 短期 (v0.5.0)
**不建议直接集成**

理由：
1. zig-mquickjs 移植不完整
2. 与 Monkey 语言目标不一致
3. 会引入 C 依赖，降低代码纯净度

### 中期
**可考虑学习其技术**

参考点：
1. 紧凑 GC 实现
2. 低内存字节码设计
3. ROM 执行模式

### 长期
**可能的未来方向**

如果项目目标扩展到：
- 多语言支持
- 嵌入式部署
- JavaScript 互操作

则可重新评估集成。

---

## 替代方案

### 1. 继续完善当前 Monkey 实现
- 修复闭包环境生命周期问题
- 添加更多内置函数
- 优化性能

### 2. 实现自己的紧凑 GC
参考 mquickjs 的设计，但用纯 Zig 实现

### 3. 探索其他 Zig 项目
- [zig-js](https://github.com/nickhsmith/zig-js) - Zig 的 JavaScript 绑定
- [bun](https://github.com/oven-sh/bun) - 使用 Zig 的 JS 运行时

---

## 结论

**当前不建议集成 zig-mquickjs**

主要原因：
1. 项目移植不完整，大部分仍是 C 代码
2. 与 star-kirby-lang 的目标（Monkey 解释器）不直接相关
3. 会增加项目复杂性和依赖

**建议**：继续专注于完善 Zig 版 Monkey 解释器，在未来版本中可以考虑参考 mquickjs 的技术来优化内存管理。

---

## 附录: 快速测试命令

如果想单独测试 zig-mquickjs：

```bash
# 克隆仓库
git clone https://github.com/vExcess/zig-mquickjs.git
cd zig-mquickjs

# 构建 (必须使用 ReleaseFast)
zig build -Doptimize=ReleaseFast

# 运行测试
zig build test -Doptimize=ReleaseFast

# 运行 REPL
./zig-out/bin/mqjs
```

---

*文档创建: 2026-01-03*
