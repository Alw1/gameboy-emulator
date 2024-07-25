pub mod cpu;
pub mod debug;

use std::io;
use std::io::prelude::*;
use std::fs::File;


use cpu::cpu::*;


fn main() {

    let mut cpu = CPU::new();
    cpu.set_register(0x00, Register::B);
    cpu.set_register(0x29, Register::A);
    cpu.set_register(0x19, Register::PC);
    cpu.set_register(0x37, Register::C);
    cpu.opcode_handler(0x04);
    cpu.opcode_handler(0x04);
    cpu.opcode_handler(0x04);
    print_cpu_debug(&cpu);


    // println!("{temp:b}");

    // println!("{}",cpu.get_register(Register::F));
    // dbg!(cpu);

    // while true {
    //     //execute instruction
    //     // let mut input = String::new();

    //     // match io::stdin().read_line(&mut input) {
    //     //     Ok(_) => {
    //     //         let trimmed_input = input.trim();
    //     //         println!("You entered: {}", trimmed_input);
    //     //     }
    //     //     Err(e) => {
    //     //         eprintln!("Failed to read input: {}", e);
    //     //     }
    //     // }

    //     print_cpu_debug(&cpu);
    // }
}


fn print_cpu_debug (cpu : &CPU) {
    println!("--Registers--");
    println!("A: {:x}  F: {:x}", cpu.get_register(Register::A),cpu.get_register(Register::F));
    println!("B: {:x}   C: {:x}", cpu.get_register(Register::B),cpu.get_register(Register::C));
    println!("D: {:x}   E: {:x}", cpu.get_register(Register::D),cpu.get_register(Register::E));
    println!("H: {:x}   L: {:x}", cpu.get_register(Register::H),cpu.get_register(Register::L));
    println!("SP: {:x}       ", cpu.get_register(Register::SP));
    println!("PC: {:x}       ", cpu.get_register(Register::PC));
    println!("");
    println!("");
    println!("--Flags--");
    println!("Z: {:x}       ", cpu.get_register(Register::F)>>8);
    println!("N: {:x}       ", cpu.get_register(Register::F)>>7);
    println!("H: {:x}       ", cpu.get_register(Register::F)>>6);
    println!("C: {:x}       ", cpu.get_register(Register::F)>>5);
    println!("");
    println!("");
}


