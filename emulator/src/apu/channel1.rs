use super::envelope::Envelope;
use super::pattern::*;
use super::sweep::Sweep;
use serde::{Deserialize, Serialize};

const WAVE_PATTERN: [[u8; 8]; 4] = [
	[0, 0, 0, 0, 0, 0, 0, 1],
	[1, 0, 0, 0, 0, 0, 0, 1],
	[1, 0, 0, 0, 0, 1, 1, 1],
	[0, 1, 1, 1, 1, 1, 1, 0],
];


/*
The variable names with _load are the variables we use to store the initial values.
All updates to the values to perform any computation is done in the variables without
the _load in the variable name.
*/
#[derive(Serialize, Deserialize, Clone)]
pub struct Channel1 {
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
	sequence_pointer: u8,
	pub status: bool,
	pub sweep: Sweep,
	sweep_enable: bool,
	sweep_shadow: u16,
	sweep_period_counter: u8,
	volume: u8,
	pub wave_pattern: Pattern,
}

impl Channel1 {
	pub fn new() -> Channel1 {
		Channel1 {
			counter_selection: false,           // NR 14 bit 6
			dac_enabled: false,                  // Condition to check if all of envelope properties are set
			enabled: false,                     // Condition to check if channel is enabled
			envelope: Envelope::new(),          // NR 12 Enevlope
			envelope_period_counter: 0,
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
			sweep_period_counter: 0,            // Actual sweep time that is updated
			volume: 0,                          // Actual envelope volume that is updated
			wave_pattern: Pattern::HalfQuarter, // NR 11 bit 7-6
		}
	}

	pub fn read(&self, loc: u16) -> u8 {
		match loc {
			0xFF10 => self.sweep.read(),
			0xFF11 => {
				let pattern_bits = pattern_to_u8(self.wave_pattern);
				pattern_bits << 6
			}
			0xFF12 => self.envelope.read(),
			0xFF13 => (self.frequency_load & 0x00FF) as u8,
			0xFF14 => {
				let counter_selection_bit = if self.counter_selection { 1 << 6 } else { 0 };

				counter_selection_bit 
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
				self.length_counter = 64 - (val & 0x3F);
			}
			0xFF12 => {
				self.envelope.write(val);
				self.dac_enabled = val & 0xF8 != 0;
				self.volume = self.envelope.initial_volume;
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
		if self.frequency_count > 0 {
			self.frequency_count -= 1;
		} else if self.frequency_count == 0 {
			self.frequency_count = (2048 - self.frequency_load) * 4;
			self.sequence_pointer = (self.sequence_pointer + 1) % 8;

			if self.enabled && self.dac_enabled {
				self.output_volume = self.volume;
			} else {
				self.output_volume = 0;
			}
		}

		if WAVE_PATTERN[pattern_to_u8(self.wave_pattern) as usize][self.sequence_pointer as usize] == 0
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

	// NR11 Sweep register tick
	pub fn sweep_step(&mut self) {
		if self.sweep_period_counter > 0 {
			self.sweep_period_counter -= 1;
		} else {
			self.sweep_period_counter = self.sweep.period;
			let new_frequency = self.sweep_calculation();
			if new_frequency <= 2047 && self.sweep.shift > 0 {
				// Sweep shadow is X(t-1)
				self.sweep_shadow = new_frequency;
				self.frequency_load = new_frequency;
				self.sweep_calculation();
			}
		}
	}

	// Calculates new frequency based on sweep shift.
	// X(t) = X(t-1) +/- X(t-1)/2^n
	fn sweep_calculation(&mut self) -> u16 {
		// X(t-1)/2^n
		let mut new_frequency = self.sweep_shadow >> self.sweep.shift;

		new_frequency = if self.sweep.direction == 0 {
			// Sweep direction 0 is increase
			self.sweep_shadow + new_frequency
		} else {
			// Sweep direction 1 is decrease
			self.sweep_shadow - new_frequency
		};

		if new_frequency > 2047 {
			self.enabled = false;
		}

		new_frequency
	}

	// Initializes channel by resetting all values
	fn initialize(&mut self) {
		println!("initialize");
		self.enabled = true;
		if self.length_counter == 0 {
			self.length_counter = 64;
		}

		self.frequency_count = (2048 - self.frequency_load) * 4;
		self.envelope_running = true;
		self.envelope_period_counter = self.envelope.period;
		self.volume = self.envelope.initial_volume;
		self.sweep_shadow = self.frequency_load;
		self.sweep_period_counter = if self.sweep.period == 0 {
			8
		} else {
			self.sweep.period
		};
		self.sweep_enable = self.sweep_period_counter > 0 || self.sweep.shift > 0;
		if self.sweep.shift > 0 {
			self.sweep_calculation();
		}
	}

	 pub fn dac_output(&self) -> f32 {
	 	if self.dac_enabled {
			let mut duty_output = 0.0;
			if self.enabled {
				duty_output = WAVE_PATTERN[pattern_to_u8(self.wave_pattern) as usize][self.sequence_pointer as usize] as f32;
			}
	 		let vol_output: f32 = self.volume as f32 * duty_output;
	 		(vol_output / 7.5) - 1.0
	 	} else {
	 		0.0
	 	}
	 }
}


#[cfg(test)]
mod test {
    use super::*;

    fn create_test_channel1() -> Channel1 {
        Channel1::new()
    }

    #[test]
    fn test_NR10_read_write () {
        let mut ch1 = create_test_channel1();
				ch1.write(0xFF10, 0xFF);

				assert_eq!(ch1.sweep.period, 7);
				assert_eq!(ch1.sweep.direction, 1);
				assert_eq!(ch1.sweep.shift, 7);

				assert_eq!(ch1.read(0xFF10), 0x7F);
    }

		#[test]
    fn test_NR11_read_write () {
        let mut ch1 = create_test_channel1();
				ch1.write(0xFF11, 0xFF);

				let pattern_bits = pattern_to_u8(ch1.wave_pattern);
				assert_eq!(pattern_bits, 3);
				assert_eq!(ch1.length_counter, 1);

				assert_eq!(ch1.read(0xFF11), 0xC0);
    }

		#[test]
    fn test_NR12_read_write () {
        let mut ch1 = create_test_channel1();
				ch1.write(0xFF12, 0xFF);

				assert_eq!(ch1.envelope.initial_volume, 15);
				assert_eq!(ch1.envelope.direction, 1);
				assert_eq!(ch1.envelope.period, 7);
				assert_eq!(ch1.read(0xFF12), 0xFF);
    }

		#[test]
    fn test_NR13_read_write () {
        let mut ch1 = create_test_channel1();
				ch1.write(0xFF13, 0xFF);

				assert_eq!(ch1.frequency_load, 255);
				assert_eq!(ch1.read(0xFF13), 0xFF);
    }

		
		#[test]
    fn test_NR14_read_write () {
        let mut ch1 = create_test_channel1();
				ch1.write(0xFF14, 0xFF);

				assert_eq!(ch1.status, true);
				assert_eq!(ch1.counter_selection, true);
				assert_eq!(ch1.frequency_load, 1792);
				assert_eq!(ch1.read(0xFF14), 0x40);
    }
	}