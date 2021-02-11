pub struct APU {
}

impl APU {
    pub const fn new() -> Self {
        APU { }
    }
    pub fn read(&self, loc: u16) -> u8 {
        0x00
    }

    pub fn write(&mut self, loc: u16, val: u8) {
    }

}
