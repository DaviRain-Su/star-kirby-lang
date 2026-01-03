# ROADMAP - star-kirby-lang-zig 项目路线图

> **Source of Truth**: 本文档是项目的唯一权威路线图，所有功能开发必须以此为基准。

**项目**: star-kirby-lang-zig
**描述**: 使用 Zig 重新实现 Monkey 编程语言解释器
**语言**: 中文
**最后更新**: 2026-01-03

---

## 项目状态概览

### 当前版本: v0.10.0
**状态**: ✅ 已完成
**开始日期**: 2026-01-03
**完成日期**: 2026-01-03

### 版本规划

| 版本 | 状态 | 描述 | 优先级 |
|------|------|------|--------|
| v0.1.0 | ✅ 已完成 | 核心解释器功能 | 高 |
| v0.1.1 | ✅ 已完成 | 内存管理和错误修复 | 高 |
| v0.2.0 | ✅ 已完成 | 函数式编程重构 | 中 |
| v0.3.0 | ✅ 已完成 | 高级语言特性和测试完善 | 中 |
| v0.4.0 | ✅ 已完成 | 索引赋值和工具 | 中 |
| v0.4.1 | ✅ 已完成 | 闭包环境修复 | 高 |
| v0.5.0 | ✅ 已完成 | 语言增强 | 高 |
| v0.6.0 | ✅ 已完成 | 高级控制流和函数式编程 | 高 |
| v0.7.0 | ✅ 已完成 | 工具增强和 I/O 支持 | 中 |
| v0.8.0 | ✅ 已完成 | 标准库增强 | 中 |
| v0.9.0 | ✅ 已完成 | 开发体验增强 | 中 |
| v0.10.0 | ✅ 已完成 | 错误跟踪和模块系统 | 中 |
| v0.11.0 | ⏳ 计划 | 高级工具和优化 | 中 |

---

## v0.1.0 核心解释器功能 ✅

### ✅ 已完成
- [x] 项目基础架构搭建 (Zig build, zigfp 依赖)
- [x] Token 系统实现 (TokenType, Token 结构体)
- [x] Lexer 实现 (基本分词器)
- [x] AST 结构定义 (Program, Expression, Statement)
- [x] 文档结构配置
- [x] Parser 实现 (语法分析器)
- [x] Object 系统 (运行时值表示)
- [x] Evaluator 实现 (表达式求值)
- [x] REPL 实现 (交互式环境)

### 验收标准 ✅
- [x] 所有测试通过 (`zig build test`)
- [x] 支持基本算术表达式
- [x] 支持变量绑定 (let)
- [x] 支持条件表达式 (if/else)
- [x] 支持函数定义和调用
- [x] REPL 可以正常工作
- [x] 无内存泄漏

---

## v0.2.0 高级语言特性 ✅

### ✅ 已完成
- [x] 数组和哈希字面量
- [x] 内置函数 (len, first, last, rest, push, puts, type)
- [x] 字符串操作
- [x] 索引操作 (array[index], hash[key])
- [x] 错误处理机制 (使用 zigfp.Result)

### 验收标准 ✅
- [x] 完整的 Monkey 语言特性支持
- [x] 与原 Rust 实现兼容
- [x] 所有测试通过

---

## v0.3.0 高级特性和测试完善 ✅

### ✅ 已完成
- [x] 运算符优先级完整实现
- [x] 递归函数支持
- [x] 闭包支持
- [x] Arena allocator 内存管理
- [x] 完整的解析器测试 (20+ tests)
- [x] 错误情况测试 (8+ tests)
- [x] 完整的内存泄漏测试 (10+ tests)

### ✅ 在 v0.4.0 中已完成
- [x] 索引赋值操作 (arr[0] = value)

### 验收标准 ✅
- [x] 解析器单元测试覆盖率 > 80%
- [x] 所有错误路径有测试
- [x] 内存泄漏测试通过

---

## v0.4.0 索引赋值和工具 ✅

### ✅ 已完成
- [x] 索引赋值操作 (arr[0] = value, hash["key"] = value)
- [x] 数组元素修改
- [x] 哈希键值对修改
- [x] 完整测试覆盖
- [x] 性能基准测试框架 (zig build bench)

### 验收标准 ✅
- [x] 索引赋值功能完整实现
- [x] 性能基准测试框架

---

## v0.4.1 闭包环境生命周期修复 ✅

