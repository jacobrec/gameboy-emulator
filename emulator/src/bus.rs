use crate::cartridge::Cartridge;
use crate::cpu_recievable::Recievables;
// use rodio::{buffer::SamplesBuffer, source::Source, Decoder, OutputStream, OutputStreamHandle};
use serde::{Deserialize, Serialize};

use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

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

#[derive(Serialize, Deserialize)]
pub struct Bus {
    rom: Cartridge,
    ram: Vec<u8>, // Most of this will get shadowed as the code is filled in
    ppu: crate::ppu::PPU,
    apu: crate::apu::APU,
    timer: crate::timer::Timer,
    pub joypad: Joypad,
    bios: Option<Vec<u8>>,
    pub reg_ie: InterruptRegister, // 0xFFFF
    pub reg_if: InterruptRegister, // 0xFF0F
    #[serde(skip)]
    testfile: Option<File>, // TODO: remove. This is for testing only. Specifcally to hold the outputted serial data when running blargg's testroms
}

impl Clone for Bus {
    fn clone(&self) -> Self {
        Bus {
            rom: self.rom.clone(),
            ram: self.ram.clone(),
            ppu: self.ppu.clone(),
            apu: self.apu.clone(),
            timer: self.timer.clone(),
            reg_if: self.reg_if.clone(),
            reg_ie: self.reg_ie.clone(),
            bios: self.bios.clone(),
            joypad: self.joypad.clone(),
            testfile: None,
        }
    }
}

impl Bus {
    pub fn get_screen(&self) -> &crate::ppu::Screen {
        self.ppu.get_screen()
    }

    pub fn get_audio_buffer(&self) -> [f32; 4096] {
        return self.apu.get_audio_buffer();
    }
    pub fn get_audio_buffer_status(&self) -> bool {
        self.apu.get_audio_buffer_status()
    }
    pub fn set_audio_buffer_status(&mut self, status: bool) {
        self.apu.set_audio_buffer_status(status);
    }
    pub fn get_canvas(&self) -> crate::ppu::Canvas {
        return self.ppu.get_canvas();
    }

    pub fn with_bios(rom: Cartridge, bios: Vec<u8>) -> Self {
        let mut bus = Self::new(rom);
        bus.bios = Some(bios);
        bus
    }

    pub fn new(rom: Cartridge) -> Self {
        let ram = [0u8; 0x10000].to_vec();
        let ppu = crate::ppu::PPU::new();
        let apu = crate::apu::APU::new();
        let timer = crate::timer::Timer::new();
        let reg_if = InterruptRegister { data: 0 };
        let reg_ie = InterruptRegister { data: 0 };
        let bios = None;
        let joypad = Joypad {
            pins: 0b1111,
            last: 0,
        };

        let mut testfile: Option<File> = OpenOptions::new()
            .write(true)
            .create(true)
            .open("serial")
            .ok();
        Bus {
            rom,
            ram,
            ppu,
            apu,
            timer,
            joypad,
            reg_if,
            reg_ie,
            bios,
            testfile,
        }
    }

    pub fn read(&self, loc: u16) -> u8 {
        match loc {
            0x0000..=0xFF if self.bios.is_some() => self.bios.as_ref().unwrap()[loc as usize],
            0x0000..=0x3FFF => self.rom.read(loc),
            0x4000..=0x7FFF => self.rom.read(loc), // upper rom banks
            0x8000..=0x9FFF => self.ppu.read(loc),
            0xC000..=0xCFFF => self.ram[loc as usize],
            0xD000..=0xDFFF => self.ram[loc as usize],
            0xE000..=0xFDFF => self.read(loc - 0xE000 + 0xC000),
            0xA000..=0xBFFF => self.rom.read(loc), // external RAM
            0xFE00..=0xFE9F => self.ppu.readOAM(loc),
            0xFEA0..=0xFEFF => 0x00, // Unused
            0xFF00..=0xFF00 => self.joypad.pins,
            0xFF04..=0xFF07 => self.timer.read(loc),
            0xFF0F => self.reg_if.data,
            0xFF10..=0xFF26 => self.apu.read(loc),
            0xFF30..=0xFF3F => self.apu.read(loc),
            0xFF40..=0xFF4F => { self.ppu.read_reg(loc) },
            0xFF00..=0xFF7F => { print!("[UNIMPLEMENTED: Reading IO Register: {:04X}]\n{:19}", loc, ""); 0},
            0xFF80..=0xFFFE => self.ram[loc as usize], // HRAM
            0xFFFF => self.reg_ie.data,
            _ => panic!("Unimplemented read range: {:04X}", loc),
        }
    }

