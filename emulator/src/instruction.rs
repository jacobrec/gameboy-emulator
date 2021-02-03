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

pub enum Location {
    Register(RegisterLoc),
    Immediate(u8),
    Immediate16(u16),
    SP,
}

pub enum Instruction {
    Load (Location, Location), // Dest, Src
    XOR (RegisterLoc)
}

