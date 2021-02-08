use std::fs::File;
use std::io::Read;

mod utils;
mod cpu;
mod bus;
mod ppu;
mod instruction;
mod gameboy;

fn open_file(filename: &str) -> Vec<u8> {
    let mut file = File::open(&filename).expect("no file found");
    let metadata = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("read error");
    buffer
}

fn main() {
    let romdata = open_file("bootrom.bin");
    let mut gameboy = gameboy::GameboyBuilder::new()
        .load_rom(gameboy::ROM::from_data(romdata))
        .build();
    for _ in 0..100 {
        gameboy.tick();
        gameboy.print_cpu_state();
    }

    println!("Hello, world!")
}
