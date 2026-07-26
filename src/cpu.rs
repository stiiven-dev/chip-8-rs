const CHIP8_RAM : usize = 4096;
const REGISTER_COUNT: usize = 16;
const STACK_SIZE: usize = 16;

use crate::font::FONT_SET;
use crate::VIDEO_WIDTH;
use crate::VIDEO_HEIGHT;
pub struct CPU{
    pub vram: [[u8; VIDEO_WIDTH]; VIDEO_HEIGHT],
    pub vram_changed: bool,
    ram : [u8; CHIP8_RAM],
    v : [u8; REGISTER_COUNT],
    i : usize ,
    pub delay_timer : u8,
    pub sound_timer : u8,
    stack : [u16 ; STACK_SIZE],
    stack_pointer : usize,
    pc : usize ,

}

impl CPU {
    pub fn new() -> Self {
        let mut cpu = CPU {
            vram: [[0; VIDEO_WIDTH]; VIDEO_HEIGHT],
            vram_changed: false,
            ram : [0; CHIP8_RAM],
            v : [0; REGISTER_COUNT],
            i : 0 ,
            delay_timer : 0,
            sound_timer : 0,
            stack : [0 ; STACK_SIZE],
            stack_pointer : 0 ,
            pc : 0x200         //All chip-8 programs start here
        };
        cpu.load_font();
        cpu
    }

    fn load_font(&mut self) {
        for i in 0..FONT_SET.len() {
            self.ram[i] = FONT_SET[i] ;
        }
    }

    pub fn load(&mut self , data : &[u8]) -> Result<(), &str > {
        if 0x200 + data.len() > CHIP8_RAM { return Err("Out of memory : program too large") }
        for (i,instruct) in data.into_iter().enumerate()  {
            self.ram[i + 0x200 ] = *instruct ;
        }
        Ok(())
    }

    pub fn fetch_instruction(&self) -> u16 {
        (self.ram[self.pc] as u16) << 8 | (self.ram[self.pc +1] as u16)
    }

    pub fn run_instruction(&mut self , instruction : u16) {
        let nibbles  = (
            ((instruction & 0xF000) >> 12) as u8 ,
            ((instruction & 0x0F00) >> 8) as u8,
            ((instruction & 0x00F0) >> 4) as u8,
            (instruction & 0x000F) as u8
            );
        let nn = (instruction & 0x00FF) as u8;
        let nnn = (instruction & 0x0FFF) as usize;
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as usize;
        match nibbles {
            (0,0,0xE,0) => {
                self.nextpc();
                self.clear_screen()
            },
            (1,_,_,_) => {
                self.nextpc();
                self.jump_to(nnn)
            },
            (0x6,_,_,_) => {
                self.nextpc();
                self.set_reg(x,nn)
            },
            (0x7,_,_,_) => {
                self.nextpc();
                self.add_reg(x.into(),nn)
            },
            (0xA,_,_,_) => {
                self.nextpc();
                self.set_index(nnn);
            }
            (0xD,_,_,_) => {
                self.nextpc();
                self.draw(x,y,n);
            }
            (_, _, _, _) => {}
        }

    }

        fn draw(&mut self, vx: usize, vy: usize, n : usize){
        self.v[0xF]=0;

        for byte in 0..n {
            let posy = (self.v[vy] as usize + byte) % VIDEO_HEIGHT;
            for bit in 0..8 {
                let posx = (self.v[vx] as usize + bit) % VIDEO_WIDTH;
                let new_pix = (self.ram[self.i + byte] >> (7 - bit)) & 1;
                self.v[0xF]|= new_pix & self.vram[posy][posx];
                self.vram[posy][posx] ^= new_pix;

            }
        }
        self.vram_changed=true;
    }

    fn nextpc(&mut self) {
        self.pc += 2;
    }

    fn set_index(&mut self , nnn : usize) {
        self.i = nnn;
    }

    fn set_reg(&mut self , x : usize , nn : u8) {
        self.v[x] = nn;
    }

    fn add_reg(&mut self , x : usize , nn:u8) {
        self.v[x] = self.v[x].overflowing_add(nn).0;
    }

    fn jump_to(&mut self , j : usize) {
        self.pc = j ;
    }

    fn clear_screen(&mut self) {
        for y in 0..VIDEO_HEIGHT {
            for x in 0..VIDEO_WIDTH {
                self.vram[y][x] = 0 ;
            }
        }
        self.vram_changed = true;
    }
}
