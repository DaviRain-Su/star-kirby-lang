# Monkey interpter

![RUST](https://github.com/substrate-cosmos/monkey-interpter/actions/workflows/rust.yml/badge.svg)

## unsupported feature

- float, 16, 8
- Unicode, UTF-8
- 将在 Monkey 中全面支持 Unicode 和表情符号作为练习留给读者来实现
- 在生产环境中，应该将文件名和行号附加到词法单元中，以
便更好地跟踪可能出现的词法分析错误和语法分析错误。在这种情况下，最好使用
io.Reader 加上文件名来初始化词法分析器。但因为这样做会增加复杂性，所以这里
从简单处着手，仅使用字符串作为输入，忽略文件名和行号。
- 如果要在 Monkey 语言中支持更多的双字符词法单元，则应该使用名为 makeTwoCharToken 的方  法把处理步骤抽象出来。该方法会在找到某些词法单元时继续前看一个字符。对于
- Monkey 来说，目前仅有==和!=这两个双字符词法单元，所以先保持原样。


## PROCESSING

- [x] chapter 1 词法分析
- [x] chapter 2 语法分析
- [ ] chapter 3 求值
- [ ] chapter 4 扩展解释器
- [ ] chapter 5 宏系统
