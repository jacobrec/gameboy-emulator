use std::collections::VecDeque;
use std::num::Wrapping;
use serde::{Serialize, Deserialize};
use crate::cpu_recievable::{Recievables, CpuRecievable::*, Interrupt};

// TODO: add LCD_STAT interrupts
pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

pub type Screen = Vec<u8>; // u8 array. This holds colors 0-3
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
const SCX: usize = 3; // https://gbdev.io/pandocs/#ff42-scy-scroll-y-r-w-ff43-scx-scroll-x-r-w

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

#[derive(Clone,Copy,Debug, PartialEq)]
pub enum Mode {
    HBlank,
    VBlank,
    OAM,
    VRAM,
}

const DMA_TRANSFER_SIZE: u8 = 160;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DMAManager {
    start_location: u8,
    progress: Option<u8>,
}

// TODO: This will be tricky, we have to block off CPU
// access to memory while the transfer happens, as well as do the
// transfer with proper timing.
// Writing here replaces the whole OAM block with new data
// at a rate of 1 byte per cycle.
// For example, if you were to go LD $FF46, $10, the DMA would spend the next
// 160 cycles copying memory from 1000-109F to FE00-FE9F
// For example, if you were to go LD $FF46, $49, the DMA would spend the next
// 160 cycles copying the 160 bytes from 4900-499F to FE00-FE9F
impl DMAManager {
    fn new() -> Self {
        Self {
            start_location: 0,
            progress: None,
        }
    }

    pub fn next(&mut self) -> Option<(u16, u16)> {
        self.progress = self.progress.map(|x| x + 1).and_then(|x| if x > DMA_TRANSFER_SIZE { None } else { Some(x) });
        self.progress.map(|x| {
            let x = x - 1;
            println!("LOC: {:02X} {:02X}", self.start_location, x);
            let from: u16 = ((self.start_location as u16) << 8) + x as u16;
            let to: u16 = 0xFE00 + x as u16;
            (from, to)
        })
    }

    fn start_transfer(&mut self, val: u8) {
        self.start_location = val;
        self.progress = Some(0);
    }
}

#[derive(Serialize, Deserialize)]
pub struct PPU {
    screen: Screen,
    vram: Vec<u8>, // size of 0x2000
    registers: Vec<u8>,// size of 16
    tick: usize,
    oam_ram: Vec<Sprite>, // size of 40
    active_sprites: [Option<usize>; 10],
    pixel_fifo: VecDeque<PixelData>,
    pixels_pushed: usize,
    fetch_state: Wrapping<u8>,
    lx: u8,
    is_window: bool,
    #[serde(skip, default="crate::cpu_recievable::none_recivables")]
    recievables: Option<Recievables>,
    pub dma: DMAManager,
}

impl Clone for PPU {
    fn clone(&self) -> Self {
        Self {
            screen: self.screen.clone(),
            vram: self.vram.clone(), // size of 0x2000
            registers: self.registers.clone(),// size of 16
            tick: self.tick,
            oam_ram: self.oam_ram.clone(),
            active_sprites: self.active_sprites.clone(),
            pixel_fifo: self.pixel_fifo.clone(),
            pixels_pushed: self.pixels_pushed.clone(),
            fetch_state: self.fetch_state.clone(),
            lx: self.lx,
            is_window: self.is_window,
            recievables: None,
            dma: self.dma.clone(),
        }
    }
}

const TICK_WIDTH: usize = 456;
const OAM_WIDTH: usize = 80;
const EFFECTIVE_SCAN_COUNT: u8 = 153;

const color00: u8 = 0b00;
const color01: u8 = 0b01;
const color10: u8 = 0b10;
const color11: u8 = 0b11;

#[derive(Copy, Clone, Debug)]
#[derive(Serialize, Deserialize)]
struct Sprite {
    pos_x: u8,
    pos_y: u8,
    tile: u8,
    flags: u8,
}

impl Sprite {
    const fn new() -> Self {
        Sprite {
            pos_x: 0,
            pos_y: 0,
            tile:  0,
            flags: 0,
        }
    }
    fn from_memory(mem: &[u8; 0x2000], loc: usize) -> Self {
        Sprite {
            pos_y: mem[loc+0],
            pos_x: mem[loc+1],
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
    fn palette(&self) -> PixelSrc {
        if self.flags & 0b0100 > 0 {
            PixelSrc::S2
        } else {
            PixelSrc::S1
        }
    }
}

#[derive(Clone,Copy,Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
enum PixelSrc {
    BG, S1, S2
}
#[derive(Clone,Copy,Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
struct PixelData {
    value: u8, // Really this is a 2 bit number
    src: PixelSrc,
}

impl std::fmt::Display for PixelData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self.src {
            PixelSrc::BG => "BG",
            PixelSrc::S1 => "S1",
            PixelSrc::S2 => "S2",
        };
        write!(f, "<{}::{:02b}>", s, self.value)
    }
}


