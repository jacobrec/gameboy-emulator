use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Channel3 {
	counter_selection: bool,
	dac_enabled: bool,
	enabled: bool,
	frequency_count: i32,
	frequency_load: u16,
	length_counter: u8,
	pub output_volume: u8,
	position_counter: u8,
	pub status: bool,
	volume: u8,
	wave_ram: [u8; 16],
}

impl Channel3 {
	pub fn new() -> Self {
		Channel3 {
			counter_selection: false, // NR 34 bit 6
			dac_enabled: true,        // Condition to check if all of envelope properties are set
			enabled: false,           // Condition to check if channel is enabled
			frequency_count: 0,       // Actual frequency value that is updated
			frequency_load: 0,        // NR 33 and NR 34 bit 2-0
			length_counter: 0,        // NR 31 5-0
			output_volume: 0,         // Volume uesd for mixing
			position_counter: 0,      // Wave RAM pointer
			status: false,            // NR 34 bit 7
			volume: 0,                // Actual envelope volume that is updated
			wave_ram: [0; 16],        // Wave RAM
		}
	}

	pub fn read(&self, loc: u16) -> u8 {
		match loc {
			0xFF1A => {
				if self.dac_enabled {
					0x80
				} else {
					0
				}
			}
			0xFF1B => self.length_counter,
			0xFF1C => self.volume << 5,
			0xFF1D => (self.frequency_load & 0x00FF) as u8,
			0xFF1E => {
				let status_bit = if self.status { 0x80 } else { 0 };
				let counter_selection_bit = if self.counter_selection { 0x40 } else { 0 };
				let frequency_high_bits = (self.frequency_load & 0x07) as u8;

				status_bit | counter_selection_bit | frequency_high_bits
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
				self.length_counter = val;
			}
			0xFF1C => {
				self.volume = (val >> 5) & 0x03;
			}
			0xFF1D => {
				self.frequency_load = self.frequency_load & 0x0700 | val as u16;
			}
			0xFF1E => {
				self.status = if val >> 7 == 1 { true } else { false };
				self.counter_selection = if val >> 6 == 1 { true } else { false };
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
		self.frequency_count -= 1;
		if self.frequency_count <= 0 {
			self.frequency_count = (2048 - self.frequency_load as i32) * 2;
			self.position_counter = (self.position_counter + 1) & 0x1F;
		}
		if self.enabled && self.dac_enabled {
			let position = (self.position_counter / 2) as usize;
			let mut output_byte = self.wave_ram[position];

			if (self.position_counter & 1) == 0 {
				output_byte >>= 4;
			}
			output_byte &= 0xF;

			output_byte = if self.volume > 0 {
				output_byte >> self.volume - 1
			} else {
				0
			};
			self.output_volume = output_byte;
		} else {
			self.output_volume = 0;
		}
	}

	// Initializes channel
	fn initialize(&mut self) {
		self.enabled = true;
		if self.length_counter == 0 {
			self.length_counter = 255;
		}
		self.frequency_count = (2048 - self.frequency_load as i32) * 2;
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
}
