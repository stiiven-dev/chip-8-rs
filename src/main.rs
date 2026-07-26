mod font;
mod cpu;
mod drivers;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
pub const VIDEO_WIDTH: usize = 64;
pub const VIDEO_HEIGHT: usize = 32;

use std::thread;
use std::time::Duration;
use std::env;
use drivers::{DisplayDriver,CartridgeDriver};
use cpu::CPU;
fn main() {
    let sleep_duration = Duration::from_millis(2);
    let sdl_context = sdl2::init().unwrap();
    let args:Vec<String> = env::args().collect();
    let cartridge_filename = & args [1];

    let cartridge = CartridgeDriver::new(cartridge_filename);
    let mut display= DisplayDriver::new(&sdl_context);
    let mut processor = CPU::new();

    match processor.load(&cartridge.rom) {
        Ok(()) => (),
        Err(err) => {
            return eprintln!("Could not initialize CPU: {}",err);
        }
    }

    let mut last_timer_update = std::time::Instant::now();

    loop {
        //Fetch and Execute
        let opcode = processor.fetch_instruction();
        processor.run_instruction(opcode);

        //update timers at 60 Hz
        if last_timer_update.elapsed() >= Duration::from_millis(1000/60) {
            if processor.delay_timer > 0 {processor.delay_timer -= 1;}
            if processor.sound_timer > 0 {processor.sound_timer -= 1;}
            last_timer_update = std::time::Instant::now();
        }

        //Redraw if anything changed
        if processor.vram_changed {
            display.draw(&processor.vram);
            processor.vram_changed = false;
        }

        //Sleep to control CPU speed
        thread::sleep(sleep_duration);

        for event in sdl_context.event_pump().unwrap().poll_iter() {
            if let Event::Quit { .. } = event {
                return; // exit cleanly
            }
        }
    }
}
