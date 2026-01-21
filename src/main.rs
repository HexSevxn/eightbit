use std::env;
use std::fs;
use std::fs::read_to_string;

pub mod compiler;
pub mod operation;
pub mod interpreter;

use compiler::compile;
use interpreter::run_raw;
use operation::format_radix;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = args
        .get(1)
        .unwrap_or_else(|| panic!("No file given to debug."))
        .clone();
    if !path.ends_with(".x1") {
        panic!("File is not an x1 program!");
    };

    let program = read_to_string(path).expect("Error reading file.");
    let bytecode = compile(program);

    let mut content_string = String::new();
    for line in bytecode.iter() {
        let contents = line
            .iter()
            .map(|x| "0x".to_string() + &format_radix(*x as u32, 16))
            .collect::<Vec<String>>()
            .join(" ");
        content_string.push_str(&format!("{}\n", contents));
    }
    fs::write("compiled.txt", &content_string).unwrap();

    run_raw(bytecode);
}
