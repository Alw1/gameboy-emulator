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
                // AF : 0x01B0, 
                // BC : 0x0013,
                // DE : 0x00D8,
                // HL : 0x014D,
                // SP : 0xFFFE,
                // PC : 0x0100,

                //Test values
                AF : 0x0000, 
                BC : 0x0000,
                DE : 0x0000,
                HL : 0x0000,
                SP : 0x0000,
                PC : 0x1000,
        }
    }

    // //execute an instruction
    // pub fn execute_instruction(&mut self) {
    //     let mut opcode = 0; 

    //     //Increment PC if prefixed instruction
    //     if opcode == 0xCB { 
    //         opcode = (self.PC + 1) + 0x100; // Shift look up table
    //         //call Cb prefixed handler function
    //     }

    //     // will return the number of cycles taken by the instruction
    //     //let cpu_cycles = process_opcode(opcode);

    //     //delay(cpu_cycles)
    //     self.opcode_handler(opcode);
    // }

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
            Register::F => self.AF & 0x00FF,
            Register::B => self.BC >> 8,
            Register::C => self.BC & 0x00FF,
            Register::D => self.DE >> 8,
            Register::E => self.DE & 0x00FF,
            Register::H => self.HL >> 8,
            Register::L => self.HL & 0x00FF,
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
            Register::A => self.AF = (self.AF & 0x00FF) | value << 8,
            Register::F => self.AF = (self.AF & 0xFF00) | value,
            Register::B => self.BC = (self.BC & 0x00FF) | value << 8,
            Register::C => self.BC = (self.BC & 0xFF00) | value,
            Register::D => self.DE = (self.DE & 0x00FF) | value << 8,
            Register::E => self.DE = (self.DE & 0xFF00) | value,
            Register::H => self.HL = (self.DE & 0x00FF) | value << 8,
            Register::L => self.HL = (self.DE & 0xFF00) | value,
            Register::AF => self.AF = value,
            Register::BC => self.BC = value,
            Register::DE => self.DE = value,
            Register::HL => self.HL = value,
            Register::SP => self.SP = value,
            Register::PC => self.PC = value,
        }
    }

    pub fn opcode_handler(&mut self, opcode : u8) -> u8{
        
        //Function will return number of cycles taken by instruciton

        // Temp variable until I figure out memory shit

        //TODO: Make functions to pull opcode arguments from memory if called
        //      Make function to manage cpu cycles required for each opcode
        //      Add styff for interrupts

        let get_byte = 0x0; //Placeholder til memory exists to pull data after opcode
        
        match opcode {
            0x00 => {1}, // NOP
            0x01 => {
                self.set_register(get_byte, Register::BC); 
                return 4
            }, // ld r16, imm16
            0x02 => {
                let val = self.get_register(Register::B) + self.get_register(Register::C);
                self.set_register(val, Register::A);
                return 12;
            }, // LD BC, A
            0x03 => {
                let val = self.get_register(Register::BC) + 0x1;
                self.set_register(val, Register::BC);
                return 8;
            }, // INC BC
            0x04 => {
                let val = self.get_register(Register::B) + 0x1;
                self.set_register(val, Register::B);
                self.set_flag(Flag::Z);
                self.set_flag(Flag::N);
                self.set_flag(Flag::H);
                return 4;
            }, // INC B
            0x05 => {
                let val = self.get_register(Register::B) -1;
                self.set_register(val, Register::B);
                self.set_flag(Flag::Z);
                self.clear_flag(Flag::N);
                self.set_flag(Flag::H);
                return 4;
            }, // DEC B
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
            0x22 => 90, // LD HL A
            0x23 => 90, // INC HL
            0x24 => 90, // INC H
            0x25 => 90, // DEC H
            0x26 => 90, // LD H n8
            0x27 => 90, // DAA 
            0x28 => 90, //  JR Z e8
            0x29 => 90, // ADD HL HL
            0x2A => 90, // LD A HL
            _    => {panic!("ERROR: Invalid Opcode");}
       }
        // return 10
    }
    
    
}

