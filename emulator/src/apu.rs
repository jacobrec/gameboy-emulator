mod channel1;
mod channel2;
mod channel3;
mod channel4;
mod envelope;
mod pattern;
mod sweep;

use channel1::Channel1;
use channel2::Channel2;
use channel3::Channel3;
use channel4::Channel4;
use serde::{Deserialize, Serialize};
use serde_big_array::big_array;

big_array! { BigArray; }

const SAMPLE_SIZE: usize = 4096;

pub enum ChannelBit {
    Channel4Left = 1 << 7,
    Channel3Left = 1 << 6,
    Channel2Left = 1 << 5,
    Channel1Left = 1 << 4,
    Channel4Right = 1 << 3,
    Channel3Right = 1 << 2,
    Channel2Right = 1 << 1,
    Channel1Right = 1 << 0,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct APU {
    #[serde(with = "BigArray")]
    pub audio_buffer: [f32; SAMPLE_SIZE],
    audio_buffer_position: usize,
    audio_buffer_full: bool,
    channel1: Channel1,
    channel2: Channel2,
    channel3: Channel3,
    channel4: Channel4,
    channel_output_selection: u8,
    down_sample_count: u8,
    apu_enabled: bool,
    frame_sequencer: u8,
    frame_sequencer_count: u16,
    terminal1_vin: bool,
    terminal1_volume: u8,
    terminal2_vin: bool,
    terminal2_volume: u8,
}

impl APU {
    pub fn new() -> Self {
        APU {
            audio_buffer: [0.0; SAMPLE_SIZE], // Buffer that holds our audio samples
            audio_buffer_position: 0,         // Value for indexing our audio buffer
            audio_buffer_full: false,
            channel1: Channel1::new(),
            channel2: Channel2::new(),
            channel3: Channel3::new(),
            channel4: Channel4::new(),
            channel_output_selection: 0, // NR 51
            down_sample_count: 87,
            apu_enabled: false,          // Bit 7 of NR52
            frame_sequencer: 0,          //
            frame_sequencer_count: 8192, //
            terminal1_vin: false,
            terminal1_volume: 7,
            terminal2_vin: true,
            terminal2_volume: 7,
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
            0xFF25 => self.channel_output_selection,
            0xFF26 => {
                let sound_on = if self.apu_enabled { 0x80 } else { 0 };
                let channel1_on = if self.channel1.status { 0x1 } else { 0 };
                let channel2_on = if self.channel2.status { 0x2 } else { 0 };
                let channel3_on = if self.channel3.status { 0x4 } else { 0 };
                let channel4_on = if self.channel4.status { 0x8 } else { 0 };

                sound_on | channel4_on | channel3_on | channel2_on | channel1_on
            }
            _ => panic!("APU read register out of range: {:04X}", loc),
        }
    }

