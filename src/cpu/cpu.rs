// use crate::cpu::instructions;

use std::collections::btree_map::ValuesMut;

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
    AF,
    BC,
    DE,
    HL,
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
    pub fn execute_instruction(&mut self) {
        let mut opcode = self.PC; 

        //Increment PC if prefixed instruction
        if opcode == 0xCB { 
            opcode = (self.PC + 1) + 0x100; // Shift look up table
        }

        opcode_handler(opcode);
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
            Register::AF => self.AF,
            Register::BC => self.BC,
            Register::DE => self.DE,
            Register::HL => self.HL,
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
            Register::AF => self.AF = value,
            Register::BC => self.BC = value,
            Register::DE => self.DE = value,
            Register::HL => self.HL = value,
            Register::SP => self.SP = value,
            Register::PC => self.PC = value,
        }
    }

    pub fn opcode_handler(&mut self, opcode : u8) {

        // Temp variable until I figure out memory shit

        //TODO: Make functions to pull opcode arguments from memory if called
        //      Make function to manage cpu cycles required for each opcode
        //      Add styff for interrupts

        let get_byte = 0x0;
        
        match opcode {
            0x00 => {1}, // NOP
            0x01 => {self.set_register(get_byte, Register::BC)}, // ld r16, imm16
            0x02 => {1}, // LD BC, A
            0x03 => 1, // INC BC
            0x04 => 1, // INC B
            0x05 => 1, // DEC B
            0x06 => 1, // LD B n8
            0x07 => 1, // RLCA 
            0x08 => 1, // LD a16 SP
            0x09 => 1, // ADD HL BC
            0x0A => 1, // LD A BC
            0x0B => 1, // DEC BC
            0x0C => 1, // INC C
            0x0D => 1, // DEC C
            0x0E => 1, // LD C n8
            0x0F => 1, // RRCA 
            0x10 => 90, // STOP
            0x11 => 90, // LD DE 16
            0x12 => 90, // LD DE A
            0x13 => 90, // INC DE
            0x14 => 90, // INC D
            0x15 => 90, // DEC D
            0x17 => 90, // RLA
            0x18 => 90, // JR e8
            0x19 => 90, // ADD HL DE
            0x1A => 90, // LD A DE
            0x1B => 90, // DEC DE
            0x1C => 90, // INC E
            0x1D => 90, // DEC E
            0x1E => 90, // LD E n8
            0x1F => 90, // RRA
            0x20 => 90, // JR NZ e8
            0x21 => 90, // LD HL n16
            0x22 => 90, //
            _    => {panic!("ERROR: Invalid Opcode");}
       }
        // return 10
    }
    
    
}

