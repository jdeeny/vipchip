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
