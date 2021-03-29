use super::envelope::Envelope;
use super::pattern::*;
use super::sweep::Sweep;

const WAVE_PATTERN: [[i32; 8]; 4] = [
	[-1, -1, -1, -1, 1, -1, -1, -1],
	[-1, -1, -1, -1, 1, 1, -1, -1],
	[-1, -1, 1, 1, 1, 1, -1, -1],
	[1, 1, 1, 1, -1, -1, 1, 1],
];

/*
The variable names with _load are the variables we use to store the initial values.
All updates to the values to perform any computation is done in the variables without
the _load in the variable name.
*/
#[derive(Clone)]
pub struct Channel1 {
	counter_selection: bool,
	dac_enabled: bool,
	enabled: bool,
	pub envelope: Envelope,
	envelope_running: bool,
	frequency_count: i32,
	frequency_load: u16,
	length_counter: u8,
	pub output_volume: u8,
	sequence_pointer: u32,
	pub status: bool,
	pub sweep: Sweep,
	sweep_enable: bool,
	sweep_shadow: u16,
	sweep_time: i32,
	volume: u8,
	pub wave_pattern: Pattern,
}

impl Channel1 {
	pub fn new() -> Channel1 {
		Channel1 {
			counter_selection: false,           // NR 14 bit 6
			dac_enabled: true,                  // Condition to check if all of envelope properties are set
			enabled: false,                     // Condition to check if channel is enabled
			envelope: Envelope::new(),          // NR 12 Enevlope
			envelope_running: false,            // Condition to check whether envelope is on or off
			frequency_count: 0,                 // Actual frequency value that is updated
			frequency_load: 0,                  // NR 13 and NR 14 bit 2-0
			length_counter: 0,                  // NR 11 5-0
			output_volume: 0,                   // Volume uesd for mixing
			sequence_pointer: 0,                // pointer to keep track of wave_pattern location
			status: false,                      // NR 14 bit 7
			sweep: Sweep::new(),                // NR 10 register
			sweep_enable: false,                // Condition to check whether sweep is on or off
			sweep_shadow: 0,                    // Sweep shadow register
			sweep_time: 0,                      // Actual sweep time that is updated
			volume: 0,                          // Actual envelope volume that is updated
			wave_pattern: Pattern::HalfQuarter, // NR 11 bit 7-6
		}
	}

	pub fn read(&self, loc: u16) -> u8 {
		match loc {
			0xFF10 => self.sweep.read(),
			0xFF11 => {
				let pattern_bits = pattern_to_u8(self.wave_pattern);
				pattern_bits << 6 | self.length_counter & 0x3F
			}
			0xFF12 => self.envelope.read(),
			0xFF13 => (self.frequency_load & 0x00FF) as u8,
			0xFF14 => {
				let status_bit = if self.status { 0x80 } else { 0 };
				let counter_selection_bit = if self.counter_selection { 0x40 } else { 0 };
				let frequency_high_bits = (self.frequency_load & 0x07) as u8;

				status_bit | counter_selection_bit | frequency_high_bits
			}
			_ => panic!("Channel 1 read register out of range: {:04X}", loc),
		}
	}

	pub fn write(&mut self, loc: u16, val: u8) {
		match loc {
			0xFF10 => {
				self.sweep.write(val);
			}
			0xFF11 => {
				let pattern_bits = val >> 6;
				self.wave_pattern = u8_to_pattern(pattern_bits).unwrap();
				self.length_counter = val & 0x3F;
			}
			0xFF12 => {
				self.envelope.write(val);
				self.dac_enabled = val & 0xF8 != 0;
				self.volume = self.envelope.volume;
			}
			0xFF13 => {
				self.frequency_load = self.frequency_load & 0x0700 | val as u16;
			}
			0xFF14 => {
				self.status = if (val & 0x80) == 0x80 { true } else { false };
				self.counter_selection = if (val & 0x40) == 0x40 { true } else { false };
				self.frequency_load = self.frequency_load & 0x00FF | ((val & 0x07) as u16) << 8;

				if self.status {
					self.initialize();
				}
			}
			_ => panic!("Channel 1 write register out of range: {:04X}", loc),
		}
	}

	// Channel NR13/NR14 frequency tick
	pub fn tick(&mut self) {
		self.frequency_count -= 1;
		if self.frequency_count <= 0 {
			self.frequency_count = (2048 - self.frequency_load as i32) * 4;
			self.sequence_pointer = (self.sequence_pointer + 1) & 0x07;
		}
		if self.enabled && self.dac_enabled {
			self.output_volume = self.volume;
		} else {
			self.output_volume = 0;
		}

		if WAVE_PATTERN[pattern_to_u8(self.wave_pattern) as usize][self.sequence_pointer as usize] == -1
		{
			self.output_volume = 0;
		}
	}

	// NR11 Pattern register tick (sound length)
	pub fn length_step(&mut self) {
		if self.counter_selection && self.length_counter > 0 {
			self.length_counter -= 1;

			if self.length_counter == 0 {
				self.status = false;
			}
		}
	}

	// NR12 Enevelope register tick
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

	// NR11 Sweep register tick
	pub fn sweep_step(&mut self) {
		self.sweep_time -= 1;
		if self.sweep_time <= 0 {
			self.sweep_time = self.sweep.time as i32;
			if self.sweep_time == 0 {
				self.sweep_time = 8;
			}
			if self.sweep_enable && self.sweep_time > 0 {
				let new_frequency = self.sweep_calculation();
				if new_frequency <= 2047 && self.sweep.shift > 0 {
					self.sweep_shadow = new_frequency;
					self.frequency_load = new_frequency;
					self.sweep_calculation();
				}
				self.sweep_calculation();
			}
		}
	}

	// Calculates new frequency based on sweep shift.
	fn sweep_calculation(&mut self) -> u16 {
		let mut new_frequency: u16 = 0;
		new_frequency = self.sweep_shadow >> self.sweep.shift;

		new_frequency = if self.sweep.direction == 0 {
			self.sweep_shadow + new_frequency
		} else {
			self.sweep_shadow - new_frequency
		};

		if new_frequency > 2047 {
			self.enabled = false;
		}

		new_frequency
	}

	// Initializes channel by resetting all values
	fn initialize(&mut self) {
		self.enabled = true;
		if self.length_counter == 0 {
			self.length_counter = 63;
		}

		self.frequency_count = (2048 - self.frequency_load as i32) * 4;
		self.envelope_running = true;
		self.envelope.length = self.envelope.length_load as i32;
		self.volume = self.envelope.volume;
		self.sweep_shadow = self.frequency_load;
		self.sweep_time = self.sweep.time as i32;

		if self.sweep_time == 0 {
			self.sweep_time = 8;
		}
		self.sweep_enable = self.sweep_time > 0 || self.sweep.shift > 0;
		if self.sweep.shift > 0 {
			self.sweep_calculation();
		}
	}
}
