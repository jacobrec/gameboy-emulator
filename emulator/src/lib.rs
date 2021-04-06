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
pub fn button_down(b: isize) {
    let bt = 1 << b; // ensure Emulator.ts and gameboy.rs have buttons in the same order
    unsafe {
        GAMEBOY.as_mut().unwrap().button_down(bt)
    }
}

#[wasm_bindgen]
pub fn button_up(b: isize) {
    let bt = 1 << b; // ensure Emulator.ts and gameboy.rs have buttons in the same order
    unsafe {
        GAMEBOY.as_mut().unwrap().button_up(bt)
    }
}

#[wasm_bindgen]
pub fn save_state() -> Vec<u8>{
    unsafe {
        let state = GAMEBOY.as_mut().unwrap().save();
        return bincode::serialize(&state).unwrap();
    }
}

#[wasm_bindgen]
pub fn load_state(state: Vec<u8>){
    unsafe {
        match bincode::deserialize(&state) {
            Ok(deser) =>  {
                let save: cpu::SaveState = deser;
                GAMEBOY.as_mut().unwrap().load(&save);
            }
            _ => panic!("Failed to load savestate")
        }
    }
}
