use std::sync::{Arc, RwLock};

pub struct SharedState {
    pub vram: Arc<RwLock<Vram>>,
    pub keys: Arc<RwLock<Keyboard>>,
    pub audio: Arc<RwLock<Audio>>,
}

pub struct Vram {
    pub pixels: [[u8; 32]; 64],
}
impl Vram {
    pub fn new() -> Vram {
        Vram { pixels: [[0; 32]; 64], }
    }
}


pub struct Keyboard {
    pub state: [bool; 16],
}
impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            state: [false; 16],
        }
    }
    pub fn is_down(&self, key: usize) -> bool {
        assert!(key <= 16);
        self.state[key]
    }
}


pub struct Audio {

}
impl Audio {
    pub fn new() -> Audio {
        Audio {}
    }
}
