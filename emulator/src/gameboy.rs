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
    pub fn empty() -> Self {
        Self {
            cpu: CPU::new(crate::bus::Bus::new(ROM{ data:Vec::new() })),
        }
    }

    pub fn set_state(&mut self, new_state: Gameboy) {
        self.cpu = new_state.cpu;
    }

    pub fn tick(&mut self) {
        self.cpu.tick()
    }
    pub fn get_screen(&self) -> crate::ppu::Canvas {
        return self.cpu.get_screen()
    }

    pub fn print_cpu_state(&self) {
        self.cpu.print_state();
    }
}

impl ROM {
    pub fn is_bootrom(&self) -> bool {
        true
    }

    pub fn from_data(data: Vec<u8>) -> Self {
        ROM {data}
    }
    pub fn read(&self, loc: u16) -> u8 {
       if loc < 0x00FF && self.is_bootrom() {
           self.data[loc as usize]
       } else if loc < 0x4000 { // Bank 0
           0 // TODO Static bank
       } else { // Bank 1-N (Swappable)
           0 // TODO Swap banks
       }
    }
    pub fn write(&mut self, loc: u16, val: u8) {
        // Some cartriges provide writable memory for saving
        // TODO: Find out which areas are write protected
        self.data[loc as usize] = val
    }
}