    pub fn write(&mut self, loc: u16, val: u8) {
        match loc {
            0x0000..=0x3FFF => self.rom.write(loc, val),
            0x4000..=0x7FFF => self.rom.write(loc, val), // upper rom banks
            0x8000..=0x9FFF => self.ppu.write(loc, val),
            0xC000..=0xCFFF => self.ram[loc as usize] = val,
            0xD000..=0xDFFF => self.ram[loc as usize] = val,
            0xE000..=0xFDFF => self.write(loc - 0xE000 + 0xC000, val),
            0xA000..=0xBFFF => self.rom.write(loc, val), // external RAM
            0xFE00..=0xFE9F => self.ppu.writeOAM(loc, val),
            0xFF00..=0xFF00 => self.joypad.write(val),
            0xFF50 => self.bios = None,
            0xFEA0..=0xFEFF => (), // Unused
            0xFF01 if self.testfile.is_some() => {
                self.testfile.as_mut().unwrap().write(&[val]);
            } // Serial transfer data
            0xFF02 => (),          // Serial transfer control
            0xFF04..=0xFF07 => self.timer.write(loc, val),
            0xFF0F => self.reg_if.data = val,
            0xFF10..=0xFF26 => self.apu.write(loc, val),
            0xFF30..=0xFF3F => self.apu.write(loc, val),
            0xFF40..=0xFF4F => self.ppu.write_reg(loc, val),
            0xFF00..=0xFF7F => {
                print!(
                    "[UNIMPLEMENTED: Writing IO Register: {:04X}]\n{:19}",
                    loc, ""
                )
            }
            0xFF80..=0xFFFE => self.ram[loc as usize] = val, // HRAM
            0xFFFF => self.reg_ie.data = val,
            _ => panic!("Unimplemented write range: {:04X}", loc),
        }
    }

    fn dma_transfer(&mut self) {
        match self.ppu.dma.next() {
            Some((from, to)) => {
                let val = self.read(from);
                self.write(to, val);
            }
            None => (),
        }
    }

    pub fn cpu_tick(&mut self) {
        // CPU runs at 1MHz
        // PPU runs at 2MHz
        self.ppu.tick();
        self.ppu.tick();
        self.timer.tick();
        self.dma_transfer();
        // TODO: call apu tick
        self.apu.tick();

        // let buffer = SamplesBuffer::new(1, 44100, [0.123, 0.234, 0.532, 0.523, 0.25, 0.76, 0.85]);
        // let result = stream_handle.play_raw(buffer);
        // let data: Vec<f32> = (0..4096).map(|n| -0.5 + (n % 2) as f32).collect();
        // let source2 = rodio::buffer::SamplesBuffer::new(1, 500, data);
        // self.stream_handle.play_raw(source2);
        // std::thread::sleep(std::time::Duration::from_secs(5));
    }

    pub fn stack_push(&mut self, sp: usize, data: u8) {
        self.ram[sp] = data;
    }

    pub fn stack_pop(&self, sp: usize) -> u8 {
        self.ram[sp]
    }

    pub fn set_recievables(&mut self, recievables: Recievables) {
        self.ppu.set_recievables(recievables.clone());
        self.timer.set_recievables(recievables.clone());
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
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

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Joypad {
    pins: u8,
    last: u8,
}
impl Joypad {
    pub fn set_action(&mut self, high: bool) {
        self.pins = self.pins & 0b011111 | (if high { 1 } else { 0 } << 5)
    }
    pub fn set_direction(&mut self, high: bool) {
        self.pins = self.pins & 0b101111 | (if high { 1 } else { 0 } << 4)
    }
    pub fn set_down_start(&mut self, low: bool) {
        self.pins = self.pins & 0b110111 | (if low { 0 } else { 1 } << 3)
    }
    pub fn set_up_select(&mut self, low: bool) {
        self.pins = self.pins & 0b111011 | (if low { 0 } else { 1 } << 2)
    }
    pub fn set_left_b(&mut self, low: bool) {
        self.pins = self.pins & 0b111101 | (if low { 0 } else { 1 } << 1)
    }
    pub fn set_right_a(&mut self, low: bool) {
        self.pins = self.pins & 0b111110 | (if low { 0 } else { 1 } << 0)
    }
    pub fn write(&mut self, val: u8) {
        // lower 4 bits are read only
        self.pins = (self.pins & 0b00001111) | (val & 0b00110000);
        self.update_joypad(self.last);
    }

    pub fn update_joypad(&mut self, buttonmap: u8) {
        self.last = buttonmap;
        use crate::gameboy;
        let dir = self.pins & 0b010000 == 0;
        let act = self.pins & 0b100000 == 0;
        self.set_down_start(
            dir && (buttonmap & gameboy::BUT_DOWN > 0)
                || act && (buttonmap & gameboy::BUT_START > 0),
        );
        self.set_up_select(
            dir && (buttonmap & gameboy::BUT_UP > 0)
                || act && (buttonmap & gameboy::BUT_SELECT > 0),
        );
        self.set_left_b(
            dir && (buttonmap & gameboy::BUT_LEFT > 0) || act && (buttonmap & gameboy::BUT_B > 0),
        );
        self.set_right_a(
            dir && (buttonmap & gameboy::BUT_RIGHT > 0) || act && (buttonmap & gameboy::BUT_A > 0),
        );
    }
}
