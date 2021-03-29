use super::envelope::Envelope;
use super::pattern::*;

const WAVE_PATTERN: [[i32; 8]; 4] = [
	[-1, -1, -1, -1, 1, -1, -1, -1],
	[-1, -1, -1, -1, 1, 1, -1, -1],
	[-1, -1, 1, 1, 1, 1, -1, -1],
	[1, 1, 1, 1, -1, -1, 1, 1],
];

#[derive(Clone)]
pub struct Channel2 {
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
	volume: u8,
	pub wave_pattern: Pattern,
}

impl Channel2 {
	pub fn new() -> Self {
		Channel2 {
			counter_selection: false,           // NR 24 bit 6
			dac_enabled: true,                  // Condition to check if all of envelope properties are set
			enabled: false,                     // Condition to check if channel is enabled
			envelope: Envelope::new(),          // NR 22 Enevlope
			envelope_running: false,            // Condition to check whether envelope is on or off
			frequency_count: 0,                 // Actual frequency value that is updated
			frequency_load: 0,                  // NR 23 and NR 24 bit 2-0
			length_counter: 0,                  // NR 21 5-0
			output_volume: 0,                   // Volume uesd for mixing
			sequence_pointer: 0,                // pointer to keep track of wave_pattern location
			status: false,                      // NR 24 bit 7
			volume: 0,                          // Actual envelope volume that is updated
			wave_pattern: Pattern::HalfQuarter, // NR 21 bit 7-6
		}
	}

	pub fn read(&self, loc: u16) -> u8 {
		match loc {
			0xFF16 => {
				let pattern_bits = pattern_to_u8(self.wave_pattern);
				pattern_bits << 6 | self.length_counter & 0x3F
			}
			0xFF17 => self.envelope.read(),
			0xFF18 => (self.frequency_load & 0x00FF) as u8,
			0xFF19 => {
				let status_bit = if self.status { 0x80 } else { 0 };
				let counter_selection_bit = if self.counter_selection { 0x40 } else { 0 };
				let frequency_high_bits = (self.frequency_load & 0x07) as u8;

				status_bit | counter_selection_bit | frequency_high_bits
			}
			_ => panic!("Channel 2 read register out of range: {:04X}", loc),
		}
	}

	pub fn write(&mut self, loc: u16, val: u8) {
		match loc {
			0xFF16 => {
				let pattern_bits = val >> 6;
				self.wave_pattern = u8_to_pattern(pattern_bits).unwrap();
				self.length_counter = val & 0x3F;
			}
			0xFF17 => {
				self.envelope.write(val);
				self.dac_enabled = val & 0xF8 != 0;
				self.volume = self.envelope.volume;
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

		if WAVE_PATTERN[pattern_to_u8(self.wave_pattern) as usize][self.sequence_pointer as usize]
			== -1
		{
			self.output_volume = 0;
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
	}
}
