use crate::instruction::*;
use std::collections::HashMap;

pub fn compile(program: String) -> Vec<Vec<u16>> {
    let lines = program.split("\n").collect::<Vec<&str>>();

    let mut bytecode: Vec<Vec<u16>> = Vec::new();
    let mut defined_names: HashMap<String, Vec<u16>> = HashMap::new();
    let mut defined_labels: HashMap<String, u16> = HashMap::new();
    let mut line_number: u16 = 0;
    for (index, line) in lines.iter().enumerate() {
        let contents: Vec<&str> = line.split_ascii_whitespace().collect();
        if contents.is_empty() {
            continue;
        }

        match parse_argument(&contents[0], &defined_names) {
            Some(bytes) => {
                if bytes[0] == 0x021 {
                    let name = contents
                        .get(1)
                        .unwrap_or_else(|| panic!("DEF without name on line {index}"))
                        .to_string();

                    if let Some(bytes) = contents.get(2..) {
                        if bytes.is_empty() {
                            defined_labels.insert(name, line_number);
                            continue;
                        } else {
                            defined_names.insert(
                                name,
                                bytes
                                    .iter()
                                    .flat_map(|str| match parse_argument(str, &defined_names) {
                                        Some(bytes) => bytes,
                                        None => {
                                            panic!("Error parsing argument {str} on line {index}")
                                        }
                                    })
                                    .collect(),
                            );
                        }
                    }
                } else if bytes[0] == 0x039 {
                    let name = contents
                        .get(1)
                        .unwrap_or_else(|| panic!("CALL without name on line {index}"))
                        .to_string();
                    println!("{name}, {:#?}", defined_labels);
                    let address = defined_labels.get(&name).unwrap_or_else(|| {
                        panic!("No label found with name {name} on line {index}")
                    });

                    bytecode.push(vec![0x039, *address]);
                } else {
                    bytecode.push(
                        contents
                            .iter()
                            .flat_map(|str| match parse_argument(str, &defined_names) {
                                Some(bytes) => bytes,
                                None => panic!("Error on line {index} parsing operation {str}"),
                            })
                            .collect(),
                    );
                }
            }
            None => (),
            /*{
                let opcode = parse_argument(&contents[0], &defined_names).unwrap();
                let _op = Operation::from_u16(opcode); // dumb check to see if it is a valid operation
                let mut bytes: Vec<u16> = contents
                    .get(1..)
                    .unwrap_or_else(|| panic!("No arguments given to ADD on line {index}"))
                    .iter()
                    .flat_map(|str| match parse_argument(str, &defined_names) {
                        Some(bytes) => bytes,
                        None => panic!("Error on line {index} parsing operation {str}"),
                    })
                    .collect();
                bytes.insert(0, opcode);
                bytecode.push(bytes);
            }*/
        }
        line_number += 1;
    }

    println!("{:#?}", bytecode);
    bytecode
}

pub fn parse_argument(arg: &&str, defined_names: &HashMap<String, Vec<u16>>) -> Option<Vec<u16>> {
    match parse_hex(arg) {
        Some(num) => Some(vec![num]),
        None => defined_names.get(*arg).cloned(),
    }
}

/*
0x000 -> 0x00F arithematic registers
0x010 -> 0x01F reserved registers
0x020 -> 0x03F reserved for NO REASON

0x040->0xFDF program memory

0xFE0 -> 0xFEF Stack space
*/

