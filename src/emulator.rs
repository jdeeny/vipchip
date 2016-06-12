use std::thread;

use chip8::{ Chip8, SharedState, Instruction };
use config::Config;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};




type InstructionOption = Option<Instruction>;
pub struct Emulator {
    config: Config,
    pub core: Chip8,
    instruction_cache: Vec<InstructionOption>,
    start_time: SystemTime,
    num_processed: u64,

}

impl Emulator {
    pub fn new(config: Config, state: SharedState) -> Emulator {
        let core = Chip8::new(config, state);

        let mut cache_vec: Vec<InstructionOption> = Vec::new();
        for _ in 0..config.ram_size {
            cache_vec.push(None)
        }

        Emulator {
            config: config,
            core: core,
            instruction_cache: cache_vec,
            start_time: SystemTime::now(),
            num_processed: 0,
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
            self.core.advance_pc();



            let instr = self.core.decode_instruction(codeword);
            self.core.execute(&instr);

            self.num_processed += 1;
                if self.num_processed % 10000000 == 0 {
                    //panic!{"quit for valgrind"}
                    let millions = self.num_processed / 1000000;
                    let elapsed = self.start_time.elapsed().unwrap();
                    let secs = elapsed.as_secs() as f64;
                    let nanos = elapsed.subsec_nanos()as f64;
                    let total = secs  + nanos / 1000000000.0;
                    if total > 0.0f64 {
                        let per_sec = (self.num_processed as f64) / total;

                        println!("{}M in {:.1}: {:.1} /sec = {:.1} /frame", millions, total, per_sec, per_sec / 60.0);
                    }
                }
//            thread::sleep_ms(250);
            //thread::park_timeout(Duration::new(0,50));
        }
    }

}
