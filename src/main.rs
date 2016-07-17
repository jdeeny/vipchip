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
//mod emulator;
mod fileio;
mod programs;
mod options;

use ui::Ui;
//use emulator::Emulator;
use options::parse_commandline;
use fileio::{load_file, LoaderType};

use chip8::{Config, SimulatorTask, Simulate};
use chip8::config::COSMAC_VIP;


fn main() {

    let options = parse_commandline();

    let test_program = load_file(&options.filename, LoaderType::Auto);

    let (tx_ui, rx_ui) = mpsc::channel();
//    let (tx_emulator, rx_emulator) = mpsc::channel();


/*    let state = UiState::new();

    let ui_state = state.clone();
    let emulator_state = state.clone();*/

    let simulator_task = SimulatorTask::spawn(COSMAC_VIP);

    let ui_thread = thread::spawn(move || {
        let mut ui = Ui::new(simulator_task);
        ui.run();
        tx_ui.send(0).unwrap();
    });

    /*let emulator_thread = thread::spawn(move || {
        let mut emulator = Emulator::new(config, emulator_state);
        let load_offset = 0x200;
        emulator.core.load_bytes(&test_program, load_offset);
        emulator.core.jump_pc(load_offset);
        emulator.run();
        tx_emulator.send(()).unwrap();
    });*/

    select! {
//        _ = rx_emulator.recv() => println!("emulator"),
        v = rx_ui.recv() => println!("ui: {}", v.unwrap())
        }

    // thread::sleep(std::time::Duration::new(5, 0));

}
