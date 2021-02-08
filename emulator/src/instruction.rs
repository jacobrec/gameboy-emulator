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

