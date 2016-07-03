#![feature(mpsc_select)]
#![feature(box_syntax)]
#![feature(inclusive_range_syntax)]
extern crate sdl2;
extern crate strfmt;

extern crate chip8;

use std::thread;
use std::sync::mpsc;

use std::collections::HashMap;


mod ui;
mod emulator;
mod fileio;
mod programs;

use ui::Ui;
use chip8::{Emulator, SharedState};
use emulator::Supervisor;

use chip8::Config;

fn main() {

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

    let programs = programs::examples();

    let mut test_program = programs.get("bench").unwrap().clone();

    // let mut test_program = Vec::new();
    // let mut in_file = File::open("examples/tank.ch8").unwrap();
    // let loader = BinaryLoader::new(in_file);
    // let test_program = load_file("examples/tank.ch8", LoaderType::Auto);

    print!("[");
    for b in test_program.iter() {
        print!("0x{:X}, ", b);
    }
    println!("]");

    let emulator_thread = thread::spawn(move || {
        let mut emu_supervisor = Supervisor::new(config, emulator_state);
        let load_offset = 0x200;
        emu_supervisor.core.load_bytes(&test_program, load_offset);
        emu_supervisor.core.jump_pc(load_offset);
        emu_supervisor.run();
        tx_emulator.send(()).unwrap();
    });

    // Select between signals from either thread
    select! {
        _ = rx_emulator.recv() => println!("emulator"),
        v = rx_ui.recv() => println!("ui: {}", v.unwrap())
        }

    // thread::sleep(std::time::Duration::new(5, 0));

}
