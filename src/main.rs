#![feature(mpsc_select)]
#![feature(box_syntax)]
#![feature(inclusive_range_syntax)]
extern crate sdl2;
extern crate strfmt;
extern crate clap;
#[macro_use]
extern crate nom;
extern crate chip8;

use std::thread;
use std::sync::mpsc;

mod ui;
mod emulator;
mod fileio;
mod programs;
mod options;

use ui::Ui;
use chip8::{SharedState};
use emulator::Emulator;
use options::parse_commandline;
use chip8::Config;
use fileio::{load_file, LoaderType};

fn main() {

    let options = parse_commandline();

    let mut config = Config::new();
    config.print_instructions = false;

    let (tx_ui, rx_ui) = mpsc::channel();
    let (tx_emulator, rx_emulator) = mpsc::channel();


    let state = SharedState::new();

    let ui_state = state.clone();
    let emulator_state = state.clone();
    let ui_thread = thread::spawn(move || {
        let mut ui = Ui::new(config, ui_state);
        ui.run();
        tx_ui.send(0).unwrap();
    });


    // let mut test_program = Vec::new();

    // let loader = BinaryLoader::new(in_file);

    let test_program = load_file(&options.filename, LoaderType::Auto);

    print!("[");
    for b in test_program.iter() {
        print!("0x{:2X}, ", b);
    }
    println!("]");

    let emulator_thread = thread::spawn(move || {
        let mut emulator = Emulator::new(config, emulator_state);
        let load_offset = 0x200;
        emulator.core.load_bytes(&test_program, load_offset);
        emulator.core.jump_pc(load_offset);
        emulator.run();
        tx_emulator.send(()).unwrap();
    });

    select! {
        _ = rx_emulator.recv() => println!("emulator"),
        v = rx_ui.recv() => println!("ui: {}", v.unwrap())
        }

    // thread::sleep(std::time::Duration::new(5, 0));

}
