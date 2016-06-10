use chip8::{ Chip8, Instruction, Word };

pub type Operation = fn(&Instruction, &mut Chip8);

pub fn OpAdd(inst: &Instruction, core: &mut Chip8) {
//static OpAdd: opcode_closure = |inst: Instruction, core: Chip8| {
  let lhs = core.load(inst.dest());
  let rhs = core.load(inst.src());
  let total = lhs + rhs;
  core.vf_store(total > 0xFF);  //set vF if result overflows
  core.store(inst.dest(), total & 0xFF);
}
