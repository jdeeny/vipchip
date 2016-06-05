use core::{Chip8, Address};




pub trait Opcode {
    //    fn new(instr_word: u16) -> &Opcode;
    fn execute(&self, &mut Chip8);
    fn as_u16(&self) -> u16;
    fn as_string(&self) -> String;
}


struct OpInvalid {
    instr_word: u16,
}
impl OpInvalid {
    fn new(instr_word: u16) -> OpInvalid {
        OpInvalid { instr_word: instr_word }
    }
}
impl Opcode for OpInvalid {
    #[allow(unused_variables)]
    fn execute(&self, core: &mut Chip8) {}
    fn as_u16(&self) -> u16 {
        self.instr_word
    }
    fn as_string(&self) -> String {
        format!("Invalid[{:X}]", self.instr_word).to_string()
    }
}

struct OpCls { }
impl OpCls {
    fn new() -> OpCls {
        OpCls {}
    }
}
impl Opcode for OpCls {
    fn execute(&self, core: &mut Chip8) {
        core.pixels = [[0; 64]; 32];
    }

    fn as_u16(&self) -> u16 {
        0x00E0
    }
    fn as_string(&self) -> String {
        "Cls".to_string()
    }
}

struct OpJmp {
    dest: u16,
}
impl OpJmp {
    fn new(addr: u16) -> OpJmp {
        OpJmp { dest: addr & 0x0FFF }
    }
}
impl Opcode for OpJmp {
    fn execute(&self, core: &mut Chip8) {
        let adjusted_dest = self.dest as Address - 2;
        core.jump_pc(adjusted_dest);
    }
    fn as_u16(&self) -> u16 {
        0x1000 | self.dest
    }
    fn as_string(&self) -> String {
        format!("Jmp[0x{:X}]", self.dest).to_string()
    }
}

struct OpCall {
    dest: u16,
}
impl OpCall {
    fn new(instr_word: u16) -> OpCall {
        OpCall { dest: instr_word & 0xFFF }
    }
}
impl Opcode for OpCall {
    fn execute(&self, core: &mut Chip8) {
        core.sp = core.sp + 1;
        core.stack[core.sp as usize] = core.pc;
        core.pc = self.dest as Address;
    }
    fn as_u16(&self) -> u16 {
        0x2000 | self.dest
    }
    fn as_string(&self) -> String {
        format!("Call[{:X}]", self.dest).to_string()
    }
}

struct OpSkipEqByte {
    reg: usize,
    val: u8,
}
impl OpSkipEqByte {
    fn new(reg: usize, val: u8) -> OpSkipEqByte {
        OpSkipEqByte {
            reg: reg,
            val: val,
        }
    }
}
impl Opcode for OpSkipEqByte {
    fn execute(&self, core: &mut Chip8) {
        if core.gp_reg[self.reg] == self.val {
            core.advance_pc();
        }
    }
    fn as_u16(&self) -> u16 {
        0x3000 | ((self.reg as u16) << 8) | self.val as u16
    }
    fn as_string(&self) -> String {
        format!("SkipEqByte[v{:X}?={:X}]", self.reg, self.val).to_string()
    }
}

struct OpSkipEqReg {
    rega: u8,
    regb: u8,
}
impl OpSkipEqReg {
    fn new(instr_word: u16) -> OpSkipEqReg {
        OpSkipEqReg {
            rega: ((instr_word >> 8) & 0x0F) as u8,
            regb: ((instr_word >> 4) & 0x0F) as u8,
        }
    }
}
impl Opcode for OpSkipEqReg {
    fn execute(&self, core: &mut Chip8) {
        if core.gp_reg[self.rega as usize] == core.gp_reg[self.regb as usize] {
            core.pc = core.pc + 2;
        }
    }
    fn as_u16(&self) -> u16 {
        0x5000 | (((self.rega as u16) & 0x0F) << 8) | ((self.regb as u16) & 0x0F) << 4
    }
    fn as_string(&self) -> String {
        "SkipEqReg".to_string()
    }
}


