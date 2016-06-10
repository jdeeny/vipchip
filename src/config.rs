#[derive(Copy, Clone)]
pub struct Config {
    pub ram_size: usize,
}

impl Config {
    pub fn new() -> Config {
        Config {
            ram_size: 0x2000,
        }
    }
}
