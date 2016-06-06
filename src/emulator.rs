use chip8::{Vram, Keys, Audio, Chip8 };
use std::sync::{Arc, RwLock};

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
        for _ in 0..20000000 {

            let mut instruction = self.cpu.decode_instruction(self.cpu.pc);

            println!("{:X} {:?}", instruction.as_u16(), instruction.as_string());

            self.cpu.advance_pc();
            self.cpu.execute(&mut instruction);
        }
    }

}
