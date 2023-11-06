use std::fs::File;
use std::io::Read;

const MEMORY_SIZE: usize = 4096;
const REG_COUNT: usize = 16;
const STACK_SIZE: usize = 16;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const NUM_KEYS: usize = 0; //????

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
    pub memory: [u8; MEMORY_SIZE], // 4K memory; 0x000 - 0xFFF
    pub v: [u8; REG_COUNT],    // 16 8-bit registers; 0x0 - 0xF
    pub i: u16,                    // Memory address register
    pub pc: u16,                   // Program counter
    pub stack: [u16; STACK_SIZE],  // Stack; 16 levels of 16-bit values
    pub sp: u8,                    // Stack pointer; points to the top of the stack
    pub dt: u8,                    // Delay timer
    pub st: u8,                    // Sound timer
    pub draw_flag: bool,           // Draw flag
    pub screen: [bool; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize], // Screen
    pub keypad: [bool; NUM_KEYS],  // Keys
}


fn main() {
    println!("Merhaba");
    if 2 == 0x1 {
    println!("Hello, world!");
    }
}

fn load_file(path: String) {
}
