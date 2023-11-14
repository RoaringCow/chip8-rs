use std::io::Read;
use std::fs::File;
use std::io;
use rand::random;

mod instruction;
use crate::instruction::*;

mod display;
use crate::display::*;


use sdl2::event::Event;
use sdl2::{self, keyboard::Keycode};


const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;


const CHARACTERS: [[u8; 5]; 16] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0],
    [0x20, 0x60, 0x20, 0x20, 0x70],
    [0xF0, 0x10, 0xF0, 0x80, 0xF0],
    [0xF0, 0x10, 0xF0, 0x10, 0xF0],
    [0x90, 0x90, 0xF0, 0x10, 0x10],
    [0xF0, 0x80, 0xF0, 0x10, 0xF0],
    [0xF0, 0x80, 0xF0, 0x90, 0xF0],
    [0xF0, 0x10, 0x20, 0x40, 0x40],
    [0xF0, 0x90, 0xF0, 0x90, 0xF0],
    [0xF0, 0x90, 0xF0, 0x10, 0xF0],
    [0xF0, 0x90, 0xF0, 0x90, 0x90],
    [0xE0, 0x90, 0xE0, 0x90, 0xE0],
    [0xF0, 0x80, 0x80, 0x80, 0xF0],
    [0xE0, 0x90, 0x90, 0x90, 0xE0],
    [0xF0, 0x80, 0xF0, 0x80, 0xF0],
    [0xF0, 0x80, 0xF0, 0x80, 0x80]
    ];    
    
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
    
    

fn main() -> io::Result<()> {
    let x: OpCode = instruction::OpCode(0x8000);

    let emulator = Emulator::new();
    
    let emulator = emulator.read_rom("/home/ersan/Downloads/octojam1title.ch8")?;
    for byte in &emulator.memory {
        //println!("{}", byte);
    }

    /*
    for i in 0..16 {
        for j in 0..5 {
            for y in (0..8).rev() {
                let bit = (CHARACTERS[i][j] >> y) & 1;
                print!("{}", if bit == 1 {"$"} else {" "});
            }
            println!();
            //println!("{:b}", CHARACTERS[i][j]);
        }
        println!();
    }
    */


    


    Ok(())
}



pub struct Emulator {
    pub memory: [u8; 4096],      // 4K memory; 0x000 - 0xFFF
    v: [u8; 16],             // 16 8-bit registers; 0x0 - 0xF
    i: u16,                         // Memory address register
    pc: u16,                        // Program counter
    stack: [u16; 16],       // Stack; 16 levels of 16-bit values
    sp: u8,                         // Stack pointer; points to the top of the stack
    delay_timer: u8,
    sound_timer: u8,
    display: [[bool; SCREEN_WIDTH];SCREEN_HEIGHT],
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
            display: [[true; SCREEN_WIDTH]; SCREEN_HEIGHT],
            keyboard: [],
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
        self.pc = match instruction {
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
                // Will probably change this line from converting into u16 at some point.
                if self.v[regx] as u16 + self.v[regy] as u16 > 255 { self.v[0x0F] = 1 } else { self.v[0x0F] = 0 }
                self.v[regx] += self.v[regy];
                self.pc + 2
            },    
            Instruction::Sub(regx, regy) => {
                if self.v[regx] > self.v[regy] { self.v[0x0F] = 1 } else { self.v[0x0F] = 0 }
                self.v[regx] -= self.v[regy];
                self.pc + 2
            },    
            Instruction::ShiftRight(register) => {
                self.v[0x0F] = self.v[register] & 0x1;
                self.v[register] >>= 1;
                self.pc + 2
            },    
            Instruction::ReverseSub(regx, regy) => {
                if self.v[regy] > self.v[regx] { self.v[0x0F] = 1 } else { self.v[0x0F] = 0 }
                self.v[regx] = self.v[regy] - self.v[regx];
                self.pc + 2
            },    
            Instruction::ShiftLeft(register) => {
                self.v[0x0F] = self.v[register] & 128; // Most significant bit.
                self.v[register] <<= 1;
                self.pc + 2
            },    
            Instruction::SkipIfNotEqual(regx, regy) => {
                if self.v[regx] != self.v[regy] {
                    self.pc + 4
                } else {
                    self.pc + 2
                }    
            },    
            Instruction::LoadI(address) => {
                self.i = address;
                self.pc + 2
            },    
            Instruction::JumpPlusZero(addr) => addr + (self.v[0] as u16),

            Instruction::Random(x, val) => {
                self.v[x] = val & rand::random::<u8>();
                self.pc + 2
            },    

            Instruction::Draw(regx, regy, value) => {
                let coordx = self.v[regx] as usize;
                let coordy = self.v[regy] as usize;

                let mut collision = false;


                // Display the rows and collumns of char
                for row in 0..value {
                    let pixels = self.memory[(self.i + row as u16) as usize];
                    for col in 0..8 {
                        if (pixels & (0x80 >> col)) != 0 {
                            let x = (coordx + col) % SCREEN_WIDTH as usize;
                            let y = (coordy + row as usize) % SCREEN_HEIGHT as usize;
                            // to wrap around
                            collision |= self.display[y][x];
                            self.display[y][x] ^= true;
                        }
                    }
                }
                
                self.v[0xF] = collision as u8;
                self.pc + 2
            },
            
            Instruction::SkipIfPressed(x) => {

            }
            _ => 16,


            
            /*
            SkipIfPressed(Register),           // EX9E - SKP Vx
            SkipIfNotPressed(Register),        // EXA1 - SKNP Vx

            LoadDelayTimer(Register),           // FX07 - LD Vx, DT
            WaitForKeyPress(Register),          // FX0A - LD Vx, K
            SetDelayTimer(Register),            // FX15 - LD DT, Vx
            SetSoundTimer(Register),            // FX18 - LD ST, Vx
            AddI(Register),                     // FX1E - ADD I, Vx
            LoadSprite(Register),               // FX29 - LD F, Vx
            _ => 16,
            */
        };    

    }    
}    

// "/home/ersan/Downloads/octojam1title.ch8"



fn map_sdl_keys(key: Keycode) -> Option<u8> {
    match key {
		Keycode::Num1 => Some(0x1),
		Keycode::Num2 => Some(0x2),
		Keycode::Num3 => Some(0x3),
		Keycode::Num4 => Some(0xC),
		Keycode::Q => Some(0x4),
		Keycode::W => Some(0x5),
		Keycode::E => Some(0x6),
		Keycode::R => Some(0xD),
		Keycode::A => Some(0x7),
		Keycode::S => Some(0x8),
		Keycode::D => Some(0x9),
		Keycode::F => Some(0xE),
		Keycode::Z => Some(0xA),
		Keycode::X => Some(0x0),
		Keycode::C => Some(0xB),
		Keycode::V => Some(0xF),
		_ => None,
	}
}