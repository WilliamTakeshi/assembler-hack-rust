use crate::hack::CCommand;
use anyhow::Context;
use clap::Parser;
use hack::{Comp, Dest, Jump};
pub use nom::bytes::complete::tag;
use nom::bytes::complete::take_while;
use nom::character::complete::{alphanumeric0, char};
use nom::combinator::opt;
use nom::sequence::{delimited, tuple};
use nom::IResult;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

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

    let mut line_num: u32 = 0;
    let mut mem_num: u32 = 15;
    let mut hm: HashMap<String, u32> = HashMap::from([
        ("R0".to_string(), 0),
        ("R1".to_string(), 1),
        ("R2".to_string(), 2),
        ("R3".to_string(), 3),
        ("R4".to_string(), 4),
        ("R5".to_string(), 5),
        ("R6".to_string(), 6),
        ("R7".to_string(), 7),
        ("R8".to_string(), 8),
        ("R9".to_string(), 9),
        ("R10".to_string(), 10),
        ("R11".to_string(), 11),
        ("R12".to_string(), 12),
        ("R13".to_string(), 13),
        ("R14".to_string(), 14),
        ("R15".to_string(), 15),
        ("SCREEN".to_string(), 16384),
        ("KBD".to_string(), 24576),
        ("SP".to_string(), 0),
        ("LCL".to_string(), 1),
        ("ARG".to_string(), 2),
        ("THIS".to_string(), 3),
        ("THAT".to_string(), 4),
    ]);

    // First pass (parsing the lines where the "GOTO" points)
    let _ = contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .map(|line| first_pass(line, &mut line_num, &mut hm))
        .collect::<Vec<_>>();

    // Second pass (parsing variable and mapping to memory address)
    let _ = contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("//") && !line.starts_with('('))
        .map(|line| second_pass(line, &mut mem_num, &mut hm))
        .collect::<Vec<_>>();

    println!("{:?}", hm);

    // Last pass
    let _ = contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("//") && !line.starts_with('('))
        .map(|line| translate_line(line, &hm))
        .map(|line| new_file.write_all(format!("{}\n", line.unwrap()).as_bytes()))
        .collect::<Vec<_>>();

    Ok(())
}

fn parens_parser(input: &str) -> IResult<&str, &str> {
    delimited(
        char('('),
        take_while(|c: char| c.is_alphanumeric() || c == '_' || c == '.' || c == '$'),
        char(')'),
    )(input)
}

fn a_command_with_variable_parser(input: &str) -> IResult<&str, &str> {
    let (leftover_input, _) = tag("@")(input)?;

    let (leftover_input, label) =
        take_while(|c: char| c.is_alphanumeric() || c == '_' || c == '.' || c == '$')(
            leftover_input,
        )?;

    Ok((leftover_input, label))
}

fn first_pass(
    input: &str,
    line_num: &mut u32,
    hm: &mut HashMap<String, u32>,
) -> anyhow::Result<()> {
    match input.chars().next() {
        Some('(') => {
            let (_leftover_input, label) = parens_parser(input)
                .map_err(|e| e.to_owned())
                .context("Failed to parse label inside parenthesis")?;
            hm.entry(label.to_string()).or_insert(*line_num);
            Ok(())
        }
        _ => {
            *line_num += 1;
            Ok(())
        }
    }
}

fn second_pass(
    input: &str,
    mem_num: &mut u32,
    hm: &mut HashMap<String, u32>,
) -> anyhow::Result<()> {
    match input.chars().next() {
        Some('@') => {
            let (_leftover_input, label) = a_command_with_variable_parser(input)
                .map_err(|e| e.to_owned())
                .context("Failed to parse ")?;

            if label.parse::<u32>().is_ok() {
                Ok(()) // Do nothing in case of a number
            } else {
                // println!("variable: {}", label);
                // println!("mem_num: {}", mem_num);
                if hm.get(label).is_none() {
                    *mem_num += 1;
                    hm.insert(label.to_string(), *mem_num);
                }
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

fn translate_line<'a>(input: &'a str, hm: &HashMap<String, u32>) -> Result<String, &'a str> {
    // println!("input: {}", input);
    let translated_line: IResult<&str, String> = match input.chars().next() {
        Some('@') => translate_a_command(input, hm),
        _ => {
            let (_, c_command) = parse_c_command(input).unwrap();

            Ok(("", translate_c_command(c_command)))
        }
    };

    let Ok((_leftover, result)) = translated_line else {
        return Err("Failed to parse input");
    };

    Ok(result)
}

fn translate_a_command<'a>(input: &'a str, hm: &HashMap<String, u32>) -> IResult<&'a str, String> {
    let (leftover_input, _) = tag("@")(input)?;

    if let Some(num) = hm.get(leftover_input) {
        Ok(("", format!("{:016b}", num)))
    } else {
        let my_int = leftover_input.parse::<i32>().unwrap();
        Ok(("", format!("{:016b}", my_int)))
    }
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
        take_while(|c: char| {
            c.is_ascii_uppercase()
                || c.is_numeric()
                || c == '!'
                || c == '+'
                || c == '-'
                || c == '&'
                || c == '|'
        }),
        opt(tag(";")),
        alphanumeric0,
    ))(input)?;

    // dbg!(dest_str);
    // dbg!(comp_str);
    // dbg!(jump_str);

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

    Ok((input, CCommand::new(dest, comp, jump)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_line() {
        let lines = vec![
            // ("@2", "0000000000000010"),
            // ("@3", "0000000000000011"),
            // ("D=A", "1110110000010000"),
            // ("D=D+A", "1110000010010000"),
            // ("D;JGT", "1110001100000001"),
            // ("0;JMP", "1110101010000111"),
            ("M=D&M", "1111000000001000"),
        ];

        for line in lines {
            assert_eq!(
                translate_line(line.0, &HashMap::new()),
                Ok(line.1.to_string())
            );
        }
    }
}
