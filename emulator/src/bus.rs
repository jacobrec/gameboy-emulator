use crate::cartridge::Cartridge;
use crate::cpu_recievable::Recievables;

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
    rom: Cartridge,
    ram: [u8; 0xFFFF], // Most of this will get shadowed as the code is filled in
    ppu: crate::ppu::PPU,
    apu: crate::apu::APU,
    pub reg_ie: InterruptRegister, // 0xFFFF
    pub reg_if: InterruptRegister, // 0xFF0F
}

impl Bus {
    pub fn get_screen(&self) -> crate::ppu::Screen {
        return self.ppu.get_screen()
    }

    pub fn new(rom: Cartridge) -> Self {
        let ram = [0u8; 0xFFFF];
        let ppu = crate::ppu::PPU::new();
        let apu = crate::apu::APU::new();
        let reg_if = InterruptRegister { data: 0 };
        let reg_ie = InterruptRegister { data: 0 };
        Bus { rom, ram, ppu, apu, reg_if, reg_ie }
    }

    pub fn read(&self, loc: u16) -> u8 {
        match loc {
            0x0000..=0x3FFF => self.rom.read(loc),
            0x4000..=0x7FFF => self.rom.read(loc), // upper rom banks
            0xC000..=0xCFFF => self.ram[loc as usize],
            0xD000..=0xDFFF => self.ram[loc as usize],
            0x8000..=0x9FFF => self.ppu.read(loc),
            0xFE00..=0xFE9F => self.ppu.readOAM(loc),
            0xFF0F => self.reg_if.data,
            0xFF10..=0xFF26 => self.apu.read(loc),
            0xFF30..=0xFF3F => self.apu.read(loc),
            0xFF40..=0xFF4F => { self.ppu.read_reg(loc) },
            0xFF00..=0xFF7F => { print!("[UNIMPLEMENTED: Reading IO Register]\n{:19}", ""); 0},
            0xFF80..=0xFFFE => self.ram[loc as usize], // HRAM
            0xFFFF => self.reg_ie.data,
            _ => panic!("Unimplemented read range: {:04X}", loc)
        }
    }

    pub fn write(&mut self, loc: u16, val: u8) {
        match loc {
            0x0000..=0x3FFF => self.rom.write(loc, val),
            0x4000..=0x7FFF => self.rom.write(loc, val), // upper rom banks
            0xC000..=0xCFFF => self.ram[loc as usize] = val,
            0xD000..=0xDFFF => self.ram[loc as usize] = val,
            0x8000..=0x9FFF => self.ppu.write(loc, val),
            0xFE00..=0xFE9F => self.ppu.writeOAM(loc, val),
            0xFF0F => self.reg_if.data = val,
            0xFF10..=0xFF26 => self.apu.write(loc, val),
            0xFF30..=0xFF3F => self.apu.write(loc, val),
            0xFF40..=0xFF4F => { self.ppu.write_reg(loc, val) },
            0xFF00..=0xFF7F => { print!("[UNIMPLEMENTED: Writing IO Register]\n{:19}", "")},
            0xFF80..=0xFFFE => self.ram[loc as usize] = val, // HRAM
            0xFFFF => self.reg_ie.data = val,
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

    pub fn set_recievables(&mut self, recievables: Recievables) {
        self.ppu.set_recievables(recievables.clone())
    }


}

pub struct InterruptRegister {
    pub data: u8,
}
impl InterruptRegister {
    pub fn has_interrupts(&mut self) -> bool {
        (self.data & 0b11111) > 0
    }
    pub fn get_vblank(&mut self) -> bool {
        (self.data & 0b1) > 0
    }
    pub fn get_lcdstat(&mut self) -> bool {
        (self.data & 0b10) > 0
    }
    pub fn get_joypad(&mut self) -> bool {
        (self.data & 0b100) > 0
    }
    pub fn get_timer(&mut self) -> bool {
        (self.data & 0b1000) > 0
    }
    pub fn get_serial(&mut self) -> bool {
        (self.data & 0b10000) > 0
    }
}
