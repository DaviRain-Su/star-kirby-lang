// Test Monkey Script
let greeting = "Hello from Monkey!";
puts(greeting);

// Test arithmetic
let result = 10 + 20 * 2;
puts(str(result));

// Test array operations
let numbers = [1, 2, 3, 4, 5];
let doubled = map(numbers, fn(x) { x * 2 });
puts(doubled);

// Test string functions
let text = "  Hello World  ";
puts(trim(text));
puts(upper("hello"));
puts(split("a,b,c", ","));

// Final result
reduce([1, 2, 3, 4, 5], fn(acc, x) { acc + x }, 0)
