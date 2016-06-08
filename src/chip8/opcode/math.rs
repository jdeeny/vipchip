use chip8::{ Chip8, Opcode, Word, operand };
use chip8::operand::{ Operand, operand_to_string };
use chip8::operand::Operand::{ Address, Register, ByteLiteral, NibbleLiteral, I };

pub struct OpAdd { }
impl Opcode for OpAdd {
    fn new(word: Word) -> Box<Self> where Self: Sized { Box::new(OpAdd { } ) }
    fn execute(&mut self, src: Operand, dest: Operand, core: &mut Chip8) {
        let mut a: u32;
        let mut b: u32;
        let (a, b) = match (dest, src) {
            (Register(_),   ByteLiteral(_)) => (core.load_operand(dest), core.load_operand(src)),
            (Register(_),   Register(_))    => (core.load_operand(dest), core.load_operand(src)),
            (I,             Register(_))    => (core.load_operand(dest), core.load_operand(src)),
            (_,             _)              => panic!("Invalid Add"),
        };

        let total = a + b;
        match (dest, src) {
            (Register(_), Register(_)) => { core.store_vf_flag( total > 0xFF ); },
            (_, _) => { }
        }
        core.store_operand(dest, total);
    }
    fn to_word(&self, dest: Operand, src: Operand) -> Word {
        match (dest, src) {
            (Register(r),   ByteLiteral(b)) => ( 0x7000 | (r << 8) | b ) as u16,
            (Register(d),   Register(s))    => ( 0x8004 | (d << 8) | s << 4) as u16,
            (I,             Register(r))    => ( 0xF01E | r << 8 ) as u16,
            (_,             _)              => { panic!("Invalid Add"); }
        }
    }
    fn to_string(&self, dest: Operand, src: Operand) -> String {
        format!("Add: {} += {}", operand_to_string(dest), operand_to_string(src))
    }
}

pub struct OpSub { }
impl Opcode for OpSub {
    fn new(word: Word) -> Box<Self> where Self: Sized { Box::new(OpSub { } ) }
    fn execute(&mut self, src: Operand, dest: Operand, core: &mut Chip8) {
        let mut a: u32;
        let mut b: u32;
        let (a, b) = match (dest, src) {
            (Register(_),   Register(_))    => (core.load_operand(dest), core.load_operand(src)),
            (_,             _)                => panic!("Invalid Sub"),
        };

        let total = (a as i32) - (b as i32);
        core.store_vf_flag( a > b );
        core.store_operand(dest, total as u32);
    }
    fn to_word(&self, dest: Operand, src: Operand) -> Word {
        match (dest, src) {
            (Register(d),   Register(s)) => ( 0x8005 | (d << 8) | s ) as u16,
            (_,             _)              => { panic!("Invalid Sub"); }
        }
    }
    fn to_string(&self, dest: Operand, src: Operand) -> String {
        format!("Sub: {} -= {}", operand_to_string(dest), operand_to_string(src))
    }
}

pub struct OpSubn { }
impl Opcode for OpSubn {
    fn new(word: Word) -> Box<Self> where Self: Sized { Box::new(OpSubn { } ) }
    fn execute(&mut self, src: Operand, dest: Operand, core: &mut Chip8) {
        let mut a: u32;
        let mut b: u32;
        let (a, b) = match (dest, src) {
            (Register(_),   Register(_))    => (core.load_operand(dest), core.load_operand(src)),
            (_,             _)                => panic!("Invalid Subn"),
        };

        let total = (b as i32) - (a as i32);
        core.store_vf_flag( b > a );
        core.store_operand(dest, total as u32);
    }
    fn to_word(&self, dest: Operand, src: Operand) -> Word {
        match (dest, src) {
            (Register(d),   Register(s)) => ( 0x8007 | (d << 8) | s ) as u16,
            (_,             _)              => { panic!("Invalid Subn"); }
        }
    }
    fn to_string(&self, dest: Operand, src: Operand) -> String {
        format!("Subn: {d} = {s} - {d}", d = operand_to_string(dest), s = operand_to_string(src))
    }
}



