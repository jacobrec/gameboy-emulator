mod utils;
mod cpu;
mod apu;
mod bus;
mod ppu;
mod instruction;
mod gameboy;

use wasm_bindgen::prelude::*;

static mut X: u16 = 0;
// TODO: Maybe don't use a static mut, and instead pass a reference to the
// frontend for the gb object, and just get that back each update call
// static mut GAMEBOY: gameboy::Gameboy = gameboy::Gameboy::empty();


#[wasm_bindgen]
pub fn init(romdata: Vec<u8>) {
    utils::set_panic_hook();
    let mut gameboy = gameboy::GameboyBuilder::new()
        .load_rom(gameboy::ROM::from_data(romdata))
        .build();
    gameboy.tick();
    // unsafe {
      // GAMEBOY.set_state(gameboy)
    // }
}

#[wasm_bindgen]
pub fn update(x: isize) {
    for _ in 0..x {
        unsafe {
          // GAMEBOY.tick();
          X += 1
        }
    }
}



#[wasm_bindgen]
pub fn get_screen() -> u16 {
    unsafe {
        X
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
