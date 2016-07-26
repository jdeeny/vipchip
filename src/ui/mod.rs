pub mod interface;

use std::thread;

use chip8::Config;
use chip8::{SimulatorTask, Simulate};

use self::interface::{Interface, InterfaceSdl2};
use std::time::{Duration, SystemTime};



pub struct Ui {
    simulator: SimulatorTask,
    interface: Box<Interface>,
}

impl Ui {
    pub fn new(simulator: SimulatorTask) -> Ui {
        Ui {
            simulator: simulator,
            interface: Box::new(InterfaceSdl2::new()),
        }
    }

    pub fn run(&mut self) {
        let park_duration = Duration::new(0, 5);
        let frame_period = Duration::new(0, 1000000000 / 60);
        let mut last_frame = SystemTime::now();
        'running: loop {
            {
                match self.interface.handle_input(&mut self.simulator) {
                    true => break 'running,
                    _ => (),
                }
            }
            match last_frame.elapsed() {
                Ok(elapsed) => {
                    if elapsed > frame_period {
                        self.interface.draw_screen(&self.simulator);
                        last_frame += frame_period;
                        self.simulator.timer_tick().unwrap();
                    }
                }
                _ => (),
            }
            thread::park_timeout(park_duration);
            self.simulator.step_n(10).unwrap();
        }
    }
}
