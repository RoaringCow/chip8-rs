use std::io::Read;
use std::fs::File;
use std::io;
use rand::random;
use sdl2::sys::register_t;

mod instruction;
use crate::instruction::*;

mod display;
use crate::display::*;

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;


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

    let mut emulator = Emulator::new();
    
    emulator = emulator.read_rom("/home/ersan/Downloads/life.ch8")?;
    
    let (mut display, sdl_context) = Display::new();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        // Emulator cycle
        let byte = (emulator.memory[emulator.pc as usize] as u16)  << 8 | (emulator.memory[(emulator.pc + 1) as usize] as u16); 
        
        
        // This part may lead to some issues. (The subtract 4 part)
        // But otherwise it would just give index out of bounds error.
        if emulator.pc > emulator.memory.len() as u16 - 4 {
            break;
        }
        
        //println!("{:?}    {}    {}  ", emulator.read_instruction(),  emulator.pc, byte);
        
        emulator.run_instruction(emulator.read_instruction());
        emulator.timer_ticks();
        
        /*
        for y in &emulator.display{
            for x in y {
                if !!x {print!("$");} else {print!(" ");}
            }
            println!();
        }
        */
        
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                // Handle key presses
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if let Some(key) = map_keys(key) {
                        emulator.key_down(key);
                    }
                }
                // Handle key releases
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(key) = map_keys(key) {
                        emulator.key_up(key);
                    }
                }
                _ => {}
            }
        }
        if emulator.draw_flag {
            display.draw_screen(&emulator.display);
        }
        
        // adjust as needed
        std::thread::sleep(std::time::Duration::from_micros(1000));

    }


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
    keys: [bool; 16],
    draw_flag: bool,
}    

impl Emulator {

    pub fn new() -> Emulator {
        let mut emulator = Emulator {
            memory: [0x0; 4096],
            v: [0; 16],
            i: 0x200,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            display: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
            keys: [true; 16],
            draw_flag: true,
        };    


        emulator
    }    

    pub fn key_down(&mut self, key: u8) {
        self.keys[key as usize] = true;
    }

