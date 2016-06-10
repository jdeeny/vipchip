use chip8::{ Chip8, Instruction, Word };

pub type Operation = fn(&Instruction, &mut Chip8);

pub fn OpAdd(inst: &Instruction, core: &mut Chip8) {
  let lhs = core.load(inst.dest());
  let rhs = core.load(inst.src());
  let total = lhs + rhs;
  core.vf_store(total > 0xFF);  //set vF if result overflows
  core.store(inst.dest(), total & 0xFF);
}

/// Subtract src from dest, store in dest. Set flag if NOT borrow
pub fn OpSub(inst: &Instruction, core: &mut Chip8) {
  let lhs = core.load(inst.dest()) as i32;
  let rhs = core.load(inst.src()) as i32;
  let total = ((lhs - rhs) as u32) & 0xFF;

  core.vf_store(lhs > rhs);  //set vF if NOT borrow
  core.store(inst.dest(), total);
}

/// Subtract dest from src, store in dest. Set flag if NOT borrow
pub fn OpSubN(inst: &Instruction, core: &mut Chip8) {
  let lhs = core.load(inst.src()) as i32;
  let rhs = core.load(inst.dest()) as i32;
  let total = ((lhs - rhs) as u32) & 0xFF;

  core.vf_store(lhs > rhs);  //set vF if NOT borrow
  core.store(inst.dest(), total);
}


pub fn OpOr(inst: &Instruction, core: &mut Chip8) {
  let lhs = core.load(inst.dest());
  let rhs = core.load(inst.src());
  let result = lhs | rhs;
  core.store(inst.dest(), result);
}

pub fn OpAnd(inst: &Instruction, core: &mut Chip8) {
  let lhs = core.load(inst.dest());
  let rhs = core.load(inst.src());
  let result = lhs & rhs;
  core.store(inst.dest(), result);
}

pub fn OpXor(inst: &Instruction, core: &mut Chip8) {
  let lhs = core.load(inst.dest());
  let rhs = core.load(inst.src());
  let result = lhs ^ rhs;
  core.store(inst.dest(), result);
}

/// Shifts the source right 1 bit, and stores in dest. vF set to old LSB
pub fn OpShR(inst: &Instruction, core: &mut Chip8) {
  let val = core.load(inst.src());
  core.vf_store(val & 1 == 1);
  let result = val >> 1;
  core.store(inst.dest(), result);
}

/// Shifts the source left 1 bit, and stores in dest. vF set to old MSB
pub fn OpShL(inst: &Instruction, core: &mut Chip8) {
  let val = core.load(inst.src());
  core.vf_store(val & 0x80 == 0x80);
  let result = (val << 1) & 0xFF;
  core.store(inst.dest(), result);
}



pub fn OpLoad(inst: &Instruction, core: &mut Chip8) {
    let data = core.load(inst.src());
    core.store(inst.dest(), data);
}

pub fn OpFont(inst: &Instruction, core: &mut Chip8) {
    let addr = core.config.font_addr as u32 + core.load(inst.src());
    core.store(inst.dest(), addr);
}

pub fn OpBCD(inst: &Instruction, core: &mut Chip8) {
    let val = core.load(inst.src());
    let hundreds = val / 100;
    let val = val - hundreds * 100;
    let tens = val / 10;
    let ones = val - tens * 10;

    let inital_i = core.i;      //store I so it can be restored
    core.store(inst.dest(), hundreds);
    core.i += 1;
    core.store(inst.dest(), tens);
    core.i += 1;
    core.store(inst.dest(), ones);
    core.i = inital_i;
}


pub fn OpRand(inst: &Instruction, core: &mut Chip8) {
    let mask = core.load(inst.src());
    let data = core.rng.next_u32() & mask;
    core.store(inst.dest(), data);
}



pub fn OpSkipEq(inst: &Instruction, core: &mut Chip8) {
    let lhs = core.load(inst.dest());
    let rhs = core.load(inst.src());
    if lhs == rhs {
        core.advance_pc();
    }
}

pub fn OpSkipNeq(inst: &Instruction, core: &mut Chip8) {
    let lhs = core.load(inst.dest());
    let rhs = core.load(inst.src());
    if lhs != rhs {
        core.advance_pc();
    }
}

pub fn OpSkipKey(inst: &Instruction, core: &mut Chip8) {
    let key = core.load(inst.dest()) as usize;
    let mut key_state = false;
    {
        let keys = core.state.keys.read().unwrap();
        key_state = keys.is_down(key);
    }
    if key_state { core.advance_pc(); }
}

pub fn OpSkipNKey(inst: &Instruction, core: &mut Chip8) {
    let key = core.load(inst.dest()) as usize;
    let mut key_state = false;
    {
        let keys = core.state.keys.read().unwrap();
        key_state = keys.is_down(key);
    }
    if !key_state { core.advance_pc(); }
}

pub fn OpWaitKey(inst: &Instruction, core: &mut Chip8) {
    panic!("WaitKey Unimplemented")
}


/// Jump to address
pub fn OpJump(inst: &Instruction, core: &mut Chip8) {
    let addr = core.load(inst.dest()) as usize;
    core.jump_pc(addr);
}

/// Jimp to address + V0
pub fn OpJumpV0(inst: &Instruction, core: &mut Chip8) {
    let mut addr = core.load(inst.dest()) as usize;
    addr += core.reg(0) as usize;
    core.jump_pc(addr);
}


pub fn OpCall(inst: &Instruction, core: &mut Chip8) {
    let addr = core.load(inst.dest()) as usize;
    core.stack.push(addr);
}

pub fn OpRet(inst: &Instruction, core: &mut Chip8) {
    let addr = core.stack.pop().unwrap();
    core.jump_pc(addr);
}






pub fn OpCls(inst: &Instruction, core: &mut Chip8) {
    let mut vram = core.state.vram.write().unwrap();

    vram.pixels = [[0; 32]; 64];
}

pub fn OpSprite(inst: &Instruction, core: &mut Chip8) {

    let x = core.load(inst.dest());
    let mut y = core.load(inst.src());
    let n = core.load(inst.aux());

    let mut i = core.i;

    let mut pixels = core.state.vram.read().unwrap().pixels;

    core.vf_clear();
    for _ in 0..n {
        let byte = core.ram[i];
        for bit in 0..8 {
            let pixel = if byte & (0x80 >> bit) == 0 { 0 } else { 1 };
            let x_loc = ((x + bit) & 63) as usize;
            let y_loc = (y & 31) as usize;
            pixels[x_loc][y_loc] ^= pixel;
            if pixels[x_loc][y_loc] == 0 && pixel == 1 {
                core.vf_set();
            }
        }
        i += 1;
        y += 1;
    }

     let mut vram = core.state.vram.write().unwrap();
     vram.pixels = pixels;

}

pub fn OpStash(inst: &Instruction, core: &mut Chip8) {
    let last = core.load(inst.src()) as usize;
    let i = core.i;
    for r in 0...last {
        let value = core.reg(r) as u8;
        core.set_ram(i+r, value);
    }
}

pub fn OpFetch(inst: &Instruction, core: &mut Chip8) {
    let last = core.load(inst.src()) as usize;
    let i = core.i;
    for r in 0...last {
        let v = core.ram(i+r);
        core.set_reg(r, v);
    }
}
