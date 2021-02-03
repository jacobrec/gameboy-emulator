enum Flag {
    Zero, AddSub, HalfCarry, Carry
}

pub struct CPU {
    registers: [u8; 8], // Order: H, L, D, E, B, C, A, F
    sp: u16,
    pc: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            sp: 0,
            pc: 0,
            registers: [0, 0, 0, 0, 0, 0, 0, 0]
        }
    }
    pub fn a(&self) -> u8 {self.registers[6]}
    pub fn b(&self) -> u8 {self.registers[4]}
    pub fn c(&self) -> u8 {self.registers[5]}
    pub fn d(&self) -> u8 {self.registers[2]}
    pub fn e(&self) -> u8 {self.registers[3]}
    pub fn f(&self) -> u8 {self.registers[7]}
    pub fn h(&self) -> u8 {self.registers[0]}
    pub fn l(&self) -> u8 {self.registers[1]}
    pub fn af(&self) -> u16 {(self.a() << 8) as u16 | self.f() as u16}
    pub fn bc(&self) -> u16 {(self.b() << 8) as u16 | self.c() as u16}
    pub fn de(&self) -> u16 {(self.d() << 8) as u16 | self.e() as u16}
    pub fn hl(&self) -> u16 {(self.h() << 8) as u16 | self.l() as u16}

    pub fn set_a(&mut self, v: u8) {self.registers[6] = v}
    pub fn set_b(&mut self, v: u8) {self.registers[4] = v}
    pub fn set_c(&mut self, v: u8) {self.registers[5] = v}
    pub fn set_d(&mut self, v: u8) {self.registers[2] = v}
    pub fn set_e(&mut self, v: u8) {self.registers[3] = v}
    pub fn set_f(&mut self, v: u8) {self.registers[7] = v}
    pub fn set_h(&mut self, v: u8) {self.registers[0] = v}
    pub fn set_l(&mut self, v: u8) {self.registers[1] = v}
    pub fn set_af(&mut self, v: u16) {self.set_a((v >> 8) as u8); self.set_f((v & 0xFF) as u8)}
    pub fn set_bc(&mut self, v: u16) {self.set_b((v >> 8) as u8); self.set_c((v & 0xFF) as u8)}
    pub fn set_de(&mut self, v: u16) {self.set_d((v >> 8) as u8); self.set_e((v & 0xFF) as u8)}
    pub fn set_hl(&mut self, v: u16) {self.set_h((v >> 8) as u8); self.set_l((v & 0xFF) as u8)}

    pub fn get_flag(&self, f: Flag) -> u8 {
        let bit = match f {
            Flag::Zero => 7,
            Flag::AddSub => 6,
            Flag::HalfCarry => 5,
            Flag::Carry => 4
        };
        (self.f() >> bit) & 1
    }
}
