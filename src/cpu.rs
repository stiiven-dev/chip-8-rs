const CHIP8_RAM : usize = 4096;
const REGISTER_COUNT: usize = 16;
const STACK_SIZE: usize = 16;

use rand;
use rand::RngExt;
use crate::font::FONT_SET;
use crate::VIDEO_WIDTH;
use crate::VIDEO_HEIGHT;
pub struct CPU{
    pub vram: [[u8; VIDEO_WIDTH]; VIDEO_HEIGHT],
    pub vram_changed: bool,
    ram : [u8; CHIP8_RAM],
    pub v : [u8; REGISTER_COUNT],
    i : usize ,
    pub delay_timer : u8,
    pub sound_timer : u8,
    stack : [u16 ; STACK_SIZE],
    stack_pointer : usize,
    pub pc : usize ,
    pub keypad : [bool ; 16],
    pub keypad_waiting : bool,
    pub keypad_register : usize,

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
            pc : 0x200  ,       //All chip-8 programs start here
            keypad : [false;16],
            keypad_waiting: false,
            keypad_register:0,
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
            (0,0,0xE,0xE)=>{
                self.r#return();
            },
            (0x1,_,_,_) => {
                self.nextpc();
                self.jump_to(nnn)
            },
            (0x2,_,_,_) => {
                self.nextpc();
                self.call(nnn);
            },
            (0x3,_,_,_)=>{
                self.nextpc();
                self.seq(self.v[x],nn);  //skip if equal (immediate)
            },
            (0x4,_,_,_) => {
                self.nextpc();
                self.snq(self.v[x],nn);  //skip if not equal (immediate)
            },
            (0x5,_,_,_) => {
                self.nextpc();
                self.seq(self.v[x],self.v[y]); //skip if equal
            }
            (0x6,_,_,_) => {
                self.nextpc();
                self.set_reg(x,nn)
            },
            (0x7,_,_,_) => {
                self.nextpc();
                self.add_reg(x.into(),nn)
            },
            (0x8,_,_,0)=>{
                self.nextpc();
                self.set_reg(x,self.v[y]);
            },
            (0x8,_,_,1)=>{
                self.nextpc();
                self.or(x,y);
            },
            (0x8,_,_,2)=>{
                self.nextpc();
                self.and(x,y);
            },
            (0x8,_,_,3)=>{
                self.nextpc();
                self.xor(x,y);
            },
            (0x8,_,_,4)=>{
                self.nextpc();
                self.add(x,y);
            },
            (0x8,_,_,5)=>{
                self.nextpc();
                let rhs = self.v[x];
                let lhs = self.v[y];
                self.v[x] = rhs.wrapping_sub(lhs) ;
                if rhs >= lhs {
                    self.v[0xF] = 1;
                }else { self.v[0xF] = 0 };
            },
            (0x8,_,_,6) => {
                self.nextpc();
                self.shr(x,y);
            },
            (0x8,_,_,7)=>{
                self.nextpc();
                let rhs = self.v[y];
                let lhs = self.v[x];
                self.v[x] = rhs.wrapping_sub(lhs) ;
                if rhs >= lhs {
                    self.v[0xF] = 1;
                }else { self.v[0xF] = 0 };
            },
            (0x8,_,_,0xE) => {
                self.nextpc();
                self.shl(x,y);
            },
            (0x9,_,_,_) => {
                self.nextpc();
                self.snq(self.v[x],self.v[y]); //skip if not equal
            },
            (0xA,_,_,_) => {
                self.nextpc();
                self.set_index(nnn);
            },
            (0xB,_,_,_) => {
                self.pc = (self.v[0] as usize) + nnn ;
            },
            (0xC,_,_,_) => {
                self.nextpc();
                let mut rng = rand::rng();
                self.v[x] = rng.random::<u8>() & nn;
            },
            (0xD,_,_,_) => {
                self.nextpc();
                self.draw(x,y,n);
            },
            (0xE,_,9,0xE)=> {
              self.nextpc();
                if self.keypad[self.v[x] as usize ] { self.nextpc() }
            },
            (0xE,_,0xA,0x1)=> {
              self.nextpc();
                if ! self.keypad[self.v[x] as usize ] { self.nextpc() }
            },
            (0xF,_,0,7)=>{
                self.nextpc();
                self.v[x] = self.delay_timer;
            },
            (0xF,_,0,0xA)=>{
                self.keypad_waiting = true;
                self.keypad_register = x ;
            },
            (0xF,_,1,5)=>{
                self.nextpc();
                self.delay_timer = self.v[x];
            },
            (0xF,_,1,8)=>{
                self.nextpc();
                self.sound_timer = self.v[x];
            }
            (0xF,_,1,0xE) => {
                self.nextpc();
                self.i += self.v[x] as usize ;
            },
            (0xF,_,2,9)=>{
                self.nextpc();
                self.i = (self.v[x] as usize) *5;
            }
            (0xF,_,3,3) => {
                self.nextpc();
                let val = self.v[x];
                self.ram[self.i] = val / 100 ;
                self.ram[self.i +1] = (val % 100)/ 10 ;
                self.ram[self.i +2] = val % 10 ;

            },
            (0xF,_,5,5) => {
                self.nextpc();
                for i in 0..x + 1 {
                    self.ram[self.i + i] = self.v[i];
                }
            },
            (0xF,_,6,5) => {
                self.nextpc();
                for i in 0..x + 1 {
                    self.v[i] = self.ram[self.i + i];
                }
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

    fn set_reg(&mut self , x : usize , y : u8) {
        self.v[x] = y;
    }

    fn add_reg(&mut self , x : usize , nn:u8) {
        self.v[x] = self.v[x].overflowing_add(nn).0;
    }

    fn jump_to(&mut self , j : usize) {
        self.pc = j ;
    }

    fn call(&mut self, addr:usize) {
        self.stack[self.stack_pointer] = self.pc as u16 ;
        self.stack_pointer+=1;
        self.pc = addr
    }

    fn r#return(&mut self) {
        self.stack_pointer-=1;
        self.pc = self.stack[self.stack_pointer].into();

    }

    fn seq(&mut self,x:u8,y:u8){
        if x == y {
            self.pc+=2;
        }
    }

    fn snq(&mut self,x:u8,y:u8){
        if x != y {
            self.pc+=2;
        }
    }

    pub fn or(&mut self , x:usize , y:usize){
        self.v[x] = self.v[x] | self.v[y];
        self.v[0xF]=0;
    }

    pub fn and(&mut self , x:usize , y:usize){
        self.v[x] = self.v[x] & self.v[y];
        self.v[0xF]=0;
    }
    #[allow(unused_comparisons)]
    pub fn add(&mut self , x:usize , y:usize){
        let vx = self.v[x] as u16;
        let vy = self.v[y] as u16;
        let result = vx + vy;
        self.v[x] = result as u8;
        self.v[0x0f] = if result > 0xFF { 1 } else { 0 };
    }
    pub fn xor(&mut self , x:usize , y:usize){
        self.v[x] = self.v[x] ^ self.v[y];
        self.v[0xF]=0;
    }

    pub fn shr(&mut self, x:usize,y:usize){
        self.v[x]=self.v[y];
        let carry = self.v[x] & 1;
        self.v[x] >>= 1;
        self.v[0xF]=carry;
    }
    pub fn shl(&mut self, x:usize,y:usize){
        self.v[x]=self.v[y];
        let carry = (self.v[x] & 0x80) >> 7  ;
        self.v[x] <<= 1;
        self.v[0xF]=carry;
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
