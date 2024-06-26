use crate::ast::Node;
use crate::lexer::lexer;
use crate::object::environment::Environment;
use crate::object::ObjectInterface;
use crate::parser::Parser;
use std::io;
use std::io::BufRead;
use std::io::Write;

const PROMPT: &str = ">> ";

const MONKEY_FACE: &str = r#"            __,__
.--.  .-"     "-.  .--.
/ .. \/  .-. .-.  \/ .. \
| |  '|  /   Y   \  |'  | |
| \   \  \ 0 | 0 /  /   / |
\ '- ,\.-"""""""-./, -' /
   ''-' /_   ^ ^   _\ '-''
       |  \._   _./  |
       \   \ '~' /   /
        '._ '-=-' _.'
           '-----'
"#;

pub fn start(std_in: io::Stdin, mut std_out: io::Stdout) -> anyhow::Result<()> {
    let mut std_buffer_reader = io::BufReader::new(std_in);
    let mut env = Environment::new();

    loop {
        let _ = std_out.write_all(PROMPT.as_ref());
        let _ = std_out.flush();

        let mut buffer_reader = String::new();
        let _line = std_buffer_reader.read_line(&mut buffer_reader);

        let lexer = lexer(buffer_reader.as_str());
        let lexer = match lexer {
            Ok((_, value)) => value,
            Err(error) => {
                print_parser_error(io::stdout(), error.to_string());
                continue;
            }
        };

        let parser = Parser::new(lexer);
        let mut parser = match parser {
            Ok(value) => value,
            Err(error) => {
                print_parser_error(io::stdout(), error.to_string());
                continue;
            }
        };

        let program = parser.parse_program();
        let program = match program {
            Ok(value) => value,
            Err(error) => {
                print_parser_error(io::stdout(), error.to_string());
                continue;
            }
        };

        let program_node: Node = program.into();
        let evaluated = program_node.eval(&mut env);
        match evaluated {
            Ok(value) => {
                let value = value.inspect();
                let _ = std_out.write_all(format!("{value}\n").as_ref());
                let _ = std_out.flush();
            }
            Err(error) => {
                tracing::error!("{error}");
                continue;
            }
        }
    }
}

fn print_parser_error(mut std_out: io::Stdout, error: String) {
    let _ret = std_out.write_all(MONKEY_FACE.as_bytes());
    let _ret = std_out.write_all("Woops! We ran into some monkey business here!\n".as_bytes());
    let _ret = std_out.write_all(" parser errors:\n".as_bytes());
    let _ret = std_out.write_all(format!("\t{error}\n").as_bytes());
}