struct OpSkipNotEqByte {
    reg: u8,
    val: u8,
}
impl OpSkipNotEqByte {
    fn new(instr_word: u16) -> OpSkipNotEqByte {
        OpSkipNotEqByte {
            reg: ((instr_word >> 8) & 0x0F) as u8,
            val: (instr_word & 0x00FF) as u8,
        }
    }
}
impl Opcode for OpSkipNotEqByte {
    fn execute(&self, core: &mut Chip8) {
        if core.gp_reg[self.reg as usize] != self.val {
            core.pc = core.pc + 2;
        }
    }
    fn as_u16(&self) -> u16 {
        0x4000 | ((self.reg as u16) << 8) | self.val as u16
    }
    fn as_string(&self) -> String {
        "SkipNotEqByte".to_string()
    }
}


struct OpLdByte {
    reg: u8,
    val: u8,
}
impl OpLdByte {
    fn new(reg: u8, byte: u8) -> OpLdByte {
        OpLdByte {
            reg: reg,
            val: byte,
        }
    }
}
impl Opcode for OpLdByte {
    fn execute(&self, core: &mut Chip8) {
        core.gp_reg[self.reg as usize] = self.val;
    }
    fn as_u16(&self) -> u16 {
        0x6000 | ((self.reg as u16) << 8) | self.val as u16
    }
    fn as_string(&self) -> String {
        format!("LdByte[v{:X}:={:X}]", self.reg, self.val).to_string()
    }
}

struct OpLdReg {
    dest: u8,
    src: u8,
}
impl OpLdReg {
    fn new(dest: u8, src: u8) -> OpLdReg {
        OpLdReg {
            dest: dest,
            src: src,
        }
    }
}
impl Opcode for OpLdReg {
    fn execute(&self, core: &mut Chip8) {
        core.gp_reg[self.dest as usize] = core.gp_reg[self.src as usize];
    }
    fn as_u16(&self) -> u16 {
        0x8000 | ((self.dest as u16) << 8) | (self.src as u16) << 4
    }
    fn as_string(&self) -> String {
        "LdReg".to_string()
    }
}



struct OpAddByte {
    reg: u8,
    val: u8,
}
impl OpAddByte {
    fn new(reg: u8, byte: u8) -> OpAddByte {
        OpAddByte {
            reg: reg,
            val: byte,
        }
    }
}
impl Opcode for OpAddByte {
    fn execute(&self, core: &mut Chip8) {
        let addend = core.gp_reg[self.reg as usize] as u32;
        let total = addend + (self.val as u32);
        core.gp_reg[self.reg as usize] = total as u8;
    }
    fn as_u16(&self) -> u16 {
        0x7000 | ((self.reg as u16) << 8) | self.val as u16
    }
    fn as_string(&self) -> String {
        "AddByte".to_string()
    }
}

struct OpAddReg {
    dest: u8,
    src: u8,
}
impl OpAddReg {
    fn new(dest: u8, src: u8) -> OpAddReg {
        OpAddReg {
            dest: dest,
            src: src,
        }
    }
}
impl Opcode for OpAddReg {
    fn execute(&self, core: &mut Chip8) {

        let initial: u32 = core.gp_reg[self.dest as usize] as u32;
        let addend: u32 = core.gp_reg[self.src as usize] as u32;

        let total = initial + addend;
        if total > 0xFF { core.vf_set(); }

        core.gp_reg[self.dest as usize] = total as u8;
    }

    fn as_u16(&self) -> u16 {
        0x8004 | ((self.dest as u16) << 8) | (self.src as u16) << 4
    }
    fn as_string(&self) -> String {
        "AddReg".to_string()
    }
}



