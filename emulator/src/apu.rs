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

#[derive(Clone)]
pub struct Channel1 {
    sweep: Sweep,
    wave_pattern: Pattern,
    length_counter: u8,
    envelope: Envelope,
    frequency: u16,
    counter_selection: bool,
    status: bool,
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
            status: false,
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
                let status_bit = if self.status { 0x80 } else { 0 };
                let counter_selection_bit = if self.counter_selection { 0x40 } else { 0 };
                let frequency_high_bits = (self.frequency & 0x07) as u8;

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
            }
            0xFF13 => {
                self.frequency = self.frequency & 0x0700 | val as u16;
            }
            0xFF14 => {
                self.status = if (val & 0x80) == 0x80 { true } else { false };
                self.counter_selection = if (val & 0x40) == 0x40 { true } else { false };
                self.frequency = self.frequency & 0x00FF | ((val & 0x07) as u16) << 8;
            }
            _ => panic!("Channel 1 write register out of range: {:04X}", loc),
        }
    }

    pub fn tick(&mut self) {
        if self.counter_selection && self.length_counter > 0 {
            self.length_counter -= 1;

            if self.length_counter == 0 {
                self.status = false;
            }
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
    status: bool,
}

impl Channel2 {
    pub fn new() -> Self {
        Channel2 {
            wave_pattern: Pattern::HalfQuarter,
            envelope: Envelope::new(),
            length_counter: 0,
            frequency: 0,
            counter_selection: false,
            status: false,
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
                let status_bit = if self.status { 0x80 } else { 0 };
                let counter_selection_bit = if self.counter_selection { 0x40 } else { 0 };
                let frequency_high_bits = (self.frequency & 0x07) as u8;

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
            }
            0xFF18 => {
                self.frequency = self.frequency & 0x0700 | val as u16;
            }
            0xFF19 => {
                self.status = if (val & 0x80) == 0x80 { true } else { false };
                self.counter_selection = if (val & 0x40) == 0x40 { true } else { false };
                self.frequency = self.frequency & 0x00FF | ((val & 0x07) as u16) << 8;
            }
            _ => panic!("Channel 2 write register out of range: {:04X}", loc),
        }
    }

    pub fn tick(&mut self) {
        if self.counter_selection && self.length_counter > 0 {
            self.length_counter -= 1;

            if self.length_counter == 0 {
                self.status = false;
            }
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
    status: bool,
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
            status: false,
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
                let status_bit = if self.status { 0x80 } else { 0 };
                let counter_selection_bit = if self.counter_selection { 0x40 } else { 0 };
                let frequency_high_bits = (self.frequency & 0x07) as u8;

                status_bit | counter_selection_bit | frequency_high_bits
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
                self.status = if val >> 7 == 1 { true } else { false };
                self.counter_selection = if val >> 6 == 1 { true } else { false };
                self.frequency = self.frequency & 0x00FF | ((val & 0x07) as u16) << 8;
            }
            0xFF30..=0xFF3F => {
                self.wave_ram[(loc as usize - 0xFF30) - 1] = val;
            }
            _ => panic!("Channel 3 write register out of range: {:04X}", loc),
        }
    }

    pub fn tick(&mut self) {
        if self.counter_selection && self.length_counter > 0 {
            self.length_counter -= 1;

            if self.length_counter == 0 {
                self.status = false;
            }
        }
    }
}

#[derive(Clone)]
pub struct Channel4 {
    length_counter: u8,
    envelope: Envelope,
    polynomial_counter: u8,
    counter_selection: bool,
    status: bool,
}

impl Channel4 {
    pub fn new() -> Self {
        Channel4 {
            length_counter: 0,
            envelope: Envelope::new(),
            polynomial_counter: 0,
            counter_selection: false,
            status: false,
        }
    }

