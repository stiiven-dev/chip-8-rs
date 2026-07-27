use std::fs::File;
use std::io::prelude::*;

pub struct CartridgeDriver {
    pub rom : [u8;3584],
}

impl CartridgeDriver {
    pub fn new(filename : &str) -> Self {
        let mut f = File::open(filename).expect("file not found");
        let mut buffer = [0u8 ; 3584];

        if let Ok(bytes_read) =f.read(&mut buffer) {
            bytes_read
        }else { 0 };

        CartridgeDriver{
            rom: buffer,
        }
    }
}