pub fn run_raw(instructions: Vec<Vec<u16>>) {
    let mut memory: [u16; 4096] = [0; 4096];
    let stack_base: usize = 0xFE0;
    let stack_pointer: usize = 0x011;
    let program_counter: usize = 0x010;

    loop {
        if let Some(line) = instructions.get(memory[program_counter] as usize) {
            if line.is_empty() {
                continue;
            }

            let op = Operation::from_u16(*line.first().unwrap());
            match op {
                Operation::NOP => continue,
                Operation::DEF => unreachable!(),
                Operation::MOV => {
                    let src = *line.get(1).unwrap_or_else(|| {
                        panic!("No SRC for MOV on line {}", memory[program_counter])
                    });
                    let dest = *line.get(2).unwrap_or_else(|| {
                        panic!("No DEST for MOV on line {}", memory[program_counter])
                    });

                    //Double check that a valid address was given
                    if check_address(src) && check_address(dest) {
                        memory[dest as usize] = memory[src as usize];
                    }
                }
                Operation::ADD => {
                    let src = *line.get(1).unwrap_or_else(|| {
                        panic!("No SRC for ADD on line {}", memory[program_counter])
                    });
                    let dest = *line.get(2).unwrap_or_else(|| {
                        panic!("No DEST for ADD on line {}", memory[program_counter])
                    });

                    //Double check that a valid address was given
                    if check_address(src) && check_address(dest) {
                        memory[dest as usize] += memory[src as usize];
                    }
                }
                Operation::SUB => {
                    let src = *line.get(1).unwrap_or_else(|| {
                        panic!("No SRC for SUB on line {}", memory[program_counter])
                    });
                    let dest = *line.get(2).unwrap_or_else(|| {
                        panic!("No DEST for SUB on line {}", memory[program_counter])
                    });

                    //Double check that a valid address was given
                    if check_address(src) && check_address(dest) {
                        memory[dest as usize] -= memory[src as usize];
                    }
                }
                Operation::INC => {
                    let dest = *line.get(1).unwrap_or_else(|| {
                        panic!("No DEST for INC on line {}", memory[program_counter])
                    });

                    if check_address(dest) {
                        memory[dest as usize] += 1;
                    }
                }
                Operation::DEC => {
                    let dest = *line.get(1).unwrap_or_else(|| {
                        panic!("No DEST for DEC on line {}", memory[program_counter])
                    });

                    if check_address(dest) {
                        memory[dest as usize] -= 1;
                    }
                }
                Operation::MUL => {
                    let src = *line.get(1).unwrap_or_else(|| {
                        panic!("No SRC for MUL on line {}", memory[program_counter])
                    });
                    let dest = *line.get(2).unwrap_or_else(|| {
                        panic!("No DEST for MUL on line {}", memory[program_counter])
                    });

                    //Double check that a valid address was given
                    if check_address(src) && check_address(dest) {
                        memory[dest as usize] *= memory[src as usize];
                    }
                }
                Operation::DIV => {
                    let src = *line.get(1).unwrap_or_else(|| {
                        panic!("No SRC for DIV on line {}", memory[program_counter])
                    });
                    let dest = *line.get(2).unwrap_or_else(|| {
                        panic!("No DEST for DIV on line {}", memory[program_counter])
                    });

                    //Double check that a valid address was given
                    if check_address(src) && check_address(dest) {
                        memory[dest as usize] /= memory[src as usize];
                    }
                }
                Operation::MOD => {
                    let src = *line.get(1).unwrap_or_else(|| {
                        panic!("No SRC for MOD on line {}", memory[program_counter])
                    });
                    let dest = *line.get(2).unwrap_or_else(|| {
                        panic!("No DEST for MOD on line {}", memory[program_counter])
                    });

                    //Double check that a valid address was given
                    if check_address(src) && check_address(dest) {
                        memory[dest as usize] %= memory[src as usize];
                    }
                }
                Operation::AND => {
                    let src = *line.get(1).unwrap_or_else(|| {
                        panic!("No SRC for AND on line {}", memory[program_counter])
                    });
                    let dest = *line.get(2).unwrap_or_else(|| {
                        panic!("No DEST for AND on line {}", memory[program_counter])
                    });

                    //Double check that a valid address was given
                    if check_address(src) && check_address(dest) {
                        memory[dest as usize] &= memory[src as usize];
                    }
                }
                Operation::OR => {
                    let src = *line.get(1).unwrap_or_else(|| {
                        panic!("No SRC for OR on line {}", memory[program_counter])
                    });
                    let dest = *line.get(2).unwrap_or_else(|| {
                        panic!("No DEST for OR on line {}", memory[program_counter])
                    });

                    //Double check that a valid address was given
                    if check_address(src) && check_address(dest) {
                        memory[dest as usize] |= memory[src as usize];
                    }
                }
                Operation::XOR => {
                    let src = *line.get(1).unwrap_or_else(|| {
                        panic!("No SRC for XOR on line {}", memory[program_counter])
                    });
                    let dest = *line.get(2).unwrap_or_else(|| {
                        panic!("No DEST for XOR on line {}", memory[program_counter])
                    });

                    //Double check that a valid address was given
                    if check_address(src) && check_address(dest) {
                        memory[dest as usize] ^= memory[src as usize];
                    }
                }
                Operation::NOT => {
                    let dest = *line.get(1).unwrap_or_else(|| {
                        panic!("No DEST for INC on line {}", memory[program_counter])
                    });

                    if check_address(dest) {
                        memory[dest as usize] = !memory[dest as usize];
                    }
                }
                Operation::SHL => {
                    let src = *line.get(1).unwrap_or_else(|| {
                        panic!("No SRC for SHL on line {}", memory[program_counter])
                    });
                    let dest = *line.get(2).unwrap_or_else(|| {
                        panic!("No DEST for SHL on line {}", memory[program_counter])
                    });

                    //Double check that a valid address was given
                    if check_address(src) && check_address(dest) {
                        memory[dest as usize] <<= memory[src as usize];
                    }
                }
                Operation::SHR => {
                    let src = *line.get(1).unwrap_or_else(|| {
                        panic!("No SRC for SHR on line {}", memory[program_counter])
                    });
                    let dest = *line.get(2).unwrap_or_else(|| {
                        panic!("No DEST for SHR on line {}", memory[program_counter])
                    });

                    //Double check that a valid address was given
                    if check_address(src) && check_address(dest) {
                        memory[dest as usize] >>= memory[src as usize];
                    }
                }
                Operation::JMP => {
                    let address = *line.get(1).unwrap_or_else(|| {
                        panic!("No ADDR for JMP on line {}", memory[program_counter])
                    });
                    memory[0x010] = address;
                }
                Operation::JG => (),
                Operation::JL => (),
                Operation::JZ => (),
                Operation::JNZ => (),
                Operation::CMP => (),
                Operation::PUSH => {
                    let src = *line.get(1).unwrap_or_else(|| {
                        panic!("No SRC for PUSH on line {}", memory[program_counter])
                    });

                    memory[stack_pointer] += 1;
                    memory[stack_base + memory[stack_pointer] as usize] = memory[src as usize];
                }
                Operation::POP => {
                    let dest = *line.get(1).unwrap_or_else(|| {
                        panic!("No DEST for POP on line {}", memory[program_counter])
                    });

                    memory[dest as usize] = memory[stack_base + memory[stack_pointer] as usize];
                    memory[stack_pointer] -= 1;
                }
                Operation::IMM => {
                    let immediate = *line.get(1).unwrap_or_else(|| {
                        panic!("No IMM value on line {}", memory[program_counter])
                    });
                    let dest = *line.get(2).unwrap_or_else(|| {
                        panic!("No destination for IMM on line {}", memory[program_counter])
                    });
                    if check_address(dest) {
                        memory[dest as usize] = immediate;
                    }
                }
                Operation::CALL => {
                    let address = *line.get(1).unwrap_or_else(|| {
                        panic!("No ADDR for JMP on line {}", memory[program_counter]);
                    });
                    memory[stack_base + memory[stack_pointer] as usize] = memory[program_counter];
                    memory[stack_pointer] += 1;
                    memory[program_counter] = address;
                },
                // ATTEMPT TO SUBTRACT WITH OVERFLOW?
                Operation::RET => {
                    memory[program_counter] = memory[stack_base + memory[stack_pointer] as usize] + 1;
                    memory[stack_pointer] -= 1;
                }
                Operation::HLT => break,
            }
        } else {
            break;
        }

        memory[program_counter] += 1;
    }

    println!("{:#?}", memory.get(0..15));
}

fn check_address(address: u16) -> bool {
    ((address & 0b0000_0001_1111) > 0) || ((address & 0b1111_1100_0000) > 0)
}
