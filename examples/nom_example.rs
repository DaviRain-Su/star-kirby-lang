use nom::IResult;
use std::error::Error;

pub fn do_nothing_parser(input: &str) -> IResult<&str, &str> {
    Ok((input, ""))
}

fn main() -> Result<(), Box<dyn Error>> {
    let (remaining_input, output) = do_nothing_parser("my_input")?;
    assert_eq!(remaining_input, "my_input");
    assert_eq!(output, "");

    Ok(())
}