    pub fn read(&self, loc: u16) -> u8 {
        match loc {
            0xFF20 => self.length_counter,
            0xFF21 => self.envelope.read(),
            0xFF22 => self.polynomial_counter,
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
                self.envelope.write(val);
            }
            0xFF22 => {
                self.polynomial_counter = val;
            }
            0xFF23 => {
                self.status = if val >> 7 == 1 { true } else { false };
                self.counter_selection = if val >> 6 == 1 { true } else { false };
            }
            _ => panic!("Channel 4 write register out of range: {:04X}", loc),
        }
    }

    pub fn tick(&mut self) {
        if self.counter_selection && self.length_counter > 0 {
            self.length_counter -= 1;

            if self.length_counter == 0 {
                self.status = false;
            }
        }
    }
}

pub struct APU {
    terminal1_vin: bool,
    terminal1_volume: u8,
    terminal2_vin: bool,
    terminal2_volume: u8,
    channel_selection: u8,
    channel1: Channel1,
    channel2: Channel2,
    channel3: Channel3,
    channel4: Channel4,
    tick: usize,
    sound_control_registers: [u8; 3],
    enabled: bool,
}

impl APU {
    pub fn new() -> Self {
        APU {
            terminal1_vin: false,
            terminal1_volume: 7,
            terminal2_vin: true,
            terminal2_volume: 7,
            channel_selection: 0,
            channel1: Channel1::new(),
            channel2: Channel2::new(),
            channel3: Channel3::new(),
            channel4: Channel4::new(),
            tick: 0,
            sound_control_registers: [0; 3],
            enabled: false,
        }
    }

    pub fn read(&self, loc: u16) -> u8 {
        match loc {
            0xFF10..=0xFF14 => self.channel1.read(loc),
            0xFF16..=0xFF19 => self.channel2.read(loc),
            0xFF1A..=0xFF1E => self.channel3.read(loc),
            0xFF30..=0xFF3F => self.channel3.read(loc),
            0xFF20..=0xFF23 => self.channel4.read(loc),
            0xFF24 => {
                let t1_vin = if self.terminal1_vin { 0x80 } else { 0 };
                let t1_volume = (self.terminal1_volume << 4) & 0x7;

                let t2_vin = if self.terminal2_vin { 0x8 } else { 0 };
                t1_vin | t1_volume | t2_vin | self.terminal2_volume
            }
            0xFF25 => self.channel_selection,
            0xFF26 => {
                let sound_on = if self.enabled { 0x80 } else { 0 };
                let channel1_on = if self.channel1.status { 0x1 } else { 0 };
                let channel2_on = if self.channel1.status { 0x2 } else { 0 };
                let channel3_on = if self.channel1.status { 0x4 } else { 0 };
                let channel4_on = if self.channel1.status { 0x8 } else { 0 };

                sound_on | channel4_on | channel3_on | channel2_on | channel1_on
            }
            _ => panic!("APU read register out of range: {:04X}", loc),
        }
    }

    pub fn write(&mut self, loc: u16, val: u8) {
        if self.enabled {
            match loc {
                0xFF10..=0xFF14 => self.channel1.write(loc, val),
                0xFF16..=0xFF19 => self.channel2.write(loc, val),
                0xFF1A..=0xFF1E => self.channel3.write(loc, val),
                0xFF30..=0xFF3F => self.channel3.write(loc, val),
                0xFF20..=0xFF23 => self.channel4.write(loc, val),
                0xFF24 => {
                    if val & 0x80 == 0x80 {
                        self.terminal1_vin = true;
                    } else {
                        self.terminal1_vin = false;
                    }
                    self.terminal1_volume = (val >> 4) & 0x7;

                    if val & 0x8 == 0x8 {
                        self.terminal2_vin = true;
                    } else {
                        self.terminal2_vin = false;
                    }
                    self.terminal2_volume = val & 0x7;
                }
                0xFF25 => {
                    self.channel_selection = val;
                }
                0xFF26 => {
                    self.enabled = val & 0x80 == 0x80;
                }
                _ => panic!("APU write register out of range: {:04X}", loc),
            }
        }
    }

    pub fn tick(&mut self) {
        self.channel1.tick();
        self.channel2.tick();
        self.channel3.tick();
        self.channel4.tick();
    }
}
