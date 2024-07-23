pub mod cpu;
use cpu::{Registers, CPU};

fn main() {

    let cpu = CPU::new();

    let temp = cpu.registers.AF;

    println!("{temp:b}");

    dbg!(cpu);

    println!("Hello, world!");
}
