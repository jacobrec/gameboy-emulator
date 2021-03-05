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
    SP
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
    IndirectLiteral(u16),
    SP,
    SPOffset(i8),
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
    AddImm (u8),
    AdcImm (u8),
    SubImm (u8),
    SbcImm (u8),
    AndImm (u8),
    XorImm (u8),
    OrImm (u8),
    CpImm (u8),
    AddSp (i8),
    Rlc (RegisterLoc),
    Rrc (RegisterLoc),
    Rl (RegisterLoc),
    Rla,
    Rra,
    Rlca,
    Rrca,
    Cpl,
    Ccf,
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
    Call (Option<JmpFlag>, u16),
    Ret (Option<JmpFlag>),
    Inc16(Register16Loc),
    Dec16(Register16Loc),
    AddHL16(Register16Loc),
    Reti,
    Rst(u8),
    Jmp (Jump, Option<JmpFlag>),
    EI,
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
            Self::SP => "SP",
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
            Self::IndirectLiteral(offset) => write!(f, "({})", offset),
            Self::SP => write!(f, "SP"),
            Self::SPOffset(e8) => write!(f, "(SP+{:X})", e8),
            Self::ZeroPageC => write!(f, "($FF00+C)"),
            Self::ZeroPageAbsolute(v) => write!(f, "($FF00+${:X})", v),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (op, args) = match self {
            Self::Nop           => ("NOP", String::new()),
            Self::EI            => ("EI", String::new()),
            Self::Load(l1, l2)  => ("LD", format!(" {},{}", l1, l2)),
            Self::Add(r)        => ("ADD", format!(" {}", r)),
            Self::Adc(r)        => ("ADC", format!(" {}", r)),
            Self::Sub(r)        => ("SUB", format!(" {}", r)),
            Self::Sbc(r)        => ("SBC", format!(" {}", r)),
            Self::And(r)        => ("AND", format!(" {}", r)),
            Self::Xor(r)        => ("XOR", format!(" {}", r)),
            Self::Or(r)         => ("OR", format!(" {}", r)),
            Self::AddImm(i)     => ("ADD", format!(" {}", i)),
            Self::AdcImm(i)     => ("ADC", format!(" {}", i)),
            Self::SubImm(i)     => ("SUB", format!(" {}", i)),
            Self::SbcImm(i)     => ("SBC", format!(" {}", i)),
            Self::AndImm(i)     => ("AND", format!(" {}", i)),
            Self::XorImm(i)     => ("XOR", format!(" {}", i)),
            Self::OrImm(i)      => ("OR", format!(" {}", i)),
            Self::AddSp(i)      => ("ADD SP", format!(" {}", i)),
            Self::Inc(r)        => ("INC", format!(" {}", r)),
            Self::Dec(r)        => ("DEC", format!(" {}", r)),
            Self::Cp(r)         => ("CP", format!(" {}", r)),
            Self::CpImm(i)      => ("CP", format!(" {}", i)),
            Self::Rlc(r)        => ("RLC", format!(" {}", r)),
            Self::Rrc(r)        => ("RRC", format!(" {}", r)),
            Self::Rl(r)         => ("RL", format!(" {}", r)),
            Self::Rla           => ("RLA", String::new()),
            Self::Rra           => ("RRA", String::new()),
            Self::Rlca          => ("RLCA", String::new()),
            Self::Rrca          => ("RRCA", String::new()),
            Self::Cpl           => ("CPL", String::new()),
            Self::Ccf           => ("CCF", String::new()),
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
            Self::Inc16(r)      => ("INC", format!(" {}", r)),
            Self::Dec16(r)      => ("DEC", format!(" {}", r)),
            Self::AddHL16(r)    => ("ADD", format!(" HL, {}", r)),
            Self::Reti          => ("RETI", String::new()),
            Self::Rst(n)        => ("RST", format!(" {}", n)),
            Self::Call(cc, l)   => {
                match cc {
                    Some(flag) => ("Call", format!(" {}, ${:04X}", flag, l)),
                    None => ("CALL", format!(" ${:04X}", l))
                }
            }

            Self::Ret(cc)       => {
                match cc {
                    Some(flag) => ("RET", format!("{}", flag)),
                    None => ("RET", String::new()),
                }
            }
            Self::Jmp(j, f)    => {
                match j {
                    Jump::Absolute(_) => {
                        match f {
                            Some(flag) => ("JP", format!("{}", flag)),
                            None => ("JP", String::new())
                        }
                    },
                    Jump::Relative(_) => {
                        match f {
                            Some(flag) => ("JR", format!(" {}", flag)),
                            None => ("JR", String::new())
                        }
                    }
                }
            }
            Self::Halt  => ("HALT", String::new()),
        };
        write!(f, "{}{}", op, args)
    }
}
