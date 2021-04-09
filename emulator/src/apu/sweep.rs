use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Sweep {
  pub period: u8,
  pub direction: u8,
  pub shift: u8,
}

impl Sweep {
  pub fn new() -> Self {
    Sweep {
      period: 0,
      direction: 0,
      shift: 0,
    }
  }

  pub fn read(&self) -> u8 {
    let direction: u8 = self.direction << 3;
    let period: u8 = self.period << 4;
    direction | period | self.shift
  }

  pub fn write(&mut self, value: u8) {
    self.period = (value >> 4) & 0x07;
    self.direction = (value >> 3) & 1;
    self.shift = value & 0x07;
  }
}
