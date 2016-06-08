
#[derive(Copy, Clone)]
pub enum Operand {
    Register(usize),
    Address(usize),
    ByteLiteral(usize),
    NibbleLiteral(usize),
    I,
    Indirect,
    SoundTimer,
    DelayTimer,
    No
}

pub fn operand_to_string(op: Operand) -> String {
    match op {
        Operand::Register(r)        => format!("v{:X}", r),
        Operand::Address(a)         => format!("@0x{:X}", a),
        Operand::ByteLiteral(b)     => format!("0x{:02X}", b),
        Operand::NibbleLiteral(n)   => format!("0x{:01X}", n),
        Operand::I                  => format!("I"),
        Operand::Indirect           => format!("Indirect"),
        Operand::SoundTimer         => format!("ST"),
        Operand::DelayTimer         => format!("DT"),
        Operand::No                 => format!("none"),
    }
}
