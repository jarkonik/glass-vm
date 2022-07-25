#[repr(u8)]
enum OpCode {
    Mov = 0x00,
}

#[repr(u8)]
#[derive(Debug)]
pub enum Reg {
    A = 0x00,
    B = 0x01,
    C = 0x02,
    D = 0x03,
    E = 0x04,
    F = 0x05,
    G = 0x06,
    H = 0x07,
}

#[repr(u8)]
#[derive(Debug)]
pub enum OpPlace {
    Reg(Reg),
    Imm(u8),
}

#[repr(u16)]
#[derive(Debug)]
pub enum Op {
    Mov(OpPlace, Reg) = OpCode::Mov as u16,
}

pub trait Encode {
    fn encode(&self) -> u8;
}

impl Encode for Op {
    fn encode(&self) -> u8 {
        match self {
            Op::Mov(src, dst) => {
                todo!();
                // let mut encoded = 0;

                // encoded |= *src as u8;
                // encoded |= *dst as u8 << 4;
                // encoded
            }
        }
    }
}

#[derive(Default)]
pub struct CPU {
    registers: [u8; 8],
}

pub trait Run {
    fn run(&self);
}

trait Execute {
    fn execute(&mut self, op: &Op);
}

impl Execute for CPU {
    fn execute(&mut self, op: &Op) {
        match op {
            Op::Mov(src, dst) => {
                let src = match src {
                    OpPlace::Reg(reg) => self.registers[*reg as usize],
                    OpPlace::Imm(_) => todo!(),
                };
                self.registers[*dst as usize] = src;
            }
        }
    }
}

trait Set {
    fn set(&mut self, reg: &Reg, value: u8);
}

impl Set for CPU {
    fn set(&mut self, reg: &Reg, value: u8) {
        self.registers[*reg as usize] = value;
    }
}

trait Get {
    fn get(&mut self, reg: &Reg) -> u8;
}
impl Get for CPU {
    fn get(&mut self, reg: &Reg) -> u8 {
        self.registers[*reg as usize]
    }
}

impl Run for CPU {
    fn run(&self) {
        todo!()
    }
}

#[test]
fn test_move_to_register() {
    let mut cpu = CPU::default();
    cpu.set(&Reg::A, 0xEF);

    cpu.execute(&Op::Mov(OpPlace::Reg(Reg::A), Reg::B));

    assert_eq!(cpu.get(&Reg::B), 0xEF);
}