### ✅ 已完成
- [x] 修复 `evalCallExpression` 中 extended_env 过早释放问题
- [x] 支持返回闭包 (函数工厂模式)
- [x] 支持函数组合 (compose)
- [x] 更新 benchmark 使用完整闭包测试

### 技术方案
- 将 `extended_env` 从栈分配改为堆分配
- 条件释放: 只在返回值不是函数对象时释放环境
- 闭包保留环境引用，支持后续调用

### 验收标准 ✅
- [x] `makeAdder(5)(10)` = 15
- [x] 多个闭包独立工作
- [x] 函数组合正确执行
- [x] 所有测试通过
- [x] benchmark 闭包测试通过

---

## v0.5.0 语言增强 ✅

### ✅ 额外运算符
- [x] `<=` 小于等于
- [x] `>=` 大于等于
- [x] `%` 取模运算
- [x] `&&` 逻辑与 (短路求值)
- [x] `||` 逻辑或 (短路求值)

### ✅ 新内置函数
- [x] `print(...)` - 打印不带换行
- [x] `println(...)` - 打印带换行 (puts 别名)
- [x] `str(value)` - 转换为字符串
- [x] `int(string)` - 解析字符串为整数
- [x] `keys(hash)` - 获取哈希表所有键
- [x] `values(hash)` - 获取哈希表所有值

### ✅ While 循环
- [x] `while (condition) { body }` 语法支持
- [x] 最大迭代保护 (防止无限循环)

### 验收标准 ✅
- [x] 所有运算符正确实现
- [x] 所有内置函数有测试
- [x] while 循环正确工作
- [x] 所有测试通过

---

## v0.6.0 高级控制流和函数式编程 ✅

### ✅ For 循环
- [x] `for (item in array) { body }` 语法
- [x] 支持遍历数组和 range

### ✅ Break/Continue
- [x] `break` 退出循环
- [x] `continue` 跳过当前迭代
- [x] 在 while 和 for 中都能工作

### ✅ 新内置函数
- [x] `range(n)` / `range(start, end)` / `range(start, end, step)`
- [x] `map(array, fn)` - 映射
- [x] `filter(array, fn)` - 过滤
- [x] `reduce(array, fn, initial)` - 归约

### 验收标准 ✅
- [x] 所有新语法正确实现
- [x] 函数式编程工具可用
- [x] 所有测试通过

---

## v0.7.0 工具增强和 I/O 支持 ✅

### ✅ 文件 I/O 内置函数
- [x] `readFile(path)` - 读取文件内容
- [x] `writeFile(path, content)` - 写入文件
- [x] `appendFile(path, content)` - 追加到文件
- [x] `fileExists(path)` - 检查文件是否存在

### ✅ 字符串操作内置函数
- [x] `split(str, delimiter)` - 分割字符串
- [x] `join(array, delimiter)` - 连接数组元素
- [x] `trim(str)` - 去除首尾空白
- [x] `upper(str)` / `lower(str)` - 大小写转换
- [x] `contains(str, substr)` - 检查包含
- [x] `replace(str, old, new)` - 替换字符串
- [x] `charAt(str, index)` - 获取字符
- [x] `substring(str, start, end)` - 获取子串
- [x] `indexOf(str, substr)` - 查找子串索引

### ✅ 注释支持
- [x] 单行注释 `// comment`
- [x] 多行注释 `/* comment */`

### ✅ 脚本文件执行
- [x] 支持 `.monkey` 和 `.mk` 扩展名
- [x] `zig build run -- script.monkey`

### ✅ 输出优化
- [x] Array 显示 `[1, 2, 3]` 而非 `[array]`
- [x] Hash 显示 `{"a": 1}` 而非 `{hash}`

### 验收标准 ✅
- [x] 所有文件 I/O 函数正确实现
- [x] 所有字符串操作函数正确实现
- [x] 注释被正确跳过
- [x] 脚本执行正常工作
- [x] 所有测试通过
- [x] 所有基准测试通过

---

## v0.8.0 标准库增强 ✅

### ✅ 数学内置函数
- [x] `abs(n)` - 绝对值
- [x] `min(a, b)` / `min(array)` - 最小值
- [x] `max(a, b)` / `max(array)` - 最大值
- [x] `pow(base, exp)` - 幂运算
- [x] `sqrt(n)` - 整数平方根
- [x] `sum(array)` - 数组求和

### ✅ 数组操作扩展
- [x] `reverse(array)` - 反转数组
- [x] `sort(array)` - 排序数组
- [x] `find(array, fn)` - 查找元素
- [x] `some(array, fn)` - 存在匹配
- [x] `every(array, fn)` - 全部匹配
- [x] `slice(array, start, end)` - 数组切片
- [x] `concat(array1, array2)` - 连接数组
- [x] `flatten(array)` - 展平数组