impl PPU {
    pub fn new() -> Self {
        let mut ppu =
            PPU {
                screen: [color00; SCREEN_WIDTH * SCREEN_HEIGHT].to_vec(),
                vram: [0u8; 0x2000].to_vec(),
                registers: [0u8; 0x10].to_vec(),
                oam_ram: [Sprite::new(); 40].to_vec(),
                tick: 0,
                active_sprites: [None; 10],
                pixel_fifo: VecDeque::new(),
                pixels_pushed: 0,
                fetch_state: Wrapping(0),
                is_window: false,
                lx: 0,
                recievables: None,
                dma: DMAManager::new(),
            };
        ppu.registers[LY] = 143;
        ppu
    }

    pub fn get_canvas(&self) -> Canvas {
        // Hex colors need to be in ABGR order for direct loading
        const color00: u32 = 0xFF0FBC9C;
        const color01: u32 = 0xFF0FAC8B;
        const color10: u32 = 0xFF306230;
        const color11: u32 = 0xFF0F380F;
        let mut canvas = [0u32; SCREEN_WIDTH * SCREEN_HEIGHT];
        for i in 0..(SCREEN_WIDTH * SCREEN_HEIGHT) {
            canvas[i] = match self.screen[i] {
                0b00 => color00,
                0b01 => color01,
                0b10 => color10,
                _ => color11,
            }
        }
        canvas

    }

    pub fn get_screen(&self) -> &Screen {
        &self.screen
    }

    fn lookup_color(&self, p: PixelData) -> u8 {
        let palette = match p.src {
            PixelSrc::BG => self.registers[BGP],
            PixelSrc::S1 => self.registers[OBP0],
            PixelSrc::S2 => self.registers[OBP1],
        };
        let num = palette >> ((p.value & 0b11) << 1) & 0b11;
        match num {
            0b00 => color00,
            0b01 => color01,
            0b10 => color10,
            0b11 => color11,
            _ => unreachable!("number should be mod 4"),
        }
    }

    fn tilemap_loc(&self, num: u8) -> u16 {
        if num > 0 {
            return 0x1C00 // 9C00 - 8000
        } else {
            return 0x1800 // 9800 - 8000
        }

    }
    fn get_effective_y(&self) -> u8 {
        let y = ((self.registers[LY] as u16 + self.registers[SCY] as u16) & 0xFF) as u8;
        y
    }
    fn window_or_background_tilemap_loc(&self, tilemap_loc: u16) -> usize {
        let y = self.get_effective_y();
        let x = self.lx;
        let offset = (y as usize) / 8 * 32 + (x as usize) / 8;
        let tile_idx = self.vram[tilemap_loc as usize + offset];
        if self.registers[LCD_CONTROL_REGISTER] & 0b10000 > 0 {
            return tile_idx as usize * 16;
        } else {
            return ((tile_idx as i8) as isize * 16 + 0x1000) as usize;
        }

    }
    fn background_tilemap_loc(&self) -> usize {
        let tilemap_loc = self.tilemap_loc(self.registers[LCD_CONTROL_REGISTER] & 0b00001000);
        self.window_or_background_tilemap_loc(tilemap_loc)
    }
    fn window_tilemap_loc(&self) -> usize {
        let tilemap_loc = self.tilemap_loc(self.registers[LCD_CONTROL_REGISTER] & 0b01000000);
        self.window_or_background_tilemap_loc(tilemap_loc)
    }
    fn sprite_tile_loc(&self, idx: u8) -> usize {
        idx as usize * 16
    }


    fn overlay_sprites(&mut self) {
        let y = self.registers[LY];
        let x = self.lx;
        for s in self.active_sprites.iter() {
            if let Some(s) = s {
                let s = self.oam_ram[*s];
                if s.pos_x == x + 8 { // TODO: this will not render sprites that are off left edge
                    let ey = 16 + y as isize - s.pos_y as isize;
                    let sp = self.decode_tile(self.sprite_tile_loc(s.tile), ey as usize);


                    for i in 0..8 {
                        if sp[i].value != 0 {
                            if let Some(e) = self.pixel_fifo.get_mut(i) {
                                *e = sp[i];
                            }
                        }
                    }
                }
            }
        }

    }

