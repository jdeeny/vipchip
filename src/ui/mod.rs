pub mod interface;

use std::thread;

use chip8::{Vram, Keys, Audio};

use self::interface::{Interface, InterfaceSdl2};
use std::sync::{Arc, RwLock};
use std::thread::sleep;
use std::time::{Duration,SystemTime};

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
        let frame_period = Duration::new(0, 1000000000 / 60);
        let mut last_frame = SystemTime::now();
        'running: loop {
            {
                match self.interface.handle_input(&mut self.keys) {
                    true => break 'running,
                    _ => ()
                }
            }
            match last_frame.elapsed() {
                Ok(elapsed) => {
                    if elapsed > frame_period {
                        let vram = self.vram.read().unwrap();
                        let pixels = vram.pixels;
                        self.interface.draw_screen(&pixels);
                        last_frame += frame_period;
                    }
                },
                _ => ()
            }
            thread::park_timeout_ms(50);
        }
    }
}
