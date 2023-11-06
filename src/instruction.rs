// Added types to read the instructions more easily.
pub type Address = u16;
pub type Register = usize;

impl Opcode {
    // Return 0x0X00 from opcode
    fn oxoo(&self) -> usize {
        ((self.0 & 0x0F00) >> 8) as usize
    }

    // Return 0x00Y0 from opcode
    fn ooyo(&self) -> usize {
        ((self.0 & 0x00F0) >> 4) as usize
    }

    // Return 0x000N from opcode
    fn ooon(&self) -> u8 {
        (self.0 & 0x000F) as u8
    }

    // Return 0x00NN from opcode
    fn oonn(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    // Return 0x0NNN from opcode
    fn onnn(&self) -> u16 {
        self.0 & 0x0FFF
    }
}


// Instructions for chip-8
pub enum Instruction {
    ClearDisplay, // 00E0 - CLS
    Return,       // 00EE - RET

    Jump(Address),                      // 1NNN - JP addr
    Call(Address),                      // 2NNN - CALL addr
    SkipEqual(Register, u8),            // 3XNN - SE Vx, byte
    SkipNotEqual(Register, u8),         // 4XNN - SNE Vx, byte
    SkipEqualXY(Register, Register),    // 5XY0 - SE Vx, Vy
    Load(Register, u8),                 // 6XNN - LD Vx, byte
    Add(Register, u8),                  // 7XNN - ADD Vx, byte

    Move(Register, Register),           // 8XY0 - LD Vx, Vy
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

impl Instruction {

    pub fn new(opcode: OpCode) {
        match opcode.0 & 0xF000 {
            0x0000 => match opcode.0.ooon() {
                0x0000 => Some(Instruction::ClearDisplay),
                0x000E => Some(Instruction::Return)
            }
            0x1000 => Some(Instruction::Jump(opcode.onnn())),
            0x2000 => Some(Instruction::Call(opcode.onnn())),
            0x3000 => Some(Instruction::SkipEqual(opcode.oxoo, opcode.oonn)),
            0x4000 => Some(Instruction::SkipNotEqual(opcode.oxoo, opcode.onnn)),
            0x5000 => Some(Instruction::SkipEqualXY(opcode.oxoo, opcode.ooyo)),
            0x6000 => Some(Instruction::Load(opcode.oxoo, opcode.oonn)),
            0x7000 => Some(Instruction::Add(opcode.oxoo, opcode.oonn)),
        }
    }
}