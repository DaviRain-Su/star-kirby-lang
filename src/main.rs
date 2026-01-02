use star_kirby_lang::repl;
use star_kirby_lang::telemetry;
use std::io;

fn main() -> anyhow::Result<()> {
    let subscriber =
        telemetry::get_subscriber("star-kirby-lang".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber)?;
    println!(
        "Hello {}! This is the Monkey programming language!",
        whoami::username()
    );
    println!("Feel free to type in commands");
    repl::start(io::stdin(), io::stdout())?;

    Ok(())
}
