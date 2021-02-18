use crate::instruction::{Instruction, Location, Register16Loc, RegisterLoc, Jump, JmpFlag, Offset};

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
    // Should be set low (C , E or L) then set high (B, D or H)
    fn set_af(&mut self, v: u16) {self.set_a((v >> 8) as u8); self.set_f((v & 0xFF) as u8)}
    fn set_bc(&mut self, v: u16) {self.set_b((v >> 8) as u8); self.set_c((v & 0xFF) as u8)}
    fn set_de(&mut self, v: u16) {self.set_d((v >> 8) as u8); self.set_e((v & 0xFF) as u8)}
    fn set_hl(&mut self, v: u16) {self.set_h((v >> 8) as u8); self.set_l((v & 0xFF) as u8)}

    pub fn get_flag(&self, f: Flag) -> bool {
        let bit = match f {
            Flag::Zero => 7,
            Flag::AddSub => 6,
            Flag::HalfCarry => 5,
            Flag::Carry => 4
        };
        1 == ((self.f() >> bit) & 1)
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


    fn get_register(&mut self, r: RegisterLoc) -> u8{
        match r {
            RegisterLoc::A => self.a(),
            RegisterLoc::B => self.b(),
            RegisterLoc::C => self.c(),
            RegisterLoc::D => self.d(),
            RegisterLoc::E => self.e(),
            RegisterLoc::H => self.h(),
            RegisterLoc::L => self.l(),
            RegisterLoc::MemHL => self.read(self.hl()),
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
            RegisterLoc::MemHL => self.write(self.hl(), val),
        }
    }

    fn get_register16(&self, r: Register16Loc) -> u16 {
        match r {
            Register16Loc::BC => self.bc(),
            Register16Loc::DE => self.de(),
            Register16Loc::HL => self.hl(),
            Register16Loc::AF => self.af(),
        }
    }

    fn set_register16(&mut self, r: Register16Loc, val: u16) {
        match r {
            Register16Loc::BC => self.set_bc(val),
            Register16Loc::DE => self.set_de(val),
            Register16Loc::HL => self.set_hl(val),
            Register16Loc::AF => self.set_af(val),
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

    fn read(&mut self, loc: u16) -> u8 {
        let data = self.bus.read(loc);
        self.clock();
        return data
    }

    fn write(&mut self, loc: u16, val: u8) {
        let data = self.bus.write(loc, val);
        self.clock();
    }

    fn next(&mut self) -> u8 {
        let data = self.read(self.pc);
        self.pc += 1;
        return data
    }
    fn next_signed(&mut self) -> i8 {
        self.next() as i8
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

    fn check_jmp_flag(&self, f: JmpFlag) -> bool {
        match f {
            JmpFlag::Zero => self.get_flag(Flag::Zero),
            JmpFlag::NoZero => !self.get_flag(Flag::Zero),
            JmpFlag::Carry => self.get_flag(Flag::Carry),
            JmpFlag::NoCarry => !self.get_flag(Flag::Carry),
        }
    }

    fn is_add_half_carry(&self, target: u8, value: u8) -> bool {
        ((target & 0x0F).wrapping_add(value & 0x0F)) & 0x10 == 0x10
    }

    fn is_add_carry(&self, target: u8, value: u8) -> bool {
        let (val, overflow) = target.overflowing_add(value);
        return overflow
    }

    fn is_add_half_carry16(&self, target: u16, value: u16) -> bool {
        ((target & 0x00FF).wrapping_add(value & 0x00FF)) & 0x0100 == 0x0100
    }

    fn is_add_carry16(&self, target: u16, value: u16) -> bool {
        let (val, overflow) = target.overflowing_add(value);
        return overflow
    }

    fn is_subtract_half_carry(&self, target: u8, value: u8) -> bool {
        let (val, overflow) = (target & 0x0F).overflowing_sub(value & 0x0F);
        return if overflow { true } else { val < 0 }
    }

    fn is_subtract_carry(&self, target: u8, value: u8) -> bool {
        let (val, overflow) = target.overflowing_sub(value);
        return overflow
    }

    fn is_subtract_half_carry16(&self, target: u16, value: u16) -> bool {
        let (val, overflow) = (target & 0x00FF).overflowing_sub(value & 0x00FF);
        return if overflow { true } else { val < 0 }
    }

    fn is_subtract_carry16(&self, target: u16, value: u16) -> bool {
        let (val, overflow) = target.overflowing_sub(value);
        return overflow
    }

    fn next_op(&mut self) -> Instruction {
        let data = self.next();
        
        let reg = Self::register_from_data(data);
        let regloc = Location::Register(reg);
        match data { // https://gbdev.io/gb-opcodes/optables/
            0x00 => Instruction::Nop,

            // top quarter
            0x01 => Instruction::Load(Location::Register16(Register16Loc::BC), Location::Immediate16(self.next16())), // LD BC n16
            0x11 => Instruction::Load(Location::Register16(Register16Loc::DE), Location::Immediate16(self.next16())), // LD DE n16
            0x20 => Instruction::Jmp(Jump::Relative(self.next_signed()), JmpFlag::NoZero), // JR NZ, r8
            0x21 => Instruction::Load(Location::Register16(Register16Loc::HL), Location::Immediate16(self.next16())), // LD HL n16
            0x31 => Instruction::Load(Location::SP, Location::Immediate16(self.next16())), // LD SP n16
            0x32 => Instruction::Load(Location::Indirect(Offset::HLDec), Location::Register(RegisterLoc::A)), // LD (HL-) n16

            // Increment implemented manuall because can't use the lower 3 bits to determine register 
            0x04 => Instruction::Inc(RegisterLoc::B),
            0x14 => Instruction::Inc(RegisterLoc::D),
            0x24 => Instruction::Inc(RegisterLoc::H),
            // 0x34 => Instruction::Inc(RegisterLoc::MemHL), // Getting data from (HL) not yet implemented
            0x0C => Instruction::Inc(RegisterLoc::C),
            0x1C => Instruction::Inc(RegisterLoc::E),
            0x2C => Instruction::Inc(RegisterLoc::L),
            0x3C => Instruction::Inc(RegisterLoc::A),

            // Increment implemented manuall because can't use the lower 3 bits to determine register 
            0x05 => Instruction::Dec(RegisterLoc::B),
            0x15 => Instruction::Dec(RegisterLoc::D),
            0x25 => Instruction::Dec(RegisterLoc::H),
            // 0x35 => Instruction::Dec(RegisterLoc::MemHL), // Getting data from (HL) not yet implemented
            0x0D => Instruction::Dec(RegisterLoc::C),
            0x1D => Instruction::Dec(RegisterLoc::E),
            0x2D => Instruction::Dec(RegisterLoc::L),
            0x3D => Instruction::Dec(RegisterLoc::A),

            0x06 => Instruction::Load(Location::Register(RegisterLoc::B), Location::Immediate(self.next())),
            0x16 => Instruction::Load(Location::Register(RegisterLoc::D), Location::Immediate(self.next())),
            0x26 => Instruction::Load(Location::Register(RegisterLoc::H), Location::Immediate(self.next())),
            0x36 => Instruction::Load(Location::Register(RegisterLoc::MemHL), Location::Immediate(self.next())),
            0x0E => Instruction::Load(Location::Register(RegisterLoc::C), Location::Immediate(self.next())),
            0x1E => Instruction::Load(Location::Register(RegisterLoc::E), Location::Immediate(self.next())),
            0x2E => Instruction::Load(Location::Register(RegisterLoc::L), Location::Immediate(self.next())),
            0x3E => Instruction::Load(Location::Register(RegisterLoc::A), Location::Immediate(self.next())),

            // middle 2 quarters
            0x76 => Instruction::Halt,
            0x40..=0x47 => Instruction::Load(Location::Register(RegisterLoc::B), regloc),
            0x48..=0x4F => Instruction::Load(Location::Register(RegisterLoc::C), regloc),
            0x50..=0x57 => Instruction::Load(Location::Register(RegisterLoc::D), regloc),
            0x58..=0x5F => Instruction::Load(Location::Register(RegisterLoc::E), regloc),
            0x60..=0x67 => Instruction::Load(Location::Register(RegisterLoc::H), regloc),
            0x68..=0x6F => Instruction::Load(Location::Register(RegisterLoc::L), regloc),
            0x70..=0x77 => Instruction::Load(Location::Register(RegisterLoc::MemHL), regloc),
            0x78..=0x7F => Instruction::Load(Location::Register(RegisterLoc::A), regloc),
            0x80..=0x87 => Instruction::Add(reg),
            0x88..=0x8F => Instruction::Adc(reg),
            0x90..=0x97 => Instruction::Sub(reg),
            0x98..=0x9F => Instruction::Sbc(reg),
            0xA0..=0xA7 => Instruction::And(reg),
            0xA8..=0xAF => Instruction::Xor(reg),
            0xB0..=0xB7 => Instruction::Or(reg),
            0xB8..=0xBF => Instruction::Cp(reg),

            //TODO: Implement register_from_data function that can read 16 bit registers  
            // Bottom quarter ~ 0xC0 - 0xFF
            0xC0 => Instruction::Ret(Some(JmpFlag::NoZero)),
            0xC1 => Instruction::Pop(Register16Loc::BC),
            0xC5 => Instruction::Push(Register16Loc::BC),
            0xC8 => Instruction::Ret(Some(JmpFlag::Zero)),
            0xC9 => Instruction::Ret(None),

            0xD0 => Instruction::Ret(Some(JmpFlag::NoCarry)),
            0xD1 => Instruction::Pop(Register16Loc::DE),
            0xD5 => Instruction::Push(Register16Loc::DE),
            0xD8 => Instruction::Ret(Some(JmpFlag::Carry)), 
            0xD9 => Instruction::Reti,

            0xE0 => Instruction::Load(Location::ZeroPageAbsolute(self.next()), Location::Register(RegisterLoc::A)), // LD (a8), A
            0xE1 => Instruction::Pop(Register16Loc::HL),
            0xE2 => Instruction::Load(Location::ZeroPageC, Location::Register(RegisterLoc::A)),
            0xE5 => Instruction::Push(Register16Loc::HL),

            0xF0 => Instruction::Load(Location::Register(RegisterLoc::A), Location::ZeroPageAbsolute(self.next())), // LD A, (a8)
            0xF1 => Instruction::Pop(Register16Loc::AF), 
            0xF5 => Instruction::Push(Register16Loc::AF), 

            0xCB => self.next_op_extended(),

            _ => panic!("Unimplemented Instruction 0x{:02X}", data)
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

    fn stack_pop(&mut self)  -> u16{

        let low_data = self.bus.stack_pop(self.sp as usize);
        self.sp += 1;
        self.clock();

        let high_data = self.bus.stack_pop(self.sp as usize);
        self.sp += 1;
        self.clock();

        (high_data as u16) << 8  | low_data as u16
        
    }

    fn stack_push(&mut self, data: u16) {
        self.sp -= 1;
        self.bus.stack_push(self.sp as usize, (data >> 8) as u8);
        self.clock();

        self.sp -= 1;
        self.bus.stack_push(self.sp as usize, (data & 0xFF) as u8);
        self.clock();
    }

    fn set_pc(&mut self, loc: u16) {
        self.clock();
        self.pc = loc;
    }

    fn execute(&mut self, op: Instruction) {
        print!("{:<15} => ", format!("{}", op));
        fn isLoc16Bit (l: Location) -> bool {
            match l {
                Location::Immediate16(_) => true,
                Location::SP => true,
                _ => false,
            }
        }

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
                        Location::ZeroPageC => self.read(0xFF00 + (self.c() as u16)),
                        Location::ZeroPageAbsolute(v) => self.read(0xFF00 + (v as u16)),
                        Location::Indirect(off) => {
                            let hl = self.hl();
                            match off {
                                Offset::HLDec => self.set_hl(hl - 1),
                                Offset::HLInc => self.set_hl(hl + 1),
                            };
                            self.read(hl)
                        }
                        _ => panic!("8 bit src in 16 bit load")
                    };

                    match dest {
                        Location::Immediate(_) => panic!("Immediate cannot be a destination"),
                        Location::Register(r) => self.set_register(r, v8),
                        Location::ZeroPageC => self.write(0xFF00 + (self.c() as u16), v8),
                        Location::ZeroPageAbsolute(v) => self.write(0xFF00 + (v as u16), v8),
                        Location::Indirect(off) => {
                            let hl = self.hl();
                            match off {
                                Offset::HLDec => self.set_hl(hl - 1),
                                Offset::HLInc => self.set_hl(hl + 1),
                            };
                            self.write(hl, v8)
                        }
                        _ => panic!("8 bit dest in 16 bit load")
                    }
                }
            },
            Instruction::Bit(v3, r) => {
                let eq1: bool = ((self.get_register(r) >> v3) & 1) == 1;
                self.set_flag(Flag::Zero, !eq1);
                self.set_flag(Flag::AddSub, false);
                self.set_flag(Flag::HalfCarry, true);
            },
            Instruction::Add(r) => {
                
                let a_val: u8 = self.get_register(RegisterLoc::A);
                let reg_val: u8 = self.get_register(r);

                let is_half_carry = self.is_add_half_carry(a_val, reg_val);
                let is_carry = self.is_add_carry(a_val, reg_val);
                let new_val = a_val.wrapping_add(reg_val);
                
                self.set_register(RegisterLoc::A, new_val);
                self.set_flag(Flag::Zero, new_val == 0);
                self.set_flag(Flag::AddSub, false);
                self.set_flag(Flag::HalfCarry, is_half_carry); 
                self.set_flag(Flag::Carry, is_carry); 
            },
            Instruction::Sub(r) => {
                
                let a_val: u8 = self.get_register(RegisterLoc::A);
                let reg_val: u8 = self.get_register(r);

                let is_half_carry = self.is_subtract_half_carry(a_val, reg_val);
                let is_carry = self.is_subtract_carry(a_val, reg_val);
                let new_val = a_val.wrapping_sub(reg_val);
                
                self.set_register(RegisterLoc::A, new_val);
                self.set_flag(Flag::Zero, new_val == 0);
                self.set_flag(Flag::AddSub, true);
                self.set_flag(Flag::HalfCarry, is_half_carry); 
                self.set_flag(Flag::Carry, is_carry); 
            },
            Instruction::And(r) => {
                let nv = self.a() ^& self.get_register(r);
                self.set_flag(Flag::Zero, nv == 0);
                self.set_flag(Flag::AddSub, false);
                self.set_flag(Flag::HalfCarry, true);
                self.set_flag(Flag::Carry, false);
                self.set_a(nv)
            },
            Instruction::Xor(r) => {
                let nv = self.a() ^ self.get_register(r);
                self.set_flag(Flag::Zero, nv == 0);
                self.set_flag(Flag::AddSub, false);
                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::Carry, false);
                self.set_a(nv)
            },
            Instruction::Or(r) => {
                let nv = self.a() | self.get_register(r);
                self.set_flag(Flag::Zero, nv == 0);
                self.set_flag(Flag::AddSub, false);
                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::Carry, false);
                self.set_a(nv)
            },
            Instruction::Inc(r) => {
                
                let old_val: u8 = self.get_register(r);
                let is_half_carry = self.is_add_half_carry(old_val, 1 as u8);
                let new_val = old_val.wrapping_add(1);
                
                self.set_register(r, new_val);
                self.set_flag(Flag::Zero, new_val == 0);
                self.set_flag(Flag::AddSub, false);
                self.set_flag(Flag::HalfCarry, is_half_carry); 
            },
            Instruction::Dec(r) => {

                let old_val: u8 = self.get_register(r);
                let is_half_carry = self.is_subtract_half_carry(old_val, 1 as u8);
                let new_val = old_val.wrapping_sub(1);
                
                self.set_register(r, new_val);
                self.set_flag(Flag::Zero, new_val == 0);
                self.set_flag(Flag::AddSub, true);
                self.set_flag(Flag::HalfCarry, is_half_carry); 
            }
            Instruction::Pop(r) => { // https://rgbds.gbdev.io/docs/v0.4.2/gbz80.7#POP_r16
                let stack_val = self.stack_pop(); 
                match r {
                    Register16Loc::BC => {
                        self.set_bc(stack_val);
                    },
                    Register16Loc::DE => {
                        self.set_de(stack_val);
                    },
                    Register16Loc::HL => {
                        self.set_hl(stack_val);
                    },
                    Register16Loc::AF => { 
                        self.set_af(stack_val);
                    },

                }
            },
            Instruction::Push(r) => { // https://rgbds.gbdev.io/docs/v0.4.2/gbz80.7#PUSH_r16
                match r {
                    Register16Loc::BC => {
                        self.stack_push(self.bc());
                    },
                    Register16Loc::DE => {
                        self.stack_push(self.de());
                    },
                    Register16Loc::HL => {
                        self.stack_push(self.hl());
                    },
                    Register16Loc::AF => { 
                        self.stack_push(self.af());
                    },
                }
                self.clock();
            },
            Instruction::Reti => { // https://rgbds.gbdev.io/docs/v0.4.2/gbz80.7#RETI
                // Return from subroutine and enable interupts 
            },
            Instruction::Ret(cc) => { // https://rgbds.gbdev.io/docs/v0.4.2/gbz80.7#RET_cc
                if let Some(flag) = cc { 
                    self.clock();
                    if self.check_jmp_flag(flag) {
                        let loc = self.stack_pop();
                        self.set_pc(loc);
                    }

                } else { 
                    let loc = self.stack_pop();
                    self.set_pc(loc);
                }
            }
            Instruction::Jmp(j, f) => {
                let b = self.check_jmp_flag(f);
                if b {
                    let dest = match j {
                        Jump::Relative(v8) => ((self.pc as i32) + (v8 as i32)) as u16,
                        Jump::Absolute(v16) => v16,
                    };
                    self.clock();
                    self.pc = dest;
                }
            },
            _ => unimplemented!("TODO")
        }
    }

}

