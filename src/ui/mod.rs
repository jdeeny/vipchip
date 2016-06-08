pub mod interface;

use std::thread;

use chip8::{SharedState};

use self::interface::{Interface, InterfaceSdl2};
use std::sync::{Arc, RwLock};
use std::thread::sleep;
use std::time::{Duration,SystemTime};

pub struct Ui {
    state: SharedState,
    interface: Box<Interface>,
}

impl Ui {
    pub fn new(state: SharedState) -> Ui {
        Ui {
            state: state,
            interface: Box::new(InterfaceSdl2::new())
        }
    }

    pub fn run(&mut self) {
        let frame_period = Duration::new(0, 1000000000 / 60);
        let mut last_frame = SystemTime::now();
        'running: loop {
            {
                match self.interface.handle_input(&mut self.state) {
                    true => break 'running,
                    _ => ()
                }
            }
            match last_frame.elapsed() {
                Ok(elapsed) => {
                    if elapsed > frame_period {
                        let vram = self.state.vram.read().unwrap();
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
