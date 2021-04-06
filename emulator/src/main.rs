use rodio::queue;
use rodio::{buffer::SamplesBuffer, OutputStream, OutputStreamHandle};
use std::env;
use std::fs::File;
use std::io::Read;
use std::thread;
use std::time::{Duration, Instant};

mod apu;
mod bus;
mod cartridge;
mod cpu;
mod cpu_recievable;
mod gameboy;
mod instruction;
mod ppu;
mod timer;
mod utils;

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
    None,
    CPU,
    CPUAlt,
    AsciiHalf,
}
struct Args {
    display: Display,
    stepmode: bool,
}
fn cleanup_screen(d: Display) {
    match d {
        Display::None => (),
        Display::CPU => (),
        Display::CPUAlt => (),
        Display::AsciiHalf => println!("{}[0m{}[?1049l", ESC, ESC),
    }
}
impl Drop for Args {
    fn drop(&mut self) {
        cleanup_screen(self.display);
    }
}

fn get_args() -> Args {
    let args: Vec<String> = env::args().collect();
    let mut display = Display::None;
    let mut stepmode = false;
    if args.iter().any(|x| x == "--ascii") {
        display = Display::AsciiHalf;
        println!("Display: Ascii");
        print!("{}[?1049h", ESC);
    }

    if args.iter().any(|x| x == "--step") {
        stepmode = true;
    }

    if args.iter().any(|x| x == "--alt") {
        display = Display::CPUAlt;
    }
    if args.iter().any(|x| x == "--cpu") {
        display = Display::CPU;
    }
    Args { display, stepmode }
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
    for row in 0..(ppu::SCREEN_HEIGHT / 2) {
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

fn main_loop(mut gameboy: gameboy::Gameboy, args: Args, saver: Saver) {
    let mut start = Instant::now();
    let mut frametime = Instant::now();
    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // let (tx, mut rx) = queue::queue(false);

    // let audio_buffer = gameboy.get_audio_buffer();
    // let data: Vec<f32> = (0..4096).map(|n| -0.5 + (n % 2) as f32).collect();
    // let sample_buffer = SamplesBuffer::new(2, 44100, audio_buffer);
    // let result = stream_handle.play_raw(sample_buffer);

    loop {
        match args.display {
            Display::None => (),
            Display::CPUAlt => gameboy.print_alt(),
            Display::CPU => gameboy.print_cpu_state(),
            Display::AsciiHalf => {
                let duration = frametime.elapsed();
                if duration.as_secs_f64() > (1.0 / 17.0) {
                    frametime = Instant::now();
                    ascii_half_print(&gameboy.get_screen())
                }
            }
        }
        gameboy.tick();

        // let audio_buffer = gameboy.get_audio_buffer();
        // let sample_buffer = SamplesBuffer::new(2, 44100, audio_buffer);
        // stream_handle.play_raw(sample_buffer);

        let mut duration = start.elapsed();
        let desiredtime = Duration::from_nanos(1000);
        let elapsed = desiredtime.checked_sub(duration);
        match elapsed {
            None => start = start.checked_add(desiredtime).unwrap(),
            Some(x) => {
                start = Instant::now();
                thread::sleep(x)
            }
        }

        let savestatefile = "savestate";
        match saver.lock().unwrap().pop_front() {
            Some(SignalOp::SaveState) => {
                let mut f = BufWriter::new(File::create(savestatefile).unwrap());
                let state = gameboy.save();
                bincode::serialize_into(&mut f, &state);
            }
            Some(SignalOp::LoadState) => {
                let state = open_file(savestatefile);
                match bincode::deserialize(&state) {
                    Ok(deser) => {
                        let save: cpu::SaveState = deser;
                        gameboy.load(&save);
                    }
                    _ => println!("Failed to load savestate"),
                }
            }
            None => (),
        }
    }
}

use bincode::serialize_into;
use std::collections::VecDeque;
use std::io::BufWriter;
use std::sync::{Arc, Mutex};

enum SignalOp {
    SaveState,
    LoadState,
}
type Saver = Arc<Mutex<VecDeque<SignalOp>>>;

fn main() {
    // let romdata = open_file("cpu_instrs.gb");
    // let romdata = open_file("testrom/jtest.gb");
    let romdata = open_file("tetris.gb");
    // let romdata = open_file("testrom/dtest.gb");
    let bios = open_file("bootrom.bin"); // gameboy state now starts after bootrom has complete
    let mut gameboy = gameboy::GameboyBuilder::new()
        .load_rom(cartridge::Cartridge::from_data(romdata))
        .load_bios(bios)
        .build();

    let args = get_args();
    let d = args.display;
    let mut db = cpu::DebugOptions::default();
    let saver: Saver = Arc::new(Mutex::new(VecDeque::new()));

    gameboy.button_down(gameboy::BUT_RIGHT);

    match args.display {
        Display::None => db.debug_print = false,
        Display::CPU => db.debug_print = true,
        Display::AsciiHalf => db.debug_print = false,
        Display::CPUAlt => db.debug_print = false,
    }

    db.debug_step = args.stepmode;
    gameboy.set_debug_options(db);

    ctrlc::set_handler(move || {
        println!("Cleaning up");
        cleanup_screen(d);
        println!("Bye!");
        std::process::exit(0x01);
    })
    .expect("Error setting Ctrl-C handler");

    // use signal_hook::{iterator::Signals, SIGUSR1, SIGUSR2};
    // let signals = Signals::new(&vec![SIGUSR1, SIGUSR2]).unwrap();
    // let saver2 = saver.clone();
    // thread::spawn(move || {
    //     for sig in signals.forever() {
    //         match sig {
    //             SIGUSR1 => saver2.lock().unwrap().push_back(SignalOp::SaveState),
    //             SIGUSR2 => saver2.lock().unwrap().push_back(SignalOp::LoadState),
    //             _ => println!("Received signal {:?}", sig),
    //         }
    //     }
    // });

    main_loop(gameboy, args, saver);
}
