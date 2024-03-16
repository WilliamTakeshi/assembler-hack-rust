use crate::hack::CCommand;
use hack::{Comp, Dest, Jump};
pub use nom::bytes::complete::tag;
use nom::bytes::complete::take_while;
use nom::character::complete::alphanumeric0;
use nom::combinator::opt;
use nom::sequence::tuple;
use nom::IResult;
use std::fs::File;
use std::io::prelude::*;
// use anyhow::Result;
use clap::Parser;

mod hack;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the input file
    #[arg(short, long)]
    input: String,

    /// Number of the output file
    #[arg(short, long)]
    output: String,
}
fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut file = File::open(args.input)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut new_file = File::create(args.output)?;

    let _ = contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .map(|line| translate_line(line))
        .map(|line| new_file.write_all(format!("{}\n", line.unwrap()).as_bytes()))
        .collect::<Vec<_>>();

    Ok(())
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

fn translate_c_command(c: CCommand) -> String {
    let jump = translate_jump(c.jump);
    let comp = translate_comp(c.comp);
    let dest = translate_dest(c.dest);
    format!("111{}{}{}", comp, dest, jump)
}

fn parse_c_command(input: &str) -> IResult<&str, CCommand> {
    // C commands have the form
    // dest=comp;jump

    let (input, (dest_str, _, comp_str, _, jump_str)) = tuple((
        alphanumeric0,
        opt(tag("=")),
        take_while(|c: char| c.is_ascii_uppercase() || c.is_numeric() || c == '!' || c == '+' || c == '-'),
        opt(tag(";")),
        alphanumeric0,
    ))(input)?;

    dbg!(dest_str);
    dbg!(comp_str);
    dbg!(jump_str);

    let comp: Comp = if !comp_str.is_empty() {
        comp_str.parse().unwrap()
    } else {
        dest_str.parse().unwrap()
    };
    let dest: Dest = if !comp_str.is_empty() {
        dest_str.parse().unwrap()
    } else {
        comp_str.parse().unwrap()
    };
    let jump = jump_str.parse().unwrap();

    Ok((
        input,
        CCommand::new(
            dest,
            comp,
            jump,
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_line() {
        let lines = vec![
            ("@2", "0000000000000010"),
            ("@3", "0000000000000011"),
            ("D=A", "1110110000010000"),
            ("D=D+A", "1110000010010000"),
            ("D;JGT", "1110001100000001"),
            ("0;JMP", "1110101010000111"),
        ];

        for line in lines {
            assert_eq!(translate_line(line.0), Ok(line.1.to_string()));
        }
    }
}
