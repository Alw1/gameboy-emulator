pub enum Flag {
    Z = 0x0080,
    N = 0x0040,
    H = 0x0020,
    C = 0x0010,
}

#[derive(Debug)]
pub struct Registers {
    pub AF: u16, //Accumulator & Flags
    pub BC: u16, //Registers B & C
    pub DE: u16, //Registers D & E
    pub HL: u16, //Registers H & L
    pub SP: u16, //Stack Pointer
    pub PC: u16, //Program Counter
}

impl Registers {
    pub fn new() -> Self {
        //Initialize registers to start addresses
        Registers {
            AF : 0x0100, 
            BC : 0x0013,
            DE : 0x00D8,
            HL : 0x014D,
            SP : 0xFFFE,
            PC : 0x0100,
        }
    }

    pub fn set_flag(&mut self, flag: Flag) {
        self.AF |= flag as u16; 
    }

    pub fn clear_flag(&mut self, flag: Flag) {
        self.AF &= (flag as u16) ^ 0xFFFF;
    }
}

#[derive(Debug)]
pub struct CPU {
    pub registers : Registers,
 
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers : Registers::new(),

        }
    }

    //execute an instruction
    pub fn execute(&mut self, instruction : u16) {

    }
}

fn instruction_handler(instruction : u16) -> u16 {

    let opcode: u16 = (instruction & 0xF000) >> 12;

    match instruction {
        0x77 => 1023,
        _    => 1023
    }
     
}