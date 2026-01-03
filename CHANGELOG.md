# Changelog

All notable changes to star-kirby-lang will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.11.0] - Advanced Tools and Optimization (2026-01-03)

### Added - Error Position Display
- Parser errors now show "Parse Error at line X, column Y: message"
- Runtime errors now show "Runtime Error at line X, column Y: message"
- Location tracking for both parsing and evaluation phases

### Added - Debugging Tools
- `--tokens` flag: Print tokenized stream before parsing
- `--ast` flag: Print abstract syntax tree after parsing
- `--debug` flag: Enable debug output (framework in place)
- Enhanced help text with new options

### Fixed - Compilation Issues
- Fixed format string issues for Zig 0.15 compatibility
- Corrected slice formatting in error messages
- Updated Token format function for proper display

### Performance - Benchmark Suite
- Comprehensive benchmark suite for performance testing
- Tests arithmetic, functions, recursion, arrays, hashes, strings, closures
- Measures execution time with warmup and multiple iterations
- Outputs timing statistics (total, average, min, max)

## [0.10.0] - Error Tracking and Module System (2026-01-03)

### Added - Token Position Tracking
- Token structure now includes `line` and `column` fields
- Lexer tracks current line number and column number
- Position information preserved during tokenization

### Added - Command-line Arguments
- `args()` - Returns command-line arguments as array of strings
- Script arguments passed after script name are accessible
- First element is the script name

### Added - Module Import System
- `import(path)` - Imports and executes another Monkey file
- Imported variables and functions become available in current environment
- Supports relative paths
- Circular import detection (already imported files are skipped)

### Technical Details
- Token struct extended with line/column tracking
- Lexer readChar() updates line/column on each character
- Import system uses evalStatementForBuiltin for evaluation
- Variable names duplicated to outlive source file content
- Special handling in evaluator for import (requires environment access)

### Examples
```monkey
// Command-line arguments
// script.monkey:
puts(args());
// Run: zig build run -- script.monkey arg1 arg2
// Output: ["script.monkey", "arg1", "arg2"]

// Module import
// utils.monkey:
let double = fn(x) { x * 2 };
let triple = fn(x) { x * 3 };

// main.monkey:
import("utils.monkey");
puts(double(5));  // 10
puts(triple(5));  // 15
```

---

## [0.9.0] - Developer Experience Enhancement (2026-01-03)

### Added - Random Functions
- `rand()` - Returns a random integer
- `rand(n)` - Returns random integer from 0 to n-1
- `rand(min, max)` - Returns random integer from min to max-1
- `shuffle(array)` - Returns a randomly shuffled copy of the array

### Added - Type Check Functions
- `isInt(value)` - Check if value is an integer
- `isStr(value)` - Check if value is a string
- `isBool(value)` - Check if value is a boolean
- `isArray(value)` - Check if value is an array
- `isHash(value)` - Check if value is a hash
- `isFunc(value)` - Check if value is a function
- `isNull(value)` - Check if value is null

### Added - String Functions
- `startsWith(str, prefix)` - Check if string starts with prefix
- `endsWith(str, suffix)` - Check if string ends with suffix
- `repeat(str, n)` - Repeat string n times
- `padLeft(str, len, char)` - Left pad string to length
- `padRight(str, len, char)` - Right pad string to length

### Added - Math Functions
- `sign(n)` - Returns -1, 0, or 1 based on sign
- `clamp(n, min, max)` - Clamp value to range
- `gcd(a, b)` - Greatest common divisor
- `lcm(a, b)` - Least common multiple
- `avg(array)` - Average of array elements
- `product(array)` - Product of array elements

### Added - Utility Functions
- `assert(condition, message?)` - Assert with optional error message
- `typeof(value)` - Alias for type()
- `default(value, fallback)` - Return fallback if value is null

### Technical Details
- Total of 25 new built-in functions added
- Total built-in count: 70+
- All new functions registered in builtins.zig getBuiltin()
- Random number generation uses std.Random.DefaultPrng with time-based seed

### Examples
```monkey
// Random functions
rand(10);                    // 0-9
shuffle([1, 2, 3, 4, 5]);    // randomly shuffled

// Type checks
isInt(42);                   // true
isStr("hello");              // true
isArray([1, 2, 3]);          // true
isFunc(fn(x) { x });         // true

// String functions
startsWith("hello", "hel");  // true
endsWith("hello", "lo");     // true
repeat("ab", 3);             // "ababab"
padLeft("42", 5, "0");       // "00042"

// Math functions
sign(-5);                    // -1
clamp(15, 0, 10);            // 10
gcd(12, 8);                  // 4
lcm(4, 6);                   // 12
avg([1, 2, 3, 4, 5]);        // 3
product([1, 2, 3, 4]);       // 24

// Utility functions
assert(x > 0, "x must be positive");
typeof(42);                  // "INTEGER"
default(null, "fallback");   // "fallback"
```

---

## [0.8.0] - Standard Library Enhancement (2026-01-03)

