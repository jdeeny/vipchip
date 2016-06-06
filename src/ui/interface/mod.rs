
use sdl2;
use sdl2::{Sdl, VideoSubsystem};
use sdl2::render::Renderer;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;

use std::sync::{Arc, RwLock};
use chip8::{Vram, Keys, Audio};

const WINDOW_WIDTH: u32 = 10 * 64;
const WINDOW_HEIGHT: u32 = 10 * 32;


pub trait Interface {
    fn draw_screen(&mut self, pixels: &[[u8; 32]; 64]);
    //fn get_key_state(&self) -> [bool; 16];
    fn handle_input(&mut self, &mut Arc<RwLock<Keys>>) -> bool;
}


pub struct InterfaceSdl2 {
    sdl_context: Sdl,
    video_subsys: VideoSubsystem,
    renderer: Renderer<'static>,
}
impl InterfaceSdl2 {
    pub fn new() -> InterfaceSdl2 {
        let sdl_context = sdl2::init().unwrap();
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys.window("chippy", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let renderer = window.renderer().accelerated().build().unwrap();

        InterfaceSdl2 {
            sdl_context: sdl_context,
            video_subsys: video_subsys,
            renderer: renderer
        }
    }

}
impl Interface for InterfaceSdl2 {
    fn draw_screen(&mut self, pixels: &[[u8; 32]; 64]) {
        self.renderer.clear();

        let mut x = 0;
        for col in pixels.iter() {
            let mut y = 0;
            for dot in col.iter() {
                if *dot > 0 {
                    self.renderer.set_draw_color(Color::RGB(0xf0, 0xF0, 0xF0));
                } else {
                    self.renderer.set_draw_color(Color::RGB(0x00, 0x00, 0x00));
                }
                self.renderer.fill_rect(Rect::new(10*x, 10*y, 10, 10)).unwrap();
                y += 1;
            }
            x += 1;
        }

        self.renderer.present();
    }

    //fn get_key_state(&self) -> [bool; 16] {
    //    [false; 16]
    //}

    fn handle_input(&mut self, keys: &mut Arc<RwLock<Keys>>) -> bool {
        let mut events = self.sdl_context.event_pump().unwrap();
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} => return false,
                _ => ()
            }
        }

        let mut key_state = [false; 16];

        for scancode in events.keyboard_state().pressed_scancodes() {
            match scancode {
                Scancode::Backspace => { return true; },
                Scancode::Num1 => { key_state[0x1] = true; },
                Scancode::Num2 => { key_state[0x2] = true; },
                Scancode::Num3 => { key_state[0x3] = true; },
                Scancode::Num4 => { key_state[0xC] = true; },
                Scancode::Q => { key_state[0x4] = true; },
                Scancode::W => { key_state[0x5] = true; },
                Scancode::E => { key_state[0x6] = true; },
                Scancode::R => { key_state[0xD] = true; },
                Scancode::A => { key_state[0x7] = true; },
                Scancode::S => { key_state[0x8] = true; },
                Scancode::D => { key_state[0x9] = true; },
                Scancode::F => { key_state[0xE] = true; },
                Scancode::Z => { key_state[0xA] = true; },
                Scancode::X => { key_state[0x0] = true; },
                Scancode::C => { key_state[0xB] = true; },
                Scancode::V => { key_state[0xF] = true; },
                _ => ()
            }
        }
        keys.write().unwrap().state = key_state;
        false
    }

}
