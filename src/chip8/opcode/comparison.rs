

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
