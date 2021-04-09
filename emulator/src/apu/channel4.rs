use super::envelope::Envelope;
use serde::{Deserialize, Serialize};

const DIVISORS: [u16; 8] = [8, 16, 32, 48, 64, 80, 96, 112];

#[derive(Serialize, Deserialize, Clone)]
pub struct Channel4 {
  counter_selection: bool,
  counter_step: u8,
  dac_enabled: bool,
  dividing_ratio: u8,
  enabled: bool,
  pub envelope: Envelope,
  envelope_period_counter: u8,
  envelope_running: bool,
  frequency_count: u16,
  length_counter: u8,
  lfsr: u16,
  pub output_volume: u8,
  sequence_pointer: u32,
  shift_clock_frequency: u8,
  pub status: bool,
  volume: u8,
}

impl Channel4 {
  pub fn new() -> Self {
    Channel4 {
      counter_selection: false,  // NR 44 bit 6
      counter_step: 0,           // Counter step
      dac_enabled: true,         // Condition to check if all of envelope properties are set
      dividing_ratio: 0,         // Dividing ratio
      enabled: false,            // Condition to check if channel is enabled
      envelope: Envelope::new(), // NR 42 Enevlope
      envelope_period_counter: 0,
      envelope_running: false,   // Condition to check whether envelope is on or off
      frequency_count: 0,        // Actual frequency value that is updated
      length_counter: 0,         // NR 41 5-0
      lfsr: 0,                   // linear feedback shift register
      output_volume: 0,          // Volume uesd for mixing
      sequence_pointer: 0,       // pointer to keep track of wave_pattern location
      shift_clock_frequency: 0,  // Shift clock frequency
      status: false,             // NR 44 bit 7
      volume: 0,                 // Actual envelope volume that is updated
    }
  }

  pub fn read(&self, loc: u16) -> u8 {
    match loc {
      0xFF20 => self.length_counter,
      0xFF21 => self.envelope.read(),
      0xFF22 => self.shift_clock_frequency << 4 | self.counter_step << 3 | self.dividing_ratio,
      0xFF23 => {
        let counter_selection_bit = if self.counter_selection { 0x40 } else { 0 };
        counter_selection_bit
      }
      _ => panic!("Channel 4 read register out of range: {:04X}", loc),
    }
  }

  pub fn write(&mut self, loc: u16, val: u8) {
    match loc {
      0xFF20 => {
        self.length_counter = val & 0x3F;
      }
      0xFF21 => {
        self.dac_enabled = (val & 0xF8) != 0;
        self.envelope.write(val);
      }
      0xFF22 => {
        self.shift_clock_frequency = val >> 4;
        self.counter_step = (val >> 3) & 0x01;
        self.dividing_ratio = val & 0x07;
      }
      0xFF23 => {
        self.status = if (val & 0x80) == 0x80 { true } else { false };
				self.counter_selection = if (val & 0x40) == 0x40 { true } else { false };

        if self.status {
          self.initialize();
        }
      }
      _ => panic!("Channel 4 write register out of range: {:04X}", loc),
    }
  }

  pub fn tick(&mut self) {
    if self.frequency_count > 0 {
      self.frequency_count -= 1;
    } else if self.frequency_count == 0 {
      self.frequency_count = DIVISORS[self.dividing_ratio as usize] << self.shift_clock_frequency;
      let result = self.lfsr & 1 ^ ((self.lfsr >> 1) & 1);
      self.lfsr >>= 1;
      self.lfsr |= result << 14;
      if self.counter_step == 1 {
        self.lfsr &= !0x40;
        self.lfsr |= result << 6;
      }

      if self.enabled && self.dac_enabled && (self.lfsr & 1 == 0) {
        self.output_volume = self.volume;
      } else {
        self.output_volume = 0;
      }
    }
  }

  // Initializes channel by resetting all values
  fn initialize(&mut self) {
    self.enabled = true;
    if self.length_counter == 0 {
      self.length_counter = 63;
    }
    self.frequency_count = DIVISORS[self.dividing_ratio as usize] << self.shift_clock_frequency as u16;
    self.envelope_running = true;
    self.envelope_period_counter = self.envelope.period;
    self.volume = self.envelope.initial_volume;
    self.lfsr = 0x7FFF;
  }

  pub fn length_step(&mut self) {
    if self.counter_selection && self.length_counter > 0 {
      self.length_counter -= 1;

      if self.length_counter == 0 {
        self.status = false;
      }
    }
  }

  // NR 42 Enevelope register tick
  pub fn envelope_step(&mut self) {
    if self.envelope_period_counter > 0 {
      self.envelope_period_counter -= 1;
    } else if self.envelope_period_counter == 0 {
      self.envelope_period_counter = if self.envelope_period_counter == 0 {
        8
      } else {
        self.envelope.period
      };

      // println!("Envelope length: {}", self.envelope.length);
      if self.envelope_running {
        // println!("Envelope running & Envelope length > 0");
        // println!("Volume is: {}", self.volume);
        if self.envelope.direction == 1 && self.volume < 15 {
          // println!("Increasing volume");
          self.volume += 1;
        } else if self.envelope.direction == 0 && self.volume > 0 {
          // println!("Decreasing volume");
          self.volume -= 1;
        }
      }

      if self.volume == 0 || self.volume == 15 {
        self.envelope_running = false;
      }
    }
  }

  // Waveform output is the first bit of lfsr inverted. Returns 0 or 1
  fn waveform_output(&self) -> u8 {
    ((!self.lfsr) & 1) as u8
  }

  // Return a value between 0-15
  fn volume_output(&self) -> u8 {
    if self.enabled {
      self.waveform_output() * self.volume
    } else {
      0
    }
  }

  // Return a value between [-1.0,+1.0]
  pub fn dac_output(&self) -> f32 {
    if self.dac_enabled {
      let vol_output = self.volume_output() as f32;
      (vol_output / 7.5) - 1.0
    } else {
      0.0
    }
  }

}


#[cfg(test)]
mod test {
    use super::*;

    fn create_test_channel4() -> Channel4 {
        Channel4::new()
    }

		#[test]
    fn test_NR41_read_write () {
        let mut ch4 = create_test_channel4();
				ch4.write(0xFF20, 0xFF);

				assert_eq!(ch4.length_counter, 0x3F);
				assert_eq!(ch4.read(0xFF20), 0x3F);
    }

		#[test]
    fn test_NR42_read_write () {
      let mut ch4 = create_test_channel4();
      ch4.write(0xFF21, 0xFF);

      assert_eq!(ch4.envelope.initial_volume, 15);
      assert_eq!(ch4.envelope.direction, 1);
      assert_eq!(ch4.envelope.period, 7);
      assert_eq!(ch4.read(0xFF21), 0xFF);
    }

		#[test]
    fn test_NR43_read_write () {
        let mut ch4 = create_test_channel4();
				ch4.write(0xFF22, 0xFF);

				assert_eq!(ch4.shift_clock_frequency, 15);
        assert_eq!(ch4.counter_step, 1);
        assert_eq!(ch4.dividing_ratio, 7);
				assert_eq!(ch4.read(0xFF22), 0xFF);
    }

		
		#[test]
    fn test_NR44_read_write () {
        let mut ch4 = create_test_channel4();
				ch4.write(0xFF23, 0xFF);

				assert_eq!(ch4.status, true);
				assert_eq!(ch4.counter_selection, true);
				assert_eq!(ch4.read(0xFF23), 0x40);
    }
	}