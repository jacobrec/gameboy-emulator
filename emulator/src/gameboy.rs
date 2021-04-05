use crate::cartridge::Cartridge;
use crate::cpu::CPU;

pub struct GameboyBuilder {
    rom: Option<Cartridge>,
    bios: Option<Vec<u8>>,
}

pub struct Gameboy {
    cpu: CPU,
}

impl GameboyBuilder {
    pub fn new() -> Self {
        return Self {
            rom: None,
            bios: None,
        };
    }

    pub fn load_rom(mut self, rom: Cartridge) -> Self {
        self.rom = Some(rom);
        self
    }

    pub fn load_bios(mut self, b: Vec<u8>) -> Self {
        self.bios = Some(b);
        self
    }

    pub fn build(&self) -> Gameboy {
        if let Some(rom) = self.rom.clone() {
            if let Some(bios) = &self.bios {
                return Gameboy {
                    cpu: CPU::with_bios(crate::bus::Bus::with_bios(rom, bios.clone())),
                };
            } else {
                return Gameboy {
                    cpu: CPU::post_bootrom(crate::bus::Bus::new(rom)),
                };
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
        return self.cpu.get_screen();
    }

    pub fn get_audio_buffer(&self) -> [f32; 4096] {
        self.cpu.get_audio_buffer()
    }
    pub fn get_audio_buffer_status(&self) -> bool {
        self.cpu.get_audio_buffer_status()
    }
    pub fn set_audio_buffer_status(&mut self, status: bool) {
        self.cpu.set_audio_buffer_status(status);
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

    pub fn save(&self) -> crate::cpu::SaveState {
        crate::cpu::SaveState::create(&self.cpu)
    }

    pub fn load(&mut self, save: &crate::cpu::SaveState) {
        self.cpu = save.load()
    }
}