#[cfg(test)]
mod test {
    use crate::bus::Bus;
    use crate::gameboy::ROM;
    use crate::cpu::{CPU, Flag};
    use crate::instruction::{Register16Loc, RegisterLoc};

    fn create_test_cpu(instruction_set: Vec<u8>) -> CPU {
        CPU::new(Bus::new(ROM::from_data(instruction_set)))
    }

    #[test]
    fn test_getter_setter_register() {
        let mut test_cpu = create_test_cpu(vec![]);

        test_cpu.set_register16(Register16Loc::BC, 0x0B0C);
        test_cpu.set_register16(Register16Loc::DE, 0x0D0E);
        test_cpu.set_register16(Register16Loc::HL, 0x080C);
        test_cpu.set_register16(Register16Loc::AF, 0x0A00);
        
        assert_eq!(test_cpu.get_register(RegisterLoc::B), 0x0B);
        assert_eq!(test_cpu.get_register(RegisterLoc::C), 0x0C);
        assert_eq!(test_cpu.get_register(RegisterLoc::D), 0x0D);
        assert_eq!(test_cpu.get_register(RegisterLoc::E), 0x0E);
        assert_eq!(test_cpu.get_register(RegisterLoc::H), 0x08);
        assert_eq!(test_cpu.get_register(RegisterLoc::L), 0x0C);
        assert_eq!(test_cpu.get_register(RegisterLoc::A), 0x0A);
        assert_eq!(test_cpu.get_flag(Flag::Zero), false);
        assert_eq!(test_cpu.get_flag(Flag::AddSub), false);
        assert_eq!(test_cpu.get_flag(Flag::Carry), false);
        assert_eq!(test_cpu.get_flag(Flag::HalfCarry), false);
    }

