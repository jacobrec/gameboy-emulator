use std::fmt::Display;

#[derive(Copy, Clone, Debug)]
pub enum JmpFlag {
    NoZero,
    Zero,
    Carry,
    NoCarry,
}

#[derive(Copy, Clone, Debug)]
pub enum Jump {
    Relative(i8),
    Absolute(u16),
}

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
pub enum Register16Loc {
    BC,
    DE,
    HL,
    AF,
}

#[derive(Copy, Clone, Debug)]
pub enum Offset {
    HLInc,
    HLDec,
    BC,
    DE,
}

#[derive(Copy, Clone, Debug)]
pub enum Location {
    Register(RegisterLoc),
    Register16(Register16Loc),
    Immediate(u8),
    Immediate16(u16),
    Indirect(Offset),
    SP,
    ZeroPageC,
    ZeroPageAbsolute(u8),
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
    Inc (RegisterLoc),
    Dec (RegisterLoc),
    Bit (u8, RegisterLoc),
    Res (u8, RegisterLoc),
    Set (u8, RegisterLoc),
    Pop (Register16Loc),
    Push (Register16Loc),
    Ret (Option<JmpFlag>),
    Reti,
    Jmp (Jump, JmpFlag),
    Halt,
    Nop
}

impl Display for Jump {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Absolute(u) => write!(f, "${:04X}", u),
            Self::Relative(u) => write!(f, "{}", u),
        }
    }
}

impl Display for JmpFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Self::Carry => "C",
            Self::NoCarry => "NC",
            Self::Zero => "Z",
            Self::NoZero => "NZ",
        };
        write!(f, "{}", s)
    }
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

impl Display for Register16Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Self::BC => "BC",
            Self::DE => "DE",
            Self::HL => "HL",
            Self::AF => "AF",
        };
        write!(f, "{}", s)
    }
}

impl Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::HLInc => write!(f, "HL-"),
            Self::HLDec => write!(f, "HL+"),
            Self::BC => write!(f, "BC"),
            Self::DE => write!(f, "DE"),
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Register(rl) => write!(f, "{}", rl),
            Self::Register16(rl) => write!(f, "{}", rl),
            Self::Immediate(b) => write!(f, "${:X}", b),
            Self::Immediate16(b) => write!(f, "${:X}", b),
            Self::Indirect(offset) => write!(f, "({})", offset),
            Self::SP => write!(f, "SP"),
            Self::ZeroPageC => write!(f, "($FF00+C)"),
            Self::ZeroPageAbsolute(v) => write!(f, "($FF00+${:X})", v),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (op, args) = match self {
            Self::Nop           => ("NOP", String::new()),
            Self::Load(l1, l2)  => ("LD", format!(" {},{}", l1, l2)),
            Self::Add(r)        => ("ADD", format!(" {}", r)),
            Self::Adc(r)        => ("ADC", format!(" {}", r)),
            Self::Sub(r)        => ("SUB", format!(" {}", r)),
            Self::Sbc(r)        => ("SBC", format!(" {}", r)),
            Self::And(r)        => ("AND", format!(" {}", r)),
            Self::Xor(r)        => ("XOR", format!(" {}", r)),
            Self::Or(r)         => ("OR", format!(" {}", r)),
            Self::Inc(r)         => ("INC", format!(" {}", r)),
            Self::Dec(r)         => ("DEC", format!(" {}", r)),
            Self::Cp(r)         => ("CP", format!(" {}", r)),
            Self::Rlc(r)        => ("RLC", format!(" {}", r)),
            Self::Rrc(r)        => ("RRC", format!(" {}", r)),
            Self::Rl(r)         => ("RL", format!(" {}", r)),
            Self::Rr(r)         => ("RR", format!(" {}", r)),
            Self::Sla(r)        => ("SLA", format!(" {}", r)),
            Self::Sra(r)        => ("SRA", format!(" {}", r)),
            Self::Swap(r)       => ("SWAP", format!(" {}", r)),
            Self::Srl(r)        => ("SRL", format!(" {}", r)),
            Self::Bit(i, r)     => ("BIT", format!(" {},{}", i, r)),
            Self::Res(i, r)     => ("RES", format!(" {},{}", i, r)),
            Self::Set(i, r)     => ("SET", format!(" {},{}", i, r)),
            Self::Pop(r)        => ("POP", format!(" {}", r)),
            Self::Push(r)       => ("PUSH", format!(" {}", r)),
            Self::Reti          => ("RETI", String::new()),
            Self::Ret(cc)       => {
                if let Some(flag) = cc {
                    (match flag {
                        JmpFlag::NoZero => "NZ",
                        JmpFlag::Zero   => "Z",
                        JmpFlag::Carry  => "C",
                        JmpFlag::NoCarry=> "NC",
                    }, format!("RET {}", flag))
                }
                else {
                    ("RET", String::new())
                }      
            }
            Self::Jmp(j, f)    => {
                (match j {
                    Jump::Absolute(_) => "JP",
                    Jump::Relative(_) => "JR"
                }, format!(" {},{}", f, j))
            }
            Self::Halt  => ("HALT", String::new()),
        };
        write!(f, "{}{}", op, args)
    }
}
