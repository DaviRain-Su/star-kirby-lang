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