use crate::hack::C_Command;
use hack::{Comp, Dest, Jump};
pub use nom::bytes::complete::tag;
use nom::bytes::complete::take_while1;
use nom::character::complete::{alphanumeric0, multispace0};
use nom::combinator::opt;
use nom::IResult;
// use anyhow::Result;

mod hack;
fn main() {
    println!("Hello, world!");
}

fn translate_line(input: &str) -> Result<String, &str> {
    println!("input: {}", input);

    let foo: IResult<&str, String> = match input.chars().next() {
        Some('@') => translate_a_command(input),
        _ => {
            let (_, c_command) = parse_c_command(input).unwrap();

            Ok(("", translate_c_command(c_command)))
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

fn translate_comp(comp: Comp) -> String {
    match comp {
        Comp::Zero => "0101010".to_owned(),
        Comp::One => "0111111".to_owned(),
        Comp::MinusOne => "0111010".to_owned(),
        Comp::D => "0001100".to_owned(),
        Comp::A => "0110000".to_owned(),
        Comp::NotD => "0001101".to_owned(),
        Comp::NotA => "0110001".to_owned(),
        Comp::MinusD => "0001111".to_owned(),
        Comp::MinusA => "0110011".to_owned(),
        Comp::Dplus1 => "0011111".to_owned(),
        Comp::Aplus1 => "0110111".to_owned(),
        Comp::Dminus1 => "0001110".to_owned(),
        Comp::Aminus1 => "0110010".to_owned(),
        Comp::DplusA => "0000010".to_owned(),
        Comp::DminusA => "0010011".to_owned(),
        Comp::AminusD => "0000111".to_owned(),
        Comp::DandA => "0000000".to_owned(),
        Comp::DorA => "0010101".to_owned(),
        Comp::M => "1110000".to_owned(),
        Comp::NotM => "1110001".to_owned(),
        Comp::MinusM => "1110011".to_owned(),
        Comp::Mplus1 => "1110111".to_owned(),
        Comp::Mminus1 => "1110010".to_owned(),
        Comp::DplusM => "1000010".to_owned(),
        Comp::DminusM => "1010011".to_owned(),
        Comp::MminusD => "1000111".to_owned(),
        Comp::DandM => "1000000".to_owned(),
        Comp::DorM => "1010101".to_owned(),
    }
}

fn translate_dest(dest: Dest) -> String {
    match dest {
        Dest::Null => "000".to_owned(),
        Dest::M => "001".to_owned(),
        Dest::D => "010".to_owned(),
        Dest::DM => "011".to_owned(),
        Dest::A => "100".to_owned(),
        Dest::AM => "101".to_owned(),
        Dest::AD => "110".to_owned(),
        Dest::ADM => "111".to_owned(),
    }
}

fn translate_jump(jump: Jump) -> String {
    match jump {
        Jump::Null => "000".to_owned(),
        Jump::JGT => "001".to_owned(),
        Jump::JEQ => "010".to_owned(),
        Jump::JGE => "011".to_owned(),
        Jump::JLT => "100".to_owned(),
        Jump::JNE => "101".to_owned(),
        Jump::JLE => "110".to_owned(),
        Jump::JMP => "111".to_owned(),
    }
}

fn translate_c_command(c: C_Command) -> String {
    let jump = translate_jump(c.jump);
    let comp = translate_comp(c.comp);
    let dest = translate_dest(c.dest);
    format!("111{}{}{}", comp, dest, jump)
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

    #[test]
    fn test_translate_line3() {
        assert_eq!(translate_line("D=A"), Ok("1110110000010000".to_string()));
    }

    #[test]
    fn test_translate_line4() {
        assert_eq!(translate_line("D=D+A"), Ok("1110000010010000".to_string()));
    }

    // #[test]
    // fn test_translate_line4() {
    //     assert_eq!(translate_line("D=D+A"), Ok("111000010010000".to_string()));
    // }
}
