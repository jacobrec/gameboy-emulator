use crate::cpu::CPU;

#[derive(Clone)]
struct ROM {
    data: Vec<u8>
}

struct GameboyBuilder {
    rom: Option<ROM>,
}

struct Gameboy {
    rom: ROM,
    cpu: CPU,
}

impl GameboyBuilder {
    pub fn new() -> Self {
        return Self{rom: None}
    }

    pub fn load_rom(mut self, rom: ROM) -> Self {
        self.rom = Some(rom);
        self
    }

    pub fn build(&self) -> Gameboy {
        if let Some(rom) = self.rom.clone() {
          return Gameboy {
              rom,
              cpu: CPU::new()
          }
        }
        panic!("Builder not fully initialized")
    }
}

impl Gameboy {
}
