use chip8::{ Chip8, Operand, OperandKind };
use chip8::operation::*;


pub type Word = u16;

/// Type to hold instruction word pattern
pub type Pattern = [Coding; 4];

pub enum Coding {
    C(u8),
    D,
    S,
    A,
}


struct InstructionTable {
    table: Vec<InstructionDef>,
}
impl InstructionTable {
    pub fn new() -> InstructionTable {
        use chip8::OperandKind::*;
        use self::Coding::*;

        let mut itable: Vec<InstructionDef> = vec!(
            InstructionDef::new(OpCls,      Unused,     Unused,     Unused,     [C(0x0), C(0x0), C(0xE), C(0x0)], "Cls"),
            InstructionDef::new(OpRet,      Unused,     Unused,     Unused,     [C(0x0), C(0x0), C(0xE), C(0x0)], "Ret"),

            InstructionDef::new(OpJump,     Literal12,  Unused,     Unused,     [C(0x1), D,      D,      D     ], "Jump {d}"),
            InstructionDef::new(OpCall,     Literal12,  Unused,     Unused,     [C(0x2), D,      D,      D     ], "Call {d}"),
            InstructionDef::new(OpSkipEq,   Register,   Literal8,   Unused,     [C(0x3), D,      S,      S     ], "SkipEq {d}, {s}"),
            InstructionDef::new(OpSkipNeq,  Register,   Literal8,   Unused,     [C(0x4), D,      S,      S     ], "SkipNeq {d}, {s}"),
            InstructionDef::new(OpSkipEq,   Register,   Register,   Unused,     [C(0x5), D,      S,      C(0x0)], "SkipEq {d}, {s}"),

            InstructionDef::new(OpLoad,     Register,   Literal8,   Unused,     [C(0x6), D,      S,      S     ], "Load {d}, {s}"),

            InstructionDef::new(OpAdd,      Register,   Literal8,   Unused,     [C(0x7), D,      S,      S     ], "Add {d}, {s}"),

            InstructionDef::new(OpLoad,     Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x0)], "Load {d}, {s}"),

            InstructionDef::new(OpOr,       Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x1)], "Or {d}, {s}"),
            InstructionDef::new(OpAnd,      Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x2)], "And {d}, {s}"),
            InstructionDef::new(OpXor,      Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x3)], "Xor {d}, {s}"),
            InstructionDef::new(OpAdd,      Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x4)], "Add {d}, {s}"),
            InstructionDef::new(OpSub,      Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x5)], "Sub {d}, {s}"),
            InstructionDef::new(OpShR,      Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x6)], "ShR {d}, {s}"),
            InstructionDef::new(OpSubN,     Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x7)], "SubN {d}, {s}"),
            InstructionDef::new(OpShL,      Register,   Register,   Unused,     [C(0x8), D,      S,      C(0xE)], "ShL {d}, {s}"),

            InstructionDef::new(OpSkipNeq,  Register,   Register,   Unused,     [C(0x9), D,      S,      C(0x0)], "SkipNeq {d}, {s}"),
            InstructionDef::new(OpLoad,     I,          Literal12,  Unused,     [C(0xA), S,      S,      S     ], "Load {d}, {s}"),

            InstructionDef::new(OpJumpV0,   Literal12,  Unused,     Unused,     [C(0xB), D,      D,      D     ], "JumpV0 {d}"),
            InstructionDef::new(OpRand,     Register,   Literal8,   Unused,     [C(0xC), D,      S,      S     ], "Rand {d}, {s}"),

            InstructionDef::new(OpSprite,   Register,   Register,   Literal4,   [C(0xD), D,      S,      A     ], "Sprite {d}, {s}, {a}"),

            InstructionDef::new(OpSkipKey,  Register,   Unused,     Unused,     [C(0xE), D,      C(0x9), C(0xE)], "SkipKey {d}"),
            InstructionDef::new(OpSkipNKey, Register,   Unused,     Unused,     [C(0xE), D,      C(0xA), C(0x1)], "SkipNKey {d}"),

            InstructionDef::new(OpLoad,     Register,   DelayTimer, Unused,     [C(0xF), D,      C(0x0), C(0x7)], "Load {d}, {s}"),

            InstructionDef::new(OpWaitKey,  Register,   Unused,     Unused,     [C(0xF), D,      C(0x0), C(0xA)], "WaitKey {d}"),
            InstructionDef::new(OpLoad,     DelayTimer, Register,   Unused,     [C(0xF), S,      C(0x1), C(0x5)], "Load {d}, {s}"),
            InstructionDef::new(OpLoad,     SoundTimer, Register,   Unused,     [C(0xF), S,      C(0x1), C(0x8)], "Load {d}, {s}"),

            InstructionDef::new(OpAdd,      I,          Register,   Unused,     [C(0xF), D,      C(0x1), C(0xE)], "Add {d}, {s}"),

            InstructionDef::new(OpFont,     I,          Register,   Unused,     [C(0xF), S,      C(0x2), C(0x9)], "Font {d}, {s}"),

            InstructionDef::new(OpBCD,      IndirectI,  Register,   Unused,     [C(0xF), S,      C(0x3), C(0x3)], "BCD {d}, {s}"),

            InstructionDef::new(OpStash,    IndirectI,  Register,   Unused,     [C(0xF), S,      C(0x5), C(0x5)], "Stash {s}"),
            InstructionDef::new(OpFetch,    Register,   IndirectI,  Unused,     [C(0xF), D,      C(0x6), C(0x5)], "Fetch {d}"),

        );

        InstructionTable { table: itable }
    }
}