### ✅ 系统交互
- [x] `getenv(name)` - 环境变量
- [x] `time()` - 时间戳
- [x] `sleep(ms)` - 暂停执行

### ✅ 类型转换
- [x] `bool(value)` - 转布尔
- [x] `array(string)` - 字符串转数组

### ✅ 交互式 REPL
- [x] `--repl` 参数启动交互模式
- [x] 支持 exit/quit 退出

### 验收标准 ✅
- [x] 所有新函数正确实现 (19个)
- [x] 所有测试通过
- [x] 所有基准测试通过

---

## v0.9.0 开发体验增强 ✅

### ✅ 随机数函数
- [x] `rand()` - 随机整数
- [x] `rand(n)` - 0 到 n-1
- [x] `rand(min, max)` - min 到 max-1
- [x] `shuffle(array)` - 打乱数组

### ✅ 类型检查函数
- [x] `isInt(value)` - 检查是否为整数
- [x] `isStr(value)` - 检查是否为字符串
- [x] `isBool(value)` - 检查是否为布尔值
- [x] `isArray(value)` - 检查是否为数组
- [x] `isHash(value)` - 检查是否为哈希
- [x] `isFunc(value)` - 检查是否为函数
- [x] `isNull(value)` - 检查是否为 null

### ✅ 更多字符串函数
- [x] `startsWith(str, prefix)` - 检查前缀
- [x] `endsWith(str, suffix)` - 检查后缀
- [x] `repeat(str, n)` - 重复字符串 n 次
- [x] `padLeft(str, len, char)` - 左填充
- [x] `padRight(str, len, char)` - 右填充

### ✅ 更多数学函数
- [x] `sign(n)` - 返回符号 (-1, 0, 1)
- [x] `clamp(n, min, max)` - 限制范围
- [x] `gcd(a, b)` - 最大公约数
- [x] `lcm(a, b)` - 最小公倍数
- [x] `avg(array)` - 平均值
- [x] `product(array)` - 乘积

### ✅ 实用函数
- [x] `assert(condition, message?)` - 断言
- [x] `typeof(value)` - type() 别名
- [x] `default(value, fallback)` - 默认值

### 验收标准 ✅
- [x] 所有新函数正确实现 (25个)
- [x] 所有测试通过
- [x] 所有基准测试通过

---

## v0.10.0 错误跟踪和模块系统 ✅

### ✅ 错误位置跟踪
- [x] Token 结构增加 line 和 column 字段
- [x] Lexer 跟踪当前行号和列号
- [ ] Parser 错误包含位置信息（待后续版本）
- [ ] Evaluator 错误包含位置信息（待后续版本）

### ✅ 命令行参数
- [x] `args()` - 返回命令行参数数组
- [x] 脚本文件后的参数传递给脚本

### ✅ 模块导入
- [x] `import(path)` - 导入并执行另一个 Monkey 文件
- [x] 导入的变量/函数在当前环境可用
- [x] 支持相对路径
- [x] 循环导入检测

### 验收标准 ✅
- [x] 所有新功能正确实现
- [x] 所有测试通过
- [x] 所有基准测试通过

---

## v0.11.0 高级工具和优化 ⏳

### ⏳ 计划功能
- [ ] Parser/Evaluator 错误显示行号列号
- [ ] JIT 编译 (可选)
- [ ] 调试工具
- [ ] 代码格式化
- [ ] 语言服务器协议 (LSP) 支持

---

## 开发原则

### 文档驱动开发
1. **文档先行**: 所有功能必须先在 ROADMAP.md 和 Story 文件中规划
2. **Story 同步**: 代码实现必须与 Story 文件保持同步
3. **测试验证**: 所有代码修改必须通过完整测试套件

### 质量要求
- **零内存泄漏**: 使用 Zig 的编译时内存安全特性
- **类型安全**: 充分利用 Zig 的类型系统
- **性能优化**: 利用 zigfp 函数式编程库的优势
- **错误处理**: 使用 Result 类型进行健壮的错误处理

### 技术栈
- **语言**: Zig 0.15.x
- **函数式编程**: zigfp 库
- **构建工具**: Zig Build System
- **测试框架**: Zig 内置测试
- **文档**: Markdown (中文)

---

## 风险评估