### Added - Math Built-in Functions
- `abs(n)` - Returns absolute value of an integer
- `min(a, b)` / `min(array)` - Returns minimum value
- `max(a, b)` / `max(array)` - Returns maximum value
- `pow(base, exp)` - Power operation (base^exp)
- `sqrt(n)` - Integer square root
- `sum(array)` - Sum of all array elements

### Added - Array Operation Built-in Functions
- `reverse(array)` - Returns array in reverse order
- `sort(array)` - Returns sorted array (integers, ascending)
- `find(array, fn)` - Returns first element matching predicate
- `some(array, fn)` - Returns true if any element matches
- `every(array, fn)` - Returns true if all elements match
- `slice(array, start, end)` - Returns array slice
- `concat(array1, array2)` - Concatenates two arrays
- `flatten(array)` - Flattens nested arrays by one level

### Added - System Interaction Built-in Functions
- `getenv(name)` - Gets environment variable value
- `time()` - Returns current Unix timestamp in milliseconds
- `sleep(ms)` - Pauses execution for specified milliseconds

### Added - Type Conversion Built-in Functions
- `bool(value)` - Converts value to boolean (truthy/falsy)
- `array(string)` - Converts string to array of characters

### Added - Interactive REPL
- `zig build run -- --repl` starts interactive mode
- Line-by-line input evaluation
- `exit` or `quit` to exit
- Ctrl+D to exit

### Technical Details
- Total of 19 new built-in functions added
- Total built-in count: 50+
- All new functions registered in builtins.zig getBuiltin()
- Interactive REPL uses POSIX stdin for cross-platform compatibility

### Examples
```monkey
// Math functions
abs(-5);               // 5
min(3, 7);             // 3
max([5, 2, 8, 1]);     // 8
pow(2, 10);            // 1024
sqrt(16);              // 4
sum([1, 2, 3, 4, 5]);  // 15

// Array operations
reverse([1, 2, 3]);              // [3, 2, 1]
sort([3, 1, 4, 1, 5]);           // [1, 1, 3, 4, 5]
find([1, 2, 3, 4], fn(x) { x > 2 }); // 3
some([1, 2, 3], fn(x) { x > 2 }); // true
every([1, 2, 3], fn(x) { x > 0 }); // true
slice([1, 2, 3, 4, 5], 1, 4);    // [2, 3, 4]
concat([1, 2], [3, 4]);          // [1, 2, 3, 4]
flatten([[1, 2], [3, 4]]);       // [1, 2, 3, 4]

// System interaction
getenv("HOME");        // "/home/user"
time();                // 1704268800000

// Type conversion
bool(0);               // false
bool("hello");         // true
array("abc");          // ["a", "b", "c"]
```

---

## [0.7.0] - Tooling and I/O Support (2026-01-03)

### Added - File I/O Built-in Functions
- `readFile(path)` - Read file contents as string
- `writeFile(path, content)` - Write string to file (create/overwrite)
- `appendFile(path, content)` - Append content to file
- `fileExists(path)` - Check if file exists (returns boolean)

### Added - String Manipulation Built-in Functions
- `split(str, delimiter)` - Split string into array
- `join(array, delimiter)` - Join array elements with delimiter
- `trim(str)` - Remove leading/trailing whitespace
- `upper(str)` - Convert to uppercase
- `lower(str)` - Convert to lowercase
- `contains(str, substr)` - Check if string contains substring
- `replace(str, old, new)` - Replace all occurrences
- `charAt(str, index)` - Get character at index
- `substring(str, start, end)` - Get substring
- `indexOf(str, substr)` - Find index of substring (-1 if not found)

### Added - Comment Support
- Single-line comments: `// comment`
- Multi-line comments: `/* comment */`
- Comments are properly skipped by the lexer

### Added - Script File Execution
- Execute `.monkey` or `.mk` files directly: `zig build run -- script.monkey`
- Script errors are properly reported
- Result is displayed (unless null)

### Improved - Array and Hash Output
- Arrays now display as `[1, 2, 3]` instead of `[array]`
- Hashes now display as `{"a": 1, "b": 2}` instead of `{hash}`
- Nested structures display correctly

### Technical Details
- `skipWhitespaceAndComments()` in lexer.zig handles comment parsing
- `executeScript()` in main.zig handles file reading and evaluation
- Object.inspect() properly formats arrays and hashes with element contents
- All new builtins registered in builtins.zig getBuiltin()

### Examples
```monkey
// File I/O
writeFile("test.txt", "Hello!");
let content = readFile("test.txt");
puts(content);  // Hello!

// String operations
split("a,b,c", ",");        // ["a", "b", "c"]
join(["a", "b"], "-");      // "a-b"
trim("  hello  ");          // "hello"
upper("hello");             // "HELLO"
lower("HELLO");             // "hello"
contains("hello", "ell");   // true
replace("hello", "l", "L"); // "heLLo"
charAt("hello", 0);         // "h"
substring("hello", 1, 4);   // "ell"
indexOf("hello", "ll");     // 2

// Comments
// This is a single-line comment
/* This is a
   multi-line comment */

// Improved output
puts([1, 2, 3]);            // [1, 2, 3]
puts({"a": 1});             // {"a": 1}
```

