use std::io::Read;
use std::fs::File;
use std::io;


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
        let opcode: OpCode = instruction::OpCode((self.memory[self.pc as usize] as u16) | (self.memory[(self.pc + 1) as usize] as u16));
        // 16 bit oku
        Instruction::new(opcode)
    }

    fn run_instruction(&mut self, instruction: Instruction) {
        self.pc = match instruction{
            //Instruction::ClearDisplay => todo!(), //clear display
            Instruction::Return => {
                // Set the program counter to return position
                self.sp -= 1;
                self.stack[self.sp as usize] + 2
            }
            Instruction::Jump(address) => address,
            Instruction::Call(address) => {
                // go to an adress but to return.
                self.stack[self.sp as usize] = self.pc as u16;
                self.sp += 1;
                address
            }
            Instruction::SkipIfEqualsByte(register, value) => {
                if self.v[register] == value {
                    self.pc + 4
                } else {
                    self.pc + 2
                }
            }
            Instruction::SkipIfNotEqualsByte(register, value) => {
                if self.v[register] != value {
                    self.pc + 4
                } else {
                    self.pc + 2
                }
            },
            Instruction::SkipIfEqual(regx, regy) => {
                if self.v[regx] == self.v[regy] {
                    self.pc + 4
                } else {
                    self.pc + 2
                }
            },
            Instruction::LoadByte(register, value) => {
                self.v[register] = value;
                self.pc + 2
            },
            Instruction::AddByte(register, value) => {
                self.v[register] = self.v[register] + value;
                self.pc + 2
            },
            Instruction::Move(regx, regy) => {
                self.v[regx] = self.v[regy];
                self.pc + 2
            },
            Instruction::Or(regx, regy) => {
                self.v[regx] = self.v[regx] | self.v[regy];
                self.pc + 2
            },
            Instruction::And(regx, regy) => {
                self.v[regx] = self.v[regx] & self.v[regy];
                self.pc + 2
            },
            Instruction::Xor(regx, regy) => {
                self.v[regx] = self.v[regx] ^ self.v[regy];
                self.pc + 2
            },
            Instruction::Add(regx, regy) => {
                let (res, overflow) = self.v[x].overflowing_add(self.v[y]);
                if self.v[regx] + self.v[regy] > 255 { self.v[0x0F] = 1 } else { self.v[0x0F] = 0 }
                self.v[regx] += self.v[regy];
                self.pc + 2
            },
            Instruction::Sub(regx, regy) => {
                if self.v[regx] > self.v[regy] { self.v[0x0F] = 1 } else { self.v[0x0F] = 0 }
                self.v[regx] -= self.v[regy];
                self.pc + 2
            },
            Instruction::ShiftRight(register) {
                self.v[0x0F] = self.v[x] & 0x1;
                self.v[x] >>= 1;
                self.pc + 2
            },

            Add(Register, Register),          // 8XY4 - ADD Vx, Vy
            Sub(Register, Register),          // 8XY5 - SUB Vx, Vy
            ShiftRight(Register),               // 8XY6 - SHR Vx {, Vy}
            ReverseSub(Register, Register),          // 8XY7 - SUBN Vx, Vy
            ShiftLeft(Register),                // 8XYE - SHL Vx {, Vy}

        };

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

