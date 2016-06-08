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
