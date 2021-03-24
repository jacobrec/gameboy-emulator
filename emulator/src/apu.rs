#[derive(Clone)]
pub struct Envelope {
    initial_volume: u8,
    direction: u8,
    length: u8,
}

impl Envelope {
    pub fn new() -> Self {
        Envelope {
            initial_volume: 0,
            direction: 0,
            length: 0,
        }
    }

    pub fn read(&self) -> u8 {
        let direction: u8 = self.direction << 3;
        let volume: u8 = self.initial_volume << 4;
        volume | direction | self.length
    }

    pub fn write(&mut self, value: u8) {
        self.initial_volume = (value >> 4) & 0x0F;
        self.direction = (value >> 3) & 1;
        self.length = value & 0x07;
    }
}

#[derive(Clone)]
pub struct Sweep {
    time: u8,
    direction: u8,
    shift: u8,
}

impl Sweep {
    pub fn new() -> Self {
        Sweep {
            time: 0,
            direction: 0,
            shift: 0,
        }
    }

    pub fn read(&self) -> u8 {
        let direction: u8 = self.direction << 3;
        let time: u8 = self.time << 4;
        0x80 | direction | time | self.shift
    }

    pub fn write(&mut self, value: u8) {
        self.time = (value >> 4) & 0x07;
        self.direction = (value >> 3) & 1;
        self.shift = value & 0x07;
    }
}

#[derive(Clone, Copy)]
pub enum Pattern {
    HalfQuarter = 0,
    Quarter = 1,
    Half = 2,
    ThreeQuarters = 3,
}

fn u8_to_pattern(value: u8) -> Option<Pattern> {
    match value {
        0 => Some(Pattern::HalfQuarter),
        1 => Some(Pattern::Quarter),
        2 => Some(Pattern::Half),
        3 => Some(Pattern::ThreeQuarters),
        _ => None,
    }
}

fn pattern_to_u8(pattern: Pattern) -> u8 {
    match pattern {
        Pattern::HalfQuarter => 0,
        Pattern::Quarter => 1,
        Pattern::Half => 2,
        Pattern::ThreeQuarters => 3,
    }
}

// struct WavePatternDuty {
//     pattern: Pattern,
//     length: u8
// }

// impl WavePatternDuty {

//     pub fn new() -> Self {
//         WavePatternDuty {
//             pattern: Pattern::HalfQuarter,
//             length: 0
//         }

//     }

//     pub fn read(&self) -> u8 {
//         let pattern: u8 = u8_from_pattern(self.pattern) << 6;
//         pattern | self.length
//     }

//     pub fn write(&mut self, value: u8) {
//         self.pattern = pattern_from_u8((value >> 6) & 0x3);
//         self.length = value & 0x3F;
//     }

//     pub fn pattern_from_u8(value: u8) -> Option<Pattern> {
//         match value {
//         0 => Some(HalfQuarter),
//         1 => Some(Quarter),
//         2 => Some(Half),
//         3 => Some(ThreeQuarters),
//         _ => None,
//         }
//     }

//     pub fn u8_from_pattern(pattern: Pattern) -> u8 {
//         match value {
//             Pattern::HalfQuarter => 0,
//             Pattern::Quarter => 1,
//             Pattern::Half => 2,
//             Pattern::ThreeQuarters => 3,
//             _ => write!("Pattern out of range")
//         }
//     }
// }

#[derive(Clone)]
pub struct Channel1 {
    sweep: Sweep,
    wave_pattern: Pattern,
    length_counter: u8,
    envelope: Envelope,
    frequency: u16,
    counter_selection: bool,
    initial: bool,
}

impl Channel1 {
    pub fn new() -> Channel1 {
        Channel1 {
            sweep: Sweep::new(),
            wave_pattern: Pattern::HalfQuarter,
            envelope: Envelope::new(),
            length_counter: 0,
            frequency: 0,
            counter_selection: false,
            initial: false,
        }
    }

