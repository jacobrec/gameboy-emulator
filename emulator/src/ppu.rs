pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
pub type Canvas = [u32; SCREEN_WIDTH * SCREEN_HEIGHT]; // rgba u32 array. This will get passed and loaded into canvas
pub struct PPU {
    screen: Canvas,
    vram: [u8; 0x2000],
}
const color1: u32 = 0xFFFFB5FF;
const color2: u32 = 0x7BC67BFF;
const color3: u32 = 0x6B8C42FF;
const color4: u32 = 0x5A3921FF;

impl PPU {
    pub const fn new() -> Self {
        PPU {
            screen: [color1; SCREEN_WIDTH * SCREEN_HEIGHT],
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