    #[test]
    fn test_push() {
        let rom_data = vec![0xC5,0xD5,0xE5,0xF5];
        let mut test_cpu = create_test_cpu(rom_data);

        test_cpu.set_register16(Register16Loc::BC, 0x0B0C);
        test_cpu.set_register16(Register16Loc::DE, 0x0D0E);
        test_cpu.set_register16(Register16Loc::HL, 0x080C);
        test_cpu.set_register16(Register16Loc::AF, 0x0A00);
        test_cpu.sp = 0xFFFE;
        
        test_cpu.tick();    
        test_cpu.tick();    
        test_cpu.tick();    
        test_cpu.tick();    

        assert_eq!(test_cpu.bus.stack_pop(0xFFFD), 0x0B);
        assert_eq!(test_cpu.bus.stack_pop(0xFFFC), 0x0C);

        assert_eq!(test_cpu.bus.stack_pop(0xFFFB), 0x0D);
        assert_eq!(test_cpu.bus.stack_pop(0xFFFA), 0x0E);

        assert_eq!(test_cpu.bus.stack_pop(0xFFF9), 0x08);
        assert_eq!(test_cpu.bus.stack_pop(0xFFF8), 0x0C);

        assert_eq!(test_cpu.bus.stack_pop(0xFFF7), 0x0A);
        assert_eq!(test_cpu.bus.stack_pop(0xFFF6), 0x00);
        assert_eq!(test_cpu.sp, 0xFFF6);
        assert_eq!(test_cpu.cycles, 16*4);
        
    }