struct OpOrReg {
    dest: u8,
    src: u8,
}
impl OpOrReg {
    fn new(dest: u8, src: u8) -> OpOrReg {
        OpOrReg {
            dest: dest,
            src: src,
        }
    }
}
impl Opcode for OpOrReg {
    fn execute(&self, core: &mut Chip8) {
        core.gp_reg[self.dest as usize] = core.gp_reg[self.dest as usize] |
                                          core.gp_reg[self.src as usize];
    }
    fn as_u16(&self) -> u16 {
        0x8001 | ((self.dest as u16) << 8) | (self.src as u16) << 4
    }
    fn as_string(&self) -> String {
        "OrReg".to_string()
    }
}


struct OpAndReg {
    dest: u8,
    src: u8,
}
impl OpAndReg {
    fn new(dest: u8, src: u8) -> OpAndReg {
        OpAndReg {
            dest: dest,
            src: src,
        }
    }
}
impl Opcode for OpAndReg {
    fn execute(&self, core: &mut Chip8) {
        core.gp_reg[self.dest as usize] = core.gp_reg[self.dest as usize] &
                                          core.gp_reg[self.src as usize];
    }
    fn as_u16(&self) -> u16 {
        0x8002 | ((self.dest as u16) << 8) | (self.src as u16) << 4
    }
    fn as_string(&self) -> String {
        "AndReg".to_string()
    }
}


struct OpXorReg {
    dest: usize,
    src: usize,
}
impl OpXorReg {
    fn new(dest: u8, src: u8) -> OpXorReg {
        OpXorReg {
            dest: dest as usize,
            src: src as usize,
        }
    }
}
impl Opcode for OpXorReg {
    fn execute(&self, core: &mut Chip8) {
        let result = core.gp_reg[self.dest] | core.gp_reg[self.src];
        core.set_reg(self.dest, result as u8);
    }
    fn as_u16(&self) -> u16 {
        0x8003 | ((self.dest as u16) << 8) ^ (self.src as u16) << 4
    }
    fn as_string(&self) -> String {
        "XorReg".to_string()
    }
}

// Todo: fix subtraction flag handling
struct OpSubReg {
    dest: u8,
    src: u8,
}
impl OpSubReg {
    fn new(dest: u8, src: u8) -> OpSubReg {
        OpSubReg {
            dest: dest,
            src: src,
        }
    }
}
impl Opcode for OpSubReg {
    fn execute(&self, core: &mut Chip8) {
        core.gp_reg[self.dest as usize] = core.gp_reg[self.dest as usize] -
                                          core.gp_reg[self.src as usize];
    }
    fn as_u16(&self) -> u16 {
        0x8005 | ((self.dest as u16) << 8) | (self.src as u16) << 4
    }
    fn as_string(&self) -> String {
        "SubReg".to_string()
    }
}

struct OpShrReg {
    dest: usize,
    src: usize,
}
impl OpShrReg {
    fn new(dest: u8, src: u8) -> OpShrReg {
        OpShrReg {
            dest: dest as usize,
            src: src as usize,
        }
    }
}
impl Opcode for OpShrReg {
    fn execute(&self, core: &mut Chip8) {

        let initial: u32 = core.gp_reg[self.dest] as u32;
        let shift: u32 = core.gp_reg[self.src] as u32;

        if initial & 1 == 1 { core.vf_set(); } else { core.vf_clear(); }

        let result = initial >> shift;

        core.set_reg(self.dest, result as u8);
    }

    fn as_u16(&self) -> u16 {
        0x8006 | ((self.dest as u16) << 8) | (self.src as u16) << 4
    }
    fn as_string(&self) -> String {
        "ShrReg".to_string()
    }
}


