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

    LoadDelayTimer(Register),                // FX07 - LD Vx, DT
    WaitForKeyPress(Register),             // FX0A - LD Vx, K
    SetDelayTimer(Register),                 // FX15 - LD DT, Vx
    SetSoundTimer(Register),                 // FX18 - LD ST, Vx
    AddI(Register),                     // FX1E - ADD I, Vx
    LoadSprite(Register),                 // FX29 - LD F, Vx
    StoreBCD(Register),                 // FX33 - LD B, Vx
    StoreRegisters(Register),           // FX55 - LD [I], Vx
    LoadRegisters(Register),               // FX65 - LD Vx, [I]
}

impl Instruction {

    pub fn new(opcode: OpCode) {
        match opcode.0 & 0xF000 {
            0x0000 => match opcode.ooon() {
                0x0000 => Some(Instruction::ClearDisplay),
                0x000E => Some(Instruction::Return),
                _ => None,
            },
            0x1000 => Some(Instruction::Jump(opcode.onnn())),
            0x2000 => Some(Instruction::Call(opcode.onnn())),
            0x3000 => Some(Instruction::SkipIfEqualsByte(opcode.oxoo(), opcode.oonn())),
            0x4000 => Some(Instruction::SkipIfNotEqualsByte(
                opcode.oxoo(),
                opcode.oonn(),
            )),
            0x5000 => Some(Instruction::SkipIfEqual(opcode.oxoo(), opcode.ooyo())),
            0x6000 => Some(Instruction::LoadByte(opcode.oxoo(), opcode.oonn())),
            0x7000 => Some(Instruction::AddByte(opcode.oxoo(), opcode.oonn())),
            0x8000 => match opcode.ooon() {
                0x0000 => Some(Instruction::Move(opcode.oxoo(), opcode.ooyo())),
                0x0001 => Some(Instruction::Or(opcode.oxoo(), opcode.ooyo())),
                0x0002 => Some(Instruction::And(opcode.oxoo(), opcode.ooyo())),
                0x0003 => Some(Instruction::Xor(opcode.oxoo(), opcode.ooyo())),
                0x0004 => Some(Instruction::Add(opcode.oxoo(), opcode.ooyo())),
                0x0005 => Some(Instruction::Sub(opcode.oxoo(), opcode.ooyo())),
                0x0006 => Some(Instruction::ShiftRight(opcode.oxoo())),
                0x0007 => Some(Instruction::ReverseSub(opcode.oxoo(), opcode.ooyo())),
                0x000E => Some(Instruction::ShiftLeft(opcode.oxoo())),
                _ => None,
            },
            0x9000 => Some(Instruction::SkipIfNotEqual(opcode.oxoo(), opcode.ooyo())),
            0xA000 => Some(Instruction::LoadI(opcode.onnn())),
            0xB000 => Some(Instruction::JumpPlusZero(opcode.onnn())),
            0xC000 => Some(Instruction::Random(opcode.oxoo(), opcode.oonn())),
            0xD000 => Some(Instruction::Draw(
                opcode.oxoo(),
                opcode.ooyo(),
                opcode.ooon(),
            )),
            0xE000 => match opcode.oonn() {
                0x009E => Some(Instruction::SkipIfPressed(opcode.oxoo())),
                0x00A1 => Some(Instruction::SkipIfNotPressed(opcode.oxoo())),
                _ => None,
            },
            0xF000 => match opcode.oonn() {
                0x0007 => Some(Instruction::LoadDelayTimer(opcode.oxoo())),
                0x000A => Some(Instruction::WaitForKeyPress(opcode.oxoo())),
                0x0015 => Some(Instruction::SetDelayTimer(opcode.oxoo())),
                0x0018 => Some(Instruction::SetSoundTimer(opcode.oxoo())),
                0x001E => Some(Instruction::AddI(opcode.oxoo())),
                0x0029 => Some(Instruction::LoadSprite(opcode.oxoo())),
                0x0033 => Some(Instruction::StoreBCD(opcode.oxoo())),
                0x0055 => Some(Instruction::StoreRegisters(opcode.oxoo())),
                0x0065 => Some(Instruction::LoadRegisters(opcode.oxoo())),
                _ => None,
            },
            _ => None,
        }
    }
}