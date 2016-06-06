#![feature(mpsc_select)]

extern crate sdl2;

use std::thread;
use std::sync::{Arc, RwLock};
use std::sync::mpsc;

pub mod chip8;
pub mod ui;
pub mod emulator;


use ui::Ui;
use chip8::{ Vram, Keys, Audio };
use emulator::Emulator;



fn main() {

    let (tx_ui, rx_ui) = mpsc::channel();
    let (tx_emulator, rx_emulator) = mpsc::channel();


    let vram = Arc::new(RwLock::new(Vram::new()));
    let keys = Arc::new(RwLock::new(Keys::new()));
    let audio = Arc::new(RwLock::new(Audio::new()));

    let ui_vram = vram.clone();
    let ui_keys = keys.clone();
    let ui_audio = audio.clone();
    let emulator_vram = vram.clone();
    let emulator_keys = keys.clone();
    let emulator_audio = audio.clone();

    let ui_thread = thread::spawn(move || {
        let mut ui = Ui::new(ui_vram, ui_keys, ui_audio);
        ui.run();
        tx_ui.send(0).unwrap();
    });

    let test_program: Vec<u8> = vec![   //0x71, 0x24,
                                        0xC3, 0x1F, 0xC4, 0x0F, 0xA2, 0x30, 0xD3, 0x48,
                                        0xD3, 0x48, 0x60, 0x05, 0xE0, 0xA1, 0x74, 0xFF,
                                        0x60, 0x08, 0xE0, 0xA1, 0x74, 0x01, 0x60, 0x07,
                                        0xE0, 0xA1, 0x73, 0xFF, 0x60, 0x09, 0xE0, 0xA1,
                                        0x73, 0x01, 0xD3, 0x48, 0xFF, 0x07, 0x3F, 0x00,
                                        0x12, 0x24, 0x6F, 0x03, 0xFF, 0x15, 0x12, 0x08,
                                        0x70, 0x70, 0x20, 0x70, 0xA8, 0x20, 0x50, 0x50,
                                    ];



    let emulator_thread = thread::spawn(move || {
        let mut emulator = Emulator::new(emulator_vram, emulator_keys, emulator_audio);
        emulator.cpu.load_hex(&test_program, 0x200);
        emulator.cpu.jump_pc(0x200);
        emulator.run();
        tx_emulator.send(()).unwrap();
    });

    select! {
        _ = rx_emulator.recv() => println!("emulator"),
        v = rx_ui.recv() => println!("ui: {}", v.unwrap())
        }

    //thread::sleep(std::time::Duration::new(5, 0));

}