pub struct OpOr { }
impl Opcode for OpOr {
    fn new(word: Word) -> Box<Self> where Self: Sized { Box::new(OpOr { } ) }
    fn execute(&mut self, dest: Operand, src: Operand, core: &mut Chip8) {
        let mut a: u32;
        let mut b: u32;
        let (a, b) = match (dest, src) {
            (Register(_),   Register(_))   => (core.load_operand(dest), core.load_operand(src)),
            (_,             _)                => panic!("Invalid Or"),
        };

        let result = a | b;
        core.store_operand(dest, result);
    }
    fn to_word(&self, dest: Operand, src: Operand) -> Word {
        match (dest, src) {
            (Register(d),   Register(s))    => ( 0x8001 | (d << 8) | (s << 4)) as u16,
            (_,             _)              => { panic!("Invalid Or"); }
        }
    }
    fn to_string(&self, dest: Operand, src: Operand) -> String {
        format!("Or: {} |= {}", operand_to_string(dest), operand_to_string(src))
    }
}

pub struct OpAnd { }
impl Opcode for OpAnd {
    fn new(word: Word) -> Box<Self> where Self: Sized { Box::new(OpAnd { } ) }
    fn execute(&mut self, dest: Operand, src: Operand, core: &mut Chip8) {
        let mut a: u32;
        let mut b: u32;
        let (a, b) = match (dest, src) {
            (Register(_),   Register(_))   => (core.load_operand(dest), core.load_operand(src)),
            (_,             _)                => panic!("Invalid And"),
        };

        let result = a & b;
        core.store_operand(dest, result);
    }
    fn to_word(&self, dest: Operand, src: Operand) -> Word {
        match (dest, src) {
            (Register(d),   Register(s))    => ( 0x8002 | (d << 8) | (s << 4) ) as u16,
            (_,             _)              => { panic!("Invalid And"); }
        }
    }
    fn to_string(&self, dest: Operand, src: Operand) -> String {
        format!("And: {} |= {}", operand_to_string(dest), operand_to_string(src))
    }
}

pub struct OpXor { }
impl Opcode for OpXor {
    fn new(word: Word) -> Box<Self> where Self: Sized { Box::new(OpXor { } ) }
    fn execute(&mut self, dest: Operand, src: Operand, core: &mut Chip8) {
        let mut a: u32;
        let mut b: u32;
        let (a, b) = match (dest, src) {
            (Register(_),   Register(_))   => (core.load_operand(dest), core.load_operand(src)),
            (_,             _)                => panic!("Invalid Xor"),
        };

        let result = a ^ b;
        core.store_operand(dest, result);
    }
    fn to_word(&self, dest: Operand, src: Operand) -> Word {
        match (dest, src) {
            (Register(d),   Register(s))    => ( 0x8003 | (d << 8) | (s << 4) ) as u16,
            (_,             _)              => { panic!("Invalid Xor"); }
        }
    }
    fn to_string(&self, dest: Operand, src: Operand) -> String {
        format!("Xor: {} |= {}", operand_to_string(dest), operand_to_string(src))
    }
}

pub struct OpShr { }
impl Opcode for OpShr {
    fn new(word: Word) -> Box<Self> where Self: Sized { Box::new(OpShr { } ) }
    fn execute(&mut self, dest: Operand, src: Operand, core: &mut Chip8) {
        let mut x: u32;
        let mut y: u32;
        let (x, y) = match (dest, src) {
            (Register(_),   Register(_))   => (core.load_operand(dest), core.load_operand(src)),
            (_,             _)                => panic!("Invalid Shr"),
        };

        let result = y << 1;
        core.store_operand(dest, result);
    }
    fn to_word(&self, dest: Operand, src: Operand) -> Word {
        match (dest, src) {
            (Register(d),   Register(s))    => ( 0x8006 | (d << 8) | (s << 4) ) as u16,
            (_,             _)              => { panic!("Invalid Shr"); }
        }
    }
    fn to_string(&self, dest: Operand, src: Operand) -> String {
        format!("Shr: {} := {} << 1", operand_to_string(dest), operand_to_string(src))
    }
}

