use chip8::{Vram, Keys, Audio, Chip8 };
use std::sync::{Arc, RwLock};

pub struct Emulator {
    vram: Arc<RwLock<Vram>>,
    keys: Arc<RwLock<Keys>>,
    audio: Arc<RwLock<Audio>>,
    cpu: Chip8,
}

impl Emulator {
    pub fn new(vram: Arc<RwLock<Vram>>, keys: Arc<RwLock<Keys>>, audio: Arc<RwLock<Audio>>) -> Emulator {
        let mut cpu = Chip8::new();

        Emulator {
            vram: vram,
            keys: keys,
            audio: audio,
            cpu: cpu,
        }
    }

    pub fn run(&mut self) {
        print!("" );
    }
}
