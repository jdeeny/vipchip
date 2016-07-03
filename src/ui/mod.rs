pub mod interface;

use std::thread;

use chip8::{Config, SharedState};

use self::interface::{Interface, InterfaceSdl2};
use std::time::{Duration, SystemTime};



pub struct Ui {
    config: Config,
    state: SharedState,
    interface: Box<Interface>,
}

impl Ui {
    pub fn new(config: Config, state: SharedState) -> Ui {
        Ui {
            config: config,
            state: state,
            interface: Box::new(InterfaceSdl2::new()),
        }
    }

    pub fn run(&mut self) {
        let park_duration = Duration::new(0, 50);
        let frame_period = Duration::new(0, 1000000000 / 60);
        let mut last_frame = SystemTime::now();
        'running: loop {
            {
                match self.interface.handle_input(&mut self.state) {
                    true => break 'running,
                    _ => (),
                }
            }
            match last_frame.elapsed() {
                Ok(elapsed) => {
                    if elapsed > frame_period {
                        self.interface.draw_screen(&mut self.state);
                        last_frame += frame_period;
                    }
                }
                _ => (),
            }
            thread::park_timeout(park_duration);
        }
    }
}
