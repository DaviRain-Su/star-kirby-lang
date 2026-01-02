# 🌟Star Kirby

Monkey 编程语言解释器 - 多语言实现

![RUST](https://github.com/substrate-cosmos/monkey-interpter/actions/workflows/rust.yml/badge.svg)

## 实现版本

### 🚀 Zig 实现 (当前开发中)
**位置**: `zig/` 目录
**状态**: v0.1.0-alpha 开发中
**特性**:
- 零成本抽象，编译时优化
- 内存安全，无垃圾回收
- 函数式编程支持 (zigfp 库)
- 高性能原生代码

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
- [x] Token 系统 (TokenType, Token 结构体)
- [x] Lexer 实现 (分词器)
- [x] AST 结构定义
- [ ] Parser 实现 (语法分析)
- [ ] Object 系统 (运行时值)
- [ ] Evaluator 实现 (求值器)
- [ ] REPL 实现 (交互式环境)
- [ ] 完整测试套件

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

## 项目结构

```
star-kirby-lang/
├── src/                    # Rust 实现
├── zig/                    # Zig 实现
│   ├── src/               # 源代码
│   ├── docs/              # 文档
│   └── examples/          # 示例
├── stories/               # Story 文件
├── ROADMAP.md            # 项目路线图
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
