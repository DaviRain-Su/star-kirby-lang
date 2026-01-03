# Monkey Language Examples

这个目录包含了各种 Monkey 编程语言的示例和测试文件，用于演示语言的特性和调试工具。

## 文件说明

### 基本功能示例
- `test_args.monkey` - 演示命令行参数访问 (`args()` 函数)
- `test_import.monkey` - 演示模块导入功能 (`import()` 函数)
- `test_debug.monkey` - 演示调试输出

### 错误处理示例
- `test_error.monkey` - 基础错误处理测试
- `test_parse_error.monkey` - 解析错误示例
- `test_parse_error2.monkey` - 另一种解析错误
- `test_parse_error3.monkey` - 语法错误示例
- `test_parse_error4.monkey` - 不完整代码错误
- `test_runtime_error.monkey` - 运行时错误示例

### 调试工具示例
- `test_tokens.monkey` - 用于测试 `--tokens` 调试选项

## 运行示例

```bash
# 进入 zig 目录
cd zig

# 运行示例
zig build run -- ../examples/test_args.monkey

# 使用调试选项查看更多信息
zig build run -- --tokens ../examples/test_args.monkey
zig build run -- --ast ../examples/test_args.monkey
```

## 调试选项

- `--tokens`: 显示词法分析结果
- `--ast`: 显示语法树结构
- `--debug`: 启用调试输出

这些示例文件不仅用于测试语言功能，也作为学习 Monkey 语言的参考资料。