extern crate sdl2;

mod interface;
mod core;

use interface::{Interface, InterfaceSdl2};
use core::Chip8;

fn main() {
    let mut core = Chip8::new();
    let mut interface: &mut Interface = &mut InterfaceSdl2::new() as &mut Interface;

    let test_program: Vec<u8> = vec![   //0x71, 0x24,
                                        0xC3, 0x1F, 0xC4, 0x0F, 0xA2, 0x30, 0xD3, 0x48,
                                        0xD3, 0x48, 0x60, 0x05, 0xE0, 0xA1, 0x74, 0xFF,
                                        0x60, 0x08, 0xE0, 0xA1, 0x74, 0x01, 0x60, 0x07,
                                        0xE0, 0xA1, 0x73, 0xFF, 0x60, 0x09, 0xE0, 0xA1,
                                        0x73, 0x01, 0xD3, 0x48, 0xFF, 0x07, 0x3F, 0x00,
                                        0x12, 0x24, 0x6F, 0x03, 0xFF, 0x15, 0x12, 0x08,
                                        0x70, 0x70, 0x20, 0x70, 0xA8, 0x20, 0x50, 0x50,
                                    ];

    core.load_hex(&test_program, 0x200);
    core.pc = 0x200;

    core.dump_reg();
//    core.dump_pixels();

    for _ in 0..2000 {
        let instruction = core.decode_instruction(core.pc);

        println!("{:X} {:?}", instruction.as_u16(), instruction.as_string());

        core.set_key_state(interface.get_key_state());
        instruction.execute(&mut core);
        core.advance_pc();

        core.dump_reg();
//        core.dump_pixels();
        interface.draw_screen(&core.pixels);
    }



}
