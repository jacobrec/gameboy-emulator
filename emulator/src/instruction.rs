#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub enum Location {
    Register(RegisterLoc),
    Immediate(u8),
    Immediate16(u16),
    SP,
}

#[derive(Copy, Clone)]
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
    Halt,
    Nop
}

