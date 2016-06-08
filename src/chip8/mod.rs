extern crate rand;
use self::rand::{thread_rng, Rng};

pub use self::operand::{ Operand };
use chip8::operand::Operand::*;
pub use self::instruction::{ Instruction, Word };
pub use self::opcode::{ Opcode };
pub use self::font::FONT_CHIP8_4X5;
pub use self::state::SharedState;

pub mod operand;
mod instruction;
pub mod opcode;
mod font;
pub mod state;


pub struct Chip8 {
    state: SharedState,
    gp_reg: [u8; 16],
    i: usize,
    pc: usize,
    delay_timer: u8,
    sound_timer: u8,
    ram: [u8; 4 * 1024],
    stack: Vec<usize>,
    rng: Box<Rng>,
}



impl Chip8 {

    pub fn new(state: SharedState) -> Chip8 {
        let font = &FONT_CHIP8_4X5;
        let mut ram = [0; 4 *1024];

        //copy font to beginning of RAM, into the 0-0x200 area
        ram[0..font.len()].copy_from_slice(font);

        Chip8 {
            state: state,
            gp_reg: [0; 16],
            i: 0,
            pc: 0,
            delay_timer: 0,
            sound_timer: 0,
            ram: ram,
            stack: vec!(),
            rng: Box::new(thread_rng()),
        }
    }

    pub fn load_hex(&mut self, bytes: &Vec<u8>, addr: usize) {
        let mut i = addr;
        for b in bytes {
            self.ram[i] = *b;
            i += 1;
        }
    }

    pub fn decode_instruction(&self, addr: usize) -> Instruction {
        let hi = self.ram[addr];
        let lo = self.ram[addr + 1];
        let word = (hi as u16) << 8 | lo as u16;

        Instruction::from_word(word)
    }

    pub fn set_reg(&mut self, reg: usize, val: u8) {
        self.gp_reg[reg] = val;
    }
    pub fn vf_clear(&mut self) {
        self.gp_reg[0xF] = 0;
    }
    pub fn vf_set(&mut self) {
        self.gp_reg[0xF] = 1;
    }

    pub fn store_vf_flag(&mut self, flag: bool) {
        self.gp_reg[0xF] = if flag { 1 } else { 0 };
    }

    pub fn pc(&self) -> usize {
        self.pc
    }
    pub fn advance_pc(&mut self) {
        self.pc += 2;
    }

    pub fn jump_pc(&mut self, addr: usize) {
        self.pc = addr;
    }

    pub fn decrement_timers(&mut self) {
        if self.delay_timer > 0 { self.delay_timer -= 1; }
        if self.sound_timer > 0 { self.sound_timer -= 1; }
    }

    pub fn load_operand(&self, src: Operand) -> u32 {
        match src {
            Register(r)        => self.gp_reg[r] as u32,
            Address(a)         => self.ram[a] as u32,
            I                  => self.i as u32,
            Indirect           => self.ram[self.i] as u32,
            ByteLiteral(b)     => b as u32,
            NibbleLiteral(n)   => (n & 0x0F) as u32,
            SoundTimer         => self.sound_timer as u32,
            DelayTimer         => self.delay_timer as u32,
            No                 => panic!("Cannot load nothing"),
        }
    }

    pub fn store_operand(&mut self, dest: Operand, val: u32) {
        match dest {
            Register(r)         => { self.gp_reg[r] = (val & 0xFF) as u8; },
            Address(a)          => { self.ram[a] = (val & 0xFF) as u8; },
            I                   => { self.i = (val & 0xFFFF) as usize; },
            Indirect            => { self.ram[self.i] = val as u8; },
            SoundTimer          => { self.sound_timer = val as u8; },
            DelayTimer          => { self.delay_timer = val as u8; },
            ByteLiteral(_) | NibbleLiteral(_)
                                => { panic!("Cannot store a literal."); },
            No                  => { panic!("cannot store nothing"); }
        }
    }


    pub fn dump_reg(&self) {
        print!("Reg: ");
        for r in self.gp_reg.iter() {
            print!("{:X} ", r);
        }
        println!("");
        //println!("i:{:X} pc:{:X} stack{}", self.i, self.pc, self.stack.len());
    }
    pub fn dump_pixels(&self) {
        let vram = self.state.vram.read().unwrap();

        for row in vram.pixels.iter() {
            for dot in row.iter() {
                print!("{:?}", dot);
            }
            println!("");
        }
    }

    pub fn execute(&mut self, instruction: &mut Instruction ) {
        instruction.execute(self);
    }
}
