use std::path::Path;

use sdl2;
use sdl2::{Sdl, VideoSubsystem};
use sdl2::render::Renderer;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;

use chip8::{SharedState};



const PIXEL_WIDTH: u32 = 12;
const PIXEL_HEIGHT: u32 = PIXEL_WIDTH;

const SCREEN_WIDTH: u32 = PIXEL_WIDTH * 64;
const SCREEN_HEIGHT: u32 = PIXEL_WIDTH * 32;

const BUTTON_HEIGHT: u32 = (SCREEN_HEIGHT - 3 * BUTTON_SEP_HEIGHT) / 4;
const BUTTON_WIDTH: u32 = BUTTON_HEIGHT;
const BUTTON_SEP_WIDTH: u32 = 3;
const BUTTON_SEP_HEIGHT: u32 = BUTTON_SEP_WIDTH;

const KEYBOARD_WIDTH: u32 = BUTTON_WIDTH * 4 + BUTTON_SEP_WIDTH * 3;
const KEYBOARD_HEIGHT: u32 = BUTTON_HEIGHT * 4 + BUTTON_SEP_HEIGHT * 3;

const KEYBOARD_SEP_WIDTH: u32 = 6;

const WINDOW_WIDTH:u32 = SCREEN_WIDTH + KEYBOARD_SEP_WIDTH + KEYBOARD_WIDTH;
const WINDOW_HEIGHT:u32 = SCREEN_HEIGHT;
const KEYBOARD_XOFFSET:u32 = SCREEN_WIDTH + KEYBOARD_SEP_WIDTH;



pub trait Interface {
    fn draw_screen(&mut self, state: &mut SharedState);
    //fn get_key_state(&self) -> [bool; 16];
    fn handle_input(&mut self, state: &mut SharedState) -> bool;
}


#[allow(unused_variables)]
pub struct InterfaceSdl2 {
    sdl_context: Sdl,
    video_subsys: VideoSubsystem,
    renderer: Renderer<'static>,
}
impl InterfaceSdl2 {
    pub fn new() -> InterfaceSdl2 {
        let sdl_context = sdl2::init().unwrap();
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys.window("chipr", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let renderer = window.renderer().accelerated().build().unwrap();


        InterfaceSdl2 {
            sdl_context: sdl_context,
            video_subsys: video_subsys,
            renderer: renderer,
        }

    }
    fn render_vram(&mut self, pixels: &[[u8; 32]; 64]) {
        let mut x = 0;
        for col in pixels.iter() {
            let mut y = 0;
            for dot in col.iter() {
                if *dot > 0 {
                    self.renderer.set_draw_color(Color::RGB(0xf0, 0xF0, 0xF0));
                } else {
                    self.renderer.set_draw_color(Color::RGB(0x00, 0x00, 0x00));
                }
                self.renderer.fill_rect(Rect::new((PIXEL_WIDTH as i32) * x, PIXEL_HEIGHT as i32 * y, PIXEL_WIDTH, PIXEL_HEIGHT)).unwrap();
                y += 1;
            }
            x += 1;
        }
    }


    fn draw_ui(&mut self, state: &mut SharedState) {
        let keys = *state.keys.read().unwrap();
        let mut i = 0;
        for y in 0..4 {
            for x in 0..4 {
                if keys.is_down(i) {
                    self.renderer.set_draw_color(Color::RGB(0xf0, 0xF0, 0xF0));
                } else {
                    self.renderer.set_draw_color(Color::RGB(0x60, 0x60, 0x60));
                }
                let xx = BUTTON_WIDTH * x + BUTTON_SEP_WIDTH * x + KEYBOARD_XOFFSET;
                let yy = BUTTON_HEIGHT * y + BUTTON_SEP_HEIGHT * y;
                let rect = Rect::new(xx as i32, yy as i32, BUTTON_WIDTH, BUTTON_HEIGHT);
                self.renderer.fill_rect(rect).unwrap();

                i += 1;
            }
        }
    }


}
impl Interface for InterfaceSdl2 {
    fn draw_screen(&mut self, state: &mut SharedState)
    {
        self.render_vram(&state.vram.read().unwrap().pixels);
        self.draw_ui(state);
        self.renderer.present();
    }




    //fn get_key_state(&self) -> [bool; 16] {
    //    [false; 16]
    //}

    fn handle_input(&mut self, state: &mut SharedState) -> bool {
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
        state.keys.write().unwrap().state = key_state;
        false
    }

}