    #[test]
    fn test_pop() {
        let rom_data = vec![0xC1, 0xD1, 0xE1, 0xF1];
        let mut test_cpu = create_test_cpu(rom_data);

        test_cpu.set_register16(Register16Loc::BC, 0xFFFF);
        test_cpu.set_register16(Register16Loc::DE, 0xFFFF);
        test_cpu.set_register16(Register16Loc::HL, 0xFFFF);
        test_cpu.set_register16(Register16Loc::AF, 0xFFFF);
        test_cpu.sp = 0xFFF6; 

        test_cpu.tick(); 
        test_cpu.tick(); 
        test_cpu.tick(); 
        test_cpu.tick(); 

        assert_eq!(test_cpu.sp, 0xFFFE);
        assert_eq!(test_cpu.get_register16(Register16Loc::BC), 0x0000);
        assert_eq!(test_cpu.get_register16(Register16Loc::DE), 0x0000);
        assert_eq!(test_cpu.get_register16(Register16Loc::HL), 0x0000);
        assert_eq!(test_cpu.get_register16(Register16Loc::AF), 0x0000);
        assert_eq!(test_cpu.cycles, 12*4);
    }

    #[test]
    fn test_ret() {
       let rom_data = vec![0xC9];
       let mut test_cpu = create_test_cpu(rom_data);

       test_cpu.sp = 0xFFFC; 

       test_cpu.tick(); 

       assert_eq!(test_cpu.sp, 0xFFFE);
       assert_eq!(test_cpu.pc, 0x0000);
       assert_eq!(test_cpu.cycles, 16);

    }

