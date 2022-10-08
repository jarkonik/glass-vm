use crate::cpu::{EncodeTwoBytes, Op, OpPlace, Reg};
use phf::phf_map;

const MOV_NAME: &str = "mov";
const ADD_NAME: &str = "add";
const JMP_NAME: &str = "jmp";
const JE_NAME: &str = "je";
const JNE_NAME: &str = "jne";
const CMP_NAME: &str = "cmp";
const INT_NAME: &str = "int";

static REGISTERS: phf::Map<&'static str, Reg> = phf_map! {
    "A" => Reg::A,
    "B" => Reg::B,
    "C" => Reg::C,
};

fn parse_src(src: Option<&str>) -> OpPlace {
    match src {
        Some(x) if REGISTERS.get(x.trim()).is_some() => {
            OpPlace::Reg(*REGISTERS.get(x.trim()).unwrap())
        }
        Some(src) if src.trim().parse::<u8>().is_ok() => OpPlace::Imm(src.parse::<u8>().unwrap()),
        Some(src) => panic!("Unknown source {}", src),
        None => panic!("No source given for {}", MOV_NAME),
    }
}

fn parse_dst(dst: Option<&str>) -> Reg {
    match dst {
        Some(x) if REGISTERS.get(x.trim()).is_some() => *REGISTERS.get(x.trim()).unwrap(),
        Some(src) => panic!("Unknown dst {}", src),
        None => panic!("No source given for {}", MOV_NAME),
    }
}

pub fn assembly(str: &str) -> Vec<u16> {
    // let mut labels = HashMap::new();

    let commands = str
        .split("\n")
        .filter_map(|line| {
            if line.trim().len() == 0 {
                return None;
            } else {
                return Some(line.trim());
            }
        })
        .filter_map(|line| {
            let mut iter = line.splitn(3, ' ');
            let command = iter.next().expect("No command");
            let src = iter.next();
            let dst = iter.next();

            match command.trim() {
                MOV_NAME => Some(Op::Mov(parse_src(src), parse_dst(dst))),
                ADD_NAME => Some(Op::Add(parse_src(src), parse_dst(dst))),
                CMP_NAME => Some(Op::Cmp(parse_src(src), parse_dst(dst))),
                JMP_NAME => Some(Op::Jmp(parse_src(src))),
                JE_NAME => Some(Op::Je(parse_src(src))),
                JNE_NAME => Some(Op::Jne(parse_src(src))),
                INT_NAME => Some(Op::Int(parse_src(src))),
                // x if x.ends_with(":") => {
                //     labels.insert(x, Label(0xEF));
                //     None
                // }
                _ => panic!("Unknown command {}", command),
            }
        });

    commands.map(|command| command.encode()).collect()
}

#[test]
fn test_assembly_mov_immediate_to_register() {
    let upper = assembly("mov 154 B")[0];

    // opcode
    assert!(upper & (1 << 15) == 0);
    assert!(upper & (1 << 14) == 0);
    assert!(upper & (1 << 13) == 0);
    assert!(upper & (1 << 12) == 0);

    // dst
    assert!(upper & (1 << 11) == 0);
    assert!(upper & (1 << 10) == 0);
    assert!(upper & (1 << 9) != 0);

    // immediate bit
    assert!(upper & (1 << 8) != 0);

    // immediate value
    assert!(upper & (1 << 7) != 0);
    assert!(upper & (1 << 6) == 0);
    assert!(upper & (1 << 5) == 0);
    assert!(upper & (1 << 4) != 0);
    assert!(upper & (1 << 3) != 0);
    assert!(upper & (1 << 2) == 0);
    assert!(upper & (1 << 1) != 0);
    assert!(upper & (1 << 0) == 0);
}

#[test]
fn test_assembly_mov_register_to_register() {
    let upper = assembly("mov A B")[0];

    // opcode
    assert!(upper & (1 << 15) == 0);
    assert!(upper & (1 << 14) == 0);
    assert!(upper & (1 << 13) == 0);
    assert!(upper & (1 << 12) == 0);

    // dst
    assert!(upper & (1 << 11) == 0);
    assert!(upper & (1 << 10) == 0);
    assert!(upper & (1 << 9) != 0);

    // immediate bit
    assert!(upper & (1 << 8) == 0);

    // src
    assert!(upper & (1 << 7) == 0);
    assert!(upper & (1 << 6) == 0);
    assert!(upper & (1 << 5) == 0);
    assert!(upper & (1 << 4) == 0);
    assert!(upper & (1 << 3) == 0);
    assert!(upper & (1 << 2) == 0);
    assert!(upper & (1 << 1) == 0);
    assert!(upper & (1 << 0) == 0);
}

#[test]
fn test_assembly_add_immediate_to_register() {
    let upper = assembly("add 12 B")[0];

    // opcode
    assert!(upper & (1 << 15) == 0);
    assert!(upper & (1 << 14) == 0);
    assert!(upper & (1 << 13) == 0);
    assert!(upper & (1 << 12) != 0);

    // dst
    assert!(upper & (1 << 11) == 0);
    assert!(upper & (1 << 10) == 0);
    assert!(upper & (1 << 9) != 0);

    // immediate bit
    assert!(upper & (1 << 8) != 0);

    // src
    assert!(upper & (1 << 7) == 0);
    assert!(upper & (1 << 6) == 0);
    assert!(upper & (1 << 5) == 0);
    assert!(upper & (1 << 4) == 0);
    assert!(upper & (1 << 3) != 0);
    assert!(upper & (1 << 2) != 0);
    assert!(upper & (1 << 1) == 0);
    assert!(upper & (1 << 0) == 0);
}

#[test]
fn test_assembly_add_register_to_register() {
    let upper = assembly("add A B")[0];

    // opcode
    assert!(upper & (1 << 15) == 0);
    assert!(upper & (1 << 14) == 0);
    assert!(upper & (1 << 13) == 0);
    assert!(upper & (1 << 12) != 0);

    // dst
    assert!(upper & (1 << 11) == 0);
    assert!(upper & (1 << 10) == 0);
    assert!(upper & (1 << 9) != 0);

    // immediate bit
    assert!(upper & (1 << 8) == 0);

    // src
    assert!(upper & (1 << 7) == 0);
    assert!(upper & (1 << 6) == 0);
    assert!(upper & (1 << 5) == 0);
    assert!(upper & (1 << 4) == 0);
    assert!(upper & (1 << 3) == 0);
    assert!(upper & (1 << 2) == 0);
    assert!(upper & (1 << 1) == 0);
    assert!(upper & (1 << 0) == 0);
}
