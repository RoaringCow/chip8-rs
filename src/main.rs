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

pub struct OpCode(u16);

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

pub type Register = usize;
pub type Address = u16;

pub enum Instruction {
    ClearDisplay, // 00E0 - CLS
    Return,       // 00EE - RET

    Jump(Address),                      // 1NNN - JP addr
    Call(Address),                      // 2NNN - CALL addr
    SkipEqual(Register, u8),                 // 3XNN - SE Vx, byte
    SkipNotEqual(Register, u8),              // 4XNN - SNE Vx, byte
    SkipEqualXY(Register, Register),              // 5XY0 - SE Vx, Vy
    Load(Register, u8),                      // 6XNN - LD Vx, byte
    Add(Register, u8),                       // 7XNN - ADD Vx, byte

    Move(Register, Register),                // 8XY0 - LD Vx, Vy
    Or(Register, Register),             // 8XY1 - OR Vx, Vy
    And(Register, Register),            // 8XY2 - AND Vx, Vy
    Xor(Register, Register),            // 8XY3 - XOR Vx, Vy
    AddXY(Register, Register),          // 8XY4 - ADD Vx, Vy
    SubXY(Register, Register),          // 8XY5 - SUB Vx, Vy
    ShiftRight(Register),               // 8XY6 - SHR Vx {, Vy}
    SubYX(Register, Register),          // 8XY7 - SUBN Vx, Vy
    ShiftLeft(Register),                // 8XYE - SHL Vx {, Vy}

    SkipNotEqualXY(Register, Register), // 9XY0 - SNE Vx, Vy
    LoadI(Address),                     // ANNN - LD I, addr
    JumpV0(Address),                    // BNNN - JP V0, addr
    Random(Register, u8),               // CXNN - RND Vx, byte
    Draw(Register, Register, u8),       // DXYN - DRW Vx, Vy, nibble

    SkipKeyPressed(Register),           // EX9E - SKP Vx
    SkipKeyNotPressed(Register),        // EXA1 - SKNP Vx

    LoadDelay(Register),                // FX07 - LD Vx, DT
    WaitKeyPress(Register),             // FX0A - LD Vx, K
    SetDelay(Register),                 // FX15 - LD DT, Vx
    SetSound(Register),                 // FX18 - LD ST, Vx
    AddI(Register),                     // FX1E - ADD I, Vx
    LoadFont(Register),                 // FX29 - LD F, Vx
    StoreBCD(Register),                 // FX33 - LD B, Vx
    StoreRegisters(Register),           // FX55 - LD [I], Vx
    LoadMemory(Register),               // FX65 - LD Vx, [I]
}



fn main() {
    if 2 == 0x1 {
    println!("Hello, world!");
    }
}

fn load_file(path: String) {
}