pub struct OpShl { }
impl Opcode for OpShl {
    fn new(word: Word) -> Box<Self> where Self: Sized { Box::new(OpShl { } ) }
    fn execute(&mut self, dest: Operand, src: Operand, core: &mut Chip8) {
        let mut x: u32;
        let mut y: u32;
        let (x, y) = match (dest, src) {
            (Register(_),   Register(_))   => (core.load_operand(dest), core.load_operand(src)),
            (_,             _)                => panic!("Invalid Shl"),
        };

        core.store_vf_flag( (x & 1) == 1 );
        core.store_operand(dest, y >> 1);
    }
    fn to_word(&self, dest: Operand, src: Operand) -> Word {
        match (dest, src) {
            (Register(d),   Register(s))    => ( 0x800E | (d << 8) | (s << 4) ) as u16,
            (_,             _)              => { panic!("Invalid Shl"); }
        }
    }
    fn to_string(&self, dest: Operand, src: Operand) -> String {
        format!("Shl: {} := {} << 1", operand_to_string(dest), operand_to_string(src))
    }
}


pub struct OpRand { }
impl Opcode for OpRand {
    fn new(word: Word) -> Box<Self> where Self: Sized { Box::new(OpRand { } ) }
    fn execute(&mut self, dest: Operand, src: Operand, core: &mut Chip8) {
        let mask = match (dest, src) {
            (Register(_),   ByteLiteral(_))   => core.load_operand(src),
            (_,             _)             => panic!("Invalid Shl"),
        };

        let random = core.rng.next_u32() & mask;
        core.store_operand(dest, random);
    }
    fn to_word(&self, dest: Operand, src: Operand) -> Word {
        match (dest, src) {
            (Register(d),   ByteLiteral(mask))    => ( 0xC000 | (d << 8) | mask ) as u16,
            (_,             _)              => { panic!("Invalid Rand"); }
        }
    }
    fn to_string(&self, dest: Operand, src: Operand) -> String {
        format!("Rand: {} := ?? & {}", operand_to_string(dest), operand_to_string(src))
    }
}

pub struct OpBcd { }
impl Opcode for OpBcd {
    fn new(word: Word) -> Box<Self> where Self: Sized { Box::new(OpBcd { } ) }
    fn execute(&mut self, dest: Operand, src: Operand, core: &mut Chip8) {
        let mut val = match (dest, src) {
            (I,   Register(_))   => core.load_operand(src),
            (_,             _)   => panic!("Invalid Shl"),
        };

        let hundreds = val / 100;
        val = val - (hundreds * 100);
        let tens = val / 10;
        let ones = val - (tens * 10);
        core.ram[core.i] = hundreds as u8;
        core.ram[core.i+1] = tens as u8;
        core.ram[core.i+2] = ones as u8;
    }
    fn to_word(&self, dest: Operand, src: Operand) -> Word {
        match (dest, src) {
            (Register(d),   ByteLiteral(mask))    => ( 0xF033 | (d << 8) ) as u16,
            (_,             _)              => { panic!("Invalid Bcd"); }
        }
    }
    fn to_string(&self, dest: Operand, src: Operand) -> String {
        format!("Bcd: [i][i+1][i+2] := BCD({})", operand_to_string(src))
    }
}


#[test]
fn test_op_add() {
    let w: Word = 0x7123;
    let i = Instruction::new(w);

    assert!(w == i.as_u16);
    assert!(i.dest == Register(1));
    assert!(i.src == ByteLiteral(0x23));

    let w: Word = 0xF41E;
    let i = Instruction::new(w);
    assert!(w == i.as_u16);
    assert!(i.dest == I);
    assert!(i.src == Register(4));
}
