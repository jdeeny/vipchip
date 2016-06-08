use std::thread;

use chip8::{ Chip8, SharedState, Instruction };
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};

pub struct Emulator {
    pub core: Chip8,
}

impl Emulator {
    pub fn new(state: SharedState) -> Emulator {
        let core = Chip8::new(state);

        Emulator {
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

            let mut instruction: Instruction = self.core.decode_instruction(self.core.pc());
            {
                //println!("{:X}: {:X} {}", self.core.pc(), instruction.as_word(), instruction.as_string());
                self.core.dump_reg();
            }

            self.core.advance_pc();
            instruction.execute(&mut self.core);

            thread::park_timeout(Duration::new(0,50));
        }
    }

}
