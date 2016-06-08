
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