    fn decode_tile(&self, loc: usize, line: usize) -> [PixelData; 8] {
        let vloc = loc + line * 2;
        let bg_tile_low = self.vram[vloc];
        let bg_tile_high = self.vram[vloc + 1];
        let gen = move |i: usize| {
            let bh = (bg_tile_high >> (7 - i)) & 1;
            let bl = (bg_tile_low >> (7 - i)) & 1;
            bh << 1 | bl
        };
        let mut v = [PixelData{value: 0, src: PixelSrc::BG}; 8];
        for i in 0..v.len() {
            v[i].value = gen(i)
        }
        // println!("Tile loc: {:04X} on line {}. Effective y is {}. Decoded to {:?}",
        //  loc + 0x8000, line,
        //  (self.registers[LY] as usize + self.registers[SCY] as usize) & 0xFF,
        //  v
        // );
        v

    }

    fn fetch(&mut self) -> Option<[PixelData; 8]> {
        self.fetch_state += Wrapping(1);
        if let 0 = (self.fetch_state & Wrapping(0b111)).0 { // only update on last part of cycle
            // TODO: Window/Obj lookup
            let bg = self.background_tilemap_loc();
            //println!("{}", bg);
            let y = self.get_effective_y();
            let loc = bg as usize;
            let r = Some(self.decode_tile(loc, (y & 0b111) as usize));
            self.lx += 8;
            r
        } else {
            None
        }
    }

    fn sendif(&mut self, i: Interrupt) {
        match &self.recievables {
            Some(r) => r.send(SendInterrupt(i)),
            None => ()
        }
    }
    pub fn tick(&mut self) {
        self.tick += 1;
        self.registers[LCD_STATUS_REGISTER] &= 0b11111011 |
            if self.registers[LY] == self.registers[LYC] { self.sendif(Interrupt::LCDStat); 1 }
            else { 0 } << 2;
        match self.get_mode() {
            Mode::HBlank => {
                if self.tick > TICK_WIDTH {
                    self.registers[LY] += 1;
                    if self.registers[LY] >= 144 {
                        self.sendif(Interrupt::VBlank);
                        self.set_mode(Mode::VBlank);
                    } else {
                        self.set_mode(Mode::OAM);
                    }
                    self.tick = 0;
                }
            },
            Mode::VBlank => {
                if self.tick > TICK_WIDTH {
                    self.registers[LY] += 1;
                    if self.registers[LY] > EFFECTIVE_SCAN_COUNT {
                        self.registers[LY] = 0;
                        self.set_mode(Mode::OAM);
                    }
                    self.tick = 0;
                }

            },
            Mode::OAM => {
                if self.tick == 1 {
                    self.active_sprites = [None; 10];
                } else if self.tick == OAM_WIDTH {
                    // OAM lookup, this is normally done over 20 dots, but we'll just do it at the end
                    let mut i = 0;
                    let s_size = 8;
                    let ly = self.registers[LY];
                    for j in 0..40 {
                        let sj = self.oam_ram[j];
                        if sj.pos_x != 0 && ly + 16 >= sj.pos_y && ly + 16 < sj.pos_y + s_size {
                            self.active_sprites[i] = Some(j);
                            i += 1;
                        }
                        if i >= 10 {
                            break;
                        }
                    }
                    self.set_mode(Mode::VRAM);
                }
            },
            Mode::VRAM => {
                // render a pixel
                // 8 pixel cycle with fetcher
                // FIFO runs at 4 MHz, Fetch runs at 2MHz
                // FIFO     FETCH
                // Push     Read Tile #
                // Push
                // Push     Read Data 0
                // Push
                // Push     Read Data 1
                // Push
                // Push     Idle
                // Push
                //
                // When we hit window, the fifo is cleared, and the fetch switches to window
                let in_window = self.pixels_pushed >= self.registers[WX] as usize && self.registers[LY] >= self.registers[WY];
                let valid_window = in_window && self.registers[LCD_CONTROL_REGISTER] & 0b10000 > 0;
                if valid_window && !self.is_window { // window enabled
                    self.is_window = true;
                    self.pixel_fifo.clear();
                } else if self.is_window && !valid_window {
                    self.is_window = false;
                    self.pixel_fifo.clear();
                }

                if self.pixel_fifo.len() > 8 {
                    self.overlay_sprites();
                    let p = self.pixel_fifo.pop_front().unwrap(); // checked in the if statement
                    let y = self.registers[LY];
                    let x = self.pixels_pushed;
                    let color = self.lookup_color(p);
                    //println!("Pixel {}: (x, y)[{},{}] -> Color: {:X}", p, x, y, color);
                    if self.lx >= self.registers[SCX] {
                        self.screen[(x as usize) + (y as usize) * SCREEN_WIDTH] = color;
                        self.pixels_pushed += 1;
                    }
                }

                let new_pixels = self.fetch();
                if let Some(px) = new_pixels {
                    for p in px.iter() {
                        self.pixel_fifo.push_back(*p);
                    }
                }

                if self.pixels_pushed >= 160 {
                    self.pixels_pushed = 0;
                    self.lx = 0;
                    self.set_mode(Mode::HBlank);
                    self.pixel_fifo.clear();
                }
            },
        }
    }
    fn set_mode(&mut self, mode: Mode) {
        let v = match mode {
            Mode::HBlank => 0b00,
            Mode::VBlank => 0b01,
            Mode::OAM => 0b10,
            Mode::VRAM => 0b11,
        };

        self.registers[LCD_STATUS_REGISTER] &= 0b11111100;
        self.registers[LCD_STATUS_REGISTER] |= v;
    }
    fn get_mode(&self) -> Mode {
        match self.registers[LCD_STATUS_REGISTER] & 0b11 {
            0b00 => Mode::HBlank,
            0b01 => Mode::VBlank,
            0b10 => Mode::OAM,
            0b11 => Mode::VRAM,
            _ => unreachable!("exhaustive match pattern")
        }
    }

