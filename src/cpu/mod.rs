enum OpCode {
    Mov = 0x00,
    Add = 0x01,
    Jmp = 0x02,
    Sub = 0x03,
    Cmp = 0x04,
    Je = 0x05,
    Jne = 0x06,
    Int = 0x07,
}

#[derive(Debug, Clone, Copy)]
pub enum Reg {
    A = 0x00,
    B = 0x01,
    C = 0x02,
}

#[derive(Debug)]
pub enum OpPlace {
    Reg(Reg),
    Imm(u8),
}

trait EncodeByte {
    fn encode(&self) -> u8;
}
impl EncodeByte for Reg {
    fn encode(&self) -> u8 {
        *self as u8
    }
}

impl EncodeByte for OpPlace {
    fn encode(&self) -> u8 {
        match self {
            OpPlace::Reg(reg) => reg.encode(),
            OpPlace::Imm(imm) => *imm,
        }
    }
}

#[derive(Debug)]
pub enum Op {
    Mov(OpPlace, Reg),
    Add(OpPlace, Reg),
    Sub(OpPlace, Reg),
    Cmp(OpPlace, Reg),
    Jmp(OpPlace),
    Je(OpPlace),
    Jne(OpPlace),
    Int(OpPlace),
}

pub trait EncodeTwoBytes {
    fn encode(&self) -> u16;
}

impl EncodeTwoBytes for Op {
    fn encode(&self) -> u16 {
        let src = match self {
            Op::Mov(src, _) => src,
            Op::Add(src, _) => src,
            Op::Sub(src, _) => src,
            Op::Cmp(src, _) => src,
            Op::Jmp(src) => src,
            Op::Je(src) => src,
            Op::Jne(src) => src,
            Op::Int(src) => src,
        };

        let dst = match self {
            Op::Mov(_, dst) => Some(dst),
            Op::Add(_, dst) => Some(dst),
            Op::Sub(_, dst) => Some(dst),
            Op::Cmp(_, dst) => Some(dst),
            Op::Jmp(_) => None,
            Op::Je(_) => None,
            Op::Jne(_) => None,
            Op::Int(_) => None,
        };

        let mut encoded: u16 = 0;

        // opcode(0-3)
        encoded |= match self {
            Op::Mov(_, _) => OpCode::Mov as u16,
            Op::Add(_, _) => OpCode::Add as u16,
            Op::Sub(_, _) => OpCode::Sub as u16,
            Op::Cmp(_, _) => OpCode::Cmp as u16,
            Op::Jmp(_) => OpCode::Jmp as u16,
            Op::Je(_) => OpCode::Je as u16,
            Op::Jne(_) => OpCode::Jne as u16,
            Op::Int(_) => OpCode::Int as u16,
        } as u16;

        // dst(4-6)
        encoded <<= 3;
        if let Some(dst) = dst {
            encoded |= dst.encode() as u16;
        }

        match src {
            OpPlace::Imm(_) => {
                // immediate bit(7)
                encoded <<= 1;
                encoded |= 1;

                // src(8-15)
                encoded <<= 8;
                encoded |= src.encode() as u16;
            }
            OpPlace::Reg(_) => {
                // immediate bit(7)
                encoded <<= 1;
                encoded |= 0;

                // padding(8-13) + src(13-15)
                encoded <<= 8;
                encoded |= src.encode() as u16;
            }
        }
        encoded
    }
}

#[derive(Default)]
pub struct CPU {
    registers: [u8; 8],
    pc: u8,
    zero_flag: bool,
    carry_flag: bool,
}

pub trait Run {
    fn run(&mut self, program: &[u16]);
}

trait Execute {
    fn execute(&mut self, op: &Op);
}

impl CPU {
    pub fn pc(&self) -> u8 {
        self.pc
    }
}

impl Execute for CPU {
    fn execute(&mut self, op: &Op) {
        match op {
            Op::Mov(src, dst) => {
                let src = match src {
                    OpPlace::Reg(reg) => self.registers[*reg as usize],
                    OpPlace::Imm(val) => *val,
                };
                self.registers[*dst as usize] = src;
            }
            Op::Add(src, dst) => {
                let src = match src {
                    OpPlace::Reg(reg) => self.registers[*reg as usize],
                    OpPlace::Imm(val) => *val,
                };
                self.registers[*dst as usize] += src;
            }
            Op::Jmp(src) => {
                let src = match src {
                    OpPlace::Reg(reg) => self.registers[*reg as usize],
                    OpPlace::Imm(val) => *val,
                };
                self.pc = src as u8;
            }
            Op::Je(src) => {
                let src = match src {
                    OpPlace::Reg(reg) => self.registers[*reg as usize],
                    OpPlace::Imm(val) => *val,
                };
                if self.zero_flag {
                    self.pc = src as u8;
                }
            }
            Op::Jne(src) => {
                let src = match src {
                    OpPlace::Reg(reg) => self.registers[*reg as usize],
                    OpPlace::Imm(val) => *val,
                };
                if !self.zero_flag {
                    self.pc = src as u8;
                }
            }
            Op::Int(src) => {
                let src = match src {
                    OpPlace::Reg(reg) => self.registers[*reg as usize],
                    OpPlace::Imm(val) => *val,
                };

                match src {
                    0 => print!("{}", self.registers[Reg::B as usize] as char),
                    _ => panic!("Unsupported interrupt"),
                }
            }
            Op::Sub(src, dst) => {
                let src = match src {
                    OpPlace::Reg(reg) => self.registers[*reg as usize],
                    OpPlace::Imm(val) => *val,
                };
                let (result, borrow) = self.registers[*dst as usize].borrowing_sub(src, false);
                self.carry_flag = borrow;
                self.zero_flag = result == 0;
                self.registers[*dst as usize] = result;
            }
            Op::Cmp(src, dst) => {
                let src = match src {
                    OpPlace::Reg(reg) => self.registers[*reg as usize],
                    OpPlace::Imm(val) => *val,
                };
                let (result, borrow) = self.registers[*dst as usize].borrowing_sub(src, false);
                self.carry_flag = borrow;
                self.zero_flag = result == 0;
            }
        }
    }
}

