pub type Canvas = [u32; 160 * 144]; // rgba u32 array. This will get passed and loaded into canvas
pub struct PPU {
    screen: Canvas,
    vram: [u8; 0x2000],
}

impl PPU {
    pub const fn new() -> Self {
        PPU {
            screen: [0u32; 160 * 144],
            vram: [0u8; 0x2000],
        }
    }

    pub fn get_screen(&self) -> Canvas {
        self.screen
    }

    pub fn tick(&mut self) {
    }

    // Both read and write expect loc to be in the address range 0x8000..=0x9FFF
    pub fn write(&mut self, loc: u16, val: u8) {
        let l = loc as usize - 0x8000;
        self.vram[l] = val
    }
    pub fn read(&self, loc: u16) -> u8 {
        let l = loc as usize - 0x8000;
        self.vram[l]
    }
}