    pub fn key_up(&mut self, key: u8) {
        self.keys[key as usize] = false;
    }
    pub fn timer_ticks(&mut self) {
        // Decrement delay timer if it's greater than zero every tick
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP");
            }
            self.sound_timer -= 1;
        }
    }

    pub fn read_rom<P: std::convert::AsRef<std::path::Path>>(mut self, path: P) -> io::Result<Emulator> {
        let file = File::open(path)?;
        for (location, byte) in file.bytes().enumerate() {
            self.memory[0x200 + location] = byte?;
        }    
        Ok(self)
    }    
    
    fn read_instruction(&self) -> Option<Instruction> {
        let opcode: OpCode = instruction::OpCode((self.memory[self.pc as usize] as u16) << 8 | (self.memory[(self.pc + 1) as usize] as u16));
        // 16 bit oku
        Instruction::new(opcode)
    }    

    fn run_instruction(&mut self, instruction: Option<Instruction>) {
        // println!("{}   {}   {}   {}   {}   {}   {}   {}   {}   {}   : {}  {:?}", self.v[0], self.v[1], self.v[2], self.v[3], self.v[4], self.v[5], self.v[6], self.v[7], self.v[13], self.v[14], self.pc, instruction);
        self.pc = match instruction {
            Some(Instruction::ClearDisplay) => {self.display = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT]; self.pc + 2}, //clear display
            Some(Instruction::Return) => {
                // Set the program counter to return position
                self.sp -= 1;
                self.stack[self.sp as usize] + 2
            },
            Some(Instruction::Jump(address)) => address,
            Some(Instruction::Call(address)) => {
                // go to an adress but to return.
                self.stack[self.sp as usize] = self.pc as u16;
                self.sp += 1;
                address
            },  
            Some(Instruction::SkipIfEqualsByte(register, value)) => {
                if self.v[register] == value {
                    self.pc + 4
                } else {
                    self.pc + 2
                }    
            },    
            Some(Instruction::SkipIfNotEqualsByte(register, value)) => {
                if self.v[register] != value {
                    self.pc + 4
                } else {
                    self.pc + 2
                }    
            },    
            Some(Instruction::SkipIfEqual(regx, regy)) => {
                if self.v[regx] == self.v[regy] {
                    self.pc + 4
                } else {
                    self.pc + 2
                }    
            },    
            Some(Instruction::LoadByte(register, value)) => {
                self.v[register] = value;
                self.pc + 2
            },    
            Some(Instruction::AddByte(register, value)) => {
                self.v[register] = self.v[register] + value;
                self.pc + 2
            },    
            Some(Instruction::Move(regx, regy)) => {
                self.v[regx] = self.v[regy];
                self.pc + 2
            },    
            Some(Instruction::Or(regx, regy)) => {
                self.v[regx] = self.v[regx] | self.v[regy];
                self.pc + 2
            },    
            Some(Instruction::And(regx, regy))=> {
                self.v[regx] = self.v[regx] & self.v[regy];
                self.pc + 2
            },    
            Some(Instruction::Xor(regx, regy)) => {
                self.v[regx] = self.v[regx] ^ self.v[regy];
                self.pc + 2
            },    
            Some(Instruction::Add(regx, regy)) => {
                // Will probably change this line from converting into u16 at some point.
                if self.v[regx] as u16 + self.v[regy] as u16 > 255 { self.v[0x0F] = 1 } else { self.v[0x0F] = 0 }
                self.v[regx] += self.v[regy];
                self.pc + 2
            },    
            Some(Instruction::Sub(regx, regy)) => {
                if self.v[regx] > self.v[regy] { self.v[0x0F] = 1 } else { self.v[0x0F] = 0 }
                self.v[regx] -= self.v[regy];
                self.pc + 2
            },    
            Some(Instruction::ShiftRight(register)) => {
                self.v[0x0F] = self.v[register] & 0x1;
                self.v[register] >>= 1;
                self.pc + 2
            },    
            Some(Instruction::ReverseSub(regx, regy)) => {
                if self.v[regy] > self.v[regx] { self.v[0x0F] = 1 } else { self.v[0x0F] = 0 }
                self.v[regx] = self.v[regy] - self.v[regx];
                self.pc + 2
            },    
            Some(Instruction::ShiftLeft(register)) => {
                self.v[0x0F] = self.v[register] & 128; // Most significant bit.
                self.v[register] <<= 1;
                self.pc + 2
            },    
            Some(Instruction::SkipIfNotEqual(regx, regy)) => {
                if self.v[regx] != self.v[regy] {
                    self.pc + 4
                } else {
                    self.pc + 2
                }    
            },    
            Some(Instruction::LoadI(address)) => {
                self.i = address;
                self.pc + 2
            },    
            Some(Instruction::JumpPlusZero(addr)) => addr + (self.v[0] as u16),

            Some(Instruction::Random(x, val)) => {
                self.v[x] = val & rand::random::<u8>();
                self.pc + 2
            },    

            Some(Instruction::Draw(regx, regy, value)) => {
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

            Some(Instruction::SkipIfPressed(x)) => {
                let key = self.v[x];
                if self.keys[key as usize] {
                    self.pc + 2
                }else {
                   self.pc
                }
            },

            Some(Instruction::SkipIfNotPressed(x)) => {
                let key = self.v[x];
                if !self.keys[key as usize] {
                    self.pc + 2
                } else{
                    self.pc
                }
            },

            Some(Instruction::LoadDelayTimer(register)) => {
                self.v[register] = self.delay_timer;
                self.pc + 2
            },
            
            Some(Instruction::WaitForKeyPress(register)) => {
                let mut pressed = false;
                for (i, &key) in self.keys.iter().enumerate() {
                    if !!key {
                        self.v[register] = i as u8;
                        pressed = true;
                        break;
                    }
                }
                if pressed {
                    self.pc + 2
                }else {
                    self.pc
                }
            },

            Some(Instruction::SetDelayTimer(register)) => {
                self.delay_timer = self.v[register];
                self.pc + 2
            },

            Some(Instruction::SetSoundTimer(register)) => {
                self.sound_timer = self.v[register];
                self.pc + 2
            },
            
            Some(Instruction::AddI(register)) => {
                self.i += self.v[register] as u16;
                self.pc + 2
            },

            Some(Instruction::LoadSprite(register)) => {
                self.i = self.v[register] as u16 * 5;
                self.pc + 2
            },

            Some(Instruction::StoreBCD(register)) => {
                self.memory[self.i as usize] = self.v[register] / 100; // hundreds
                self.memory[self.i as usize + 1] = (self.v[register] / 10) % 10; // tens
                self.memory[self.i as usize + 2] = (self.v[register] % 100) % 10; // ones
                self.pc + 2
            },

            Some(Instruction::StoreRegisters(register)) => {
                for i in 0..=register {
                    self.memory[self.i as usize + i] = self.v[i];
                }
                self.pc + 2
            },

            Some(Instruction::LoadRegisters(register)) => {
                for i in 0..=register {
                    self.v[i] = self.memory[self.i as usize + i];
                }
                self.pc + 2
            },

            None => {
                eprintln!("Unsupported instruction: {:?}", instruction);
                self.pc
            }
        };    

    }    
}    

// "/home/ersan/Downloads/octojam1title.ch8"



fn map_keys(key: Keycode) -> Option<u8> {
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