use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Channel3 {
	counter_selection: bool,
	dac_enabled: bool,
	enabled: bool,
	frequency_count: u16,
	frequency_load: u16,
	length_counter: u16,
	position_counter: u8,
	pub status: bool,
	volume: u8,
	wave_ram: [u8; 16],
	sample_byte: u8,
	capacitor: f32,
}

impl Channel3 {
	pub fn new() -> Self {
		Channel3 {
			counter_selection: false, // NR 34 bit 6
			dac_enabled: false,       // Condition to check if all of envelope properties are set
			enabled: false,           // Condition to check if channel is enabled
			frequency_count: 0,       // Actual frequency value that is updated
			frequency_load: 0,        // NR 33 and NR 34 bit 2-0
			length_counter: 0,        // NR 31 5-0
			position_counter: 0,      // Wave RAM pointer
			status: false,            // NR 34 bit 7
			volume: 0,                // Actual envelope volume that is updated
			wave_ram: [0; 16],        // Wave RAM
			sample_byte: 0,
			capacitor: 0.0,
		}
	}

	pub fn read(&self, loc: u16) -> u8 {
		match loc {
			0xFF1A => {
				let output = if self.dac_enabled { 0x80 } else { 0 };
				0x7F | output
			}
			0xFF1B => 0xFF,
			0xFF1C => 0x9F | (self.volume << 5),
			0xFF1D => 0xFF | (self.frequency_load & 0x00FF) as u8,
			0xFF1E => {
				let counter_selection_bit = if self.counter_selection { 1 << 6 } else { 0 };
				0xBF | counter_selection_bit
			}
			0xFF30..=0xFF3F => self.wave_ram[loc as usize - 0xFF30],
			_ => panic!("Channel 3 read register out of range: {:04X}", loc),
		}
	}

	pub fn write(&mut self, loc: u16, val: u8) {
		match loc {
			0xFF1A => {
				self.dac_enabled = if (val & 0x80) == 0x80 { true } else { false };
			}
			0xFF1B => {
				self.length_counter = 256 - (val as u16);
			}
			0xFF1C => {
				self.volume = (val >> 5) & 0x03;
			}
			0xFF1D => {
				self.frequency_load = self.frequency_load & 0x0700 | val as u16;
			}
			0xFF1E => {
				self.status = if (val & 0x80) == 0x80 { true } else { false };
				self.counter_selection = if (val & 0x40) == 0x40 { true } else { false };
				self.frequency_load = self.frequency_load & 0x00FF | ((val & 0x07) as u16) << 8;

				if self.status {
					self.initialize();
				}
			}
			0xFF30..=0xFF3F => {
				self.wave_ram[loc as usize - 0xFF30] = val;
			}
			_ => panic!("Channel 3 write register out of range: {:04X}", loc),
		}
	}

	// Channel NR33/NR34 frequency tick
	pub fn tick(&mut self) {
		if self.frequency_count > 0 {
			self.frequency_count -= 1;
		} else if self.frequency_count == 0 {
			self.frequency_count = (2048 - self.frequency_load) * 2;
			self.position_counter = (self.position_counter + 1) & 0x1F;

			let position = (self.position_counter / 2) as usize;
			let mut output_byte = self.wave_ram[position];

			if (self.position_counter & 1) == 0 {
				output_byte >>= 4;
			}
			output_byte &= 0xF;

			self.sample_byte = output_byte
		}
	}

	// Initializes channel
	fn initialize(&mut self) {
		self.enabled = true;
		if self.length_counter == 0 {
			self.length_counter = 256;
		}
		self.frequency_count = (2048 - self.frequency_load) * 2;
		self.position_counter = 0;
	}

	pub fn length_step(&mut self) {
		if self.counter_selection && self.length_counter > 0 {
			self.length_counter -= 1;

			if self.length_counter == 0 {
				self.status = false;
			}
		}
	}

	// Return a value in [0,15]
	//	fn volume_output(&self) -> u8 {
	//		if self.enabled {
	//			// Shift by volume code
	//			self.sample_byte >> self.volume
	//		} else {
	//			0
	//		}
	//	}

	// 	pub fn dac_output(&self) -> f32 {
	// 		if self.dac_enabled {
	// 			let mut vol_output = 0.0;
	// 			if self.enabled {
	// 				// Shift by volume code
	// 				vol_output = (self.sample_byte >> self.volume) as f32
	// 			}
	// //			let vol_output = self.volume_output() as f32;
	// 			(vol_output / 7.5) - 1.0
	// 		} else {
	// 			0.0
	// 		}
	// 	}

	pub fn dac_output(&mut self) -> f32 {
		let mut dac_output = 0.0;

		if self.dac_enabled {
			let dac_input = (self.sample_byte >> self.volume) as f32;
			dac_output = dac_input - self.capacitor;
			self.capacitor = dac_input - dac_output * 0.996;
		}

		dac_output
	}
}

#[cfg(test)]
mod test {
	use super::*;

	fn create_test_channel3() -> Channel3 {
		Channel3::new()
	}

	#[test]
	fn test_NR30_read_write() {
		let mut ch3 = create_test_channel3();
		ch3.write(0xFF1A, 0xFF);

		assert_eq!(ch3.dac_enabled, true);

		assert_eq!(ch3.read(0xFF1A), 0xFF);
	}

	#[test]
	fn test_NR31_read_write() {
		let mut ch3 = create_test_channel3();
		ch3.write(0xFF1B, 0xFF);

		assert_eq!(ch3.length_counter, 1);

		assert_eq!(ch3.read(0xFF1B), 0xFF);
	}

	#[test]
	fn test_NR32_read_write() {
		let mut ch3 = create_test_channel3();
		ch3.write(0xFF1C, 0xFF);

		assert_eq!(ch3.volume, 3);
		assert_eq!(ch3.read(0xFF1C), 0xFF);
	}

	#[test]
	fn test_NR33_read_write() {
		let mut ch3 = create_test_channel3();
		ch3.write(0xFF1D, 0xFF);

		assert_eq!(ch3.frequency_load, 255);
		assert_eq!(ch3.read(0xFF1D), 0xFF);
	}

	#[test]
	fn test_NR34_read_write() {
		let mut ch3 = create_test_channel3();
		ch3.write(0xFF1E, 0xFF);

		assert_eq!(ch3.status, true);
		assert_eq!(ch3.counter_selection, true);
		assert_eq!(ch3.frequency_load, 1792);
		assert_eq!(ch3.read(0xFF1E), 0xFF);
	}
}
