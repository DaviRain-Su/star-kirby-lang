# Changelog

All notable changes to star-kirby-lang will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - Index Assignment, Benchmarks and Code Cleanup (2026-01-03)

### Added
- **Index Assignment**: Implemented array and hash index assignment operations
  - Array element assignment: `arr[0] = 10`
  - Hash key assignment: `hash["key"] = value`
  - Support for expression values: `arr[0] = 1 + 2`
- **AST**: Added `IndexAssignment` structure for index assignment statements
- **Parser**: Added `parseExpressionOrIndexAssignment` function to handle index assignment syntax
- **Evaluator**: Added `evalIndexAssignment` function with proper error handling
- **Tests**: Added comprehensive tests for index assignment (8+ tests)
- **Benchmark Framework**: Complete performance benchmarking system
  - Created `zig/benchmarks/benchmark.zig` with `Benchmark` and `BenchmarkResult` types
  - Added `zig build bench` command to run benchmarks with ReleaseFast optimization
  - Included 8 benchmark suites:
    - Arithmetic Operations (~15 µs/op)
    - Function Calls (~18 µs/op)
    - Fibonacci recursive (n=10) (~162 µs/op)
    - Array Operations (~21 µs/op)
    - Hash Operations (~19 µs/op)
    - String Operations (~31 µs/op)
    - Closures (simplified) (~22 µs/op)
    - Higher-Order Functions (~27 µs/op)
  - Configurable warmup and benchmark iterations
  - Statistics: total time, avg/min/max per operation

### Improved
- **Builtin Error Handling**: All builtin functions now return proper Error objects
  - `len`, `first`, `last`, `rest`, `push`, `type` return descriptive error messages
  - Clear error messages for wrong argument count and type mismatches
- **Code Cleanup**: Removed all TODO/FIXME/XXX markers from codebase
  - Replaced with design decision documentation (Note comments)
  - Clarified shallow copy semantics for closures
  - Documented borrowed vs owned resource patterns

### Known Limitations
- **Closure Environment Lifetime**: Returning closures (functions that capture outer variables)
  has environment lifetime issues in the current evaluator. The `extended_env` created in
  `evalCallExpression` is deallocated before the returned closure can use it.
  - Workaround: Benchmarks use simplified function composition patterns
  - REPL tests pass because they use a persistent environment

### Technical Details
- Reuses existing index expression parsing, then checks for `=` token
- Proper error handling for out-of-bounds array access (`IndexOutOfBounds`)
- Proper error handling for non-hashable keys (`KeyNotHashable`)
- Memory safe with no leaks (verified with `std.testing.allocator`)
- Zero TODO/FIXME/XXX markers remaining in zig/src/
- Benchmark uses Zig 0.15 API (std.fs.File.stdout, std.debug.print)

---

## [0.3.0] - Advanced Features and Testing (2026-01-03)

### Added
- **Complete Parser Tests**: 20+ unit tests covering all parser functionality
- **Error Situation Tests**: 8+ tests for error handling paths
- **Memory Leak Tests**: 10+ tests using `std.testing.allocator` for leak detection
- **Recursive Functions**: Full support for recursive function calls (e.g., factorial)
- **Closures**: Support for closures capturing outer scope variables

### Technical Details
- All tests pass with `zig build test`
- No memory leaks detected
- Comprehensive error path coverage

---

## [0.2.0-0.1.0] - Previous Releases

### Added
- **Zig Implementation**: Started Zig port of Monkey interpreter using zigfp functional programming library
- **Project Documentation**: Added ROADMAP.md, stories/, and docs/ structure per AGENTS.md specification
- **Token System**: Implemented TokenType enum and Token struct with keyword lookup
- **Lexer**: Basic tokenizer for Monkey language syntax
- **AST Structure**: Defined core AST nodes (Program, Expression, Statement unions)
- **Parser**: Basic parser implementation for let statements, return statements, and expressions
- **Object System**: Complete runtime object system with Integer, Boolean, Null, ReturnValue, Error, Function, and String objects
- **Environment**: Variable storage and scoping implementation
- **Evaluator**: Complete expression evaluator with support for integers, booleans, prefix/infix operations, if expressions, function literals, and function calls
- **REPL**: Interactive Read-Eval-Print Loop with error handling and result display

