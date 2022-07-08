use crate::lexer::Lexer;
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

    loop {
        let _ = std_out.write_all(PROMPT.as_ref());
        let _ = std_out.flush();

        let mut buffer_reader = String::new();
        let _line = std_buffer_reader.read_line(&mut buffer_reader);

        let lexer = Lexer::new(buffer_reader.as_str());
        let lexer = match lexer {
            Ok(value) => value,
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

        let _ = std_out.write_all(format!("{}\n", program).as_ref());
        let _ = std_out.flush();
    }
}

fn print_parser_error(mut std_out: io::Stdout, error: String) {
    let _ret = std_out.write_all(MONKEY_FACE.as_bytes());
    let _ret = std_out.write_all("Woops! We ran into some monkey business here!\n".as_bytes());
    let _ret = std_out.write_all(" parser errors:\n".as_bytes());
    let _ret = std_out.write_all(format!("\t{}\n", error).as_bytes());
}