    #[test]
    fn test_ret_z() {
        let rom_data = vec![0xC8, 0xC8];
        let mut test_cpu = create_test_cpu(rom_data);

        test_cpu.set_flag(Flag::Zero, false);
        test_cpu.sp = 0xFFFC;
 
        test_cpu.tick(); 
        assert_eq!(test_cpu.sp, 0xFFFC);
        assert_eq!(test_cpu.cycles, 8);

        test_cpu.set_flag(Flag::Zero, true);

        test_cpu.tick();
        assert_eq!(test_cpu.sp, 0xFFFE);
        assert_eq!(test_cpu.pc, 0x0000);
        assert_eq!(test_cpu.cycles, 28);
    }

    #[test]
    fn test_ret_nz() {
        let rom_data = vec![0xC0, 0xC0];
        let mut test_cpu = create_test_cpu(rom_data);

        test_cpu.set_flag(Flag::Zero, true);
        test_cpu.sp = 0xFFFC;
 
        test_cpu.tick(); 
        assert_eq!(test_cpu.sp, 0xFFFC);
        assert_eq!(test_cpu.cycles, 8);

        test_cpu.set_flag(Flag::Zero, false);

        test_cpu.tick();
        assert_eq!(test_cpu.sp, 0xFFFE);
        assert_eq!(test_cpu.pc, 0x0000);
        assert_eq!(test_cpu.cycles, 28);

  
    }

