use chip8::{ Chip8, Opcode, Word };
use chip8::operand::{ Operand, operand_to_string };
use chip8::operand::Operand::{ Register, No };


pub struct OpSkipKey { }
impl Opcode for OpSkipKey {
    #[allow(unused_variables)]
    fn new(word: Word) -> Box<Self> where Self: Sized { Box::new(OpSkipKey { } ) }
    fn execute(&mut self, src: Operand, dest: Operand, core: &mut Chip8) {
        let key_reg = match (dest, src) {
            (Register(_),   No)             => core.load_operand(dest),
            (_,             _)              => panic!("Invalid SkipKey"),
        };

        let mut key_state = false;
        {
            let n = core.gp_reg[key_reg as usize];
            let keys = core.state.keys.read().unwrap();
            if keys.is_down(n as usize) {
                key_state = true;
            }
        }
        if key_state {
            core.advance_pc();
        }

    }
    fn to_word(&self, dest: Operand, src: Operand) -> Word {
        match (dest, src) {
            (Register(reg),   No)           => 0xE09E | (reg << 8) as Word,
            (_,             _)              => { panic!("Invalid SkipKey"); },
        }
    }
    #[allow(unused_variables)]
    fn to_string(&self, dest: Operand, src: Operand) -> String {
        format!("SkipKey: key[{}]?", operand_to_string(dest)).to_string()
    }
}

pub struct OpSkipNkey { }
impl Opcode for OpSkipNkey {
    #[allow(unused_variables)]
    fn new(word: Word) -> Box<Self> where Self: Sized { Box::new(OpSkipNkey { } ) }
    fn execute(&mut self, src: Operand, dest: Operand, core: &mut Chip8) {
        let key_reg = match (dest, src) {
            (Register(_),   No)             => core.load_operand(dest),
            (_,             _)              => { panic!("Invalid SkipKey"); },
        };

        let mut key_state = false;
        {
            let keys = core.state.keys.read().unwrap();
            let n = core.gp_reg[key_reg as usize];
            if !keys.is_down(n as usize) {
                key_state = true;
            }
        }
        if !key_state {
            core.advance_pc();
        }
    }
    fn to_word(&self, dest: Operand, src: Operand) -> Word {
        match (dest, src) {
            (Register(reg),   No)           => 0xE0A1 | (reg << 8) as u16,
            (_,             _)              => panic!("Invalid SkipNkey"),
        }
    }
    #[allow(unused_variables)]
    fn to_string(&self, dest: Operand, src: Operand) -> String {
        format!("SkipNkey[{}]", operand_to_string(dest)).to_string()
    }
}
