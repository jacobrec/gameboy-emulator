mod utils;

use wasm_bindgen::prelude::*;


static mut X: u8 = 0;

#[wasm_bindgen]
pub fn run_emulator() {
    utils::set_panic_hook();
    loop {
        unsafe {
            X += 1;
        }
    }
}



#[wasm_bindgen]
pub fn check_x() -> u8 {
    unsafe {
        X
    }
}
