use std::env;
use std::fs::read_to_string;

pub mod compiler;
pub mod instruction;

use compiler::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = args[1].clone();
    if !path.ends_with(".e8") {
        panic!("File is not an e8 program!");
    };

    let program = read_to_string(path).expect("Error reading file.");
    let bytecode = compile(program);

    run_raw(bytecode);
}
