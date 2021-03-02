pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
pub type Canvas = [u32; SCREEN_WIDTH * SCREEN_HEIGHT]; // rgba u32 array. This will get passed and loaded into canvas

const LCD_CONTROL_REGISTER: usize = 0; // https://gbdev.io/pandocs/#ff40-lcd-control-register
const LCD_STATUS_REGISTER: usize = 1; // https://gbdev.io/pandocs/#lcd-status-register
const SCY: usize = 2; // https://gbdev.io/pandocs/#ff42-scy-scroll-y-r-w-ff43-scx-scroll-x-r-w
const SXY: usize = 3; // https://gbdev.io/pandocs/#ff42-scy-scroll-y-r-w-ff43-scx-scroll-x-r-w
const LY: usize = 4; // https://gbdev.io/pandocs/#ff44-ly-lcdc-y-coordinate-r
const LYC: usize = 5; // https://gbdev.io/pandocs/#ff45-lyc-ly-compare-r-w
const DMA: usize = 6; // https://gbdev.io/pandocs/#ff46-dma-dma-transfer-and-start-address-r-w

const BGP: usize = 7; // https://gbdev.io/pandocs/#ff47-bgp-bg-palette-data-r-w-non-cgb-mode-only
const OBP0: usize = 8; // https://gbdev.io/pandocs/#ff48-obp0-object-palette-0-data-r-w-non-cgb-mode-only
const OBP1: usize = 9; // https://gbdev.io/pandocs/#ff48-obp0-object-palette-0-data-r-w-non-cgb-mode-only

// https://gbdev.io/pandocs/#ff4a-wy-window-y-position-r-w-ff4b-wx-window-x-position-7-r-w
const WY: usize = 0xA;
const WX: usize = 0xB;

pub enum Mode {
    HBlank,
    VBlank,
    OAM,
    VRAM,
}

pub struct PPU {
    screen: Canvas,
    vram: [u8; 0x2000],
    registers: [u8; 0x10],
    mode: Mode,
    tick: usize,
    scanline: usize,
}
const TICK_WIDTH: usize = 456;
const OAM_WIDTH: usize = 80;
const EFFECTIVE_SCAN_COUNT: u8 = 153;

const color1: u32 = 0xFFFFB5FF;
const color2: u32 = 0x7BC67BFF;
const color3: u32 = 0x6B8C42FF;
const color4: u32 = 0x5A3921FF;

impl PPU {
    pub const fn new() -> Self {
        PPU {
            screen: [color1; SCREEN_WIDTH * SCREEN_HEIGHT],
            vram: [0u8; 0x2000],
            registers: [0u8; 0x10],
            mode: Mode::HBlank,
            tick: 0,
            scanline: 0,
        }
    }

    pub fn get_screen(&self) -> Canvas {
        self.screen
    }

    pub fn tick(&mut self) {
        self.tick += 1;
        match self.mode {
            Mode::HBlank => {
                if self.tick > TICK_WIDTH {
                    self.registers[LY] += 1;
                    if self.registers[LY] >= 143 {
                        // Send vblank interrupt
                        self.mode = Mode::VBlank;
                    } else {
                        self.mode = Mode::OAM;
                    }
                    self.tick = 0;
                }
            },
            Mode::VBlank => {
                if self.tick > TICK_WIDTH {
                    self.registers[LY] += 1;
                    if self.registers[LY] > EFFECTIVE_SCAN_COUNT {
                        self.registers[LY] = 0;
                        self.mode = Mode::OAM;
                    }
                    self.tick = 0;
                }

            },
            Mode::OAM => {
                if self.tick > OAM_WIDTH {
                    self.mode = Mode::VRAM;
                }
            },
            Mode::VRAM => {
                // render a pixel
            },
        }
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

    pub fn write_reg(&mut self, loc: u16, val: u8) {
        let l = loc as usize - 0xFF40;
        if l > 0xB {
            panic!("CGB functionallity is not supported")
        } else if l == 0x4 {
            panic!("0xFF44 is read only")
        }
        self.registers[l] = val
    }
    pub fn read_reg(&self, loc: u16) -> u8 {
        let l = loc as usize - 0xFF40;
        if l > 0xB {
            panic!("CGB functionallity is not supported")
        }
        self.registers[l]
    }
}
