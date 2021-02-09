use crate::instruction::{Instruction, Location, Register16Loc, RegisterLoc};

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
    pub fn get_screen(&self) -> crate::ppu::Canvas {
        return self.bus.get_screen()
    }

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
    fn af(&self) -> u16 {((self.a() as u16) << 8) | self.f() as u16}
    fn bc(&self) -> u16 {((self.b() as u16) << 8) | self.c() as u16}
    fn de(&self) -> u16 {((self.d() as u16) << 8) | self.e() as u16}
    fn hl(&self) -> u16 {((self.h() as u16) << 8) | self.l() as u16}

    fn set_a(&mut self, v: u8) {self.registers[6] = v}
    fn set_b(&mut self, v: u8) {self.registers[4] = v}
    fn set_c(&mut self, v: u8) {self.registers[5] = v}
    fn set_d(&mut self, v: u8) {self.registers[2] = v}
    fn set_e(&mut self, v: u8) {self.registers[3] = v}
    fn set_f(&mut self, v: u8) {self.registers[7] = v}
    fn set_h(&mut self, v: u8) {self.registers[0] = v}
    fn set_l(&mut self, v: u8) {self.registers[1] = v}

    // TODO: I'm not 100% sure which order things go in here
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

    pub fn set_flag(&mut self, f: Flag, b: bool) {
        let bit = match f {
            Flag::Zero => 7,
            Flag::AddSub => 6,
            Flag::HalfCarry => 5,
            Flag::Carry => 4
        };
        if b {
            self.set_f(self.f() | 1 << bit) // set bit
        } else {
            self.set_f(self.f() & !(1 << bit)) // clear bit
        }
    }


    fn get_register(&self, r: RegisterLoc) -> u8{
        match r {
            RegisterLoc::A => self.a(),
            RegisterLoc::B => self.b(),
            RegisterLoc::C => self.c(),
            RegisterLoc::D => self.d(),
            RegisterLoc::E => self.e(),
            RegisterLoc::H => self.h(),
            RegisterLoc::L => self.l(),
            RegisterLoc::MemHL => unimplemented!("(HL) indirect lookup"), // TODO: this problably is an extra tick too
        }
    }

    fn set_register(&mut self, r: RegisterLoc, val: u8) {
        match r {
            RegisterLoc::A => self.set_a(val),
            RegisterLoc::B => self.set_b(val),
            RegisterLoc::C => self.set_c(val),
            RegisterLoc::D => self.set_d(val),
            RegisterLoc::E => self.set_e(val),
            RegisterLoc::H => self.set_h(val),
            RegisterLoc::L => self.set_l(val),
            RegisterLoc::MemHL => unimplemented!("(HL) indirect lookup"), // TODO: this problably is an extra tick too
        }
    }

    fn get_register16(&self, r: Register16Loc) -> u16 {
        match r {
            Register16Loc::BC => self.bc(),
            Register16Loc::DE => self.de(),
            Register16Loc::HL => self.hl(),
        }
    }

    fn set_register16(&mut self, r: Register16Loc, val: u16) {
        match r {
            Register16Loc::BC => self.set_bc(val),
            Register16Loc::DE => self.set_de(val),
            Register16Loc::HL => self.set_hl(val),
        }
    }

    pub fn tick(&mut self) {
        let instruction = self.next_op();
        self.execute(instruction);
    }

    pub fn print_state(&self) {
        println!("CLK:{}|PC:0x{:04X}|SP:0x{:04X}|F:0x{:02X}|A:0x{:02X}|\
                  B:0x{:02X}|C:0x{:02X}|D:0x{:02X}|E:0x{:02X}|H:0x{:02X}|L:0x{:02X}",
                 self.cycles, self.pc, self.sp, self.f(), self.a(),
                 self.b(), self.c(), self.d(), self.e(), self.h(), self.l()
        )
    }

    fn clock(&mut self) {
        self.cycles += 4; // Each CPU is 4 cycles I belive
        self.bus.cpu_tick();
    }

    fn next(&mut self) -> u8 {
        let data = self.bus.read(self.pc);
        self.pc += 1;
        self.clock();
        return data
    }
    fn next16(&mut self) -> u16 {
        let lower = self.next() as u16;
        let upper = (self.next() as u16) << 8;
        upper | lower
    }
    fn register_from_data(data: u8) -> RegisterLoc {
        match data & 0b111 {
            0 => RegisterLoc::B,
            1 => RegisterLoc::C,
            2 => RegisterLoc::D,
            3 => RegisterLoc::E,
            4 => RegisterLoc::H,
            5 => RegisterLoc::L,
            6 => RegisterLoc::MemHL,
            7 => RegisterLoc::A,
            _ => unreachable!("The number must be anded with 0b111")
        }
    }
    fn next_op(&mut self) -> Instruction {
        let data = self.next();
        let reg = Self::register_from_data(data);
        let regl = Location::Register(reg);
        match data { // https://gbdev.io/gb-opcodes/optables/
            0x00 => Instruction::Nop,

            0x01 => Instruction::Load(Location::Register16(Register16Loc::BC), Location::Immediate16(self.next16())), // LD SP n16
            0x11 => Instruction::Load(Location::Register16(Register16Loc::DE), Location::Immediate16(self.next16())), // LD SP n16
            0x21 => Instruction::Load(Location::Register16(Register16Loc::HL), Location::Immediate16(self.next16())), // LD SP n16
            0x31 => Instruction::Load(Location::SP, Location::Immediate16(self.next16())), // LD SP n16

            0x76 => Instruction::Halt,
            0x40..=0x47 => Instruction::Load(Location::Register(RegisterLoc::B), regl),
            0x48..=0x4F => Instruction::Load(Location::Register(RegisterLoc::C), regl),
            0x50..=0x57 => Instruction::Load(Location::Register(RegisterLoc::D), regl),
            0x58..=0x5F => Instruction::Load(Location::Register(RegisterLoc::E), regl),
            0x60..=0x67 => Instruction::Load(Location::Register(RegisterLoc::H), regl),
            0x68..=0x6F => Instruction::Load(Location::Register(RegisterLoc::L), regl),
            0x70..=0x77 => Instruction::Load(Location::Register(RegisterLoc::MemHL), regl),
            0x78..=0x7F => Instruction::Load(Location::Register(RegisterLoc::A), regl),
            0x80..=0x87 => Instruction::Add(reg),
            0x88..=0x8F => Instruction::Adc(reg),
            0x90..=0x97 => Instruction::Sub(reg),
            0x98..=0x9F => Instruction::Sbc(reg),
            0xA0..=0xA7 => Instruction::And(reg),
            0xA8..=0xAF => Instruction::Xor(reg),
            0xB0..=0xB7 => Instruction::Or(reg),
            0xB8..=0xBF => Instruction::Cp(reg),

            0xCB => self.next_op_extended(),

            _ => panic!("Unimplemented Instruction 0x{:X}", data)
        }
    }
    fn next_op_extended(&mut self) -> Instruction {
        let data = self.next();
        let reg = Self::register_from_data(data);
        match data {
            0x00..=0x07 => Instruction::Rlc(reg),
            0x08..=0x0F => Instruction::Rrc(reg),
            0x10..=0x17 => Instruction::Rl(reg),
            0x18..=0x1F => Instruction::Rr(reg),
            0x20..=0x27 => Instruction::Sla(reg),
            0x28..=0x2F => Instruction::Sra(reg),
            0x30..=0x37 => Instruction::Swap(reg),
            0x38..=0x3F => Instruction::Srl(reg),
            0x40..=0x47 => Instruction::Bit(0, reg),
            0x48..=0x4F => Instruction::Bit(1, reg),
            0x50..=0x57 => Instruction::Bit(2, reg),
            0x58..=0x5F => Instruction::Bit(3, reg),
            0x60..=0x67 => Instruction::Bit(4, reg),
            0x68..=0x6F => Instruction::Bit(5, reg),
            0x70..=0x77 => Instruction::Bit(6, reg),
            0x78..=0x7F => Instruction::Bit(7, reg),
            0x80..=0x87 => Instruction::Res(0, reg),
            0x88..=0x8F => Instruction::Res(1, reg),
            0x90..=0x97 => Instruction::Res(2, reg),
            0x98..=0x9F => Instruction::Res(3, reg),
            0xA0..=0xA7 => Instruction::Res(4, reg),
            0xA8..=0xAF => Instruction::Res(5, reg),
            0xB0..=0xB7 => Instruction::Res(6, reg),
            0xB8..=0xBF => Instruction::Res(7, reg),
            0xC0..=0xC7 => Instruction::Set(0, reg),
            0xC8..=0xCF => Instruction::Set(1, reg),
            0xD0..=0xD7 => Instruction::Set(2, reg),
            0xD8..=0xDF => Instruction::Set(3, reg),
            0xE0..=0xE7 => Instruction::Set(4, reg),
            0xE8..=0xEF => Instruction::Set(5, reg),
            0xF0..=0xF7 => Instruction::Set(6, reg),
            0xF8..=0xFF => Instruction::Set(7, reg),
        }
    }

    fn execute(&mut self, op: Instruction) {
        print!("{:<15} => ", format!("{}", op));
        fn isLoc16Bit (l: Location) -> bool {
            match l {
                Location::Immediate16(_) => true,
                Location::SP => true,
                _ => false,
            }
        };

        match op {
            Instruction::Nop => (),
            Instruction::Load(dest, src) => {
                let is16BitMode = isLoc16Bit(dest) || isLoc16Bit(src);
                if is16BitMode {
                    let v16: u16 = match src {
                        Location::Immediate16(i) => i,
                        Location::Register16(r) => self.get_register16(r),
                        Location::SP => self.sp,
                        _ => panic!("8 bit src in 16 bit load")
                    };

                    match dest {
                        Location::Immediate16(_) => panic!("Immediate cannot be a destination"),
                        Location::SP => self.sp = v16,
                        Location::Register16(r) => self.set_register16(r, v16),
                        _ => panic!("8 bit dest in 16 bit load")
                    }
                } else {
                    let v8: u8 = match src {
                        Location::Immediate(i) => i,
                        Location::Register(r) => self.get_register(r),
                        _ => panic!("8 bit src in 16 bit load")
                    };

                    match dest {
                        Location::Immediate(_) => panic!("Immediate cannot be a destination"),
                        Location::Register(r) => self.set_register(r, v8),
                        _ => panic!("8 bit dest in 16 bit load")
                    }
                }
            },
            Instruction::Xor(r) => {
                let nv = self.a() ^ self.get_register(r);
                self.set_flag(Flag::Zero, nv != 0);
                self.set_a(nv)
            }
            _ => unimplemented!("TODO")
        }
    }

}
