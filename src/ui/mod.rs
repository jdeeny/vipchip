pub mod interface;

use chip8::{Vram, Keys, Audio};

use self::interface::{Interface, InterfaceSdl2};
use std::sync::{Arc, RwLock};
use std::thread::sleep;
use std::time::Duration;

pub struct Ui {
    vram: Arc<RwLock<Vram>>,
    keys: Arc<RwLock<Keys>>,
    audio: Arc<RwLock<Audio>>,
    interface: Box<Interface>,
}

impl Ui {
    pub fn new(vram: Arc<RwLock<Vram>>, keys: Arc<RwLock<Keys>>, audio: Arc<RwLock<Audio>>) -> Ui {
        Ui {
            vram: vram,
            keys: keys,
            audio: audio,
            interface: Box::new(InterfaceSdl2::new())
        }
    }

    pub fn run(&mut self) {
        loop {
            let vram = self.vram.read().unwrap();
            let pixels = vram.pixels;

            self.interface.draw_screen(&pixels);

        }
    }
}
