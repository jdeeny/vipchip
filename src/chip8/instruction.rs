use chip8::{ Chip8, Operand };
use chip8::operand::Operand::{ Register, ByteLiteral, I, Indirect, No };

use chip8::opcode::*;

pub type Word = u16;

pub struct Instruction {
    op: Box<Opcode>,
    src: Operand,
    dest: Operand,
}

impl Instruction {
    pub fn new(opcode: Box<Opcode>, src: Operand, dest: Operand) -> Instruction {
        Instruction { op: opcode, src: src, dest: dest }
    }
    pub fn decode(word: Word) -> Instruction {
        decode_instruction_word(word)
    }

    pub fn execute(&mut self, core: &mut Chip8) {
        self.op.execute(self.src, self.dest, core);
    }
    pub fn to_word(&mut self) -> Word {
        self.op.to_word(self.src, self.dest)
    }
    pub fn to_string(&mut self) -> String {
        self.op.to_string(self.src, self.dest)
    }
    pub fn from_word(word: Word) -> Instruction {
        decode_instruction_word(word)
    }
}




fn decode_instruction_word(word: Word) -> Instruction {

    let instruction: Instruction = match split_nibbles(word) {
        //(0x0, 0x0, 0xE, 0x0) => Box::new(OpCls::new()),
        //(0x0, 0x0, 0xE, 0xE) => Box::new(OpRet::new()),
        //(0x1, hi, mid, lo) => Box::new(OpJmp::new(join_nibbles(&[hi, mid, lo]) as u16)),
        //(0x2, _, _, _) => Box::new(OpCall::new(instr_word)),
        //(0x3, reg, hi, lo) => Box::new(OpSkipEqByte::new(reg as usize, join_nibbles(&[hi, lo]) as u8)),
        //(0x4, _, _, _) => Box::new(OpSkipNeqByte::new(instr_word)),
        //(0x5, _, _, 0x0) => Box::new(OpSkipEqReg::new(instr_word)),
        //(0x6, dest, hi, lo) => Box::new(OpLdByte::new(dest, join_nibbles(&[hi, lo]) as u8)),
        (0x7, dest, hi, lo) => Instruction::new(OpAdd::new(word), Register(dest), ByteLiteral(join_nibbles(&[hi, lo]))),
        //(0x8, dest, src, 0x0) => Box::new(OpLdReg::new(dest, src)),
        (0x8, dest, src, 0x1) => Instruction::new(OpOr::new(word), Register(dest), Register(src)),
        (0x8, dest, src, 0x2) => Instruction::new(OpAnd::new(word), Register(dest), Register(src)),
        (0x8, dest, src, 0x3) => Instruction::new(OpXor::new(word), Register(dest), Register(src)),
        (0x8, dest, src, 0x4) => Instruction::new(OpAdd::new(word), Register(dest), Register(src)),
        //(0x8, dest, src, 0x5) => Box::new(OpSubReg::new(dest, src)),
        //(0x8, dest, src, 0x6) => Box::new(OpShrReg::new(dest, src)),
        //(0xA, hi, mid, lo) => Box::new(OpLdI::new(join_nibbles(&[hi, mid, lo]))),
        //(0xB, hi, mid, lo) => Box::new(OpJmp0::new(join_nibbles(&[hi, mid, lo]) as u16)),
        (0xC, dest, hi, lo) => Instruction::new(OpRand::new(word), Register(dest), ByteLiteral(join_nibbles(&[hi, lo]))),
        //(0xD, x, y, n) => Box::new(OpSprite::new(x as usize, y as usize, n as usize)),
        (0xE, reg, 0x9, 0xE) => Instruction::new(OpSkipKey::new(word), Register(reg), No),
        (0xE, reg, 0xA, 0x1) => Instruction::new(OpSkipNkey::new(word), Register(reg), No),
        //(0xF, reg, 0x0, 0x7) => Box::new(OpLdRegDt::new(reg as usize)),
        //(0xF, reg, 0x1, 0x5) => Box::new(OpLdDtReg::new(reg as usize)),
        //(0xF, reg, 0x1, 0x8) => Box::new(OpLdStReg::new(reg as usize)),
        (0xF, reg, 0x1, 0xE) => Instruction::new(OpAdd::new(word), I, Register(reg)),
        //(0xF, reg, 0x2, 0x9) => Box::new(OpLdIFont::new(reg as usize)),
        (0xF, reg, 0x3, 0x3) => Instruction::new(OpBcd::new(word), Indirect, Register(reg)),
        //(0xF, reg, 0x5, 0x5) => Box::new(OpSaveReg::new(reg as usize)),
        //(0xF, reg, 0x6, 0x5) => Box::new(OpRestoreReg::new(reg as usize)),

        _ => Instruction::new(OpInvalid::new(word), Operand::No, Operand::No),
    };

    instruction
}

fn split_nibbles(word: Word) -> (usize, usize, usize, usize) {
    let w = word as usize;

    (
        ((w >> 12) & 0x0F),
        ((w >> 8) & 0x0F),
        ((w >> 4) & 0x0F),
        ((w >> 0) & 0x0F)
    )
}

fn join_nibbles(nibbles: &[usize]) -> usize {
    let mut result = 0;

    for n in nibbles {
        result = (result << 4) | (*n as usize);
    }
    result
}
