use crate::cpu::CPU;

#[derive(Clone)]
pub struct ROM {
    data: Vec<u8>
}

pub struct GameboyBuilder {
    rom: Option<ROM>,
}

pub struct Gameboy {
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
              cpu: CPU::new(crate::bus::Bus::new(rom))
          }
        }
        panic!("Builder not fully initialized")
    }
}

impl Gameboy {
    pub const fn empty() -> Self {
        Self {
            cpu: CPU::new(crate::bus::Bus::new(ROM{ data:Vec::new() })),
        }
    }

    pub fn set_state(&mut self, new_state: Gameboy) {
        self.cpu = new_state.cpu;
    }

    pub fn tick(&mut self) {
    }
}

impl ROM {
    pub fn from_data(data: Vec<u8>) -> Self {
        ROM {data}
    }
    pub fn read(self, loc: u16) -> u8 {
        self.data[loc as usize]
    }
    pub fn write(&mut self, loc: u16, val: u8) {
        self.data[loc as usize] = val
    }
}
