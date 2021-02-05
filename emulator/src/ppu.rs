pub type Canvas = [u32; 160 * 144]; // rgba u32 array. This will get passed and loaded into canvas
pub struct PPU {
    screen: Canvas
}

impl PPU {
    pub const fn new() -> Self {
        PPU {
            screen: [0u32; 160 * 144]
        }
    }

    pub fn get_screen(&self) -> Canvas {
        self.screen
    }

    pub fn tick(&mut self) {
    }

}
