use super::envelope::Envelope;
const DIVISORS: [i32; 8] = [8, 16, 32, 48, 64, 80, 96, 112];

#[derive(Clone)]
pub struct Channel4 {
  counter_selection: bool,
  counter_step: u8,
  dac_enabled: bool,
  dividing_ratio: u8,
  enabled: bool,
  pub envelope: Envelope,
  envelope_running: bool,
  frequency_count: i32,
  frequency_load: u16,
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
      envelope_running: false,   // Condition to check whether envelope is on or off
      frequency_count: 0,        // Actual frequency value that is updated
      frequency_load: 0,         // NR 43 and NR 44 bit 2-0
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
        let status_bit = if self.status { 0x80 } else { 0 };
        let counter_selection_bit = if self.counter_selection { 0x40 } else { 0 };

        status_bit | counter_selection_bit
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
        self.shift_clock_frequency = val >> 4 & 0x7;
        self.counter_step = val >> 3 & 0x01;
        self.dividing_ratio = val & 0x03;
      }
      0xFF23 => {
        self.status = if val >> 7 == 1 { true } else { false };
        self.counter_selection = if val >> 6 == 1 { true } else { false };

        if self.status {
          self.initialize();
        }
      }
      _ => panic!("Channel 4 write register out of range: {:04X}", loc),
    }
  }

  pub fn tick(&mut self) {
    self.frequency_count -= 1;
    if self.frequency_count <= 0 {
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
    self.frequency_count = DIVISORS[self.dividing_ratio as usize] << self.shift_clock_frequency;
    self.envelope_running = true;
    self.envelope.length = self.envelope.length_load as i32;
    self.volume = self.envelope.volume;
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
    self.envelope.length -= 1;
    if self.envelope.length <= 0 {
      self.envelope.length = self.envelope.length_load as i32;
      if self.envelope.length == 0 {
        self.envelope.length = 8;
      }

      if self.envelope_running && self.envelope.length > 0 {
        if self.envelope.direction == 1 && self.volume < 15 {
          self.volume += 1;
        } else if self.envelope.direction == 0 && self.volume > 0 {
          self.volume -= 1;
        }
      }

      if self.volume == 0 || self.volume == 15 {
        self.envelope_running = false;
      }
    }
  }
}