    #[test]
    fn test_ret_c() {
        let rom_data = vec![0xD8, 0xD8];
        let mut test_cpu = create_test_cpu(rom_data);

        test_cpu.set_flag(Flag::Carry, false);
        test_cpu.sp = 0xFFFC;
 
        test_cpu.tick(); 
        assert_eq!(test_cpu.sp, 0xFFFC);
        assert_eq!(test_cpu.cycles, 8);

        test_cpu.set_flag(Flag::Carry, true);

        test_cpu.tick();
        assert_eq!(test_cpu.sp, 0xFFFE);
        assert_eq!(test_cpu.pc, 0x0000);
        assert_eq!(test_cpu.cycles, 28);
    }

    #[test]
    fn test_ret_nc() {
        let rom_data = vec![0xD0, 0xD0];
        let mut test_cpu = create_test_cpu(rom_data);

        test_cpu.set_flag(Flag::Carry, true);
        test_cpu.sp = 0xFFFC;
 
        test_cpu.tick(); 
        assert_eq!(test_cpu.sp, 0xFFFC);
        assert_eq!(test_cpu.cycles, 8);

        test_cpu.set_flag(Flag::Carry, false);

        test_cpu.tick();
        assert_eq!(test_cpu.sp, 0xFFFE);
        assert_eq!(test_cpu.pc, 0x0000);
        assert_eq!(test_cpu.cycles, 28);
    }

