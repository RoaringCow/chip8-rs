use std::io::Read;
use std::fs::File;
use std::io;
use std::fmt;


mod instruction;
use crate::instruction::*;


/*
    Memory: 4KiB
    Registers: V0 to VF ....
    Stack: 16 slots
    framebuffer: 64x32

*/

/* 
     Memory Layout:
        |- 0x000 - 0x1FF: Chip 8 interpreter (contains font set in emulator)
        |- 0x050 - 0x0A0: Used for the built in 4x5 pixel font set (0-F)
        |- 0x200 - 0xFFF: Program ROM and work RAM
*/


pub struct Emulator {
    pub memory: [u8; 4096],      // 4K memory; 0x000 - 0xFFF
    v: [u8; 16],             // 16 8-bit registers; 0x0 - 0xF
    i: u16,                         // Memory address register
    pc: u16,                        // Program counter
    stack: [u16; 16],       // Stack; 16 levels of 16-bit values
    sp: u8,                         // Stack pointer; points to the top of the stack
    delay_timer: u8,
    sound_timer: u8,
    //display: [bool; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize],
    //keyboard: ,
}

impl Emulator {

    pub fn new() -> Emulator {
        let mut emulator = Emulator {
            memory: [0; 4096],
            v: [0; 16],
            i: 0x200,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            //display: Display::new(),
            //keyboard: Keyboard::new(),
        };


        emulator
    }


    pub fn read_rom<P: std::convert::AsRef<std::path::Path>>(mut self, path: P) -> io::Result<Emulator> {
        let file = File::open(path)?;
        for (location, byte) in file.bytes().enumerate() {
            self.memory[0x200 + location] = byte?;
        }
        Ok(self)
    }

    fn read_instruction(&self) -> Option<Instruction> {
        let opcode = (self.memory[self.pc as usize] as u16) | (self.memory[(self.pc + 1) as usize] as u16);
        // 16 bit oku
        Instruction::new(opcode)
    }

    fn run_instruction(&self) {
        self.pc = match {
            
        }
    }

}

// "/home/ersan/Downloads/octojam1title.ch8"


fn main() -> io::Result<()> {
    let x: OpCode = instruction::OpCode(0x8000);

    let emulator = Emulator::new();
    
    let emulator = emulator.read_rom("/home/ersan/Downloads/octojam1title.ch8")?;
    for byte in &emulator.memory {
        println!("{}", byte);
    }

    Ok(())
}