struct OpRand {
    reg: u8,
    mask: u8,
}
impl OpRand {
    fn new(reg: u8, mask: u8) -> OpRand {
        OpRand {
            reg: reg,
            mask: mask,
        }
    }
}
impl Opcode for OpRand {
    fn execute(&self, core: &mut Chip8) {
        core.gp_reg[self.reg as usize] = (core.rng.next_u64() as u8) & self.mask;
    }
    fn as_u16(&self) -> u16 {
        0xC000 | ((self.reg as u16) << 8) | self.mask as u16
    }
    fn as_string(&self) -> String {
        format!("Rand[v{:X}%{:X}]", self.reg, self.mask).to_string()
    }
}

struct OpLdI {
    addr: u16,
}
impl OpLdI {
    fn new(addr: u32) -> OpLdI {
        OpLdI {
            addr: addr as u16,
        }
    }
}
impl Opcode for OpLdI {
    fn execute(&self, core: &mut Chip8) {
        core.i = (self.addr & 0x0FFF) as Address;
    }
    fn as_u16(&self) -> u16 {
        0xA000 | self.addr & 0x0FFF
    }
    fn as_string(&self) -> String {
        format!("LdI[{:X}]", self.addr).to_string()
    }
}

struct OpSprite {
    x: usize,
    y: usize,
    n: usize
}
impl OpSprite {
    fn new(x: usize, y:usize, n:usize) -> OpSprite {
        OpSprite {
            x: x,
            y: y,
            n: n
        }
    }
}
impl Opcode for OpSprite {
    #[allow(dead_code)]
    fn execute(&self, core: &mut Chip8) {
        let x = self.x;
        let mut y = self.y;
        let mut i = core.i;

        for _ in 0..self.n {
            let byte = core.ram[i];
            for bit in 0..8 {
                let pixel = (byte >> bit) & 1;
                core.pixels[x+bit][y] ^= pixel;
            }
            i += 1;
            y += 1;
        }

    }
    fn as_u16(&self) -> u16 {
        0xD000 | ((self.x << 8) | (self.y << 4) | (self.n)) as u16
    }
    fn as_string(&self) -> String {
        format!("Sprite[{:?},{:?}*{:?}]", self.x, self.y, self.n).to_string()
    }
}

struct OpSkipKey {
    reg: usize
}
impl OpSkipKey {
    fn new(reg: usize) -> OpSkipKey {
        OpSkipKey {
            reg: reg
        }
    }
}
impl Opcode for OpSkipKey {
    #[allow(dead_code)]
    fn execute(&self, core: &mut Chip8) {
    }
    fn as_u16(&self) -> u16 {
        0xE09E | (self.reg << 8) as u16
    }
    fn as_string(&self) -> String {
        format!("SkipKey[v{:X}]", self.reg).to_string()
    }
}

struct OpSkipNkey {
    reg: usize
}
impl OpSkipNkey {
    fn new(reg: usize) -> OpSkipNkey {
        OpSkipNkey {
            reg: reg
        }
    }
}
impl Opcode for OpSkipNkey {
    #[allow(dead_code)]
    fn execute(&self, core: &mut Chip8) {
        core.advance_pc();
    }
    fn as_u16(&self) -> u16 {
        0xE09E | (self.reg << 8) as u16
    }
    fn as_string(&self) -> String {
        format!("SkipNkey[v{:X}]", self.reg).to_string()
    }
}

struct OpLdRegDt {
    reg: usize
}
impl OpLdRegDt {
    fn new(reg: usize) -> OpLdRegDt {
        OpLdRegDt {
            reg: reg
        }
    }
}
impl Opcode for OpLdRegDt {
    #[allow(dead_code)]
    fn execute(&self, core: &mut Chip8) {
        core.gp_reg[self.reg] = core.delay_timer;
    }
    fn as_u16(&self) -> u16 {
        0xF007 | (self.reg << 8) as u16
    }
    fn as_string(&self) -> String {
        format!("LdRegDt[v{:X}]", self.reg).to_string()
    }
}

