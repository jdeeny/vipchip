use std::thread;

use chip8::{ Chip8, Operand, Instruction, Word };
use chip8::operand::Operand::{ Register, Address, };

pub use self::math::*;

//mod comparison;
//mod display;
//mod flowcontrol;
//mod keyboard;
mod math;
//mod memory;

pub trait Opcode {
    fn new(word: Word) -> Box<Self> where Self: Sized;
    fn execute(&mut self, src: Operand, dest: Operand, &mut Chip8);
    fn to_word(&self, src: Operand, dest: Operand) -> Word;
    fn to_string(&self, src: Operand, dest: Operand) -> String;
}


/// An unrecognized instruction
pub struct OpInvalid {
    instruction_word: Word,  //store the entire u16 since it can't be recreated
}
impl OpInvalid { }
impl Opcode for OpInvalid {
    fn new(word: Word) -> Box<Self> where Self: Sized { Box::new(OpInvalid { instruction_word: word }) }
    fn execute(&mut self, src: Operand, dest: Operand, core: &mut Chip8) { thread::sleep_ms(1000);}
    fn to_word(&self, src: Operand, dest: Operand) -> Word {
        self.instruction_word
    }
    fn to_string(&self, src: Operand, dest: Operand) -> String { format!("Unknown: 0x{:X}", self.instruction_word) }

}





#[test]
fn test_join_nibbles() {
    assert!(join_nibbles(&[1, 6]) == 0x0016);
    assert!(join_nibbles(&[2, 3, 4]) == 0x0234);
    assert!(join_nibbles(&[0xF, 0x4, 0x0, 0x7]) == 0xF407);
}
