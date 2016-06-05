
use sdl2;
use sdl2::{Sdl, VideoSubsystem};
use sdl2::render::Renderer;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

const WINDOW_WIDTH: u32 = 1600;
const WINDOW_HEIGHT: u32 = 900;


pub trait Interface {
    fn draw_screen(&mut self, pixels: &[[u8; 64]; 32]);
    fn get_key_state(&self) -> [bool; 16];
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
    fn draw_screen(&mut self, pixels: &[[u8; 64]; 32]) {
        self.renderer.clear();

        let mut y = 0;
        for row in pixels.iter() {
            let mut x = 0;
            for dot in row.iter() {
                if *dot > 0 {
                    self.renderer.set_draw_color(Color::RGB(0xf0, 0xF0, 0xF0));
                } else {
                    self.renderer.set_draw_color(Color::RGB(0x00, 0x00, 0x00));
                }
                self.renderer.fill_rect(Rect::new(10*x, 10*y, 10, 10));
                x += 1;
            }
            y += 1;
        }

        self.renderer.present();
    }

    fn get_key_state(&self) -> [bool; 16] {
        [false; 16]
    }
}