///Defines a specific kind of instructions
///
///It has a unique signature: the kind of opcode and the kinds of the locations dest, src, aux
///
///pattern defines how the instruction is decoded:
///     C(n) is a constant nibble. The codeword must match for this instruction to be valid.
///     D, S, and A are value markers that indicate which nibbles represent which operand's value.
///     If more than one nibble is used for the same operand, the leftmost nibble is most significant
///     and the rightmost is least significant.
///     D indicates dest, S src, and A aux.
pub struct InstructionDef {
  opcode: Operation,
  dest_kind: OperandKind,
  src_kind: OperandKind,
  aux_kind: OperandKind,
  pattern: Pattern,
  mnemonic: String,
}

impl InstructionDef {
    pub fn new(opcode: Operation, dest: OperandKind, src: OperandKind, aux:OperandKind, pattern: Pattern, mnemonic: &str) -> InstructionDef {
        InstructionDef {
            opcode: opcode,
            dest_kind: dest,
            src_kind: src,
            aux_kind: aux,
            pattern: pattern,
            mnemonic: mnemonic.to_string(),
        }
    }
}

pub struct Instruction {
  def: InstructionDef,
  codeword: Word,
  dest: Operand,
  src: Operand,
  aux: Operand,
}

impl Instruction {
    pub fn new(def: InstructionDef, codeword: Word) -> Instruction {
        let mut dest_data: usize = 0;
        let mut src_data: usize = 0;
        let mut aux_data: usize = 0;

        let mut word = codeword as usize;
        for coding in def.pattern.iter() {
            let nibble = (word & 0xF000) >> 12;
            word <<= 4;
            match *coding {
                self::Coding::C(_) => {},
                self::Coding::D => dest_data = (dest_data << 4) | nibble,
                self::Coding::S => src_data = (src_data << 4) | nibble,
                self::Coding::A => aux_data = (aux_data << 4) | nibble,
            }
        }

        let mut dest: Operand;
        let mut src: Operand;
        let mut aux: Operand;
        {
            dest = Self::specify_operand(&def.dest_kind, dest_data);
            src = Self::specify_operand(&def.src_kind, src_data);
            aux = Self::specify_operand(&def.aux_kind, aux_data);
        }

        Instruction {
            def: def,
            codeword: codeword,
            dest: dest,
            src: src,
            aux: aux,
        }
    }
    fn specify_operand(kind: &OperandKind, data: usize) -> Operand {
        match *kind {
            OperandKind::Register => Operand::Register(data),
            OperandKind::I => Operand::I,
            OperandKind::Address12 => Operand::Address12(data),
            OperandKind::IndirectI => Operand::IndirectI,
            OperandKind::Literal12 => Operand::Literal12(data),
            OperandKind::Literal8 => Operand::Literal8(data),
            OperandKind::Literal4 => Operand::Literal4(data),
            OperandKind::DelayTimer => Operand::DelayTimer,
            OperandKind::SoundTimer => Operand::SoundTimer,
            OperandKind::Unused => Operand::Nowhere,
        }
    }

    pub fn dest(&self) -> Operand {
        self.dest
    }

    pub fn src(&self) -> Operand {
        self.dest
    }

    pub fn aux(&self) -> Operand {
        self.dest
    }

}

/*























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
*/
