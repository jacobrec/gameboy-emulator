#[derive(Clone)]
pub struct Envelope {
    initial_volume: u8,
    direction: u8,
    length: u8
}

impl Envelope {
    pub fn new() -> Self {
        Envelope {
            initial_volume: 0,
            direction: 0,
            length: 0
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
    shift: u8
}

impl Sweep {
    pub fn new() -> Self {
        Sweep {
            time: 0,
            direction: 0,
            shift: 0
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
    sound_length: u8,
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
            sound_length: 0,
            frequency: 0,
            counter_selection: false,
            initial: false
        }
    }
}
pub struct APU {
    channel1: Channel1,
    tick: usize
}

impl APU {
    pub fn new() -> Self {
        APU { 
            channel1: Channel1::new(),
            tick: 0
        }
    }

    fn u8_to_pattern(&self, value: u8) -> Option<Pattern> {
        match value {
            0 => Some(Pattern::HalfQuarter),
            1 => Some(Pattern::Quarter),
            2 => Some(Pattern::Half),
            3 => Some(Pattern::ThreeQuarters),
            _ => None
        }
    } 

    pub fn read(&self, loc: u16) -> u8 {
        0x00
    }

    pub fn write(&mut self, loc: u16, val: u8) {
        match loc {
            0xFF10 => {
                self.channel1.sweep.write(val);
            },
            0xFF11 => {
                let pattern_bits = val >> 6;
                self.channel1.wave_pattern = self.u8_to_pattern(pattern_bits).unwrap();
                self.channel1.sound_length = 64 - (val & 0x3F);
            },
            _ => { print!("APU write register out of range.") }
        }
    }

    pub fn tick(&mut self) {
    }

}