    pub fn write(&mut self, loc: u16, val: u8) {
        if !self.apu_enabled && loc != 0xFF26 && loc < 0xFF30 {
            return;
        }

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
                self.channel_output_selection = val;
            }
            0xFF26 => {
                self.apu_enabled = val & 0x80 == 0x80;
            }
            _ => panic!("APU write register out of range: {:04X}", loc),
        }
    }

    /*
    Frame sequencer timing:
    Step Length   Envelope   Sweep
    ------------------------------------
    0    Clock       -         -
    1    -           -         -
    2    Clock       -         Clock
    3    -           -         -
    4    Clock       -         -
    5    -           -         -
    6    Clock       -         Clock
    7    -           Clock     -
    ------------------------------------
    Rate 256 Hz      64 Hz     128 Hz
    */
    // https://www.reddit.com/r/EmuDev/comments/5gkwi5/gb_apu_sound_emulation/
    // Solution is adapted from: https://github.com/GhostSonic21/GhostBoy/blob/master/GhostBoy/APU.cpp
    pub fn tick(&mut self) {
        self.frame_sequencer_count -= 1;
        if self.frame_sequencer_count == 0 {
            self.frame_sequencer_count = 8192;
            match self.frame_sequencer {
                0 | 4 => {
                    // Length counter is cloked on each other step
                    self.clock_length();
                }
                2 | 6 => {
                    // Length counter is cloked on each other step
                    self.clock_length();
                    // Sweep generator clocked on every 2nd and 6th step
                    self.channel1.sweep_step();
                }
                7 => {
                    // Envelope clocked on every 7th step
                    self.clock_envelope();
                }
                _ => (),
            };

            // Increment frame_sequencer and mod to reset once it counts past 8
            self.frame_sequencer = (self.frame_sequencer + 1) % 8;
        }

        // Clock frequency of all channels
        self.clock_frequency();

        self.down_sample_count -= 1;
        if self.down_sample_count <= 0 {
            self.down_sample_count = 87;
            let mut bufferin_s02: f32 = 0.5;
            let mut bufferin_s01: f32;

            // NOTE: not sure if this is the correct way to get volume.
            let volume = (128 * self.terminal1_volume as i32) / 7;

            // Mix audio if bit is set in sound selection register NR51
            if self.channel_output_selection & 0x10 == ChannelBit::Channel1Left as u8 {
                // Mix left audio terminal with channel 1
                bufferin_s01 = self.channel1.output_volume as f32 / 100.0;
                self.mix(&mut bufferin_s02, bufferin_s01, volume);
            }
            if self.channel_output_selection & 0x20 == ChannelBit::Channel2Left as u8 {
                // Mix left audio terminal with channel 2
                bufferin_s01 = self.channel2.output_volume as f32 / 100.0;
                self.mix(&mut bufferin_s02, bufferin_s01, volume);
            }
            if self.channel_output_selection & 0x40 == ChannelBit::Channel3Left as u8 {
                // Mix left audio terminal with channel 3
                bufferin_s01 = self.channel3.output_volume as f32 / 100.0;
                self.mix(&mut bufferin_s02, bufferin_s01, volume);
            }
            if self.channel_output_selection & 0x80 == ChannelBit::Channel4Left as u8 {
                // Mix left audio terminal with channel 4
                bufferin_s01 = self.channel4.output_volume as f32 / 100.0;
                self.mix(&mut bufferin_s02, bufferin_s01, volume);
            }
            // Fill buffer with mixed sample
            self.audio_buffer[self.audio_buffer_position] = bufferin_s02;

            if self.channel_output_selection & 0x01 == ChannelBit::Channel1Right as u8 {
                // Mix left audio terminal with channel 1
                bufferin_s01 = self.channel1.output_volume as f32 / 100.0;
                self.mix(&mut bufferin_s02, bufferin_s01, volume);
            }
            if self.channel_output_selection & 0x02 == ChannelBit::Channel2Right as u8 {
                // Mix left audio terminal with channel 2
                bufferin_s01 = self.channel2.output_volume as f32 / 100.0;
                self.mix(&mut bufferin_s02, bufferin_s01, volume);
            }
            if self.channel_output_selection & 0x04 == ChannelBit::Channel3Right as u8 {
                // Mix left audio terminal with channel 3
                bufferin_s01 = self.channel3.output_volume as f32 / 100.0;
                self.mix(&mut bufferin_s02, bufferin_s01, volume);
            }
            if self.channel_output_selection & 0x08 == ChannelBit::Channel4Right as u8 {
                // Mix left audio terminal with channel 4
                bufferin_s01 = self.channel4.output_volume as f32 / 100.0;
                self.mix(&mut bufferin_s02, bufferin_s01, volume);
            }

            // Fill buffer with mixed sample
            self.audio_buffer[self.audio_buffer_position + 1] = bufferin_s02;
            self.audio_buffer_position += 2;
        }

        // Reset buffer position if we reach max
        if self.audio_buffer_position >= SAMPLE_SIZE {
            self.audio_buffer_full = true;
            self.audio_buffer_position = 0;
        }
    }

    fn clock_length(&mut self) {
        self.channel1.length_step();
        self.channel2.length_step();
        self.channel3.length_step();
        self.channel4.length_step();
    }

    fn clock_envelope(&mut self) {
        self.channel1.envelope_step();
        self.channel2.envelope_step();
        self.channel4.envelope_step();
    }

    fn clock_frequency(&mut self) {
        self.channel1.tick();
        self.channel2.tick();
        self.channel3.tick();
        self.channel4.tick();
    }

    // https://github.com/emscripten-ports/SDL2/blob/master/src/audio/SDL_mixer.c
    // Adapted into rust from the source above. This function takes two audio buffers
    // and mixes them, performing addition, volume adjustment, and overflow clipping.
    fn mix(&self, dst: &mut f32, src: f32, volume: i32) {
        let fmax_volume = 1.0f32 / 128f32;
        let fvolume = volume as f32;
        let max_audioval = std::f32::MAX as f64;
        let min_audioval = std::f32::MIN as f64;

        // volume adjustment
        let src1 = src * fvolume * fmax_volume;
        let src2 = *dst;

        // addition
        let mut dst_sample = src1 as f64 + src2 as f64;

        if dst_sample > max_audioval {
            dst_sample = max_audioval;
        } else if dst_sample < min_audioval {
            dst_sample = min_audioval;
        }
        *dst = dst_sample as f32;
    }

    pub fn get_audio_buffer(&self) -> [f32; SAMPLE_SIZE] {
        self.audio_buffer
    }

    pub fn get_audio_buffer_status(&self) -> bool {
        self.audio_buffer_full
    }

    pub fn set_audio_buffer_status(&mut self, status: bool) {
        self.audio_buffer_full = status;
    }
}