    pub fn read(&self, loc: u16) -> u8 {
        match loc {
            0xFF10 => self.sweep.read(),
            0xFF11 => {
                let pattern_bits = pattern_to_u8(self.wave_pattern);
                pattern_bits << 6 | self.length_counter & 0x1F
            }
            0xFF12 => self.envelope.read(),
            0xFF13 => (self.frequency & 0x00FF) as u8,
            0xFF14 => {
                let initial_bit = if self.initial { 0x80 } else { 0 };
                let counter_selection_bit = if self.counter_selection { 0x40 } else { 0 };
                let frequency_high_bits = (self.frequency & 0x07) as u8;

                initial_bit | counter_selection_bit | frequency_high_bits
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
            }
            0xFF13 => {
                self.frequency = self.frequency & 0x0700 | val as u16;
            }
            0xFF14 => {
                self.initial = if (val & 0x80) == 0x80 { true } else { false };
                self.counter_selection = if (val & 0x40) == 0x40 { true } else { false };
                self.frequency = self.frequency & 0x00FF | ((val & 0x07) as u16) << 8;
            }
            _ => panic!("Channel 1 write register out of range: {:04X}", loc),
        }
    }
}

#[derive(Clone)]
pub struct Channel2 {
    wave_pattern: Pattern,
    length_counter: u8,
    envelope: Envelope,
    frequency: u16,
    counter_selection: bool,
    initial: bool,
}

impl Channel2 {
    pub fn new() -> Self {
        Channel2 {
            wave_pattern: Pattern::HalfQuarter,
            envelope: Envelope::new(),
            length_counter: 0,
            frequency: 0,
            counter_selection: false,
            initial: false,
        }
    }

    pub fn read(&self, loc: u16) -> u8 {
        match loc {
            0xFF16 => {
                let pattern_bits = pattern_to_u8(self.wave_pattern);
                pattern_bits << 6 | self.length_counter & 0x1F
            }
            0xFF17 => self.envelope.read(),
            0xFF18 => (self.frequency & 0x00FF) as u8,
            0xFF19 => {
                let initial_bit = if self.initial { 0x80 } else { 0 };
                let counter_selection_bit = if self.counter_selection { 0x40 } else { 0 };
                let frequency_high_bits = (self.frequency & 0x07) as u8;

                initial_bit | counter_selection_bit | frequency_high_bits
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
            }
            0xFF18 => {
                self.frequency = self.frequency & 0x0700 | val as u16;
            }
            0xFF19 => {
                self.initial = if (val & 0x80) == 0x80 { true } else { false };
                self.counter_selection = if (val & 0x40) == 0x40 { true } else { false };
                self.frequency = self.frequency & 0x00FF | ((val & 0x07) as u16) << 8;
            }
            _ => panic!("Channel 2 write register out of range: {:04X}", loc),
        }
    }
}

#[derive(Clone)]
pub struct Channel3 {
    enabled: bool,
    length_counter: u8,
    volume: u8,
    frequency: u16,
    counter_selection: bool,
    initial: bool,
    wave_ram: [u8; 16],
}

impl Channel3 {
    pub fn new() -> Self {
        Channel3 {
            enabled: false,
            length_counter: 0,
            volume: 0,
            frequency: 0,
            counter_selection: false,
            initial: false,
            wave_ram: [0; 16],
        }
    }

    pub fn read(&self, loc: u16) -> u8 {
        match loc {
            0xFF1A => {
                if self.enabled {
                    0x80
                } else {
                    0
                }
            }
            0xFF1B => self.length_counter,
            0xFF1C => self.volume << 5,
            0xFF1D => (self.frequency & 0x00FF) as u8,
            0xFF1E => {
                let initial_bit = if self.initial { 0x80 } else { 0 };
                let counter_selection_bit = if self.counter_selection { 0x40 } else { 0 };
                let frequency_high_bits = (self.frequency & 0x07) as u8;

                initial_bit | counter_selection_bit | frequency_high_bits
            }
            0xFF30..=0xFF3F => self.wave_ram[(loc as usize - 0xFF30) - 1],
            _ => panic!("Channel 3 read register out of range: {:04X}", loc),
        }
    }

