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
= FEA0-FEFF = Unused
= FF00-FF7F = I/O registers
= FF80-FFFE = HRAM
= FFFF-FFFF = Interrupt Enable Register
*/

pub struct Bus {
    rom: ROM,
    ram: [u8; 0xFFFF], // Most of this will get shadowed as the code is filled in
    ppu: crate::ppu::PPU,
}

impl Bus {
    pub fn get_screen(&self) -> crate::ppu::Canvas {
        return self.ppu.get_screen()
    }

    pub const fn new(rom: ROM) -> Self {
        let ram = [0u8; 0xFFFF];
        let ppu = crate::ppu::PPU::new();
        Bus { rom, ram, ppu }
    }

    pub fn read(&self, loc: u16) -> u8 {
        match loc {
            0x0000..=0x3FFF => self.rom.read(loc),
            0xC000..=0xCFFF => self.ram[loc as usize],
            0xD000..=0xDFFF => self.ram[loc as usize],
            _ => panic!("Unimplemented read range: {}", loc)
        }
    }

    pub fn write(&mut self, loc: u16, val: u8) {
        match loc {
            0x0000..=0x3FFF => self.rom.write(loc, val),
            0xC000..=0xCFFF => self.ram[loc as usize] = val,
            0xD000..=0xDFFF => self.ram[loc as usize] = val,
            _ => panic!("Unimplemented read range: {}", loc)
        }
    }

    pub fn cpu_tick(&mut self) {
        // CPU runs at 1MHz
        // PPU runs at 2MHz
        self.ppu.tick();
        self.ppu.tick();
    }
}