    // Both read and write expect loc to be in the address range 0x8000..=0x9FFF
    pub fn write(&mut self, loc: u16, val: u8) {
        let l = loc as usize - 0x8000;
        // print!("PPU Write: [{:04X}] = {:02X}. During mode {:?}\n", loc, val, self.get_mode());
        match self.get_mode() {
            Mode::VRAM => self.vram[l] = val, // TODO: do nothing
            _ => self.vram[l] = val,
        }
    }
    pub fn read(&self, loc: u16) -> u8 {
        let l = loc as usize - 0x8000;
        match self.get_mode() {
            Mode::VRAM => self.vram[l], // TODO: this should be 0xFF
            _ => self.vram[l],
        }
    }
    pub fn writeOAM(&mut self, loc: u16, val: u8) {
        let l = (loc as usize - 0xFE00) / 4;
        match self.get_mode() {
            Mode::VRAM => (),
            Mode::OAM => (),
            _ => match loc & 0b11 {
                0 => self.oam_ram[l].pos_y = val,
                1 => self.oam_ram[l].pos_x = val,
                2 => self.oam_ram[l].tile = val,
                3 => self.oam_ram[l].flags = val,
                _ => unreachable!("exhaustive match pattern")
            }
        }
    }
    pub fn readOAM(&self, loc: u16) -> u8 {
        let l = (loc as usize - 0xFE00) / 4;
        match self.get_mode() {
            Mode::VRAM => 0xFF,
            Mode::OAM => 0xFF,
            _ => match loc & 0b11 {
                    0 => self.oam_ram[l].pos_y,
                    1 => self.oam_ram[l].pos_x,
                    2 => self.oam_ram[l].tile,
                    3 => self.oam_ram[l].flags,
                    _ => unreachable!("exhaustive match pattern")
                }
        }
    }

    pub fn write_reg(&mut self, loc: u16, val: u8) {
        let l = loc as usize - 0xFF40;
        self.registers[l] = val;
        if l > 0xB {
            panic!("CGB functionallity is not supported")
        } else if l == DMA {
            self.dma.start_transfer(val);
        } else if l == LY {
            panic!("0xFF44 is read only")
        }
    }
    pub fn read_reg(&self, loc: u16) -> u8 {
        let l = loc as usize - 0xFF40;
        if l > 0xB {
            panic!("CGB functionallity is not supported")
        }
        self.registers[l]
    }

