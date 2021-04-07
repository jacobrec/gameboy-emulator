use crate::cpu::*;
use std::io::prelude::*;
use std::io::{self, BufRead};
use serde::{Serialize, Deserialize};

#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct DebugOptions {
    pub debug_print: bool,
    pub debug_step: bool,
    pub break_points: Vec<u16>, // wait for enter when pc is here
    pub watch_points: Vec<u16>, // wait for enter when this memory is written to
    pub pause_on_branch: bool,
}
impl DebugOptions {
    pub fn default() -> Self {
        Self {
            debug_print: true,
            debug_step: false,
            pause_on_branch: false,
            break_points: Vec::new(),
            watch_points: Vec::new(),
        }
    }
}

pub fn runline(cpu: &mut CPU) {
    let stdin = io::stdin();
    loop {
        print!("dbg> ");
        io::stdout().flush();
        let line = stdin.lock().lines().next().unwrap().unwrap();
        cpu.debug_options.debug_step = true;
        let line1 = line.to_lowercase();
        let mut line_data = line1.split_whitespace();
        let cmd = line_data.next();
        match cmd {
            None | Some("s") | Some("step") => break,
            Some("r") | Some("run") | Some("c") | Some("continue") => {
                cpu.debug_options.debug_step = false;
                break;
            },
            Some("p") | Some("print") => print(cpu, line_data),
            Some("b") | Some("break") | Some("breakpoints") => manage_datapoints(&mut cpu.debug_options.break_points, line_data, "break"),
            Some("w") | Some("watch") | Some("watchpoints") => manage_datapoints(&mut cpu.debug_options.watch_points, line_data, "watch"),
            Some("h") | Some("help") => help(),
            Some("set") => manage_settings(line_data, &mut cpu.debug_options),
            Some(_) => println!("Unknown Command. Try help")
        }
    }
}

fn with_bool<F>(mut f: F, s: &str) where  F: FnMut(bool) {
    match s {
        "off" | "0" | "false" => f(false),
        "on" | "1" | "true" => f(true),
        _ => println!("Failed to parse bool"),
    }
}
fn with_option_none_is_true(b: &mut bool, opt: Option<&str>) {
    match opt {
        Some(s) => with_bool(|x| *b = x, s),
        None => *b = true,
    }
}

fn manage_settings(mut options: std::str::SplitWhitespace, dbo: &mut DebugOptions) {
    match options.next() {
        Some("pause_on_break") => with_option_none_is_true(&mut dbo.pause_on_branch, options.next()),
        Some("cpu_print") => with_option_none_is_true(&mut dbo.debug_print, options.next()),
        Some("help") => {
            println!("Sets a value. Examples:");
            println!("  set pause_on_break");
            println!("  set cpu_print off");
        },
        Some(_) => println!("Unknown set argument. Try set help"),
        None => println!("Set requires an argument. Try set help"),
    }
}

fn manage_datapoints(items: &mut Vec<u16>, mut options: std::str::SplitWhitespace, display: &str) {
    match options.next() {
        None => {
            println!("{}points:", display);
            for x in items {
                println!("  0x{:04X} ({:5}d)", x, x);
            }
        }
        Some("rm") => {
            match options.next() {
                Some(s) => with_number(|x| {
                    items.sort();
                    items.binary_search(&x).map(|x| items.remove(x));
                }, s),
                None => println!("{} rm requires an argument", display),
            }
        },
        Some(s) => with_number(|x| items.push(x), s),
    }

}

fn help() {
    println!("Examples of Valid Commands:");
    println!("  print");
    println!("  print (hl)");
    println!("  print de");
    println!("  print a");
    println!("  print mem 16");
    println!("  print mem 0x10");
    println!("  print mem 0x100 0x200");
    println!("  break 0x100");
    println!("  watch 0xFF47");
    println!("  breakpoints");
    println!("  step");
    println!("  continue");
    println!("  set help");
    println!("  help");
}

fn with_number<F>(mut f: F, s: &str) where  F: FnMut(u16) {
    match parse_number16(s) {
        Some(i) => f(i),
        None => println!("Failed to parse u16 number from: [{}]", s)
    }
}

fn parse_number16(s: &str) -> Option<u16> {
    let t = if s.starts_with("0x") {
        u16::from_str_radix(s.trim_start_matches("0x"), 16)
    } else if s.starts_with("0b") {
        u16::from_str_radix(s.trim_start_matches("0b"), 2)
    } else {
        u16::from_str_radix(s, 10)
    };
    t.ok()
}

fn print(cpu: &mut CPU, mut options: std::str::SplitWhitespace) {
    let head = options.next();
    match head {
        None => cpu.print_state(),
        Some("mem") => {
            match (options.next(), options.next()) {
                (None, None) => println!("mem requires an argument"),
                (None, Some(_)) => unreachable!(),
                (Some(a), None) => with_number(|x| println!("{:02X}", cpu.bus.read(x)), a),
                (Some(a), Some(b)) => {
                    let start = parse_number16(a);
                    let end = parse_number16(b);

                    match (start, end) {
                        (Some(start), Some(end)) => {
                            let mut i = 0;
                            print!("{:04X}: ", start);
                            for idx in start..=end {
                                i += 1;
                                print!("{:02X} ", cpu.bus.read(idx));
                                if i % 0x10 == 0 {
                                    println!("");
                                    print!("{:04X}: ", idx + 1);
                                }
                            }
                            println!("");
                        },
                        _ => println!("Failed to parse numbers")
                    }
                }
            }
        },
        Some("a") => println!("{:02X}", cpu.a()),
        Some("b") => println!("{:02X}", cpu.b()),
        Some("c") => println!("{:02X}", cpu.c()),
        Some("d") => println!("{:02X}", cpu.d()),
        Some("e") => println!("{:02X}", cpu.e()),
        Some("h") => println!("{:02X}", cpu.h()),
        Some("l") => println!("{:02X}", cpu.l()),
        Some("(hl)") => println!("{:02X}", cpu.bus.read(cpu.hl())),
        Some("bc") => println!("{:04X}", cpu.bc()),
        Some("de") => println!("{:04X}", cpu.de()),
        Some("hl") => println!("{:04X}", cpu.hl()),
        Some(_) => println!("Unknown argument for print"),
    }

}
