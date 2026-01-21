use crate::operation::Operation;

/*
0x000 -> 0x00F arithematic registers
0x010 -> 0x01F reserved registers
0x020 -> 0x03F reserved for NO REASON

0x040->0xFDF program memory

0xFE0 -> 0xFEF Stack space

RESERVED REGISTERS
0x010 -> return register - contains the exit code of the program, can be used for function returns
0x01A -> stack pointer - contains the accumulated stack pointer
0x01B -> stack base - contains the address of the stack base
0x01C -> carry register - used in mathematical operations to indicate a carry through
0x01E -> program_counter - used to get the current instruction
*/

const RETURN_REGISTER_ADDRESS: usize = 0x010;
const STACK_POINTER_ADDRESS: usize = 0x01A;
const STACK_BASE_ADDRESS: usize = 0x01B;
const CARRY_REGISTER_ADDRESS: usize = 0x01C; // CARRY REGISTER
const PROGRAM_COUNTER_ADDRESS: usize = 0x01E; // PROGRAM COUNTER

const STACK_BASE: usize = 0xFE0; // STACK_BASE (Loaded into memory on startup)

pub fn run_raw(instructions: Vec<Vec<u16>>) {
    let mut memory: [u16; 4096] = [0; 4096];
    memory[STACK_BASE_ADDRESS] = STACK_BASE as u16;

    loop {
        if let Some(line) = instructions.get(memory[PROGRAM_COUNTER_ADDRESS] as usize) {
            if line.is_empty() {
                continue;
            }
            let op = Operation::from_u16(*line.first().unwrap());
            match op {
                Operation::NOP => {
                    memory[PROGRAM_COUNTER_ADDRESS] += 1;
                    continue;
                }
                Operation::DEF => {
                    memory[PROGRAM_COUNTER_ADDRESS] += 1;
                    continue;
                }
                Operation::MOV => {
                    let src = *line.get(1).unwrap_or_else(|| {
                        panic!("No SRC for MOV on line {}", memory[PROGRAM_COUNTER_ADDRESS])
                    });
                    let dest = *line.get(2).unwrap_or_else(|| {
                        panic!(
                            "No DEST for MOV on line {}",
                            memory[PROGRAM_COUNTER_ADDRESS]
                        )
                    });

                    //Double check that a valid address was given
                    if check_address(src) && check_address(dest) {
                        memory[dest as usize] = memory[src as usize];
                    }
                }
                Operation::ADD => {
                    let src = *line.get(1).unwrap_or_else(|| {
                        panic!("No SRC for ADD on line {}", memory[PROGRAM_COUNTER_ADDRESS])
                    });
                    let dest = *line.get(2).unwrap_or_else(|| {
                        panic!(
                            "No DEST for ADD on line {}",
                            memory[PROGRAM_COUNTER_ADDRESS]
                        )
                    });

                    //Double check that a valid address was given
                    if check_address(src) && check_address(dest) {
                        memory[dest as usize] += memory[src as usize];
                    }
                }
                Operation::SUB => {
                    let src = *line.get(1).unwrap_or_else(|| {
                        panic!("No SRC for SUB on line {}", memory[PROGRAM_COUNTER_ADDRESS])
                    });
                    let dest = *line.get(2).unwrap_or_else(|| {
                        panic!(
                            "No DEST for SUB on line {}",
                            memory[PROGRAM_COUNTER_ADDRESS]
                        )
                    });

                    //Double check that a valid address was given
                    if check_address(src) && check_address(dest) {
                        memory[dest as usize] -= memory[src as usize];
                    }
                }
                Operation::INC => {
                    let dest = *line.get(1).unwrap_or_else(|| {
                        panic!(
                            "No DEST for INC on line {}",
                            memory[PROGRAM_COUNTER_ADDRESS]
                        )
                    });

                    if check_address(dest) {
                        memory[dest as usize] += 1;
                    }
                }
                Operation::DEC => {
                    let dest = *line.get(1).unwrap_or_else(|| {
                        panic!(
                            "No DEST for DEC on line {}",
                            memory[PROGRAM_COUNTER_ADDRESS]
                        )
                    });

                    if check_address(dest) {
                        memory[dest as usize] -= 1;
                    }
                }
                Operation::MUL => {
                    let src = *line.get(1).unwrap_or_else(|| {
                        panic!("No SRC for MUL on line {}", memory[PROGRAM_COUNTER_ADDRESS])
                    });
                    let dest = *line.get(2).unwrap_or_else(|| {
                        panic!(
                            "No DEST for MUL on line {}",
                            memory[PROGRAM_COUNTER_ADDRESS]
                        )
                    });

                    //Double check that a valid address was given
                    if check_address(src) && check_address(dest) {
                        memory[dest as usize] *= memory[src as usize];
                    }
                }
                Operation::DIV => {
                    let src = *line.get(1).unwrap_or_else(|| {
                        panic!("No SRC for DIV on line {}", memory[PROGRAM_COUNTER_ADDRESS])
                    });
                    let dest = *line.get(2).unwrap_or_else(|| {
                        panic!(
                            "No DEST for DIV on line {}",
                            memory[PROGRAM_COUNTER_ADDRESS]
                        )
                    });

                    //Double check that a valid address was given
                    if check_address(src) && check_address(dest) {
                        memory[dest as usize] /= memory[src as usize];
                    }
                }
                Operation::MOD => {
                    let src = *line.get(1).unwrap_or_else(|| {
                        panic!("No SRC for MOD on line {}", memory[PROGRAM_COUNTER_ADDRESS])
                    });
                    let dest = *line.get(2).unwrap_or_else(|| {
                        panic!(
                            "No DEST for MOD on line {}",
                            memory[PROGRAM_COUNTER_ADDRESS]
                        )
                    });

                    //Double check that a valid address was given
                    if check_address(src) && check_address(dest) {
                        memory[dest as usize] %= memory[src as usize];
                    }
                }
                Operation::AND => {
                    let src = *line.get(1).unwrap_or_else(|| {
                        panic!("No SRC for AND on line {}", memory[PROGRAM_COUNTER_ADDRESS])
                    });
                    let dest = *line.get(2).unwrap_or_else(|| {
                        panic!(
                            "No DEST for AND on line {}",
                            memory[PROGRAM_COUNTER_ADDRESS]
                        )
                    });

                    //Double check that a valid address was given
                    if check_address(src) && check_address(dest) {
                        memory[dest as usize] &= memory[src as usize];
                    }
                }
                Operation::OR => {
                    let src = *line.get(1).unwrap_or_else(|| {
                        panic!("No SRC for OR on line {}", memory[PROGRAM_COUNTER_ADDRESS])
                    });
                    let dest = *line.get(2).unwrap_or_else(|| {
                        panic!("No DEST for OR on line {}", memory[PROGRAM_COUNTER_ADDRESS])
                    });

                    //Double check that a valid address was given
                    if check_address(src) && check_address(dest) {
                        memory[dest as usize] |= memory[src as usize];
                    }
                }
                Operation::XOR => {
                    let src = *line.get(1).unwrap_or_else(|| {
                        panic!("No SRC for XOR on line {}", memory[PROGRAM_COUNTER_ADDRESS])
                    });
                    let dest = *line.get(2).unwrap_or_else(|| {
                        panic!(
                            "No DEST for XOR on line {}",
                            memory[PROGRAM_COUNTER_ADDRESS]
                        )
                    });

                    //Double check that a valid address was given
                    if check_address(src) && check_address(dest) {
                        memory[dest as usize] ^= memory[src as usize];
                    }
                }
                Operation::NOT => {
                    let dest = *line.get(1).unwrap_or_else(|| {
                        panic!(
                            "No DEST for INC on line {}",
                            memory[PROGRAM_COUNTER_ADDRESS]
                        )
                    });

                    if check_address(dest) {
                        memory[dest as usize] = !memory[dest as usize];
                    }
                }
                Operation::SHL => {
                    let src = *line.get(1).unwrap_or_else(|| {
                        panic!("No SRC for SHL on line {}", memory[PROGRAM_COUNTER_ADDRESS])
                    });
                    let dest = *line.get(2).unwrap_or_else(|| {
                        panic!(
                            "No DEST for SHL on line {}",
                            memory[PROGRAM_COUNTER_ADDRESS]
                        )
                    });

                    //Double check that a valid address was given
                    if check_address(src) && check_address(dest) {
                        memory[dest as usize] <<= memory[src as usize];
                    }
                }
                Operation::SHR => {
                    let src = *line.get(1).unwrap_or_else(|| {
                        panic!("No SRC for SHR on line {}", memory[PROGRAM_COUNTER_ADDRESS])
                    });
                    let dest = *line.get(2).unwrap_or_else(|| {
                        panic!(
                            "No DEST for SHR on line {}",
                            memory[PROGRAM_COUNTER_ADDRESS]
                        )
                    });

                    //Double check that a valid address was given
                    if check_address(src) && check_address(dest) {
                        memory[dest as usize] >>= memory[src as usize];
                    }
                }
                Operation::JMP => {
                    let address = *line.get(1).unwrap_or_else(|| {
                        panic!(
                            "No ADDR for JMP on line {}",
                            memory[PROGRAM_COUNTER_ADDRESS]
                        )
                    });
                    memory[PROGRAM_COUNTER_ADDRESS] = address;
                    continue;
                }
                Operation::JG => {
                    let address = *line.get(1).unwrap_or_else(|| {
                        core_dump(&instructions, &memory);
                        panic!("No ADDR for JG on line {}", memory[PROGRAM_COUNTER_ADDRESS])
                    });
                    let arg1 = *line.get(2).unwrap_or_else(|| {
                        core_dump(&instructions, &memory);
                        panic!("No ARG1 for JG on line {}", memory[PROGRAM_COUNTER_ADDRESS]);
                    });
                    let arg2 = *line.get(3).unwrap_or_else(|| {
                        core_dump(&instructions, &memory);
                        panic!("No ARG2 for JG on line {}", memory[PROGRAM_COUNTER_ADDRESS]);
                    });
                    if memory[arg1 as usize] > memory[arg2 as usize] {
                        memory[PROGRAM_COUNTER_ADDRESS] = address;
                        continue;
                    }
                }
                Operation::JL => {
                    let address = *line.get(1).unwrap_or_else(|| {
                        core_dump(&instructions, &memory);
                        panic!("No ADDR for JL on line {}", memory[PROGRAM_COUNTER_ADDRESS])
                    });
                    let arg1 = *line.get(2).unwrap_or_else(|| {
                        core_dump(&instructions, &memory);
                        panic!("No ARG1 for JL on line {}", memory[PROGRAM_COUNTER_ADDRESS]);
                    });
                    let arg2 = *line.get(3).unwrap_or_else(|| {
                        core_dump(&instructions, &memory);
                        panic!("No ARG2 for JL on line {}", memory[PROGRAM_COUNTER_ADDRESS]);
                    });
                    if memory[arg1 as usize] < memory[arg2 as usize] {
                        memory[PROGRAM_COUNTER_ADDRESS] = address;
                        continue;
                    }
                }
                Operation::JZ => {
                    let address = *line.get(1).unwrap_or_else(|| {
                        core_dump(&instructions, &memory);
                        panic!("No ADDR for JZ on line {}", memory[PROGRAM_COUNTER_ADDRESS])
                    });
                    let arg1 = *line.get(2).unwrap_or_else(|| {
                        core_dump(&instructions, &memory);
                        panic!("No ARG1 for JZ on line {}", memory[PROGRAM_COUNTER_ADDRESS]);
                    });
                    if memory[arg1 as usize] == 0 {
                        memory[PROGRAM_COUNTER_ADDRESS] = address;
                        continue;
                    }
                }
                Operation::JNZ => {
                    let address = *line.get(1).unwrap_or_else(|| {
                        core_dump(&instructions, &memory);
                        panic!("No ADDR for JZ on line {}", memory[PROGRAM_COUNTER_ADDRESS])
                    });
                    let arg1 = *line.get(2).unwrap_or_else(|| {
                        core_dump(&instructions, &memory);
                        panic!("No ARG1 for JZ on line {}", memory[PROGRAM_COUNTER_ADDRESS]);
                    });
                    if memory[arg1 as usize] != 0 {
                        memory[PROGRAM_COUNTER_ADDRESS] = address;
                        continue;
                    }
                }
                Operation::CMP => {
                    let arg1 = *line.get(2).unwrap_or_else(|| {
                        core_dump(&instructions, &memory);
                        panic!("No ARG1 for JL on line {}", memory[PROGRAM_COUNTER_ADDRESS]);
                    });
                    let arg2 = *line.get(3).unwrap_or_else(|| {
                        core_dump(&instructions, &memory);
                        panic!("No ARG2 for JL on line {}", memory[PROGRAM_COUNTER_ADDRESS]);
                    });
                    if memory[arg1 as usize] == memory[arg2 as usize] {
                        memory[CARRY_REGISTER_ADDRESS] = 0x001;
                    }
                }
                Operation::PUSH => {
                    let src = *line.get(1).unwrap_or_else(|| {
                        panic!(
                            "No SRC for PUSH on line {}",
                            memory[PROGRAM_COUNTER_ADDRESS]
                        )
                    });

                    memory[STACK_POINTER_ADDRESS] += 1;
                    memory[memory[STACK_BASE_ADDRESS] as usize + (memory[STACK_POINTER_ADDRESS] as usize)] =
                        memory[src as usize];
                }
                Operation::POP => {
                    let dest = *line.get(1).unwrap_or_else(|| {
                        core_dump(&instructions, &memory);
                        panic!(
                            "No DEST for POP on line {}",
                            memory[PROGRAM_COUNTER_ADDRESS]
                        )
                    });

                    memory[dest as usize] =
                        memory[memory[STACK_BASE_ADDRESS] as usize + (memory[STACK_POINTER_ADDRESS] as usize)];
                    memory[STACK_POINTER_ADDRESS] -= 1;
                }
                Operation::IMM => {
                    let immediate = *line.get(1).unwrap_or_else(|| {
                        core_dump(&instructions, &memory);
                        panic!("No IMM value on line {}", memory[PROGRAM_COUNTER_ADDRESS])
                    });
                    let dest = *line.get(2).unwrap_or_else(|| {
                        core_dump(&instructions, &memory);
                        panic!(
                            "No destination for IMM on line {}",
                            memory[PROGRAM_COUNTER_ADDRESS]
                        )
                    });
                    if check_address(dest) {
                        memory[dest as usize] = immediate;
                    }
                }
                Operation::CALL => {
                    let address = *line.get(1).unwrap_or_else(|| {
                        core_dump(&instructions, &memory);
                        panic!(
                            "No ADDR for JMP on line {}",
                            memory[PROGRAM_COUNTER_ADDRESS]
                        );
                    });
                    memory[memory[STACK_BASE_ADDRESS] as usize + (memory[STACK_POINTER_ADDRESS] as usize)] =
                        memory[PROGRAM_COUNTER_ADDRESS];
                    memory[STACK_POINTER_ADDRESS] += 1;
                    memory[PROGRAM_COUNTER_ADDRESS] = address;
                    continue;
                }
                // ATTEMPT TO SUBTRACT WITH OVERFLOW?
                Operation::RET => {
                    memory[STACK_POINTER_ADDRESS] = memory[STACK_POINTER_ADDRESS]
                        .checked_sub(1)
                        .unwrap_or_else(|| {
                            core_dump(&instructions, &memory);
                            panic!("Error returning");
                        });
                    memory[PROGRAM_COUNTER_ADDRESS] =
                        memory[memory[STACK_BASE_ADDRESS] as usize + (memory[STACK_POINTER_ADDRESS] as usize)];
                }
                Operation::HLT => {
                    let exit_code = *line.get(1).unwrap_or_else(|| {
                        core_dump(&instructions, &memory);
                        panic!(
                            "No EXIT_CODE for JMP on line {}",
                            memory[PROGRAM_COUNTER_ADDRESS]
                        );
                    });
                    memory[RETURN_REGISTER_ADDRESS] = exit_code;
                    break;
                }
            }
        } else {
            break;
        }

        memory[PROGRAM_COUNTER_ADDRESS] += 1;
    }

    core_dump(&instructions, &memory);
}

