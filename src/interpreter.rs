use crate::instruction::*;
use std::collections::HashMap;

pub fn parse_program(program: String) -> Vec<Instruction> {
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
    panic!("END OF PROGRAM");

    let mut instructions: Vec<Instruction> = Vec::new();

    for (index, line) in lines.into_iter().enumerate() {
        let contents: Vec<&str> = line.split_ascii_whitespace().collect();
        if contents.is_empty() {
            instructions.push(Instruction::NOP);
            continue;
        }
        let op = Operation::from_u8(match parse_hex(contents.first().unwrap()) {
            Some(opcode) => opcode,
            None => match defined_names.get(contents[0]) {
                Some(bytes) => bytes[0],
                None => panic!("Unknown key on line {index}"),
            },
        });

        match op {
            Operation::NOP => instructions.push(Instruction::NOP),
            Operation::DEF => {
                let name = contents
                    .get(1)
                    .unwrap_or_else(|| panic!("Undefined name for DEF on line {index}"))
                    .to_string();
                let bytes = contents
                    .get(2..)
                    .unwrap_or_else(|| panic!("Empty DEF on line {index}!"))
                    .iter()
                    .flat_map(|x| parse_argument(x, &defined_names, index).unwrap())
                    .collect::<Vec<u8>>();

                defined_names.insert(name.clone(), bytes.clone());
                instructions.push(Instruction::DEF(name, bytes));
            }
            Operation::MOV => {
                // let src = parse_argument(contents.get(1));
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
            Operation::SYS => (),
            Operation::CALL => (),
            Operation::RET => (),
            Operation::HLT => {
                let exit_code =
                    parse_argument(contents.get(1).unwrap(), &defined_names, index).unwrap()[0];
                instructions.push(Instruction::HLT(exit_code))
            }
        }
    }

    println!("{:#?}", instructions);
    instructions
}

pub fn parse_argument(
    arg: &&str,
    defined_names: &HashMap<String, Vec<u8>>,
    line: usize,
) -> Option<Vec<u8>> {
    match parse_hex(arg) {
        Some(num) => Some(vec![num]),
        None => match defined_names.get(*arg) {
            Some(bytes) => Some(bytes.clone()),
            None => None,
        },
    }
}

pub fn compile(instructions: Vec<Instruction>) -> Vec<u8> {
    let program: Vec<u8> = Vec::new();

    program
}

pub fn run_raw(instructions: Vec<Instruction>) {
    let mut program_counter: u16 = 0;
}
