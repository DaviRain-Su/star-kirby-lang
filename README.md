# 🌟Star Kirby

Monkey 编程语言解释器 - 多语言实现

![RUST](https://github.com/substrate-cosmos/monkey-interpter/actions/workflows/rust.yml/badge.svg)

## 实现版本

### 🚀 Zig 实现 (当前主力版本)
**位置**: `zig/` 目录
**状态**: v0.11.0 已完成 - 高级工具和优化
**特性**:
- 零成本抽象，编译时优化
- 内存安全，无垃圾回收
- 函数式编程支持 (zigfp 库)
- 高性能原生代码
- 完整的错误位置跟踪
- 内置调试工具 (--tokens, --ast)
- 性能基准测试套件
- 70+ 内置函数

### 📚 Rust 实现 (参考实现)
**位置**: `src/` 目录
**状态**: 已完成
**特性**:
- nom 解析器库
- 完整的 Monkey 语言支持

## 不支持的功能

- 浮点数、16位和8位整数
- Unicode、UTF-8 字符串
- 宏系统 (计划中)

## Zig 实现进度

- [x] 项目架构搭建 (Zig build, zigfp 集成)
- [x] Token 系统 (TokenType, Token 结构体 + 位置跟踪)
- [x] Lexer 实现 (分词器 + 行号列号跟踪)
- [x] AST 结构定义 (完整的语法树)
- [x] Parser 实现 (语法分析 + 错误位置)
- [x] Object 系统 (运行时值表示)
- [x] Evaluator 实现 (求值器 + 错误位置)
- [x] REPL 实现 (交互式环境)
- [x] 完整测试套件 (38+ 单元测试)
- [x] 错误位置跟踪 (Parser/Evaluator 错误显示行号列号)
- [x] 调试工具 (--tokens, --ast, --debug)
- [x] 性能基准测试套件
- [x] 70+ 内置函数 (数学、字符串、数组、文件操作等)
- [x] 模块系统 (import 函数)
- [x] 高级控制流 (while, for-in, break, continue)

## 快速开始 (Zig 版本)

### 环境要求
- Zig 0.15.x 或更高版本

### 构建和运行
```bash
cd zig
zig build
zig build run
```

### 运行测试
```bash
zig build test
```

### 运行示例
项目包含多个 Monkey 语言示例：

```bash
# 查看所有示例
ls examples/

# 运行特定示例
zig build run -- examples/test_tokens.monkey

# 使用调试选项
zig build run -- --tokens examples/test_args.monkey
```

## 项目结构

```
star-kirby-lang/
├── src/                    # Rust 实现 (参考)
├── zig/                    # Zig 实现 (主力版本)
│   ├── src/               # 源代码
│   ├── benchmarks/        # 性能基准测试
│   └── build.zig          # 构建配置
├── examples/              # Monkey 语言示例和测试
├── stories/               # Story 文件
├── docs/                  # 项目文档
├── ROADMAP.md            # 项目路线图
├── CHANGELOG.md          # 变更日志
└── README.md             # 本文档
```

## 开发规范

本项目遵循严格的文档驱动开发流程：

1. **文档先行**: 功能必须先在 ROADMAP.md 和 Story 文件中规划
2. **代码实现**: 按照 Story 文件的验收标准实现
3. **测试验证**: 所有修改必须通过完整测试
4. **文档收尾**: 更新相关文档和 CHANGELOG

详细规范请参考 `AGENTS.md`。

## 贡献

欢迎贡献！请遵循以下步骤：

1. 查看 ROADMAP.md 了解规划
2. 创建相应的 Story 文件
3. 实现功能并通过测试
4. 更新文档

## 许可证

MIT License
