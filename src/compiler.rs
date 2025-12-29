use crate::instruction::*;
use std::collections::HashMap;

pub fn compile(program: String) -> Vec<Vec<u8>> {
    let lines = program.split("\n");

    let mut bytecode: Vec<Vec<u8>> = Vec::new();
    let mut defined_names: HashMap<String, Vec<u8>> = HashMap::new();
    for (index, line) in lines.into_iter().enumerate() {
        let contents: Vec<&str> = line.split_ascii_whitespace().collect();
        if contents.is_empty() {
            bytecode.push(vec![0x020]);
            continue;
        }

        match parse_argument(&contents[0], &defined_names, index) {
            Some(bytes) => {
                if bytes[0] == 0x021 {
                    defined_names.insert(
                        contents
                            .get(1)
                            .unwrap_or_else(|| panic!("DEF without name on line {index}"))
                            .to_string(),
                        contents
                            .get(2..)
                            .unwrap_or_else(|| panic!("No argument for DEF on line {index}"))
                            .iter()
                            .flat_map(|str| match parse_argument(str, &defined_names, index) {
                                Some(bytes) => bytes,
                                None => panic!("Error parsing argument {str} on line {index}"),
                            })
                            .collect(),
                    );
                } else {
                    bytecode.push(
                        contents
                            .iter()
                            .flat_map(|str| match parse_argument(str, &defined_names, index) {
                                Some(bytes) => bytes,
                                None => panic!("Error on line {index} parsing operation {str}"),
                            })
                            .collect(),
                    );
                }
            }
            None => panic!("Invalid operation on line {index}"),
        }
    }

    println!("{:#?}", bytecode);
    bytecode
}

pub fn parse_argument(
    arg: &&str,
    defined_names: &HashMap<String, Vec<u8>>,
    line: usize,
) -> Option<Vec<u8>> {
    match parse_hex(arg) {
        Some(num) => Some(vec![num]),
        None => Some(
            defined_names
                .get(*arg)
                .unwrap_or_else(|| panic!("Unknown argument on line {line}."))
                .to_vec(),
        ),
    }
}

/*
0x040->0xFFF program memory


0x010->0x01F arithematic registers



*/

pub fn run_raw(instructions: Vec<Vec<u8>>) {
    let mut memory: [u8; 4096] = [0; 4096];

    for (line_number, line) in instructions.iter().enumerate() {
        if line.is_empty() {
            continue;
        }

        let op = Operation::from_u8(*line.first().unwrap());
        match op {
            Operation::NOP => continue,
            Operation::DEF => unreachable!(),
            Operation::MOV => {
                let src = *line
                    .get(1)
                    .unwrap_or_else(|| panic!("No SRC for MOV on line {line_number}"));
                let dest = *line
                    .get(2)
                    .unwrap_or_else(|| panic!("No DEST for MOV on line {line_number}"));
                if check_address(src) && check_address(dest) {
                    memory[dest as usize] = memory[src as usize];
                }
            }
            Operation::ADD => (),
            Operation::SUB => (),
            Operation::INC => (),
            Operation::DEC => (),
            Operation::MUL => (),
            Operation::DIV => (),
            Operation::MOD => (),
            Operation::AND => (),
            Operation::OR => (),
            Operation::XOR => (),
            Operation::NOT => (),
            Operation::SHL => (),
            Operation::SHR => (),
            Operation::JMP => (),
            Operation::JG => (),
            Operation::JL => (),
            Operation::JZ => (),
            Operation::JNZ => (),
            Operation::CMP => (),
            Operation::PUSH => (),
            Operation::POP => (),
            Operation::IMM => {
                let immediate = *line
                    .get(1)
                    .unwrap_or_else(|| panic!("No IMM value on line {line_number}"));
                let dest = *line
                    .get(2)
                    .unwrap_or_else(|| panic!("No destination for IMM on line {line_number}"));
                if check_address(dest) {
                    memory[dest as usize] = immediate;
                }
            }
            Operation::CALL => (),
            Operation::RET => (),
            Operation::HLT => {}
        }
    }

    println!("{:#?}", memory.get(0..15));
}

fn check_address(address: u8) -> bool {
    ((address & 0b0001_1111) > 0) || ((address & 0b1100_0000) > 0)
}
