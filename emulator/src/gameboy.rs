use crate::cpu::CPU;
use crate::cartridge::Cartridge;


pub struct GameboyBuilder {
    rom: Option<Cartridge>,
}

pub struct Gameboy {
    cpu: CPU,
}

impl GameboyBuilder {
    pub fn new() -> Self {
        return Self{rom: None}
    }

    pub fn load_rom(mut self, rom: Cartridge) -> Self {
        self.rom = Some(rom);
        self
    }

    pub fn build(&self) -> Gameboy {
        if let Some(rom) = self.rom.clone() {
          return Gameboy {
              cpu: CPU::post_bootrom(crate::bus::Bus::new(rom))
          }
        }
        panic!("Builder not fully initialized")
    }
}

impl Gameboy {
    pub fn set_state(&mut self, new_state: Gameboy) {
        self.cpu = new_state.cpu;
    }

    pub fn tick(&mut self) {
        self.cpu.tick()
    }
    pub fn get_screen(&self) -> crate::ppu::Screen {
        return self.cpu.get_screen()
    }

    pub fn print_cpu_state(&self) {
        self.cpu.print_state();
    }

    pub fn print_alt(&mut self) {
        self.cpu.print_alt_state();
    }

    pub fn set_debug_options(&mut self, b: crate::cpu::DebugOptions) {
        self.cpu.set_debug_options(b)
    }
}