    pub fn set_recievables(&mut self, recievables: Recievables) {
        self.recievables = Some(recievables)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_test_ppu() -> PPU {
        PPU::new()
    }

    #[test]
    fn test_pixel_color_lookup () {
        let mut ppu = create_test_ppu();

        ppu.registers[BGP] = 0b11100100;
        assert_eq!(color00, ppu.lookup_color(PixelData{
            src: PixelSrc::BG,
            value: 0b00,
        }));
        assert_eq!(color01, ppu.lookup_color(PixelData{
            src: PixelSrc::BG,
            value: 0b01,
        }));
        assert_eq!(color10, ppu.lookup_color(PixelData{
            src: PixelSrc::BG,
            value: 0b10,
        }));
        assert_eq!(color11, ppu.lookup_color(PixelData{
            src: PixelSrc::BG,
            value: 0b11,
        }));

        ppu.registers[BGP] = 0b10110001;
        assert_eq!(color01, ppu.lookup_color(PixelData{
            src: PixelSrc::BG,
            value: 0b00,
        }));
        assert_eq!(color00, ppu.lookup_color(PixelData{
            src: PixelSrc::BG,
            value: 0b01,
        }));
        assert_eq!(color11, ppu.lookup_color(PixelData{
            src: PixelSrc::BG,
            value: 0b10,
        }));
        assert_eq!(color10, ppu.lookup_color(PixelData{
            src: PixelSrc::BG,
            value: 0b11,
        }));

    }

    #[test]
    fn test_tile_decode () {
        let testtile = [0x0F, 0x00, 0x0F, 0x00, 0x0F, 0x00, 0x0F, 0x00,
                        0x0F, 0xFF, 0x0F, 0xFF, 0x0F, 0xFF, 0x0F, 0xFF];
        let mut ppu = create_test_ppu();
        ppu.registers[BGP] = 0b11100100;
        ppu.registers[LCD_CONTROL_REGISTER] = 0b10010000;
        for i in 0..testtile.len(){
            ppu.vram[i+16] = testtile[i];
        }

        for i in 0..(32*32) {
            ppu.vram[0x1800 + i] = 1;
        }
        for i in 0..7 {
            assert_eq!(None, ppu.fetch())
        }
        let p00 = PixelData{src: PixelSrc::BG, value: 0b00};
        let p01 = PixelData{src: PixelSrc::BG, value: 0b01};
        let p10 = PixelData{src: PixelSrc::BG, value: 0b10};
        let p11 = PixelData{src: PixelSrc::BG, value: 0b11};
        assert_eq!(Some([p00, p00, p00, p00, p01, p01, p01, p01]), ppu.fetch());

        ppu.lx = 0;
        ppu.registers[LY] = 1;
        for i in 0..7 {
            assert_eq!(None, ppu.fetch())
        }
        assert_eq!(Some([p00, p00, p00, p00, p01, p01, p01, p01]), ppu.fetch());

        ppu.lx = 0;
        ppu.registers[LY] = 4;
        for i in 0..7 {
            assert_eq!(None, ppu.fetch())
        }
        assert_eq!(Some([p10, p10, p10, p10, p11, p11, p11, p11]), ppu.fetch());

        ppu.lx = 0;
        ppu.registers[LY] = 0;
        ppu.registers[SCY] = 6;
        for i in 0..7 {
            assert_eq!(None, ppu.fetch())
        }
        assert_eq!(Some([p10, p10, p10, p10, p11, p11, p11, p11]), ppu.fetch());
    }

    #[test]
    fn test_ppu_tick () {
        let testtile = [0x0F, 0x00, 0x0F, 0x00, 0x0F, 0x00, 0x0F, 0x00,
                        0x0F, 0xFF, 0x0F, 0xFF, 0x0F, 0xFF, 0x0F, 0xFF];
        let mut ppu = create_test_ppu();
        ppu.registers[BGP] = 0b11100100;
        ppu.registers[LCD_CONTROL_REGISTER] = 0b10010000;
        assert_eq!(ppu.get_mode(), Mode::HBlank);
        let offset = 9;
        for i in 0..(TICK_WIDTH+1) {
            ppu.tick();
        }
        ppu.registers[LY] = 0;
        ppu.registers[SCY] = 16;
        assert_eq!(ppu.get_mode(), Mode::OAM);
        for i in 0..testtile.len(){
            ppu.vram[i+offset*16] = testtile[i];
        }
        for i in 0..(32*32) {
            ppu.vram[0x1800 + i] = offset as u8;
        }
        for _ in 0..(80 + 250) {
            ppu.tick();
        }
        assert_eq!(ppu.get_mode(), Mode::HBlank);
        while ppu.get_mode() != Mode::VBlank {
            ppu.tick();
        }

        let line = 144 - 8;
        for j in line..(line+8) {
            for i in 0..8{
                print!("{:X} ", ppu.screen[SCREEN_WIDTH*j + i]);
            }
            println!();
        }

        for j in 0..SCREEN_HEIGHT {
            for i in 0..SCREEN_WIDTH {
                let should_color =
                    if (i / 4) % 2 == 0 {
                        if (j / 4) % 2 == 0 {
                            color00
                        } else {
                            color10
                        }
                    } else {
                        if (j / 4) % 2 == 0 {
                            color01
                        } else {
                            color11
                        }
                    };
                //println!("Checking ({}, {}) to be {}", i, j, format!("{:X}", should_color));
                assert_eq!(format!("{:X}", ppu.screen[SCREEN_WIDTH * j + i]), format!("{:X}", should_color));
                // assert_eq!(ppu.screen[i], should_color);
            }
        }
    }
}
