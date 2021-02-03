mod utils;
mod cpu;
mod bus;
mod ppu;
mod gameboy;

use wasm_bindgen::prelude::*;


static mut X: u16 = 0;

fn clock_tick() {
    unsafe {
        X += 1;
    }
}

#[wasm_bindgen]
pub fn update(x: isize) {
    utils::set_panic_hook();
    for _ in 0..x {
        clock_tick();
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
