type Canvas = [u32; 160 * 144]; // rgba u32 array. This will get passed and loaded into canvas
pub struct PPU {
    screen: Canvas
}

impl PPU {
    pub fn get_screen(&self) -> Canvas {
        self.screen
    }

}