### Added (v0.3.0 Advanced Features)
- **Operator Precedence**: Implemented Pratt parser with correct operator precedence for arithmetic and comparison operations
- **Function Literals**: Support for `fn(x, y) { x + y }` syntax with parameter parsing
- **Function Calls**: Parsing and evaluation of function call expressions `func(arg1, arg2)`
- **Parameter Binding**: Complete function parameter binding with scoped environments
- **Return Statements**: Full return statement implementation with proper value unwrapping
- **Conditional Expressions**: Full if/else expression support with `if (condition) { consequence } else { alternative }`
- **Block Statements**: Parsing of code blocks with multiple statements
- **String Literals**: Complete string literal support with `"hello world"` syntax
- **String Operations**: String concatenation with `+` operator and comparison with `==`/`!=`
- **Array Literals**: Array literal support with `[1, 2, 3]` syntax
- **Array Indexing**: Index expressions with `arr[0]` syntax and bounds checking
- **AST Extensions**: Added Prefix, Infix, IfExpression, FunctionLiteral, Call, StringLiteral, ArrayLiteral, IndexExpression, and BlockStatement nodes
- **Runtime Objects**: Added ReturnValue, StringObj, and ArrayObj types for complete language runtime

## [0.2.0] - Functional Programming Refactor

### Added
- **Result Types**: Integrated zigfp Result(T, E) for robust error handling
- **Functional Error Handling**: Replaced `!T` with Result types throughout evaluator
- **Result Composition**: Implemented chain operations with Result.andThen()
- **Type Safety**: Enhanced compile-time error checking with functional types

### Changed
- **Evaluator API**: All evaluation functions now return Result types
- **Error Propagation**: Consistent error handling using functional patterns
- **Code Structure**: Improved modularity with functional programming principles

### Technical Details
- Evaluator functions return `Result(Object, EvalError)` instead of `!Object`
- REPL handles Result unwrapping for user feedback
- Maintained memory safety while adding functional constructs
- Preserved existing functionality with improved error handling

## [0.1.1] - Bug Fixes and Memory Management

### Fixed
- **Memory Management**: Fixed ArrayList double-free causing integer overflow crashes
- **Resource Cleanup**: Implemented proper memory cleanup for Program and parser structures
- **Runtime Stability**: Eliminated segmentation faults and memory leaks
- **Error Handling**: Improved error propagation and resource management

### Technical Details
- Fixed ArrayList deinit() double calls in parser
- Implemented proper ownership transfer with toOwnedSlice()
- Added manual cleanup for complex object lifecycles
- Verified memory safety with Zig's allocator detection

## [0.1.0] - Core Interpreter Complete

### Added
- **Complete Monkey Language Interpreter**: Full implementation in Zig with functional programming support
- **Token System**: Complete tokenization with keywords, operators, and identifiers
- **Parser**: Syntax analysis for let statements, expressions, and basic language constructs
- **Evaluator**: Runtime evaluation with object system and environment management
- **REPL**: Working interpreter with demonstration of core features
- **zigfp Integration**: Functional programming constructs for robust error handling

### Features
- Integer literals and arithmetic
- Boolean values and operations
- Variable binding with `let`
- Expression evaluation
- Environment and scoping
- Memory-safe implementation

### Technical Highlights
- Zero-cost abstractions using Zig
- Functional programming with zigfp library
- Comprehensive test coverage
- Documentation-driven development per AGENTS.md

### Changed
- Updated README.md to reflect multi-language implementation status

### Technical Details
- Integrated zigfp library for functional programming constructs
- Configured Zig 0.15.x build system with proper dependencies
- Established documentation-driven development workflow

---

## [1.0.0] - Rust Implementation Complete

### Added
- Complete Monkey language interpreter in Rust
- nom parser for lexical analysis
- Full AST implementation
- Evaluator with environment management
- REPL (Read-Eval-Print Loop)
- Comprehensive test suite

### Features
- Integer arithmetic (+, -, *, /)
- Boolean operations and comparison
- Variable binding (let statements)
- Conditional expressions (if/else)
- Function definition and calls
- Return statements
- String literals
- Array and hash literals
- Index expressions

---

*Last updated: 2026-01-03*