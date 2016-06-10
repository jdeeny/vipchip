#[derive(Copy, Clone)]
pub struct Config {
    pub ram_size: usize,
    pub font_addr: usize,
}

impl Config {
    pub fn new() -> Config {
        Config {
            ram_size: 0x2000,
            font_addr: 0x0000,
        }
    }
}
