use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Envelope {
  pub initial_volume: u8,
  pub direction: u8,
  pub length: i32,
  pub length_load: u8,
}

impl Envelope {
  pub fn new() -> Self {
    Envelope {
      initial_volume: 0,
      direction: 0,
      length: 0,
      length_load: 0,
    }
  }

  pub fn read(&self) -> u8 {
    let direction: u8 = self.direction << 3;
    let initial_volume: u8 = self.initial_volume << 4;
    initial_volume | direction | self.length_load
  }

  pub fn write(&mut self, value: u8) {
    self.initial_volume = (value >> 4) & 0x0F;
    self.direction = (value >> 3) & 1;
    self.length_load = value & 0x07;
  }
}
