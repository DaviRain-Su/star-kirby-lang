#[macro_use]
extern crate lazy_static;
extern crate core;

pub mod ast;
pub mod error;
pub mod evaluator;
pub mod lexer;
pub mod object;
pub mod parser;
pub mod repl;
pub mod telemetry;
pub mod token;