pub trait Set {
    fn set(&mut self, reg: &Reg, value: u8);
}

impl Set for CPU {
    fn set(&mut self, reg: &Reg, value: u8) {
        self.registers[*reg as usize] = value;
    }
}

pub trait Get {
    fn get(&mut self, reg: &Reg) -> u8;
}
impl Get for CPU {
    fn get(&mut self, reg: &Reg) -> u8 {
        self.registers[*reg as usize]
    }
}

impl Run for CPU {
    fn run(&mut self, ops: &[u16]) {
        self.pc = 0;
        loop {
            if self.pc as usize >= ops.len() {
                break;
            }
            let mut op = Op::decode(ops[self.pc as usize]);
            self.pc += 1;
            self.execute(&mut op);
        }
    }
}

impl Op {
    fn decode_src(op: u16) -> OpPlace {
        if op & (0x00000001 << 8) != 0 {
            OpPlace::Imm((op & 0xFF) as u8)
        } else {
            OpPlace::Reg(Reg::decode(op & 0b111))
        }
    }

    fn decode_dst(op: u16) -> Reg {
        Reg::decode((op & (0b0000111 << 9)) >> 9)
    }

    fn decode(op: u16) -> Op {
        match op >> 12 {
            x if x == OpCode::Mov as u16 => Op::Mov(Op::decode_src(op), Op::decode_dst(op)),
            x if x == OpCode::Add as u16 => Op::Add(Op::decode_src(op), Op::decode_dst(op)),
            x if x == OpCode::Sub as u16 => Op::Sub(Op::decode_src(op), Op::decode_dst(op)),
            x if x == OpCode::Cmp as u16 => Op::Cmp(Op::decode_src(op), Op::decode_dst(op)),
            x if x == OpCode::Jmp as u16 => Op::Jmp(Op::decode_src(op)),
            x if x == OpCode::Je as u16 => Op::Je(Op::decode_src(op)),
            x if x == OpCode::Jne as u16 => Op::Jne(Op::decode_src(op)),
            x if x == OpCode::Int as u16 => Op::Int(Op::decode_src(op)),
            _ => panic!("Unknown opcode"),
        }
    }
}

impl Reg {
    fn decode(op: u16) -> Reg {
        match op {
            x if x == Reg::A as u16 => Reg::A,
            x if x == Reg::B as u16 => Reg::B,
            x if x == Reg::C as u16 => Reg::C,
            _ => panic!("Error: Unknown register"),
        }
    }
}

#[test]
fn test_mov_register_to_register() {
    let mut cpu = CPU::default();
    cpu.set(&Reg::A, 0xEF);

    cpu.execute(&Op::Mov(OpPlace::Reg(Reg::A), Reg::B));

    assert_eq!(cpu.get(&Reg::B), 0xEF);
}

#[test]
fn test_mov_immediate_to_register() {
    let mut cpu = CPU::default();

    cpu.execute(&Op::Mov(OpPlace::Imm(0xEF), Reg::B));

    assert_eq!(cpu.get(&Reg::B), 0xEF);
}

#[test]
fn test_add_immediate_to_register() {
    let mut cpu = CPU::default();
    cpu.set(&Reg::B, 0x02);

    cpu.execute(&Op::Add(OpPlace::Imm(0x01), Reg::B));

    assert_eq!(cpu.get(&Reg::B), 0x03);
}

#[test]
fn test_add_reguster_to_register() {
    let mut cpu = CPU::default();

    cpu.set(&Reg::A, 0x01);
    cpu.set(&Reg::B, 0x02);

    cpu.execute(&Op::Add(OpPlace::Reg(Reg::A), Reg::B));

    assert_eq!(cpu.get(&Reg::B), 0x03);
}

#[test]
fn test_sub_immediate_from_register() {
    let mut cpu = CPU::default();

    cpu.set(&Reg::A, 0x08);

    cpu.execute(&Op::Sub(OpPlace::Imm(0x05), Reg::A));

    assert_eq!(cpu.get(&Reg::A), 0x03);
}

#[test]
fn test_sub_register_from_register() {
    let mut cpu = CPU::default();

    cpu.set(&Reg::A, 0x08);
    cpu.set(&Reg::B, 0x05);

    cpu.execute(&Op::Sub(OpPlace::Reg(Reg::B), Reg::A));

    assert_eq!(cpu.get(&Reg::A), 0x03);
}