### 技术风险
- **中**: Zig 语言相对较新，生态系统尚在发展
- **低**: zigfp 库提供了稳定的函数式编程基础
- **低**: 参照现有的 Rust 实现，功能明确

### 时间风险
- **中**: 需要深入理解 Monkey 语言规范
- **低**: 模块化设计，便于并行开发

### 质量风险
- **低**: Zig 的内存安全特性提供保障
- **中**: 函数式编程范式的学习曲线

---

## 里程碑

### M1: 基础架构 (已完成)
**日期**: 2026-01-02
- ✅ Zig 项目搭建
- ✅ zigfp 集成
- ✅ 基础文档结构

### M2: 核心解释器 (已完成)
**完成日期**: 2026-01-03
- ✅ Parser 实现
- ✅ Evaluator 实现
- ✅ REPL 实现

### M3: 高级特性和测试完善 (已完成)
**完成日期**: 2026-01-03
- ✅ 完整测试套件 (38+ tests)
- ✅ 解析器测试
- ✅ 错误处理测试
- ✅ 内存泄漏测试

### M4: 工具增强和 I/O (已完成)
**完成日期**: 2026-01-03
- ✅ 文件 I/O 内置函数
- ✅ 字符串操作内置函数
- ✅ 注释支持
- ✅ 脚本文件执行
- ✅ 输出优化

### M5: 标准库增强 (已完成)
**完成日期**: 2026-01-03
- ✅ 数学内置函数 (abs, min, max, pow, sqrt, sum)
- ✅ 数组操作扩展 (reverse, sort, find, some, every, slice, concat, flatten)
- ✅ 系统交互 (getenv, time, sleep)
- ✅ 类型转换 (bool, array)
- ✅ 交互式 REPL

### M6: 开发体验增强 (已完成)
**完成日期**: 2026-01-03
- ✅ 随机数函数 (rand, shuffle)
- ✅ 类型检查函数 (isInt, isStr, isBool, isArray, isHash, isFunc, isNull)
- ✅ 更多字符串函数 (startsWith, endsWith, repeat, padLeft, padRight)
- ✅ 更多数学函数 (sign, clamp, gcd, lcm, avg, product)
- ✅ 实用函数 (assert, typeof, default)

### M7: 错误跟踪和模块系统 (已完成)
**完成日期**: 2026-01-03
- ✅ Token 位置跟踪 (line/column)
- ✅ Lexer 行号列号跟踪
- ✅ 命令行参数 (args)
- ✅ 模块导入 (import)
- ✅ 循环导入检测

### M8: 高级工具和优化 (计划)
**目标日期**: 2026-01-15
- ⏳ Parser/Evaluator 错误位置
- ⏳ 性能优化
- ⏳ 开发工具

---

## 依赖关系

```
v0.1.0 (已完成)
├── zigfp (函数式编程库)
├── std (Zig 标准库)
└── 原 Rust 实现 (参考)

v0.2.0 (已完成, 依赖 v0.1.0)
└── 高级语言特性

v0.3.0 (已完成, 依赖 v0.2.0)
└── 测试完善和高级特性

v0.4.0 (已完成, 依赖 v0.3.0)
└── 索引赋值和工具

v0.5.0 (已完成, 依赖 v0.4.0)
└── 语言增强 (运算符、内置函数、while 循环)

v0.6.0 (已完成, 依赖 v0.5.0)
└── 高级控制流和函数式编程

v0.7.0 (已完成, 依赖 v0.6.0)
└── 工具增强和 I/O 支持

v0.8.0 (已完成, 依赖 v0.7.0)
└── 标准库增强

v0.9.0 (已完成, 依赖 v0.8.0)
└── 开发体验增强

v0.10.0 (已完成, 依赖 v0.9.0)
└── 错误跟踪和模块系统

v0.11.0 (计划, 依赖 v0.10.0)
└── 高级工具和优化
```

---

## 贡献指南

### 开发流程
1. 在 ROADMAP.md 中规划功能
2. 创建对应的 Story 文件
3. 实现代码并同步更新文档
4. 通过所有测试
5. 更新 CHANGELOG.md

### 代码规范
- 遵循 Zig 官方编码规范
- 使用 zigfp 的函数式编程模式
- 所有公共 API 必须有文档注释
- 强制内存安全 (defer/errdefer)

### 测试要求
- 单元测试覆盖率 > 80%
- 集成测试验证端到端功能
- 性能回归测试
- 内存泄漏检测

---

*最后更新: 2026-01-03 by AI Assistant*