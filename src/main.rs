#[macro_use]
extern crate lazy_static;
extern crate core;

use std::io;

pub mod ast;
pub mod error;
pub mod evaluator;
pub mod lexer;
pub mod object;
pub mod parser;
pub mod repl;
pub mod token;

static NULL: object::null::Null = object::null::Null;
static TRUE: object::boolean::Boolean = object::boolean::Boolean { value: true };
static FALSE: object::boolean::Boolean = object::boolean::Boolean { value: false };

fn main() -> anyhow::Result<()> {
    env_logger::init();
    println!(
        "Hello {}! This is the Monkey programming language!",
        whoami::username()
    );
    println!("Feel free to type in commands");
    repl::start(io::stdin(), io::stdout())?;

    Ok(())
}
