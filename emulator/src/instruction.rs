use std::fmt::Display;

#[derive(Copy, Clone, Debug)]
pub enum RegisterLoc {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    MemHL
}

#[derive(Copy, Clone, Debug)]
pub enum Location {
    Register(RegisterLoc),
    Immediate(u8),
    Immediate16(u16),
    SP,
}

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    Load (Location, Location), // Dest, Src
    Add (RegisterLoc),
    Adc (RegisterLoc),
    Sub (RegisterLoc),
    Sbc (RegisterLoc),
    And (RegisterLoc),
    Xor (RegisterLoc),
    Or (RegisterLoc),
    Cp (RegisterLoc),
    Rlc (RegisterLoc),
    Rrc (RegisterLoc),
    Rl (RegisterLoc),
    Rr (RegisterLoc),
    Sla (RegisterLoc),
    Sra (RegisterLoc),
    Swap (RegisterLoc),
    Srl (RegisterLoc),
    Bit (u8, RegisterLoc),
    Res (u8, RegisterLoc),
    Set (u8, RegisterLoc),
    Halt,
    Nop
}

impl Display for RegisterLoc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Self::D => "D",
            Self::E => "E",
            Self::H => "H",
            Self::L => "L",
            Self::MemHL => "(HL)",
        };
        write!(f, "{}", s)
    }
}


impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Register(rl) => write!(f, "{}", rl),
            Self::Immediate(b) => write!(f, "${:X}", b),
            Self::Immediate16(b) => write!(f, "${:X}", b),
            Self::SP => write!(f, "SP"),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (op, args) = match self {
            Self::Nop           => ("NOP", String::new()),
            Self::Load(l1, l2)  => ("LOAD", format!(" {},{}", l1, l2)),
            Self::Add(r)        => ("ADD", format!(" {}", r)),
            Self::Adc(r)        => ("ADC", format!(" {}", r)),
            Self::Sub(r)        => ("SUB", format!(" {}", r)),
            Self::Sbc(r)        => ("SBC", format!(" {}", r)),
            Self::And(r)        => ("AND", format!(" {}", r)),
            Self::Xor(r)        => ("XOR", format!(" {}", r)),
            Self::Or(r)         => ("OR", format!(" {}", r)),
            Self::Cp(r)         => ("CP", format!(" {}", r)),
            Self::Rlc(r)        => ("RLC", format!(" {}", r)),
            Self::Rrc(r)        => ("RRC", format!(" {}", r)),
            Self::Rl(r)         => ("RL", format!(" {}", r)),
            Self::Rr(r)         => ("RR", format!(" {}", r)),
            Self::Sla(r)        => ("SLA", format!(" {}", r)),
            Self::Sra(r)        => ("SRA", format!(" {}", r)),
            Self::Swap(r)       => ("SWAP", format!(" {}", r)),
            Self::Srl(r)        => ("SRL", format!(" {}", r)),
            Self::Bit(i, r)     => ("Bit", format!(" {},{}", i, r)),
            Self::Res(i, r)     => ("RES", format!(" {},{}", i, r)),
            Self::Set(i, r)     => ("SET", format!(" {},{}", i, r)),
            Self::Halt  => ("HALT", String::new()),
        };
        write!(f, "{}{}", op, args)
    }
}
