use std::thread;
use chip8::{Chip8, Address};


pub trait Opcode {
    //    fn new(instr_word: u16) -> &Opcode;
    fn execute(&mut self, &mut Chip8);
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
    fn execute(&mut self, core: &mut Chip8) { thread::sleep_ms(1000);}
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
    fn execute(&mut self, core: &mut Chip8) {
        let mut vram = core.vram.write().unwrap();

        vram.pixels = [[0; 32]; 64];
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
    fn execute(&mut self, core: &mut Chip8) {
        core.jump_pc(self.dest as Address);
    }
    fn as_u16(&self) -> u16 {
        0x1000 | self.dest
    }
    fn as_string(&self) -> String {
        format!("Jmp[0x{:X}]", self.dest).to_string()
    }
}

struct OpJmp0 {
    dest: u16,
}
impl OpJmp0 {
    fn new(addr: u16) -> OpJmp0 {
        OpJmp0 { dest: addr & 0x0FFF }
    }
}
impl Opcode for OpJmp0 {
    fn execute(&mut self, core: &mut Chip8) {
        let dest = self.dest as Address + core.gp_reg[0] as Address;
        core.jump_pc(dest);
    }
    fn as_u16(&self) -> u16 {
        0xB000 | self.dest
    }
    fn as_string(&self) -> String {
        format!("Jmp0[0x{:X}]", self.dest).to_string()
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
    fn execute(&mut self, core: &mut Chip8) {
        core.stack.push(core.pc as Address);
        core.jump_pc(self.dest as Address);
    }
    fn as_u16(&self) -> u16 {
        0x2000 | self.dest
    }
    fn as_string(&self) -> String {
        format!("Call[{:X}]", self.dest).to_string()
    }
}

struct OpRet { }
impl OpRet {
    fn new() -> OpRet {
        OpRet {}
    }
}
impl Opcode for OpRet {
    fn execute(&mut self, core: &mut Chip8) {
        let addr = core.stack.pop().unwrap();
        core.jump_pc(addr as usize);
    }

