#![feature(arbitrary_enum_discriminant)]

use crate::cpu::{Run, CPU};

mod asm;
mod cpu;

fn main() {
    let cpu = CPU::default();
    cpu.run();
    println!("Hello, world!");
}
