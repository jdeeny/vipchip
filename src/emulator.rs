use std::thread;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};

use chip8::{Simulator, SharedState, Instruction, Config};


type InstructionOption = Option<Instruction>;
pub struct Emulator {
    config: Config,
    pub core: Simulator,
    start_time: SystemTime,
    num_processed: u64,
}

impl Emulator {
    pub fn new(config: Config, state: SharedState) -> Emulator {
        let core = Simulator::new(config, state);

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
        let mut last_instruction_time;

        'running: loop {
            match last_timer_tick.elapsed() {
                Ok(elapsed) => {
                    if elapsed > tick {
                        last_timer_tick += tick;
                        self.core.decrement_timers();
                    }
                }
                Err(_) => { panic!("no time elapsed")},
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
            if self.num_processed % 10000000 == 0 {
                let millions = self.num_processed as f64 / 1000000.0;
                let elapsed = self.start_time.elapsed().unwrap();
                let secs = elapsed.as_secs() as f64;
                let nanos = elapsed.subsec_nanos() as f64;
                let total = secs + nanos / 1000000000.0;
                if total > 0.0f64 {
                    let per_sec = (self.num_processed as f64) / total;

                    println!("{:.1}M in {:.1}s: {:.1} /sec = {:.1} /frame",
                             millions,
                             total,
                             per_sec,
                             per_sec / 60.0);
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
                }
                None => {}
            }

            //thread::park_timeout(Duration::new(0,50));
        }
    }
}
