// use crate::cpu::instructions;

const UPPER_WORD_MASK : u16 = 0x00FF;
const LOWER_WORD_MASK : u16 = 0x00FF;

pub enum Flag {
    Z = 0x0080, // Zero Flag
    N = 0x0040, // Subtract Flag
    H = 0x0020, // Half Carry Flag
    C = 0x0010, // Carry Flag
}

pub enum Register{
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
    SP,
    PC,
}


#[derive(Debug)]
pub struct CPU {
    //Registers
    pub AF: u16, //Accumulator & Flags
    pub BC: u16, //Registers B & C
    pub DE: u16, //Registers D & E
    pub HL: u16, //Registers H & L
    pub SP: u16, //Stack Pointer
    pub PC: u16, //Program Counter
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
                //Initial Register values
                AF : 0x01B0, 
                BC : 0x0013,
                DE : 0x00D8,
                HL : 0x014D,
                SP : 0xFFFE,
                PC : 0x0100,
        }
    }

    //execute an instruction
    pub fn execute(&mut self, instruction : u16) {

    }

    //Provide debug info for TUI
    pub fn get_state(&mut self) {



    }

    pub fn set_flag(&mut self, flag: Flag) {
        self.AF |= flag as u16; 
    }

    pub fn clear_flag(&mut self, flag: Flag) {
        self.AF &= (flag as u16) ^ 0xFFFF;
    }

    pub fn get_register(&self, register: Register) -> u16{
        match register {
            Register::A => self.AF >> 8,
            Register::F => self.AF & 0xFF00,
            Register::B => self.BC >> 8,
            Register::C => self.BC & 0xFF00,
            Register::D => self.DE >> 8,
            Register::E => self.DE & 0xFF00,
            Register::H => self.HL >> 8,
            Register::L => self.HL & 0xFF00,
            Register::SP => self.SP,
            Register::PC => self.PC,
        }
    }

    pub fn set_register(&mut self, value: u16, register: Register){
        match register {
            Register::A => self.AF = (self.AF & 0x00FF) | value,
            Register::F => self.AF = (self.AF & 0xFF00) | value,
            Register::B => self.BC = (self.BC & 0x00FF) | value,
            Register::C => self.BC = (self.BC & 0xFF00) | value,
            Register::D => self.DE = (self.DE & 0x00FF) | value,
            Register::E => self.DE = (self.DE & 0x00FF) | value,
            Register::H => self.HL = (self.DE & 0x00FF) | value,
            Register::L => self.HL = (self.DE & 0xFF00) | value,
            Register::SP => self.SP = value,
            Register::PC => self.PC = value,
        }
    }
}