    #[test]
    fn test_reti() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_add() {
        let rom_data = vec![0x80,0x80,0x80];
        let mut test_cpu = create_test_cpu(rom_data);

        test_cpu.set_register(RegisterLoc::A, 0x00);
        test_cpu.set_register(RegisterLoc::B, 0x0F);
        test_cpu.tick();

        assert_eq!(test_cpu.get_register(RegisterLoc::A), 0x0F);
        assert_eq!(test_cpu.get_flag(Flag::Zero), false);
        assert_eq!(test_cpu.get_flag(Flag::AddSub), false);
        assert_eq!(test_cpu.get_flag(Flag::HalfCarry), false);
        assert_eq!(test_cpu.get_flag(Flag::Carry), false);

        test_cpu.set_register(RegisterLoc::B, 0xF0);
        test_cpu.tick(); 

        assert_eq!(test_cpu.get_register(RegisterLoc::A), 0xFF);
        assert_eq!(test_cpu.get_flag(Flag::Zero), false);
        assert_eq!(test_cpu.get_flag(Flag::AddSub), false);
        assert_eq!(test_cpu.get_flag(Flag::HalfCarry), false);
        assert_eq!(test_cpu.get_flag(Flag::Carry), false);

        test_cpu.set_register(RegisterLoc::B, 0x01);
        test_cpu.tick(); 

        assert_eq!(test_cpu.get_register(RegisterLoc::A), 0x00);
        assert_eq!(test_cpu.get_flag(Flag::Zero), true);
        assert_eq!(test_cpu.get_flag(Flag::AddSub), false);
        assert_eq!(test_cpu.get_flag(Flag::HalfCarry), true);
        assert_eq!(test_cpu.get_flag(Flag::Carry), true);
    }