fn core_dump(instructions: &Vec<Vec<u16>>, memory: &[u16; 4096]) {

    println!("\nINTERPRETER DUMP:\n");
    if let Some(line) = instructions.get(memory[PROGRAM_COUNTER_ADDRESS] as usize) {
        println!(
            "Current Instruction: {:?} {:?}",
            Operation::from_u16(line[0]),
            line[1..].to_vec()
        );
    }
    println!(
        "Program Counter: [0x{:X}]\nReturn Register: [0x{:X}]\nStack Pointer:[0x{:X}]\nCarry Register: [0x{:X}]\nStack Base: 0x{:X}\n\n",
        memory[RETURN_REGISTER_ADDRESS],
        memory[PROGRAM_COUNTER_ADDRESS],
        memory[STACK_POINTER_ADDRESS],
        memory[CARRY_REGISTER_ADDRESS],
        memory[STACK_BASE_ADDRESS],
    );
    println!("Arithmetic Registers:\n{:?}\n", memory[0..0x10].to_vec());
    println!("Reserved Registers:\n{:?}\n", memory[0x011..0x020].to_vec());
    println!("\nDUMP END\n");
}

fn check_address(address: u16) -> bool {
    address & 0b0000_0001_1111 > 0 || address & 0b1111_1100_0000 > 0
}