---

## [0.6.0] - Advanced Control Flow and Functional Programming (2026-01-03)

### Added - For Loop
- `for (variable in iterable) { body }` statement support
- Iterate over arrays and range() results
- Proper variable binding in loop scope

### Added - Break and Continue Statements
- `break` - Exit the current loop immediately
- `continue` - Skip to the next iteration
- Works in both `while` and `for` loops
- Proper propagation through nested blocks

### Added - New Built-in Functions
- `range(n)` - Generate array [0, 1, ..., n-1]
- `range(start, end)` - Generate array [start, start+1, ..., end-1]
- `range(start, end, step)` - Generate array with custom step
- `map(array, fn)` - Apply function to each element
- `filter(array, fn)` - Keep elements where fn returns truthy
- `reduce(array, fn, initial)` - Reduce array to single value

### Technical Details
- New Token types: FOR, IN, BREAK, CONTINUE
- New AST nodes: ForStatement, BreakStatement, ContinueStatement
- New Object type: LoopControlObj for break/continue signals
- evalForStatement and evalWhileStatement handle loop control
- applyFunction helper in builtins for map/filter/reduce

### Examples
```monkey
// For loop
let sum = 0;
for (x in [1, 2, 3, 4, 5]) {
    let sum = sum + x;
};
sum  // 15

// Range function
for (i in range(5)) {
    puts(i);
};  // 0 1 2 3 4

// Break statement
let i = 0;
while (true) {
    if (i >= 5) { break; };
    let i = i + 1;
};
i  // 5

// Continue statement
let sum = 0;
for (i in range(10)) {
    if (i % 2 == 0) { continue; };
    let sum = sum + i;
};
sum  // 25 (1+3+5+7+9)

// Map
map([1, 2, 3], fn(x) { x * 2 });  // [2, 4, 6]

// Filter
filter([1, 2, 3, 4, 5], fn(x) { x % 2 == 0 });  // [2, 4]

// Reduce
reduce([1, 2, 3, 4, 5], fn(acc, x) { acc + x }, 0);  // 15
```

---

## [0.5.0] - Language Enhancements (2026-01-03)

### Added - New Operators
- **Comparison Operators**: 
  - `<=` (less than or equal)
  - `>=` (greater than or equal)
- **Arithmetic Operators**:
  - `%` (modulo)
- **Logical Operators** (with short-circuit evaluation):
  - `&&` (logical AND)
  - `||` (logical OR)

### Added - New Built-in Functions
- `print(...)` - Print without newline
- `println(...)` - Print with newline (alias for `puts`)
- `str(value)` - Convert any value to string representation
- `int(string)` - Parse string to integer
- `keys(hash)` - Get array of all keys in a hash
- `values(hash)` - Get array of all values in a hash

### Added - While Loop
- `while (condition) { body }` statement support
- Maximum iteration protection (1,000,000) to prevent infinite loops
- Proper return statement handling inside loops
- Variable updates work correctly in loop body

### Technical Details
- Updated operator precedence: OR < AND < EQUALS < COMPARISON < SUM < PRODUCT
- Short-circuit evaluation: `&&` returns false without evaluating right if left is false
- Short-circuit evaluation: `||` returns true without evaluating right if left is true
- New Token types: LTE, GTE, PERCENT, AND, OR, WHILE
- New AST node: WhileStatement with condition and body

### Examples
```monkey
// New operators
5 <= 5           // true
10 >= 5          // true
10 % 3           // 1
true && false    // false
true || false    // true
1 < 2 && 2 < 3   // true

// New built-ins
str(42)          // "42"
int("123")       // 123

// While loop
let sum = 0;
let i = 0;
while (i < 5) {
    let sum = sum + i;
    let i = i + 1;
};
sum              // 10
```

---

## [0.4.1] - Closure Environment Lifetime Fix (2026-01-03)

### Fixed
- **Closure Environment Lifetime**: Fixed critical bug where returning closures would fail
  - Root cause: `extended_env` in `evalCallExpression` was deallocated via `defer` before 
    the returned closure could use it
  - Solution: Heap-allocate `extended_env` and conditionally release only when result is 
    not a function object
  - Closures now correctly retain their captured environment

### Changed
- **Benchmark Closures**: Updated closure benchmark to use real closure patterns
  - `makeAdder` factory function returning closures
  - `compose` higher-order function combining functions
  - All closure benchmarks now pass

### Verified
- `makeAdder(5)(10)` correctly returns `15`
- `add5(3) + add10(3)` correctly returns `21` (8 + 13)
- `compose(addTen, double)(5)` correctly returns `20`
- All 8 benchmark suites pass with closures working
- All existing tests continue to pass

---

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

### Known Limitations (Fixed in v0.4.1)
- ~~**Closure Environment Lifetime**: Returning closures had environment lifetime issues~~
  - **Fixed in v0.4.1**: Closures now correctly retain their captured environment

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