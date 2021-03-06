use std::fs::File;
use std::io::Read;
use std::env;
use std::time::{Duration, Instant};

mod utils;
mod cpu;
mod bus;
mod ppu;
mod apu;
mod instruction;
mod gameboy;

static ESC: &str = "\u{001b}";

fn open_file(filename: &str) -> Vec<u8> {
    let mut file = File::open(&filename).expect("no file found");
    let metadata = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("read error");
    buffer
}
#[derive(Clone, Copy)]
enum Display {
    CPU,
    AsciiHalf,
}
struct Args {
    display: Display,
}
fn cleanup_screen(d: Display) {
    match d {
        Display::CPU => (),
        Display::AsciiHalf => println!("{}[0m{}[?1049l", ESC, ESC),
    }
}
impl Drop for Args {
    fn drop(&mut self) {
        cleanup_screen(self.display);
    }
}

fn get_args () -> Args {
    let args: Vec<String> = env::args().collect();
    let mut display = Display::CPU;
    if args.iter().any(|x| x == "--ascii") {
        display = Display::AsciiHalf;
        println!("Display: Ascii");
        print!("{}[?1049h", ESC);
    }
    Args {display}
}

fn ascii_half_print(screen: &ppu::Screen) {
    fn format_color(mut num: u8) -> u8 {
        match num & 0b11 {
            0 => 97,
            1 => 37,
            2 => 90,
            _ => 30,
        }

    }
    fn print_pixel_pair(topcolor: u8, bottomcolor: u8) {
        let top_half = "▀";
        let fg = format_color(topcolor);
        let bg = format_color(bottomcolor);
        print!("{}[{};{}m{}", ESC, fg, bg + 10, top_half)
    }
    print!("{}[1;1f", ESC);
    for row in 0..(ppu::SCREEN_HEIGHT/2) {
        for col in 0..(ppu::SCREEN_WIDTH) {
            let ctop = (row * 2) * ppu::SCREEN_WIDTH + col;
            let cbot = (row * 2 + 1) * ppu::SCREEN_WIDTH + col;
            print_pixel_pair(screen[ctop], screen[cbot]);
        }
        print!("{}[0m\n", ESC);
    }
    println!("{}[0m", ESC);
    println!("Frame")
}

fn main_loop(mut gameboy: gameboy::Gameboy, args: Args) {
    ascii_half_print(&gameboy.get_screen());
    let mut start = Instant::now();
    loop {
        match args.display {
            Display::CPU => gameboy.print_cpu_state(),
            Display::AsciiHalf => {
                let duration = start.elapsed();
                if duration.as_secs_f64() > (1.0 / 17.0) {
                    start = Instant::now();
                    ascii_half_print(&gameboy.get_screen())
                }
            }
        }
        gameboy.tick();
    }
}

fn main() {
    let romdata = open_file("testrom/jbootrom.gb");
    // let romdata = open_file("bootrom.bin");
    let gameboy = gameboy::GameboyBuilder::new()
        .load_rom(gameboy::ROM::from_data(romdata))
        .build();

    let args = get_args();
    let d = args.display;

    ctrlc::set_handler(move || {
        cleanup_screen(d);
        println!("Bye!");
        std::process::exit(0x01);
    }).expect("Error setting Ctrl-C handler");
    main_loop(gameboy, args);
}
