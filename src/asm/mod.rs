use crate::cpu::{Encode, Op, OpPlace, Reg};

const MOV_NAME: &str = "mov";

const A_REG_NAME: &str = "A";

fn assembly(str: &str) -> Vec<u8> {
    let commands = str.split("\n").map(|line| {
        let mut iter = line.splitn(3, ' ');
        let command = iter.next().expect("No command");
        let src = iter.next();
        let dst = iter.next();

        match command {
            MOV_NAME => {
                let src = match src {
                    Some(A_REG_NAME) => OpPlace::Reg(Reg::A),
                    Some(src) => panic!("Unknown source {}", src),
                    None => panic!("No source given for {}", MOV_NAME),
                };
                let dst = match dst {
                    Some(A_REG_NAME) => Reg::A,
                    Some(src) => panic!("Unknown source {}", src),
                    None => panic!("No source given for {}", MOV_NAME),
                };
                Op::Mov(src, dst)
            }
            _ => panic!("Unknown command {}", command),
        }
    });

    commands.map(|command| command.encode()).collect()
}

#[test]
fn test_assembly_mov() {
    assert_eq!(assembly("mov a, b"), vec![]);
}
