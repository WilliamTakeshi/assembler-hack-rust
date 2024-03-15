use crate::hack::C_Command;
pub use nom::bytes::complete::tag;
use nom::bytes::complete::{take_till, take_while, take_while1};
use nom::character::complete::{alphanumeric0, alphanumeric1, multispace0};
use nom::combinator::opt;
use nom::IResult;

mod hack;
fn main() {
    println!("Hello, world!");
}

fn translate_line(input: &str) -> Result<String, &str> {
    println!("input: {}", input);

    let foo: IResult<&str, String> = match input.chars().next() {
        Some('@') => translate_a_command(input),
        _ => {
            let (_, _c_command) = parse_c_command(input).unwrap();
            dbg!(_c_command);
            Ok(("", "001".to_owned()))
        }
    };

    let Ok((_leftover, result)) = foo else {
        return Err("Failed to parse input");
    };

    Ok(result)
}

fn translate_a_command(input: &str) -> IResult<&str, String> {
    let (leftover_input, _) = tag("@")(input)?;

    let my_int = leftover_input.parse::<i32>().unwrap();

    Ok(("", format!("{:016b}", my_int)))
}

fn translate_c_command(c: C_Command) -> Result<String, String> {

}

fn parse_c_command(input: &str) -> IResult<&str, C_Command> {
    // C commands have the form
    // dest=comp;jump
    let (input, _) = multispace0(input)?; // ignore leading whitespace
    let (input, dest) = alphanumeric0(input)?; // dest (optional)
    let (input, _) = multispace0(input)?; // ignore whitespace
    let (input, _) = opt(tag("="))(input)?;
    let (input, _) = multispace0(input)?; // ignore whitespace
    let (input, comp) =
        take_while1(|c: char| c.is_ascii_uppercase() || c == '!' || c == '+' || c == '-')(input)?; // comp
    let (input, _) = opt(tag(";"))(input)?;
    let (input, _) = multispace0(input)?;
    let (_, jump) = alphanumeric0(input)?; // Jump (optional)

    dbg!(dest);
    dbg!(comp);
    dbg!(jump);

    Ok((
        "",
        C_Command::new(
            dest.parse().unwrap(),
            comp.parse().unwrap(),
            jump.parse().unwrap(),
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_line1() {
        assert_eq!(translate_line("@2"), Ok("0000000000000010".to_string()));
    }

    #[test]
    fn test_translate_line2() {
        assert_eq!(translate_line("@3"), Ok("0000000000000011".to_string()));
    }

    // #[test]
    // fn test_translate_line3() {
    //     assert_eq!(translate_line("D=A"), Ok("1110110000010000".to_string()));
    // }

    #[test]
    fn test_translate_line4() {
        assert_eq!(translate_line("D=D+A"), Ok("1110110000010000".to_string()));
    }

    // #[test]
    // fn test_translate_line4() {
    //     assert_eq!(translate_line("D=D+A"), Ok("111000010010000".to_string()));
    // }
}