    pub fn write(&mut self, loc: u16, val: u8) {
        match loc {
            0xFF1A => {
                self.enabled = if (val & 0x80) == 0x80 { true } else { false };
            }
            0xFF1B => {
                self.length_counter = val;
            }
            0xFF1C => {
                self.volume = (val >> 5) & 0x03;
            }
            0xFF1D => {
                self.frequency = self.frequency & 0x0700 | val as u16;
            }
            0xFF1E => {
                self.initial = if val >> 7 == 1 { true } else { false };
                self.counter_selection = if val >> 6 == 1 { true } else { false };
                self.frequency = self.frequency & 0x00FF | ((val & 0x07) as u16) << 8;
            }
            0xFF30..=0xFF3F => {
                self.wave_ram[(loc as usize - 0xFF30) - 1] = val;
            }
            _ => panic!("Channel 3 write register out of range: {:04X}", loc),
        }
    }
}

#[derive(Clone)]
pub struct Channel4 {
    length_counter: u8,
    envelope: Envelope,
    polynomial_counter: u8,
    counter_selection: bool,
    initial: bool,
}

impl Channel4 {
    pub fn new() -> Self {
        Channel4 {
            length_counter: 0,
            envelope: Envelope::new(),
            polynomial_counter: 0,
            counter_selection: false,
            initial: false,
        }
    }

    pub fn read(&self, loc: u16) -> u8 {
        match loc {
            0xFF20 => self.length_counter,
            0xFF21 => self.envelope.read(),
            0xFF22 => self.polynomial_counter,
            0xFF23 => {
                let initial_bit = if self.initial { 0x80 } else { 0 };
                let counter_selection_bit = if self.counter_selection { 0x40 } else { 0 };

                initial_bit | counter_selection_bit
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
                self.envelope.write(val);
            }
            0xFF22 => {
                self.polynomial_counter = val;
            }
            0xFF23 => {
                self.initial = if val >> 7 == 1 { true } else { false };
                self.counter_selection = if val >> 6 == 1 { true } else { false };
            }
            _ => panic!("Channel 4 write register out of range: {:04X}", loc),
        }
    }
}

pub struct APU {
    channel1: Channel1,
    channel2: Channel2,
    channel3: Channel3,
    channel4: Channel4,
    tick: usize,
}

impl APU {
    pub fn new() -> Self {
        APU {
            channel1: Channel1::new(),
            channel2: Channel2::new(),
            channel3: Channel3::new(),
            channel4: Channel4::new(),
            tick: 0,
        }
    }

    pub fn read(&self, loc: u16) -> u8 {
        match loc {
            0xFF10..=0xFF14 => self.channel1.read(loc),
            0xFF16..=0xFF19 => self.channel2.read(loc),
            0xFF1A..=0xFF1E => self.channel3.read(loc),
            0xFF30..=0xFF3F => self.channel3.read(loc),
            0xFF20..=0xFF23 => self.channel4.read(loc),
            _ => panic!("APU read register out of range: {:04X}", loc),
        }
    }

    pub fn write(&mut self, loc: u16, val: u8) {
        match loc {
            0xFF10..=0xFF14 => self.channel1.write(loc, val),
            0xFF16..=0xFF19 => self.channel2.write(loc, val),
            0xFF1A..=0xFF1E => self.channel3.write(loc, val),
            0xFF30..=0xFF3F => self.channel3.write(loc, val),
            0xFF20..=0xFF23 => self.channel4.write(loc, val),
            _ => panic!("APU write register out of range: {:04X}", loc),
        }
    }

    pub fn tick(&mut self) {}
}
