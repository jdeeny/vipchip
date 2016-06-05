extern crate rand;
use self::rand::{thread_rng, Rng};

pub mod opcode;
pub use self::opcode::{Opcode, decode_instruction};



pub struct Vram {

}
impl Vram {
    pub fn new() -> Vram {
        Vram {}
    }
}

pub struct Keys {

}
impl Keys {
    pub fn new() -> Keys {
        Keys {}
    }
}
pub struct Audio {

}
impl Audio {
    pub fn new() -> Audio {
        Audio {}
    }
}

pub type Address = usize;

#[allow(dead_code)]
pub struct Chip8 {
    gp_reg: [u8; 16],
    i: Address,
    pub pc: Address,
    sp: Address,
    delay_timer: u8,
    sound_timer: u8,
    ram: [u8; 4 * 1024],
    stack: [Address; 256],
    pub pixels: [[u8; 64]; 32],
    font: [u8; 5 * 16],
    key_state: [bool; 16],
    rng: Box<Rng>,
}

const FONT_4X5: [u8; 5 * 16] =
    [0xF0, 0x90, 0x90, 0x90, 0xF0 /* 0 */, 0x20, 0x60, 0x20, 0x20, 0x70 /* 1 */, 0xF0, 0x10,
     0xF0, 0x80, 0xF0 /* 2 */, 0xF0, 0x10, 0xF0, 0x10, 0xF0 /* 3 */, 0x90, 0x90, 0xF0, 0x10,
     0x10 /* 4 */, 0xF0, 0x80, 0xF0, 0x10, 0xF0 /* 5 */, 0xF0, 0x80, 0xF0, 0x90,
     0xF0 /* 6 */, 0xF0, 0x10, 0x20, 0x40, 0x40 /* 7 */, 0xF0, 0x90, 0xF0, 0x90,
     0xF0 /* 8 */, 0xF0, 0x90, 0xF0, 0x10, 0xF0 /* 9 */, 0xF0, 0x90, 0xF0, 0x90,
     0x90 /* A */, 0xE0, 0x90, 0xE0, 0x90, 0xE0 /* B */, 0xF0, 0x80, 0x80, 0x80,
     0xF0 /* C */, 0xE0, 0x90, 0x90, 0x90, 0xE0 /* D */, 0xF0, 0x80, 0xF0, 0x80,
     0xF0 /* E */, 0xF0, 0x80, 0xF0, 0x80, 0x80 /* F */];


impl Chip8 {
    pub fn new() -> Chip8 {

        Chip8 {
            gp_reg: [0; 16],
            i: 0,
            pc: 0,
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            ram: [0; 4 * 1024],
            stack: [0; 256],
            pixels: [[0; 64]; 32],
            font: FONT_4X5,
            key_state: [false; 16],
            rng: Box::new(thread_rng()),
        }
    }

    pub fn load_hex(&mut self, bytes: &Vec<u8>, loc: Address) {
        let mut i = loc;
        for b in bytes {
            self.ram[i] = *b;
            i += 1;
        }
    }

    pub fn decode_instruction(&self, addr: Address) -> Box<Opcode> {
        let hi = self.ram[addr];
        let lo = self.ram[addr + 1];
        let instr = (hi as u16) << 8 | lo as u16;

        let opcode = decode_instruction(instr);

        opcode
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
    pub fn advance_pc(&mut self) {
        self.pc += 2;
    }

    pub fn jump_pc(&mut self, addr: Address) {
        self.pc = addr;
    }

    pub fn set_key_state(&mut self, keys: [bool; 16]) {
        self.key_state = keys;
    }

    pub fn dump_reg(&self) {
        print!("Reg: ");
        for r in self.gp_reg.iter() {
            print!("{:X} ", r);
        }
        println!("");
        println!("i:{:X} pc:{:X} sp{:X}", self.i, self.pc, self.sp);
    }
    pub fn dump_pixels(&self) {
        for row in self.pixels.iter() {
            for dot in row.iter() {
                print!("{:?}", dot);
            }
            println!("");
        }
    }
}
