use crate::cartridge::Cartridge;
use crate::cpu::CPU;

pub const BUT_START: u8 = 0b1;
pub const BUT_SELECT: u8 = 0b10;
pub const BUT_UP: u8 = 0b100;
pub const BUT_DOWN: u8 = 0b1000;
pub const BUT_LEFT: u8 = 0b10000;
pub const BUT_RIGHT: u8 = 0b100000;
pub const BUT_A: u8 = 0b1000000;
pub const BUT_B: u8 = 0b10000000;

pub struct GameboyBuilder {
    rom: Option<Cartridge>,
    bios: Option<Vec<u8>>,
}

pub struct Gameboy {
    pub(crate) cpu: CPU,
    buttons_pressed: u8,
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
                    buttons_pressed: 0,
                };
            } else {
                return Gameboy {
                    cpu: CPU::post_bootrom(crate::bus::Bus::new(rom)),
                    buttons_pressed: 0,
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
    pub fn get_screen(&self) -> &crate::ppu::Screen {
        return self.cpu.get_screen();
    }
    pub fn get_canvas(&self) -> crate::ppu::Canvas {
        return self.cpu.get_canvas();
    }

    pub fn get_audio_buffer(&self) -> [f32; crate::apu::SAMPLE_SIZE] {
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

    pub fn set_debug_options(&mut self, b: crate::debugger::DebugOptions) {
        self.cpu.set_debug_options(b)
    }
    pub fn debug_break(&mut self) {
        self.cpu.debug_options.debug_step = true
    }

    pub fn save(&self) -> crate::cpu::SaveState {
        crate::cpu::SaveState::create(&self.cpu)
    }

    pub fn load(&mut self, save: &crate::cpu::SaveState) {
        self.cpu = save.load()
    }

    pub fn button_down(&mut self, button: u8) {
        self.buttons_pressed |= button;
        self.cpu.update_joypad_register(self.buttons_pressed)
    }

    pub fn button_up(&mut self, button: u8) {
        self.buttons_pressed &= !button;
        self.cpu.update_joypad_register(self.buttons_pressed)
    }
}
