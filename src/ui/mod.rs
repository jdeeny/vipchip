pub mod interface;

use chip8::{Vram, Keys, Audio};

use self::interface::{Interface, InterfaceSdl2};
use std::sync::{Arc, RwLock};

pub struct Ui {
    interface: Box<Interface>,
}

impl Ui {
    pub fn new(vram: Arc<RwLock<Vram>>, keys: Arc<RwLock<Keys>>, audio: Arc<RwLock<Audio>>) -> Ui {
        Ui {
            interface: Box::new(InterfaceSdl2::new())
        }
    }

    pub fn run(&mut self) {
        loop {

        }
    }
}
