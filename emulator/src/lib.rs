mod utils;

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
pub fn check_x() -> u16 {
    unsafe {
        X
    }
}