struct OpLdDtReg {
    reg: usize
}
impl OpLdDtReg {
    fn new(reg: usize) -> OpLdDtReg {
        OpLdDtReg {
            reg: reg
        }
    }
}
impl Opcode for OpLdDtReg {
    #[allow(dead_code)]
    fn execute(&self, core: &mut Chip8) {
        core.delay_timer = core.gp_reg[self.reg];
    }
    fn as_u16(&self) -> u16 {
        0xF007 | (self.reg << 8) as u16
    }
    fn as_string(&self) -> String {
        format!("LdDtReg[v{:X}]", self.reg).to_string()
    }
}



pub fn decode_instruction(instr_word: u16) -> Box<Opcode> {

    let op: Box<Opcode> = match split_nibbles_u16(instr_word) {
        (0x0, 0x0, 0xE, 0x0) => Box::new(OpCls::new()),
        (0x1, hi, mid, lo) => Box::new(OpJmp::new(join_nibbles(&[hi, mid, lo]) as u16)),
        (0x2, _, _, _) => Box::new(OpCall::new(instr_word)),
        (0x3, reg, hi, lo) => Box::new(OpSkipEqByte::new(reg as usize, join_nibbles(&[hi, lo]) as u8)),
        (0x4, _, _, _) => Box::new(OpSkipNotEqByte::new(instr_word)),
        (0x5, _, _, 0x0) => Box::new(OpSkipEqReg::new(instr_word)),
        (0x6, dest, hi, lo) => Box::new(OpLdByte::new(dest, join_nibbles(&[hi, lo]) as u8)),
        (0x7, dest, hi, lo) => Box::new(OpAddByte::new(dest, join_nibbles(&[hi, lo]) as u8)),

        (0x8, dest, src, 0x0) => Box::new(OpLdReg::new(dest, src)),
        (0x8, dest, src, 0x1) => Box::new(OpOrReg::new(dest, src)),
        (0x8, dest, src, 0x2) => Box::new(OpAndReg::new(dest, src)),
        (0x8, dest, src, 0x3) => Box::new(OpXorReg::new(dest, src)),
        (0x8, dest, src, 0x4) => Box::new(OpAddReg::new(dest, src)),
        (0x8, dest, src, 0x5) => Box::new(OpSubReg::new(dest, src)),
        (0x8, dest, src, 0x6) => Box::new(OpShrReg::new(dest, src)),

        (0xA, hi, mid, lo) => Box::new(OpLdI::new(join_nibbles(&[hi, mid, lo]))),

        (0xC, dest, hi, lo) => Box::new(OpRand::new(dest, join_nibbles(&[hi, lo]) as u8)),
        (0xD, x, y, n) => Box::new(OpSprite::new(x as usize, y as usize, n as usize)),
        (0xE, reg, 0x9, 0xE) => Box::new(OpSkipKey::new(reg as usize)),
        (0xE, reg, 0xA, 0x1) => Box::new(OpSkipNkey::new(reg as usize)),
        (0xF, reg, 0x0, 0x7) => Box::new(OpLdRegDt::new(reg as usize)),
        (0xF, reg, 0x1, 0x5) => Box::new(OpLdDtReg::new(reg as usize)),

        _ => Box::new(OpInvalid::new(instr_word)),
    };

    op
}

fn join_nibbles(nibbles: &[u8]) -> u32 {
    let mut result = 0;

    for n in nibbles {
        result = (result << 4) | (*n as u32);
    }
    result
}

fn split_nibbles_u16(word: u16) -> (u8, u8, u8, u8) {
    (
        ((word >> 12) & 0x0F) as u8,
        ((word >> 8) & 0x0F) as u8,
        ((word >> 4) & 0x0F) as u8,
        ((word >> 0) & 0x0F) as u8
    )
}
#[test]
fn test_join_nibbles() {
    assert!(join_nibbles(&[1, 6]) == 0x0016);
    assert!(join_nibbles(&[2, 3, 4]) == 0x0234);
    assert!(join_nibbles(&[0xF, 0x4, 0x0, 0x7]) == 0xF407);
}
