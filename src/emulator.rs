use std::thread;

use chip8::{ Chip8, SharedState, Instruction };
use config::Config;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};

pub struct Emulator {
    config: Config,
    pub core: Chip8,
}

impl Emulator {
    pub fn new(config: Config, state: SharedState) -> Emulator {
        let core = Chip8::new(state);

        Emulator {
            config: config,
            core: core,
        }
    }

    pub fn run(&mut self) {
        let tick = Duration::new(0, 1000000000 / 60);

        let mut last_timer_tick = SystemTime::now();

        'running: loop {
            match last_timer_tick.elapsed() {
                Ok(elapsed) => {
                    if elapsed > tick {
                        last_timer_tick += tick;
                        self.core.decrement_timers();
                    }
                }
                Err(_) => ()
            }

            let codeword = self.core.current_codeword();
            println!("0x{:03X}: {:04X}", self.core.pc(), codeword );
            self.core.advance_pc();
            self.core.execute(codeword);

            thread::park_timeout(Duration::new(0,50));
        }
    }

}
