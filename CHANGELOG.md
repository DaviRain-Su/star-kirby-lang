# Changelog

All notable changes to star-kirby-lang will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

*Last updated: 2026-01-02*