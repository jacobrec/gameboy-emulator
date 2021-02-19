use crate::instruction::{Instruction, Location, Register16Loc, RegisterLoc, Jump, JmpFlag, Offset};
use crate::utils::*;

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
    fn pc(&self) -> u16 {self.pc}

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

    fn next_op(&mut self) -> Instruction {
        let data = self.next();
        
        let reg = Self::register_from_data(data);
        let regloc = Location::Register(reg);
        match data { // https://gbdev.io/gb-opcodes/optables/

            // Top Quarter ~ 0x00 - 0x3F
            0x00 => Instruction::Nop,
            0x20 => Instruction::Jmp(Jump::Relative(self.next_signed()), JmpFlag::NoZero), // JR NZ, r8
            // 0X30 => TODO: JR NZ, r8
            // 0X40 => TODO: JR NC, r8

            // LD (XX), d16
            0x01 => Instruction::Load(Location::Register16(Register16Loc::BC), Location::Immediate16(self.next16())),   // LD BC n16
            0x11 => Instruction::Load(Location::Register16(Register16Loc::DE), Location::Immediate16(self.next16())),   // LD DE n16
            0x21 => Instruction::Load(Location::Register16(Register16Loc::HL), Location::Immediate16(self.next16())),   // LD HL n16
            0x31 => Instruction::Load(Location::SP, Location::Immediate16(self.next16())),                              // LD SP n16

            // LD (XX), A
            0x02 => Instruction::Load(Location::Indirect(Offset::BC), Location::Register(RegisterLoc::A)),      // LD (BC), A
            0x12 => Instruction::Load(Location::Indirect(Offset::DE), Location::Register(RegisterLoc::A)),      // LD (DE), A
            0x22 => Instruction::Load(Location::Indirect(Offset::HLInc), Location::Register(RegisterLoc::A)),   // LD (HL+), A
            0x32 => Instruction::Load(Location::Indirect(Offset::HLDec), Location::Register(RegisterLoc::A)),   // LD (HL-), A
            
            // 0x03 => TODO: INC BC
            // 0x13 => TODO: INC DE
            // 0x23 => TODO: INC HL
            // 0x33 => TODO: INC SP
           
            // INC r8
            0x04 => Instruction::Inc(RegisterLoc::B),       // INC B
            0x14 => Instruction::Inc(RegisterLoc::D),       // INC D
            0x24 => Instruction::Inc(RegisterLoc::H),       // INC H
            0x34 => Instruction::Inc(RegisterLoc::MemHL),   // INC (HL)
            
            // DEC r8
            0x05 => Instruction::Dec(RegisterLoc::B),       // DEC B
            0x15 => Instruction::Dec(RegisterLoc::D),       // DEC D
            0x25 => Instruction::Dec(RegisterLoc::H),       // DEC H
            0x35 => Instruction::Dec(RegisterLoc::MemHL),   // DEC (HL)
            
            // LD r8, d8
            0x06 => Instruction::Load(Location::Register(RegisterLoc::B), Location::Immediate(self.next())),        // LD B, d8
            0x16 => Instruction::Load(Location::Register(RegisterLoc::D), Location::Immediate(self.next())),        // LD D, d8
            0x26 => Instruction::Load(Location::Register(RegisterLoc::H), Location::Immediate(self.next())),        // LD H, d8
            0x36 => Instruction::Load(Location::Register(RegisterLoc::MemHL), Location::Immediate(self.next())),    // LD (HL), d8
 
            // 0x07 => TODO: RLCA
            // 0x17 => TODO: RLA
            // 0x27 => TODO: DAA
            // 0x37 => TODO: SCF

            // 0x08 => TODO: LD (a16), SP
            // 0x18 => TODO: JR r8
            // 0x28 => TODO: JR Z, r8
            // 0x38 => TODO: JR C, r8

            // ADD HL, r16
            // 0x09 => TODO: ADD HL, BC
            // 0x19 => TODO: ADD DL, DE
            // 0x29 => TODO: ADD DL, HL
            // 0x39 => TODO: ADD DL, SP   
           
            // LD A, (XX)
            0x0A => Instruction::Load(Location::Register(RegisterLoc::A), Location::Indirect(Offset::BC)),      // LD A, (BC)
            0x1A => Instruction::Load(Location::Register(RegisterLoc::A), Location::Indirect(Offset::DE)),      // LD A, (DE)
            0x2A => Instruction::Load(Location::Register(RegisterLoc::A), Location::Indirect(Offset::HLInc)),   // LD A, (HL+)
            0x3A => Instruction::Load(Location::Register(RegisterLoc::A), Location::Indirect(Offset::HLDec)),   // LD A, (HL-)

            // INC r8
            0x0C => Instruction::Inc(RegisterLoc::C), // INC C
            0x1C => Instruction::Inc(RegisterLoc::E), // INC E
            0x2C => Instruction::Inc(RegisterLoc::L), // INC L
            0x3C => Instruction::Inc(RegisterLoc::A), // INC A

            // DEC r8
            0x0D => Instruction::Dec(RegisterLoc::C), // DEC C
            0x1D => Instruction::Dec(RegisterLoc::E), // DEC E
            0x2D => Instruction::Dec(RegisterLoc::L), // DEC L
            0x3D => Instruction::Dec(RegisterLoc::A), // DEC A
            
            // LD r8, d8
            0x0E => Instruction::Load(Location::Register(RegisterLoc::C), Location::Immediate(self.next())), // LD C, d8
            0x1E => Instruction::Load(Location::Register(RegisterLoc::E), Location::Immediate(self.next())), // LD E, d8
            0x2E => Instruction::Load(Location::Register(RegisterLoc::L), Location::Immediate(self.next())), // LD L, d8
            0x3E => Instruction::Load(Location::Register(RegisterLoc::A), Location::Immediate(self.next())), // LD A, d8
            
            // 0x0F => TODO: RRCA
            // 0x1F => TODO: RRA
            // 0x2F => TODO: CPL
            // 0x3F => TODO: CCF

            // Middle 2 Quarters ~ 0x40 - 0xBF
            0x76 => Instruction::Halt,
            0x40..=0x47 => Instruction::Load(Location::Register(RegisterLoc::B), regloc),       // LD B r8
            0x48..=0x4F => Instruction::Load(Location::Register(RegisterLoc::C), regloc),       // LD C r8
            0x50..=0x57 => Instruction::Load(Location::Register(RegisterLoc::D), regloc),       // LD D r8
            0x58..=0x5F => Instruction::Load(Location::Register(RegisterLoc::E), regloc),       // LD E r8
            0x60..=0x67 => Instruction::Load(Location::Register(RegisterLoc::H), regloc),       // LD H r8
            0x68..=0x6F => Instruction::Load(Location::Register(RegisterLoc::L), regloc),       // LD L r8
            0x70..=0x77 => Instruction::Load(Location::Register(RegisterLoc::MemHL), regloc),   // LD (HL) r8
            0x78..=0x7F => Instruction::Load(Location::Register(RegisterLoc::A), regloc),       // LD A r8
            0x80..=0x87 => Instruction::Add(reg),   // ADD A r8
            0x88..=0x8F => Instruction::Adc(reg),   // ADC A r8
            0x90..=0x97 => Instruction::Sub(reg),   // SUB A r8
            0x98..=0x9F => Instruction::Sbc(reg),   // SBC A r8
            0xA0..=0xA7 => Instruction::And(reg),   // AND A r8
            0xA8..=0xAF => Instruction::Xor(reg),   // XOR A r8
            0xB0..=0xB7 => Instruction::Or(reg),    // OR A r8
            0xB8..=0xBF => Instruction::Cp(reg),    // CP A r8

            //TODO: Implement register_from_data function that can read 16 bit registers  
            // Bottom quarter ~ 0xC0 - 0xFF
            0xC0 => Instruction::Ret(Some(JmpFlag::NoZero)),                    // RET NZ
            0xC1 => Instruction::Pop(Register16Loc::BC),                        // POP BC
            // 0xC2 => TODO: JP NZ, a16
            // 0xC3 => TODO: JP a16
            0xC4 => Instruction::Call(Some(JmpFlag::NoZero), self.next16()),    // CALL NZ, a16
            0xC5 => Instruction::Push(Register16Loc::BC),                       // PUSH BC
            // 0xC6 => TODO: ADD A, d8
            // 0xC7 => TODO: RST 00H
            0xC8 => Instruction::Ret(Some(JmpFlag::Zero)),                      // RET Z
            0xC9 => Instruction::Ret(None),                                     // RET
            // 0xCA => TODO: JP Z, a16
            // 0xCB => TODO: PREFIX
            0xCC => Instruction::Call(Some(JmpFlag::Zero), self.next16()),      // CALL Z, a16
            0xCD => Instruction::Call(None, self.next16()),                     // CALL a16
            // 0xCE => TODO: ADC A, d8
            // 0xCF => RST 08H

            0xD0 => Instruction::Ret(Some(JmpFlag::NoCarry)),                   // RET NC
            0xD1 => Instruction::Pop(Register16Loc::DE),                        // POP DE
            // 0xD2 => TODO: JP NC, a16
            0xD4 => Instruction::Call(Some(JmpFlag::NoCarry), self.next16()),   // CALL NC, a16
            0xD5 => Instruction::Push(Register16Loc::DE),                       // PUSH DE
            // 0xD6 => TODO: SUB d8
            // 0xD7 => TODO: RST 10H
            0xD8 => Instruction::Ret(Some(JmpFlag::Carry)),                     // RET C
            0xD9 => Instruction::Reti,                                          // RETI
            // 0xDA => TODO: JP C, a16
            0xDC => Instruction::Call(Some(JmpFlag::Carry), self.next16()),     // CALL C, a16
            // 0xDE => TODO: SBC A, d8
            // 0xDF => TODO: RST 18H

            0xE0 => Instruction::Load(Location::ZeroPageAbsolute(self.next()), Location::Register(RegisterLoc::A)), // LD (a8), A
            0xE1 => Instruction::Pop(Register16Loc::HL),                                                            // POP HL
            0xE2 => Instruction::Load(Location::ZeroPageC, Location::Register(RegisterLoc::A)),                     // LD (C), A
            0xE5 => Instruction::Push(Register16Loc::HL),                                                           // PUSH HL
            // 0XE6 => TODO: AND d8
            // 0XE7 => TODO: RST 20H
            // 0XE8 => TODO: ADD SP, r8
            // 0XE9 => TODO: JP HL
            // 0XEA => TODO: LD (a16), A
            // 0XEE => TODO: XOR d8
            // 0XEF => TODO: RST 28H

            0xF0 => Instruction::Load(Location::Register(RegisterLoc::A), Location::ZeroPageAbsolute(self.next())), // LD A, (a8)
            0xF1 => Instruction::Pop(Register16Loc::AF),                                                            // POP AF
            // 0XF2 => TODO: LD A, (C)
            // 0XF3 => TODO: DI
            0xF5 => Instruction::Push(Register16Loc::AF),                                                           // PUSH AF
            // 0XF6 => TODO: OR d8
            // 0XF7 => TODO: RST 30H
            // 0XF8 => TODO: LD HL, SP + r8
            // 0XF9 => TODO: LD SP, HL
            // 0XFA => TODO: LD A, (a16)
            // 0XFB => TODO: EI
            // 0XFE => TODO: CP d8
            // 0XFF => TODO: RST 38H

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

    fn indirect_lookup(&mut self, off: Offset) -> u16 {
        let hl = self.hl();
        match off {
            Offset::HLDec => { self.set_hl(hl - 1); hl },
            Offset::HLInc => { self.set_hl(hl + 1); hl },
            Offset::BC => self.bc(),
            Offset::DE => self.de(),
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
                            let loc = self.indirect_lookup(off);
                            self.read(loc)
                        }
                        _ => panic!("8 bit src in 16 bit load")
                    };

                    match dest {
                        Location::Immediate(_) => panic!("Immediate cannot be a destination"),
                        Location::Register(r) => self.set_register(r, v8),
                        Location::ZeroPageC => self.write(0xFF00 + (self.c() as u16), v8),
                        Location::ZeroPageAbsolute(v) => self.write(0xFF00 + (v as u16), v8),
                        Location::Indirect(off) => {
                            let loc = self.indirect_lookup(off);
                            self.write(loc, v8)
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

                let is_half_carry = is_add_half_carry(a_val, reg_val);
                let is_carry = is_add_carry(a_val, reg_val);
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

                let is_half_carry = is_subtract_half_carry(a_val, reg_val);
                let is_carry = is_subtract_carry(a_val, reg_val);
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
                let is_half_carry = is_add_half_carry(old_val, 1 as u8);
                let new_val = old_val.wrapping_add(1);
                
                self.set_register(r, new_val);
                self.set_flag(Flag::Zero, new_val == 0);
                self.set_flag(Flag::AddSub, false);
                self.set_flag(Flag::HalfCarry, is_half_carry); 
            },
            Instruction::Dec(r) => {

                let old_val: u8 = self.get_register(r);
                let is_half_carry = is_subtract_half_carry(old_val, 1 as u8);
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
            },
            Instruction::Call(None, l) => { // https://rgbds.gbdev.io/docs/v0.4.2/gbz80.7#RET_cc
                self.stack_push(self.pc());
                self.set_pc(l);
            },
            Instruction::Call(Some(flag), l) => {
                if self.check_jmp_flag(flag) {
                    self.stack_push(self.pc());
                    self.set_pc(l);
                }
            },
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

    #[test]
    fn test_call() {
        // XOR A
        // CALL 0x0005
        // XOR C
        // XOR B
        // RET
        // This should execute XOR A, then XOR B, then XOR C in that order
       let rom_data = vec![0xAF, 0xCD, 0x05, 0x00, 0xA9,  0xA8, 0xC9];
       let mut test_cpu = create_test_cpu(rom_data);
       test_cpu.sp = 0xFFFE;
       test_cpu.tick();
       assert_eq!(test_cpu.cycles, 4);
       test_cpu.tick();
       assert_eq!(test_cpu.sp, 0xFFFC);
       assert_eq!(test_cpu.cycles, 28);
       test_cpu.tick();
       assert_eq!(test_cpu.cycles, 32);
       test_cpu.tick();
       assert_eq!(test_cpu.sp, 0xFFFE);
       assert_eq!(test_cpu.pc, 0x0004);
       assert_eq!(test_cpu.cycles, 48);
       test_cpu.tick();
       assert_eq!(test_cpu.cycles, 52);
    }

    #[test]
    fn test_call_cc() {
        // XOR A
        // CALL Z, 0x0005
        // XOR C
        // XOR B
        // RET
        // This should execute XOR A, then XOR B, then XOR C in that order
       let rom_data = vec![0xAF, 0xCC, 0x05, 0x00, 0xA9,  0xA8, 0xC9];
       let mut test_cpu = create_test_cpu(rom_data);
       test_cpu.sp = 0xFFFE;
       test_cpu.tick();
       assert_eq!(test_cpu.cycles, 4);
       test_cpu.tick();
       assert_eq!(test_cpu.sp, 0xFFFC);
       assert_eq!(test_cpu.cycles, 28);
       test_cpu.tick();
       assert_eq!(test_cpu.cycles, 32);
       test_cpu.tick();
       assert_eq!(test_cpu.sp, 0xFFFE);
       assert_eq!(test_cpu.pc, 0x0004);
       assert_eq!(test_cpu.cycles, 48);
       test_cpu.tick();
       assert_eq!(test_cpu.cycles, 52);

        // XOR A
        // CALL NZ, 0x0005
        // XOR C
        // XOR B
        // RET
        // This should execute XOR A, then XOR C in that order
       let rom_data = vec![0xAF, 0xC4, 0x05, 0x00, 0xA9,  0xA8, 0xC9];
       let mut test_cpu = create_test_cpu(rom_data);
       test_cpu.sp = 0xFFFE;
       test_cpu.tick();
       assert_eq!(test_cpu.cycles, 4);
       test_cpu.tick();
       assert_eq!(test_cpu.sp, 0xFFFE);
       assert_eq!(test_cpu.cycles, 16);
       test_cpu.tick();
       assert_eq!(test_cpu.cycles, 20);
    }
}
