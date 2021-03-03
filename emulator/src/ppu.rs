pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
pub type Canvas = [u32; SCREEN_WIDTH * SCREEN_HEIGHT]; // rgba u32 array. This will get passed and loaded into canvas

// https://gbdev.io/pandocs/#ff40-lcd-control-register
// this register is a bitfield containing bits 76543210
// Bit   Name                               Usage notes
// 7     LCD Display Enable                 0=Off, 1=On
// 6     Window Tile Map Display Select     0=9800-9BFF, 1=9C00-9FFF
// 5     Window Display Enable              0=Off, 1=On
// 4     BG & Window Tile Data Select       0=8800-97FF, 1=8000-8FFF
// 3     BG Tile Map Display Select         0=9800-9BFF, 1=9C00-9FFF
// 2     OBJ (Sprite) Size                  0=8x8, 1=8x16
// 1     OBJ (Sprite) Display Enable        0=Off, 1=On
// 0     BG and Window Display/Priority     0=Off, 1=On
const LCD_CONTROL_REGISTER: usize = 0;

// https://gbdev.io/pandocs/#lcd-status-register
// Bit 6 - LYC=LY Coincidence Interrupt (1=Enable) (Read/Write)
// Bit 5 - Mode 2 OAM Interrupt         (1=Enable) (Read/Write)
// Bit 4 - Mode 1 V-Blank Interrupt     (1=Enable) (Read/Write)
// Bit 3 - Mode 0 H-Blank Interrupt     (1=Enable) (Read/Write)
// Bit 2 - Coincidence Flag  (0:LYC<>LY, 1:LYC=LY) (Read Only)
// Bit 1-0 - Mode Flag       (Mode 0-3, see below) (Read Only)
//           0: During H-Blank
//           1: During V-Blank
//           2: During Searching OAM
//           3: During Transferring Data to LCD Driver
const LCD_STATUS_REGISTER: usize = 1;

// scroll x and y
const SCY: usize = 2; // https://gbdev.io/pandocs/#ff42-scy-scroll-y-r-w-ff43-scx-scroll-x-r-w
const SXY: usize = 3; // https://gbdev.io/pandocs/#ff42-scy-scroll-y-r-w-ff43-scx-scroll-x-r-w

// current line, read only
const LY: usize = 4; // https://gbdev.io/pandocs/#ff44-ly-lcdc-y-coordinate-r
// This is read and writeable, bit 2 of lcd status is set if this equals ly
const LYC: usize = 5; // https://gbdev.io/pandocs/#ff45-lyc-ly-compare-r-w

const DMA: usize = 6; // https://gbdev.io/pandocs/#ff46-dma-dma-transfer-and-start-address-r-w

// pallet, maps to the hex code below
// bits 6-7: color for 11
// bits 4-5: color for 10
// bits 2-3: color for 01
// bits 0-1: color for 00
const BGP: usize = 7; // https://gbdev.io/pandocs/#ff47-bgp-bg-palette-data-r-w-non-cgb-mode-only
// same as above, but bits 0-1 are ignore, as 00 is transparant for sprites
const OBP0: usize = 8; // https://gbdev.io/pandocs/#ff48-obp0-object-palette-0-data-r-w-non-cgb-mode-only
const OBP1: usize = 9; // https://gbdev.io/pandocs/#ff48-obp0-object-palette-0-data-r-w-non-cgb-mode-only

// https://gbdev.io/pandocs/#ff4a-wy-window-y-position-r-w-ff4b-wx-window-x-position-7-r-w
// Window x and y offsets
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
    oamRAM: [Sprite; 40],
}
const TICK_WIDTH: usize = 456;
const OAM_WIDTH: usize = 80;
const EFFECTIVE_SCAN_COUNT: u8 = 153;

const color00: u32 = 0xFFFFB5FF;
const color01: u32 = 0x7BC67BFF;
const color10: u32 = 0x6B8C42FF;
const color11: u32 = 0x5A3921FF;

struct Sprite {
    pos_x: u8,
    pos_y: u8,
    tile: u8,
    flags: u8,
}

impl Sprite {
    fn new() -> Self {
        Sprite {
            pos_x: 0,
            pos_y: 0,
            tile:  0,
            flags: 0,
        }
    }
    fn from_memory(mem: &[u8; 0x2000], loc: usize) -> Self {
        let b3 = mem[loc+3];
        Sprite {
            pos_x: mem[loc],
            pos_y: mem[loc+1],
            tile:  mem[loc+2],
            flags: mem[loc+3],
        }
    }

    fn priority(&self) -> u8 {
        (self.flags & 0b1000) >> 3
    }
    fn is_x_flipped(&self) -> bool {
        self.flags & 0b0100 > 0
    }
    fn is_y_flipped(&self) -> bool {
        self.flags & 0b0100 > 0
    }
    fn palette(&self) -> u8 {
        self.flags & 0b0100
    }
}

impl PPU {
    pub const fn new() -> Self {
        PPU {
            screen: [color00; SCREEN_WIDTH * SCREEN_HEIGHT],
            vram: [0u8; 0x2000],
            registers: [0u8; 0x10],
            oamRAM: [Sprite::new(), 40],
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

    pub fn writeOAM(&mut self, loc: u16, val: u8) {
        let l = (loc as usize - 0xFE00) / 4;
        match loc % 4 {
            0 => self.oamRAM[l] = val,
            1 => self.oamRAM[l] = val,
            2 => self.oamRAM[l] = val,
            3 => self.oamRAM[l] = val,
        }
    }
    pub fn readOAM(&self, loc: u16) -> u8 {
        let l = (loc as usize - 0xFE00) / 4;
        match loc % 4 {
            0 => self.oamRAM[l].pos_x,
            1 => self.oamRAM[l].pos_y,
            2 => self.oamRAM[l].tile,
            3 => self.oamRAM[l].flags,
        }
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
