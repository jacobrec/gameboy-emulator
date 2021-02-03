use crate::instruction::{Instruction, Location, RegisterLoc};

enum Flag {
    Zero, AddSub, HalfCarry, Carry
}

pub struct CPU {
    registers: [u8; 8], // Order: H, L, D, E, B, C, A, F
    sp: u16,
    pc: u16,
    bus: crate::bus::Bus,
    cycles: usize,
}

impl CPU {
    pub const fn new(bus: crate::bus::Bus) -> Self {
        CPU {
            sp: 0,
            pc: 0,
            registers: [0, 0, 0, 0, 0, 0, 0, 0],
            cycles: 0,
            bus
        }
    }

    fn a(&self) -> u8 {self.registers[6]}
    fn b(&self) -> u8 {self.registers[4]}
    fn c(&self) -> u8 {self.registers[5]}
    fn d(&self) -> u8 {self.registers[2]}
    fn e(&self) -> u8 {self.registers[3]}
    fn f(&self) -> u8 {self.registers[7]}
    fn h(&self) -> u8 {self.registers[0]}
    fn l(&self) -> u8 {self.registers[1]}
    fn af(&self) -> u16 {(self.a() << 8) as u16 | self.f() as u16}
    fn bc(&self) -> u16 {(self.b() << 8) as u16 | self.c() as u16}
    fn de(&self) -> u16 {(self.d() << 8) as u16 | self.e() as u16}
    fn hl(&self) -> u16 {(self.h() << 8) as u16 | self.l() as u16}

    fn set_a(&mut self, v: u8) {self.registers[6] = v}
    fn set_b(&mut self, v: u8) {self.registers[4] = v}
    fn set_c(&mut self, v: u8) {self.registers[5] = v}
    fn set_d(&mut self, v: u8) {self.registers[2] = v}
    fn set_e(&mut self, v: u8) {self.registers[3] = v}
    fn set_f(&mut self, v: u8) {self.registers[7] = v}
    fn set_h(&mut self, v: u8) {self.registers[0] = v}
    fn set_l(&mut self, v: u8) {self.registers[1] = v}
    fn set_af(&mut self, v: u16) {self.set_a((v >> 8) as u8); self.set_f((v & 0xFF) as u8)}
    fn set_bc(&mut self, v: u16) {self.set_b((v >> 8) as u8); self.set_c((v & 0xFF) as u8)}
    fn set_de(&mut self, v: u16) {self.set_d((v >> 8) as u8); self.set_e((v & 0xFF) as u8)}
    fn set_hl(&mut self, v: u16) {self.set_h((v >> 8) as u8); self.set_l((v & 0xFF) as u8)}

    pub fn get_flag(&self, f: Flag) -> u8 {
        let bit = match f {
            Flag::Zero => 7,
            Flag::AddSub => 6,
            Flag::HalfCarry => 5,
            Flag::Carry => 4
        };
        (self.f() >> bit) & 1
    }

    pub fn tick(&mut self) {
        let instruction = self.next_op();
        self.execute(instruction);
    }

    fn clock(&mut self) {
        self.cycles += 4; // Each CPU is 4 cycles I belive
    }

    fn next(&mut self) -> u8 {
        let data = self.bus.read(self.pc);
        self.pc += 1;
        self.clock();
        return data
    }
    fn next16(&mut self) -> u16 {
        ((self.next() as u16) << 8) | self.next() as u16
    }
    fn next_op(&mut self) -> Instruction {
        let data = self.next();
        if data == 0x31 { // LD SP n16
            return Instruction::Load(Location::SP, Location::Immediate16(self.next16()))
        }
        panic!("Unimplemented Instruction {}", data)
    }

    fn execute(&mut self, op: Instruction) {
        match op {
            Instruction::Load(dest, src) => (),
            Instruction::XOR(loc) => (),
        }
    }

}
