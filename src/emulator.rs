use std::thread;

use chip8::{Vram, Keys, Audio, Chip8 };
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};

pub struct Emulator {
    pub cpu: Chip8,
}

impl Emulator {
    pub fn new(vram: Arc<RwLock<Vram>>, keys: Arc<RwLock<Keys>>, audio: Arc<RwLock<Audio>>) -> Emulator {
        let cpu = Chip8::new(vram, keys, audio);

        Emulator {
            cpu: cpu,
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
                        self.cpu.decrement_timers();
                    }
                  }
                  Err(_) => ()
              }

            let mut instruction = self.cpu.decode_instruction(self.cpu.pc);

            println!("{:X}: {:X} {:?}", self.cpu.pc, instruction.as_u16(), instruction.as_string());
            self.cpu.dump_reg();
            self.cpu.advance_pc();
            self.cpu.execute(&mut instruction);
            thread::park_timeout(Duration::new(0,50));
        }
    }

}
