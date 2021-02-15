use crate::gameboy::ROM;

/*
Memory Map
==========
= 0000-3FFF = 16KB ROM Bank 00
= 4000-7FFF = 16KB ROM Bank 01-NN
= 8000-9FFF = 8KB  Video RAM
= A000-BFFF = 8KB  External RAM (from cartridge)
= C000-CFFF = 4KB  Work RAM Bank 0
= D000-EFFF = 4KB  Work RAM Bank 1
= E000-FDFF = Mirror of C000-DDFF
= FE00-FE9F = OAM RAM
= FEA0-FEFF = Unused. Reads return 0xFF when OAM blocked, otherwise 00
= FF00-FF7F = I/O registers
= FF80-FFFE = HRAM
= FFFF-FFFF = Interrupt Enable Register
*/

pub struct Bus {
    rom: ROM,
    ram: [u8; 0xFFFF], // Most of this will get shadowed as the code is filled in
    ppu: crate::ppu::PPU,
    apu: crate::apu::APU,
    stack: std::vec::Vec<u8>,
}

impl Bus {
    pub fn get_screen(&self) -> crate::ppu::Canvas {
        return self.ppu.get_screen()
    }

    pub const fn new(rom: ROM) -> Self {
        let ram = [0u8; 0xFFFF];
        let ppu = crate::ppu::PPU::new();
        let apu = crate::apu::APU::new();
        let stack = Vec::new();
        Bus { rom, ram, ppu, apu, stack }
    }

    pub fn read(&self, loc: u16) -> u8 {
        match loc {
            0x0000..=0x3FFF => self.rom.read(loc),
            0xC000..=0xCFFF => self.ram[loc as usize],
            0xD000..=0xDFFF => self.ram[loc as usize],
            0x8000..=0x9FFF => self.ppu.read(loc),
            0xFF10..=0xFF26 => self.apu.read(loc),
            0xFF30..=0xFF3F => self.apu.read(loc),
            _ => panic!("Unimplemented read range: {:04X}", loc)
        }
    }

    pub fn write(&mut self, loc: u16, val: u8) {
        match loc {
            0x0000..=0x3FFF => self.rom.write(loc, val),
            0xC000..=0xCFFF => self.ram[loc as usize] = val,
            0xD000..=0xDFFF => self.ram[loc as usize] = val,
            0x8000..=0x9FFF => self.ppu.write(loc, val),
            0xFF10..=0xFF26 => self.apu.write(loc, val),
            0xFF30..=0xFF3F => self.apu.write(loc, val),
            _ => panic!("Unimplemented write range: {:04X}", loc)
        }
    }

    pub fn cpu_tick(&mut self) {
        // CPU runs at 1MHz
        // PPU runs at 2MHz
        self.ppu.tick();
        self.ppu.tick();
        // TODO: call apu tick
    }

    pub fn stack_push(&mut self, data: u8) {
        self.stack.push(data);
    }

    pub fn stack_pop(&mut self) -> Option<u8> {
        self.stack.pop()
    }

    pub fn get_stack(&self) -> &[u8] {
        &self.stack
    }
}
