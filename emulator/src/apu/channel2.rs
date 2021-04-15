use super::envelope::Envelope;
use super::pattern::*;
use serde::{Deserialize, Serialize};

const WAVE_PATTERN: [[u8; 8]; 4] = [
	[0, 0, 0, 0, 0, 0, 0, 1],
	[1, 0, 0, 0, 0, 0, 0, 1],
	[1, 0, 0, 0, 0, 1, 1, 1],
	[0, 1, 1, 1, 1, 1, 1, 0],
];

#[derive(Serialize, Deserialize, Clone)]
pub struct Channel2 {
	counter_selection: bool,
	dac_enabled: bool,
	enabled: bool,
	pub envelope: Envelope,
	envelope_period_counter: u8,
	envelope_running: bool,
	frequency_count: u16,
	frequency_load: u16,
	length_counter: u8,
	pub output_volume: u8,
	sequence_pointer: u32,
	pub status: bool,
	volume: u8,
	pub wave_pattern: Pattern,
	capacitor: f32,
}

impl Channel2 {
	pub fn new() -> Self {
		Channel2 {
			counter_selection: false,  // NR 24 bit 6
			dac_enabled: false,        // Condition to check if all of envelope properties are set
			enabled: false,            // Condition to check if channel is enabled
			envelope: Envelope::new(), // NR 22 Enevlope
			envelope_running: false,   // Condition to check whether envelope is on or off
			envelope_period_counter: 0,
			frequency_count: 0,                 // Actual frequency value that is updated
			frequency_load: 0,                  // NR 23 and NR 24 bit 2-0
			length_counter: 0,                  // NR 21 5-0
			output_volume: 0,                   // Volume uesd for mixing
			sequence_pointer: 0,                // pointer to keep track of wave_pattern location
			status: false,                      // NR 24 bit 7
			volume: 0,                          // Actual envelope volume that is updated
			wave_pattern: Pattern::HalfQuarter, // NR 21 bit 7-6
			capacitor: 0.0,
		}
	}

	pub fn read(&self, loc: u16) -> u8 {
		match loc {
			0xFF15 => 0xFF,
			0xFF16 => {
				let pattern_bits = pattern_to_u8(self.wave_pattern);
				0x3F | pattern_bits << 6
			}
			0xFF17 => self.envelope.read(),
			0xFF18 => 0xFF | (self.frequency_load & 0x00FF) as u8,
			0xFF19 => {
				let counter_selection_bit = if self.counter_selection { 1 << 6 } else { 0 };
				0xBF | counter_selection_bit
			}
			_ => panic!("Channel 2 read register out of range: {:04X}", loc),
		}
	}

	pub fn write(&mut self, loc: u16, val: u8) {
		match loc {
			0xFF15 => (),
			0xFF16 => {
				let pattern_bits = val >> 6;
				self.wave_pattern = u8_to_pattern(pattern_bits).unwrap();
				self.length_counter = 64 - (val & 0x3F);
			}
			0xFF17 => {
				self.envelope.write(val);
				self.dac_enabled = val & 0xF8 != 0;
				self.volume = self.envelope.initial_volume;
			}
			0xFF18 => {
				self.frequency_load = self.frequency_load & 0x0700 | val as u16;
			}
			0xFF19 => {
				self.status = if (val & 0x80) == 0x80 { true } else { false };
				self.counter_selection = if (val & 0x40) == 0x40 { true } else { false };
				self.frequency_load = self.frequency_load & 0x00FF | ((val & 0x07) as u16) << 8;

				if self.status {
					self.initialize();
				}
			}
			_ => panic!("Channel 2 write register out of range: {:04X}", loc),
		}
	}

	// Channel NR23/NR24 frequency tick
	pub fn tick(&mut self) {
		if self.frequency_count > 0 {
			self.frequency_count -= 1;
		} else if self.frequency_count == 0 {
			self.frequency_count = (2048 - self.frequency_load) * 4;
			self.sequence_pointer = (self.sequence_pointer + 1) % 8;
		}
	}

	// NR21 Pattern register tick (sound length)
	pub fn length_step(&mut self) {
		if self.counter_selection && self.length_counter > 0 {
			self.length_counter -= 1;

			if self.length_counter == 0 {
				self.status = false;
			}
		}
	}

	// NR22 Enevelope register tick
	pub fn envelope_step(&mut self) {
		if self.envelope_period_counter > 0 {
			self.envelope_period_counter -= 1;
		} else {
			self.envelope_period_counter = if self.envelope_period_counter == 0 {
				8
			} else {
				self.envelope.period
			};

			// println!("Envelope length: {}", self.envelope.length);
			if self.envelope_running && self.envelope.period > 0 {
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

	// Initializes channel by resetting all values
	fn initialize(&mut self) {
		self.enabled = true;
		if self.length_counter == 0 {
			self.length_counter = 64;
		}
		self.frequency_count = (2048 - self.frequency_load) * 4;
		self.envelope_running = true;
		self.envelope_period_counter = self.envelope.period;
		self.volume = self.envelope.initial_volume;
	}

	// pub fn dac_output(&self) -> f32 {
	// 	if self.dac_enabled {
	// 		let mut duty_output = 0.0;
	// 		if self.enabled {
	// 			duty_output = WAVE_PATTERN[pattern_to_u8(self.wave_pattern) as usize][self.sequence_pointer as usize] as f32;
	// 		}
	// 		let vol_output: f32 = self.volume as f32 * duty_output;
	// 		(vol_output / 7.5) - 1.0
	// 	} else {
	// 		0.0
	// 	}
	// }

	pub fn dac_output(&mut self) -> f32 {
		let mut dac_output = 0.0;

		if self.dac_enabled {
			let dac_input = WAVE_PATTERN[pattern_to_u8(self.wave_pattern) as usize]
				[self.sequence_pointer as usize] as f32;
			dac_output = dac_input - self.capacitor;
			self.capacitor = dac_input - dac_output * 0.996;
		}

		dac_output
	}
}

#[cfg(test)]
mod test {
	use super::*;

	fn create_test_channel2() -> Channel2 {
		Channel2::new()
	}

	#[test]
	fn test_NR21_read_write() {
		let mut ch2 = create_test_channel2();
		ch2.write(0xFF16, 0xFF);

		let pattern_bits = pattern_to_u8(ch2.wave_pattern);
		assert_eq!(pattern_bits, 3);
		assert_eq!(ch2.length_counter, 1);

		assert_eq!(ch2.read(0xFF16), 0xFF);
	}

	#[test]
	fn test_NR22_read_write() {
		let mut ch2 = create_test_channel2();
		ch2.write(0xFF17, 0xFF);

		assert_eq!(ch2.envelope.initial_volume, 15);
		assert_eq!(ch2.envelope.direction, 1);
		assert_eq!(ch2.envelope.period, 7);
		assert_eq!(ch2.read(0xFF17), 0xFF);
	}

	#[test]
	fn test_NR23_read_write() {
		let mut ch2 = create_test_channel2();
		ch2.write(0xFF18, 0xFF);

		assert_eq!(ch2.frequency_load, 255);
		assert_eq!(ch2.read(0xFF18), 0xFF);
	}

	#[test]
	fn test_NR24_read_write() {
		let mut ch2 = create_test_channel2();
		ch2.write(0xFF19, 0xFF);

		assert_eq!(ch2.status, true);
		assert_eq!(ch2.counter_selection, true);
		assert_eq!(ch2.frequency_load, 1792);
		assert_eq!(ch2.read(0xFF19), 0xff);
	}
}