    #[test]
    fn test_subtract() {
        let rom_data = vec![0x90,0x90,0x90];
        let mut test_cpu = create_test_cpu(rom_data);

        test_cpu.set_register(RegisterLoc::A, 0x10);
        test_cpu.set_register(RegisterLoc::B, 0x01);
        test_cpu.tick();

        assert_eq!(test_cpu.get_register(RegisterLoc::A), 0x0F);
        assert_eq!(test_cpu.get_flag(Flag::Zero), false);
        assert_eq!(test_cpu.get_flag(Flag::AddSub), true);
        assert_eq!(test_cpu.get_flag(Flag::HalfCarry), true);
        assert_eq!(test_cpu.get_flag(Flag::Carry), false);

        test_cpu.set_register(RegisterLoc::B, 0x0F);
        test_cpu.tick(); 

        assert_eq!(test_cpu.get_register(RegisterLoc::A), 0x00);
        assert_eq!(test_cpu.get_flag(Flag::Zero), true);
        assert_eq!(test_cpu.get_flag(Flag::AddSub), true);
        assert_eq!(test_cpu.get_flag(Flag::HalfCarry), false);
        assert_eq!(test_cpu.get_flag(Flag::Carry), false);

        test_cpu.set_register(RegisterLoc::B, 0x01);
        test_cpu.tick();

        assert_eq!(test_cpu.get_register(RegisterLoc::A), 0xFF);
        assert_eq!(test_cpu.get_flag(Flag::Zero), false);
        assert_eq!(test_cpu.get_flag(Flag::AddSub), true);
        assert_eq!(test_cpu.get_flag(Flag::HalfCarry), true);
        assert_eq!(test_cpu.get_flag(Flag::Carry), true);
    }

    #[test]
    fn test_inc() {
        let rom_data = vec![0x04,0x0C,0x14];
        let mut test_cpu = create_test_cpu(rom_data);

        test_cpu.set_register(RegisterLoc::B, 0x0E);
        test_cpu.tick();

        assert_eq!(test_cpu.get_register(RegisterLoc::B), 0x0F);
        assert_eq!(test_cpu.get_flag(Flag::Zero), false);
        assert_eq!(test_cpu.get_flag(Flag::AddSub), false);
        assert_eq!(test_cpu.get_flag(Flag::HalfCarry), false);

        test_cpu.set_register(RegisterLoc::C, 0x0F);
        test_cpu.tick(); 

        assert_eq!(test_cpu.get_register(RegisterLoc::C), 0x10);
        assert_eq!(test_cpu.get_flag(Flag::Zero), false);
        assert_eq!(test_cpu.get_flag(Flag::AddSub), false);
        assert_eq!(test_cpu.get_flag(Flag::HalfCarry), true);

        test_cpu.set_register(RegisterLoc::D, 0xFF);
        test_cpu.tick(); 

        assert_eq!(test_cpu.get_register(RegisterLoc::D), 0x00);
        assert_eq!(test_cpu.get_flag(Flag::Zero), true);
        assert_eq!(test_cpu.get_flag(Flag::AddSub), false);
        assert_eq!(test_cpu.get_flag(Flag::HalfCarry), true);
    }

    #[test]
    fn test_dec() {
        let rom_data = vec![0x05,0x0D,0x15];
        let mut test_cpu = create_test_cpu(rom_data);

        test_cpu.set_register(RegisterLoc::B, 0x0F);
        test_cpu.tick(); 

        assert_eq!(test_cpu.get_register(RegisterLoc::B), 0x0E);
        assert_eq!(test_cpu.get_flag(Flag::Zero), false);
        assert_eq!(test_cpu.get_flag(Flag::AddSub), true);
        assert_eq!(test_cpu.get_flag(Flag::HalfCarry), false);

        test_cpu.set_register(RegisterLoc::C, 0x10);
        test_cpu.tick(); 

        assert_eq!(test_cpu.get_register(RegisterLoc::C), 0x0F);
        assert_eq!(test_cpu.get_flag(Flag::Zero), false);
        assert_eq!(test_cpu.get_flag(Flag::AddSub), true);
        assert_eq!(test_cpu.get_flag(Flag::HalfCarry), true);

        test_cpu.set_register(RegisterLoc::D, 0x01);
        test_cpu.tick(); 

        assert_eq!(test_cpu.get_register(RegisterLoc::D), 0x00);
        assert_eq!(test_cpu.get_flag(Flag::Zero), true);
        assert_eq!(test_cpu.get_flag(Flag::AddSub), true);
        assert_eq!(test_cpu.get_flag(Flag::HalfCarry), false);
    }
}
