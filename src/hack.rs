use std::str::FromStr;



#[derive(Debug)]
pub struct C_Command {
    pub dest: Dest,
    pub comp: Comp,
    pub jump: Jump,
}

impl C_Command {
    pub fn new(dest: Dest, comp: Comp, jump: Jump) -> Self {
        Self {
            dest,
            comp,
            jump,
        }
    }
}

#[derive(Debug)]
pub enum Dest {
    Null,
    M,
    D,
    DM,
    A,
    AM,
    AD,
    ADM,
}

impl FromStr for Dest {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Dest::Null),
            "M" => Ok(Dest::M),
            "D" => Ok(Dest::D),
            "DM" => Ok(Dest::DM),
            "A" => Ok(Dest::A),
            "AM" => Ok(Dest::AM),
            "AD" => Ok(Dest::AD),
            "ADM" => Ok(Dest::ADM),
            dest => Err(format!("Error parsing dest: {}", dest)),
        }
    }
}

#[derive(Debug)]
pub enum Jump {
    Null,
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
}

impl FromStr for Jump {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Jump::Null),
            "JGT" => Ok(Jump::JGT),
            "JEQ" => Ok(Jump::JEQ),
            "JGE" => Ok(Jump::JGE),
            "JLT" => Ok(Jump::JLT),
            "JNE" => Ok(Jump::JNE),
            "JLE" => Ok(Jump::JLE),
            "JMP" => Ok(Jump::JMP),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum Comp {
    Zero,
    One,
    MinusOne,
    D,
    A,
    NotD,
    NotA,
    MinusD,
    MinusA,
    Dplus1,
    Aplus1,
    Dminus1,
    Aminus1,
    DplusA,
    DminusA,
    AminusD,
    DandA,
    DorA,
    M,
    NotM,
    Mplus1,
    Mminus1,
    DplusM,
    DminusM,
    MinusM,
    MminusD,
    DandM,
    DorM,
}

impl FromStr for Comp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Comp::Zero),
            "1" => Ok(Comp::One),
            "-1" => Ok(Comp::MinusOne),
            "D" => Ok(Comp::D),
            "A" => Ok(Comp::A),
            "!D" => Ok(Comp::NotD),
            "!A" => Ok(Comp::NotA),
            "-D" => Ok(Comp::MinusD),
            "-A" => Ok(Comp::MinusA),
            "D+1" => Ok(Comp::Dplus1),
            "A+1" => Ok(Comp::Aplus1),
            "D-1" => Ok(Comp::Dminus1),
            "A-1" => Ok(Comp::Aminus1),
            "D+A" => Ok(Comp::DplusA),
            "D-A" => Ok(Comp::DminusA),
            "A-D" => Ok(Comp::AminusD),
            "D&A" => Ok(Comp::DandA),
            "D|A" => Ok(Comp::DorA),
            "M" => Ok(Comp::M),
            "!M" => Ok(Comp::NotM),
            "M+1" => Ok(Comp::Mplus1),
            "M-1" => Ok(Comp::Mminus1),
            "D+M" => Ok(Comp::DplusM),
            "D-M" => Ok(Comp::DminusM),
            "-M" => Ok(Comp::MinusM),
            "M-D" => Ok(Comp::MminusD),
            "D&M" => Ok(Comp::DandM),
            "D|M" => Ok(Comp::DorM),
            _ => Err(()),
        }
    }
}