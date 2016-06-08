
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


struct OpLdIFont {
    reg: usize
}
impl OpLdIFont {
    fn new(reg: usize) -> OpLdIFont {
        OpLdIFont {
            reg: reg
        }
    }
}
impl Opcode for OpLdIFont {
    #[allow(dead_code)]
    fn execute(&mut self, core: &mut Chip8) {
        core.i = core.gp_reg[self.reg] as usize * 5;
    }
    fn as_u16(&self) -> u16 {
        0xF01E | (self.reg << 8) as u16
    }
    fn as_string(&self) -> String {
        format!("LdIFont[v{:X}]", self.reg).to_string()
    }
}
