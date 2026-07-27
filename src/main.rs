mod font;
mod cpu;
mod drivers;
pub const VIDEO_WIDTH: usize = 64;
pub const VIDEO_HEIGHT: usize = 32;

use std::thread;
use std::time::Duration;
use std::env;
use drivers::{DisplayDriver,CartridgeDriver};
use cpu::CPU;
use crate::drivers::InputDriver;

fn main() {
    let sleep_duration = Duration::from_millis(2);
    let sdl_context = sdl2::init().unwrap();
    let args:Vec<String> = env::args().collect();
    let cartridge_filename = & args [1];

    let cartridge = CartridgeDriver::new(cartridge_filename);
    let mut display= DisplayDriver::new(&sdl_context);
    let mut input = InputDriver::new(&sdl_context);
    let mut processor = CPU::new();

    match processor.load(&cartridge.rom) {
        Ok(()) => (),
        Err(err) => {
            return eprintln!("Could not initialize CPU: {}",err);
        }
    }

    let mut last_timer_update = std::time::Instant::now();

    while let Ok(keypad) = input.poll() {
        processor.vram_changed=false;
        processor.keypad = keypad;

        //update timers at 60 Hz
        if last_timer_update.elapsed() >= Duration::from_millis(1000/70) {
            if processor.keypad_waiting {
                for i in 0..processor.keypad.len() {
                    if processor.keypad[i] {
                        processor.keypad_waiting = false ;
                        processor.v[processor.keypad_register] = i as u8;
                    }
                }
            }
            if processor.delay_timer > 0 {processor.delay_timer -= 1;}
            if processor.sound_timer > 0 {processor.sound_timer -= 1;}

            last_timer_update = std::time::Instant::now();
        }
        //Fetch and Execute
        let opcode = processor.fetch_instruction();
        processor.run_instruction(opcode);

        //Redraw if anything changed
        if processor.vram_changed {
            display.draw(&processor.vram);
            processor.vram_changed = false;
        }

        //Sleep to control CPU speed
        thread::sleep(sleep_duration);
    }
}
