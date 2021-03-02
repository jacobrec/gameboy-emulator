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


I/O Register Details
====================
$FF00     $FF02     DMG     Port/Mode
$FF04     $FF07     DMG     Port/Mode
$FF10     $FF26     DMG     Sound
$FF30     $FF3F     DMG     Waveform RAM
$FF40     $FF4B     DMG     LCD
$FF4F               CGB     VRAM Bank Select
$FF50               DMG     Set to non-zero to disable boot ROM
$FF51     $FF55     CGB     HDMA
$FF68     $FF69     CGB     BCP/OCP
$FF70               CGB     WRAM Bank Select
*/

pub struct Bus {
    rom: ROM,
    ram: [u8; 0xFFFF], // Most of this will get shadowed as the code is filled in
    ppu: crate::ppu::PPU,
    apu: crate::apu::APU,
}

impl Bus {
    pub fn get_screen(&self) -> crate::ppu::Canvas {
        return self.ppu.get_screen()
    }

    pub const fn new(rom: ROM) -> Self {
        let ram = [0u8; 0xFFFF];
        let ppu = crate::ppu::PPU::new();
        let apu = crate::apu::APU::new();
        Bus { rom, ram, ppu, apu }
    }

    pub fn read(&self, loc: u16) -> u8 {
        match loc {
            0x0000..=0x3FFF => self.rom.read(loc),
            0xC000..=0xCFFF => self.ram[loc as usize],
            0xD000..=0xDFFF => self.ram[loc as usize],
            0x8000..=0x9FFF => self.ppu.read(loc),
            0xFF10..=0xFF26 => self.apu.read(loc),
            0xFF30..=0xFF3F => self.apu.read(loc),
            0xFF40..=0xFF4F => { self.ppu.read_reg(loc) },
            0xFF00..=0xFF7F => { print!("[UNIMPLEMENTED: Reading IO Register]\n{:19}", ""); 0},
            0xFF80..=0xFFFE => self.ram[loc as usize], // HRAM
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
            0xFF40..=0xFF4F => { self.ppu.write_reg(loc, val) },
            0xFF00..=0xFF7F => { print!("[UNIMPLEMENTED: Writing IO Register]\n{:19}", "")},
            0xFF80..=0xFFFE => self.ram[loc as usize] = val, // HRAM
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

    pub fn stack_push(&mut self, sp: usize, data: u8) {
        self.ram[sp] = data;
    }

    pub fn stack_pop(&self, sp: usize) -> u8 {
        self.ram[sp]
    }

}
