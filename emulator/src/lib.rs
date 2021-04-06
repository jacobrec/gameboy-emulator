mod utils;
mod cpu;
mod apu;
mod bus;
mod timer;
mod ppu;
mod instruction;
mod gameboy;
mod cpu_recievable;
mod cartridge;

use wasm_bindgen::prelude::*;

// TODO: Maybe don't use a static mut, and instead pass a reference to the
// frontend for the gb object, and just get that back each update call
static mut GAMEBOY: Option<gameboy::Gameboy> = None;


#[wasm_bindgen]
pub fn init(romdata: Vec<u8>) {
    utils::set_panic_hook();
    let gameboy = gameboy::GameboyBuilder::new()
        .load_rom(cartridge::Cartridge::from_data(romdata))
        .build();
    unsafe {
        GAMEBOY = Some(gameboy)
    }
}

#[wasm_bindgen]
pub fn update(x: usize) -> Vec<u32> {
    for _ in 0..x {
        unsafe {
          GAMEBOY.as_mut().unwrap().tick();
        }
    }
    unsafe {
        GAMEBOY.as_mut().unwrap().get_canvas().to_vec()
    }
}

#[wasm_bindgen]
pub fn press_button(b: isize) {
    match b {
        0 => (), // Start
        1 => (), // Select
        2 => (), // DUp
        3 => (), // DDown
        4 => (), // DLeft
        5 => (), // DRight
        6 => (), // A
        7 => (), // B
        _ => panic!("Unknown button pressed")
    }
}