    fn as_u16(&self) -> u16 {
        0x00E0
    }
    fn as_string(&self) -> String {
        "Ret".to_string()
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
    fn execute(&mut self, core: &mut Chip8) {
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
    fn execute(&mut self, core: &mut Chip8) {
        if core.gp_reg[self.rega as usize] == core.gp_reg[self.regb as usize] {
            core.pc = core.pc + 2;
        }
    }
    fn as_u16(&self) -> u16 {
        0x5000 | (((self.rega as u16) & 0x0F) << 8) | ((self.regb as u16) & 0x0F) << 4
    }
    fn as_string(&self) -> String {
        format!("SkipEqReg [v{:X}?=v{:X}]", self.rega, self.regb).to_string()
    }
}


struct OpSkipNeqByte {
    reg: u8,
    val: u8,
}
impl OpSkipNeqByte {
    fn new(instr_word: u16) -> OpSkipNeqByte {
        OpSkipNeqByte {
            reg: ((instr_word >> 8) & 0x0F) as u8,
            val: (instr_word & 0x00FF) as u8,
        }
    }
}
impl Opcode for OpSkipNeqByte {
    fn execute(&mut self, core: &mut Chip8) {
        if core.gp_reg[self.reg as usize] != self.val {
            core.pc = core.pc + 2;
        }
    }
    fn as_u16(&self) -> u16 {
        0x4000 | ((self.reg as u16) << 8) | self.val as u16
    }
    fn as_string(&self) -> String {
        format!("SkipNeqByte[v{:X}?={:X}]", self.reg, self.val).to_string()
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
    fn execute(&mut self, core: &mut Chip8) {
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
    fn execute(&mut self, core: &mut Chip8) {
        core.gp_reg[self.dest as usize] = core.gp_reg[self.src as usize];
    }
    fn as_u16(&self) -> u16 {
        0x8000 | ((self.dest as u16) << 8) | (self.src as u16) << 4
    }
    fn as_string(&self) -> String {
        format!("LdReg[v{:X}=v{:X}]", self.dest, self.src).to_string()
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
    fn execute(&mut self, core: &mut Chip8) {
        let addend = core.gp_reg[self.reg as usize] as u32;
        let total = addend + (self.val as u32);
        core.gp_reg[self.reg as usize] = (total & 0xFF) as u8;
    }
    fn as_u16(&self) -> u16 {
        0x7000 | ((self.reg as u16) << 8) | self.val as u16
    }
    fn as_string(&self) -> String {
        format!("AddByte[v{:X}+={:X}]", self.reg, self.val).to_string()
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
    fn execute(&mut self, core: &mut Chip8) {

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
    fn execute(&mut self, core: &mut Chip8) {
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
    fn execute(&mut self, core: &mut Chip8) {
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
    fn execute(&mut self, core: &mut Chip8) {
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
    fn execute(&mut self, core: &mut Chip8) {
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
    fn execute(&mut self, core: &mut Chip8) {

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
    fn execute(&mut self, core: &mut Chip8) {
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
    fn execute(&mut self, core: &mut Chip8) {
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
    fn execute(&mut self, core: &mut Chip8) {
        let x_reg = self.x;
        let y_reg = self.y;
        let x = core.gp_reg[x_reg] as usize;
        let mut y = core.gp_reg[y_reg] as usize;
        let mut i = core.i as usize;
        let mut pixels = core.vram.read().unwrap().pixels;

        core.gp_reg[0xF] = 0;
        for _ in 0..self.n {
            let byte = core.ram[i];
            for bit in 0..8 {
                let pixel = if byte & (0x80 >> bit) == 0 { 0 } else { 1 };
                let x_loc = ((x + bit) & 63) as usize;
                let y_loc = (y & 31) as usize;
                pixels[x_loc][y_loc] ^= pixel;
                if pixels[x_loc][y_loc] == 0 && pixel == 1 {
                    core.gp_reg[0xF] = 1;
                }
            }
            i += 1;
            y += 1;
        }

        let mut vram = core.vram.write().unwrap();
        vram.pixels = pixels;

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
    fn execute(&mut self, core: &mut Chip8) {
        let mut key_state = false;
        {
            let keys = core.keys.read().unwrap();
            let n = core.gp_reg[self.reg];
            if keys.check(n as usize) {
                key_state = true;
            }
        }
        if key_state {
            core.advance_pc();
        }
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
    fn execute(&mut self, core: &mut Chip8) {
        let mut key_state = false;
        {
            let keys = core.keys.read().unwrap();
            let n = core.gp_reg[self.reg];
            if keys.check(n as usize) {
                key_state = true;
            }
        }
        if !key_state {
            core.advance_pc();
        }
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
    fn execute(&mut self, core: &mut Chip8) {
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
    fn execute(&mut self, core: &mut Chip8) {
        core.delay_timer = core.gp_reg[self.reg];
    }
    fn as_u16(&self) -> u16 {
        0xF007 | (self.reg << 8) as u16
    }
    fn as_string(&self) -> String {
        format!("LdDtReg[v{:X}]", self.reg).to_string()
    }
}

struct OpLdStReg {
    reg: usize
}
impl OpLdStReg {
    fn new(reg: usize) -> OpLdStReg {
        OpLdStReg {
            reg: reg
        }
    }
}
impl Opcode for OpLdStReg {
    #[allow(dead_code)]
    fn execute(&mut self, core: &mut Chip8) {
        core.sound_timer = core.gp_reg[self.reg];
    }
    fn as_u16(&self) -> u16 {
        0xF007 | (self.reg << 8) as u16
    }
    fn as_string(&self) -> String {
        format!("LdStReg[v{:X}]", self.reg).to_string()
    }
}



struct OpAddIReg {
    reg: usize
}
impl OpAddIReg {
    fn new(reg: usize) -> OpAddIReg {
        OpAddIReg {
            reg: reg
        }
    }
}
impl Opcode for OpAddIReg {
    #[allow(dead_code)]
    fn execute(&mut self, core: &mut Chip8) {
        core.i += core.gp_reg[self.reg] as usize;
    }
    fn as_u16(&self) -> u16 {
        0xF01E | (self.reg << 8) as u16
    }
    fn as_string(&self) -> String {
        format!("AddIReg[v{:X}]", self.reg).to_string()
    }
}

struct OpSaveReg {
    reg: usize
}
impl OpSaveReg {
    fn new(reg: usize) -> OpSaveReg {
        OpSaveReg {
            reg: reg
        }
    }
}
impl Opcode for OpSaveReg {
    #[allow(dead_code)]
    fn execute(&mut self, core: &mut Chip8) {
        for i in 0..(self.reg + 1) {
            core.ram[core.i+i] = core.gp_reg[i];
        }
    }
    fn as_u16(&self) -> u16 {
        0xF055 | (self.reg << 8) as u16
    }
    fn as_string(&self) -> String {
        format!("SaveReg[v{:X}]", self.reg).to_string()
    }
}

struct OpRestoreReg {
    reg: usize
}
impl OpRestoreReg {
    fn new(reg: usize) -> OpRestoreReg {
        OpRestoreReg {
            reg: reg
        }
    }
}
impl Opcode for OpRestoreReg {
    #[allow(dead_code)]
    fn execute(&mut self, core: &mut Chip8) {
        for i in 0 .. (self.reg + 1) {
            core.gp_reg[i] = core.ram[core.i+i];
        }
    }
    fn as_u16(&self) -> u16 {
        0xF065 | (self.reg << 8) as u16
    }
    fn as_string(&self) -> String {
        format!("RestoreReg[v{:X}]", self.reg).to_string()
    }
}



pub fn decode_instruction(instr_word: u16) -> Box<Opcode> {

    let op: Box<Opcode> = match split_nibbles_u16(instr_word) {
        (0x0, 0x0, 0xE, 0x0) => Box::new(OpCls::new()),
        (0x0, 0x0, 0xE, 0xE) => Box::new(OpRet::new()),
        (0x1, hi, mid, lo) => Box::new(OpJmp::new(join_nibbles(&[hi, mid, lo]) as u16)),
        (0x2, _, _, _) => Box::new(OpCall::new(instr_word)),
        (0x3, reg, hi, lo) => Box::new(OpSkipEqByte::new(reg as usize, join_nibbles(&[hi, lo]) as u8)),
        (0x4, _, _, _) => Box::new(OpSkipNeqByte::new(instr_word)),
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
        (0xB, hi, mid, lo) => Box::new(OpJmp0::new(join_nibbles(&[hi, mid, lo]) as u16)),

        (0xC, dest, hi, lo) => Box::new(OpRand::new(dest, join_nibbles(&[hi, lo]) as u8)),
        (0xD, x, y, n) => Box::new(OpSprite::new(x as usize, y as usize, n as usize)),
        (0xE, reg, 0x9, 0xE) => Box::new(OpSkipKey::new(reg as usize)),
        (0xE, reg, 0xA, 0x1) => Box::new(OpSkipNkey::new(reg as usize)),
        (0xF, reg, 0x0, 0x7) => Box::new(OpLdRegDt::new(reg as usize)),
        (0xF, reg, 0x1, 0x5) => Box::new(OpLdDtReg::new(reg as usize)),
        (0xF, reg, 0x1, 0x8) => Box::new(OpLdStReg::new(reg as usize)),
        (0xF, reg, 0x1, 0xE) => Box::new(OpAddIReg::new(reg as usize)),
        (0xF, reg, 0x5, 0x5) => Box::new(OpSaveReg::new(reg as usize)),
        (0xF, reg, 0x6, 0x5) => Box::new(OpRestoreReg::new(reg as usize)),

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
