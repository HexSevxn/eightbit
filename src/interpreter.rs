use crate::instruction::*;

pub fn parse_program(program: String) -> Vec<Instruction> {
    let lines = program.split("\n");

    for line in lines {
        let contents: Vec<&str> = line.split_ascii_whitespace().collect();
        let op = Operation::from_u8(parse_hex(contents.get(0).unwrap_or(&"0x020")));

        match op {


            _ => (),
        }
    }

    return vec![];
}

pub fn run(instructions: Vec<Instruction>) {
    let mut program_counter: u16 = 0;

    
}