use std::thread;

use chip8::{ Chip8, SharedState, Instruction };
use config::Config;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};




type InstructionOption = Option<Instruction>;
pub struct Emulator {
    config: Config,
    pub core: Chip8,
    start_time: SystemTime,
    num_processed: u64,

}

impl Emulator {
    pub fn new(config: Config, state: SharedState) -> Emulator {
        let core = Chip8::new(config, state);

        Emulator {
            config: config,
            core: core,
            start_time: SystemTime::now(),
            num_processed: 0,
        }
    }

    pub fn run(&mut self) {
        let tick = Duration::new(0, 1000000000 / 60);
        let cycle_time = None;//Some(Duration::new(0, 100));

        let mut last_timer_tick = SystemTime::now();
        let mut last_instruction_time = SystemTime::now();

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


            last_instruction_time = SystemTime::now();
            let pc = self.core.pc();
            let codeword = self.core.current_codeword();
            self.core.advance_pc();

            let instr = self.core.decode_instruction(codeword);
            if self.config.print_instructions {
                print!("0x{:04X}: {}   ", pc, instr.to_string());
                self.core.dump_reg();
            }
            self.core.execute(&instr);

            self.num_processed += 1;
            if self.num_processed % 100000000 == 0 {
                //panic!{"quit for valgrind"}
                let millions = self.num_processed / 1000000;
                let elapsed = self.start_time.elapsed().unwrap();
                let secs = elapsed.as_secs() as f64;
                let nanos = elapsed.subsec_nanos()as f64;
                let total = secs  + nanos / 1000000000.0;
                if total > 0.0f64 {
                    let per_sec = (self.num_processed as f64) / total;

                    println!("{}M in {:.1}s: {:.1} /sec = {:.1} /frame", millions, total, per_sec, per_sec / 60.0);
                }
                self.num_processed = 0;
                self.start_time = SystemTime::now();
            }
            match cycle_time {
                Some(cycle_time) => {
                    let elapsed = last_instruction_time.elapsed().unwrap();
                    if elapsed < cycle_time {
                        thread::sleep(cycle_time - elapsed);
                    }
                },
                None => {}
            }

            //thread::park_timeout(Duration::new(0,50));
        }
    }

}
