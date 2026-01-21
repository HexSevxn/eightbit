use crate::operation::parse_hex;
use std::collections::HashMap;

pub fn compile(program: String) -> Vec<Vec<u16>> {
    let lines = program.split("\n").collect::<Vec<&str>>();

    let mut bytecode: Vec<Vec<u16>> = Vec::new();
    let mut defined_names: HashMap<String, Vec<u16>> = HashMap::new();
    let mut defined_labels: HashMap<String, u16> = HashMap::new();

    //loop for labels and definitions first
    for (index, line) in lines.iter().enumerate() {
        let line_contents: Vec<&str> = line.split_ascii_whitespace().collect();
        let mut contents = Vec::new();

        for word in line_contents {
            if word.starts_with("//") {
                break;
            }
            contents.push(word);
        }

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

                    if let Some(field) = contents.get(2..) {
                        if field.is_empty() {
                            defined_labels.insert(name, index as u16);
                        } else {
                            defined_names.insert(
                                name,
                                field
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
                }
            }
            None => {
                continue;
            }
        }
    }

    //Main bytecode compilation
    for (index, line) in lines.iter().enumerate() {
        let line_contents: Vec<&str> = line.split_ascii_whitespace().into_iter().collect();

        //loop to ignore comments
        let mut contents = Vec::new();
        for word in line_contents {
            if word.starts_with("//") {
                break;
            }
            contents.push(word);
        }

        if contents.is_empty() {
            bytecode.push(vec![0x020]);
            continue;
        }

        bytecode.push(
            contents
                .iter()
                .flat_map(|str| match parse_argument(str, &defined_names) {
                    Some(bytes) => bytes,
                    None => {
                        if let Some(label_index) = defined_labels.get(str.to_owned()) {
                            return vec![*label_index];
                        }
                        panic!("Error on line {index} with string {str}")
                    }
                })
                .collect(),
        );
    }

    let mut old_to_new_index: HashMap<u16, u16> = HashMap::new();
    let mut new_index: u16 = 0;
    for (old_index, bytes) in bytecode.clone().iter().enumerate() {
        let opcode = bytes
            .first()
            .unwrap_or_else(|| panic!("Unkown opcode on line {old_index}"));

        match opcode {
            0x020 | 0x021 => continue,
            _ => {
                old_to_new_index.insert(old_index as u16, new_index);
                new_index += 1;
            }
        }
    }

    let mut optimized_bytecode: Vec<Vec<u16>> = Vec::new();
    for (index, bytes) in bytecode.clone().iter().enumerate() {
        let opcode = bytes
            .first()
            .unwrap_or_else(|| panic!("Unkown opcode on line {index}"));
        match opcode {
            0x030 | 0x031 | 0x032 | 0x033 | 0x034 | 0x039 => {
                let mut new_bytes = bytes.clone();
                let new_target = old_to_new_index
                    .get(
                        &(new_bytes
                            .get(1)
                            .unwrap_or_else(|| panic!("No argument bytes found on line {index}."))
                            + 1),
                    )
                    .unwrap_or(&new_bytes[1]);
                new_bytes[1] = *new_target;

                optimized_bytecode.push(new_bytes);
            }
            0x020 | 0x021 => continue,
            _ => optimized_bytecode.push(bytes.clone()),
        }
    }

    optimized_bytecode
}

pub fn parse_argument(arg: &&str, defined_names: &HashMap<String, Vec<u16>>) -> Option<Vec<u16>> {
    match parse_hex(arg) {
        Some(num) => Some(vec![num]),
        None => defined_names.get(*arg).cloned(),
    }
}