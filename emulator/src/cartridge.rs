use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Cartridge {
    mapper: Mapper,
    rom: Vec<u8>,
    ram: Vec<u8>,
    ramBanks: usize,
    romBanks: usize,
}
const RAM_BANK_SIZE: usize = 8192;
const ROM_BANK_SIZE: usize = 16384;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
enum Mapper { // https://gbdev.io/pandocs/#_0147-cartridge-type
    ROM,

    // https://gbdev.io/pandocs/#mbc1
    MBC1(bool, usize, usize, bool),
    // ram enabled, bank low, bank high, simple mode
}

impl Cartridge {
    pub fn is_bootrom(&self) -> bool {
        false
    }

    pub fn test(data: Vec<u8>) -> Self {
        Self {
            mapper: Mapper::ROM,
            ramBanks: 0,
            romBanks: 0,
            rom: data,
            ram: Vec::new(),
        }
    }

    pub fn from_data(data: Vec<u8>) -> Self {
        let mapper = Cartridge::check_mapper(&data);
        let romBanks: usize = 2 * 2u32.pow(data[0x148] as u32) as usize;
        let ramBanks: usize = match data[0x149] {
            0x0 => 0,
            0x2 => 1,
            0x3 => 4,
            0x4 => 16,
            0x5 => 8,
            _ => 0,
        };
        let ram = Vec::with_capacity(RAM_BANK_SIZE * ramBanks);
        let rom = data;
        eprintln!("romBanks: {}, ramBanks: {}. Mapper: {:?}", romBanks, ramBanks, mapper);
        Self {mapper, ramBanks, romBanks, ram, rom }
    }

    fn check_mapper(data: &Vec<u8>) -> Mapper {
        const loc: usize = 0x147;
        match data[loc] {
            0x0 => Mapper::ROM,
            0x1 => Mapper::MBC1(false, 1, 0, true),
            _ => unimplemented!("Unimplemented mapper type: {:02X}", data[loc]),
        }
    }

    pub fn read(&self, loc: u16) -> u8 {
        match self.mapper {
            Mapper::ROM => if self.rom.len() < loc.into() { 0x49 } else {self.rom[loc as usize]},
            Mapper::MBC1(ram, bank, hibank, simple) => { // TODO: add ram
                let loc = loc as usize;
                // Bank ram, rom low, rom high
                let bankR = hibank * RAM_BANK_SIZE;
                let bankL = if simple { 0 } else { (hibank << 5) * ROM_BANK_SIZE};
                let bankH = if simple { if bank == 0 { 1 } else { bank } } else { hibank << 5 | bank} * ROM_BANK_SIZE;
                match loc {
                    0x0000..=0x3FFF => self.rom[bankL + (loc & 0x3FFF)],
                    0x4000..=0x7FFF => self.rom[bankH + (loc & 0x3FFF)],
                    0xA000..=0xBFFF => self.ram[bankR + (loc & 0x1FFF)],
                    _ => panic!("These read ranges should not be routed to cartridge")
                }

            }
        }
    }
    pub fn write(&mut self, loc: u16, val: u8) {
        match self.mapper {
            Mapper::ROM => (),
            Mapper::MBC1(ram, bank, hibank, simple) => {
                let bankR = hibank * RAM_BANK_SIZE;
                let nm = match loc {
                    0x0000..=0x1FFF => Mapper::MBC1(val & 0b1111 == 0xA, bank, hibank, simple),
                    0x2000..=0x3FFF => Mapper::MBC1(ram, (val & 0b11111) as usize, hibank, simple),
                    0x4000..=0x5FFF => Mapper::MBC1(ram, bank, (val & 0b11) as usize, simple),
                    0x6000..=0x7FFF => Mapper::MBC1(ram, bank, hibank, val & 0b1 != 0x1),
                    0xA000..=0xBFFF if self.ramBanks > 1 => {
                        let loc = loc as usize;
                        self.ram[bankR + loc & 0x1FFF];
                        Mapper::MBC1(ram, bank, hibank, val & 0b1 != 0x1)
                    },
                    0xA000..=0xBFFF => Mapper::MBC1(ram, bank, hibank, simple),
                    _ => panic!("This range should not be routed to cartridge: loc: 0x{:04X}", loc)
                };
                self.mapper = nm;
            }
        }
    }
}
