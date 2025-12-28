
/*

instructions start with 0x0[1<=N<=3]N (BYTE && 0010_0000 > 0)
registers start with 0x00N (BYTE && 0000_1111 > 0)

*/

//INSTRUCTION SET
#[derive(Clone, Debug)]
pub enum Instruction {
    //BASIC
    NOP,    // 0x020 / 32 -> NOP -> NO OPERATION
    DEF(String, Vec<u8>),    // 0x021 / 33 -> DEF NAME ARGS -> DEFINES A NAME TO REPRESENT A COLLECTION OF BYTES
    MOV(u8, u8),    // 0x022 / 34 -> MOV SRC DEST -> MOVES SOURCE TO DESTINATION

    // MATH
    ADD(u8, u8),    // 0x023 / 35 -> ADD SRC DEST -> ADDS SOURCE TO DESTINATION
    SUB(u8, u8),    // 0x024 / 36 -> SUB SRC DEST -> SUBTRACTS SOURCE FROM DESTINATION
    INC(u8),    // 0x025 / 37 -> INC DEST -> INCREMENTS DESTINATION
    DEC(u8),    // 0x026 / 38 -> DEC DEST -> DECREMENTS DESTINATION
    MUL(u8, u8),    // 0x027 / 39 -> MUL SRC DEST -> MULTIPLIES DEST BY SOURCE
    DIV(u8, u8),    // 0x028 / 40 -> DIV SRC DEST -> DIVIDES DEST BY SOURCE (INTEGER DIVISION)
    MOD(u8, u8),    // 0x029 / 41 -> MOD SRC DEST -> DESTINATION SET TO MODULUS OF DEST BY SRC

    //BINARY
    AND(u8, u8),    // 0x02A / 42 -> AND SRC DEST -> DEST = SRC && DEST
    OR(u8, u8),     // 0x02B / 43 -> OR SRC DEST -> DEST = SRC || DEST
    XOR(u8, u8),    // 0x02C / 44 -> XOR SRC DEST -> DEST = SRC ^ DEST
    NOT(u8),    // 0x02D / 45 -> NOT SRC DEST -> DEST = !DEST
    SHL(u8, u8),    // 0x02E / 46 -> SHL ARG TARGET -> SHIFTS LEFT TARGET BY ARG
    SHR(u8, u8),    // 0x02F / 47 -> SHR ARG TARGET -> SHIFTS RIGHT TARGET BY ARG

    // LOGICAL
    JMP(u8),    // 0x030 / 48 -> ADDRESS -> JUMPS TO ADDRESS
    JG(u16, u8, u8),     // 0x031 / 49 -> JMP ADDRESS ARG1 ARG2 -> JUMPS TO ADDRESS IF ARG1 IS GREATER THAN ARG2
    JL(u16, u8, u8),     // 0x032 / 50 -> JMP ADDRESS ARG1 ARG2 -> JUMPS TO ADDRESS IF ARG1 IS LESS THAN ARG2
    JZ(u16, u8, u8),     // 0x033 / 51 -> JMP ADDRESS ARG -> JUMPS TO ADDRESS IF ARG IS EQUAL TO ZERO
    JNZ(u16, u8),    // 0x034 / 52 -> JMP ADDRESS ARG -> JUMPS TO ADDRESS IF ARG IS NOT EQUAL TO ZERO
    CMP(u8, u8),    // 0x035 / 53 -> CMP ARG1 ARG2 -> SETS CARRY FLAG TO 1 IF ARG1 and ARG2 ARE EQUAL, SETS TO ZERO OTHERWISE

    //STACK
    PUSH(u8),   // 0x036 / 54 -> PUSH SRC -> PUSHES SRC ONTO STACK
    POP(u8),    // 0x037 / 55 -> POP DEST -> POPS STACK INTO DEST

    //PROGRAM
    IMM(u8, u8),    // 0x038 / 56 -> IMM ARG DEST -> SETS DEST TO ARG
    CALL(String),   // 0x039 / 57 -> CALL NAME -> CALLS SUBROUTINE "NAME"
    RET,    // 0x03A / 58 -> RET -> RETURNS TO PARENT ROUTINE (HALTS IN ERROR, POPS ADDRESS OFF STACK)
    HLT(u8),    // 0x03B / 59 -> HLT -> HALTS PROGRAM PROCESSING (SAFELY?)
}

//INSTRUCTIONS WITHOUT ARGS FOR EASIER PARSING
#[derive(Clone, Debug)]
pub enum Operation {
    NOP,
    DEF,   
    MOV,    
    ADD,    
    SUB,   
    INC,   
    DEC,  
    MUL,   
    DIV,    
    MOD,
    AND,    
    OR,     
    XOR,    
    NOT,    
    SHL,    
    SHR,    
    JMP,    
    JG,   
    JL,    
    JZ,     
    JNZ,   
    CMP,    
    PUSH,
    POP,
    SYS,
    CALL,
    RET,
    HLT,
}

impl Operation {
    pub fn from_u8(num: u8) -> Operation {
        return match num {
            0x020 => Self::NOP,
            0x021 => Self::DEF,
            0x022 => Self::MOV,

            0x023 => Self::ADD,
            0x024 => Self::SUB,
            0x025 => Self::INC,
            0x026 => Self::DEC,
            0x027 => Self::MUL,
            0x028 => Self::DIV,
            0x029 => Self::MOD,

            0x02A => Self::AND,
            0x02B => Self::OR,
            0x02C => Self::XOR,
            0x02D => Self::NOT,
            0x02E => Self::SHL,
            0x02F => Self::SHR,

            0x030 => Self::JMP,
            0x031 => Self::JG,
            0x032 => Self::JL,
            0x033 => Self::JZ,
            0x034 => Self::JNZ,
            0x035 => Self::CMP,

            0x036 => Self::PUSH,
            0x037 => Self::POP,

            0x038 => Self::SYS,
            0x039 => Self::CALL,
            0x03A => Self::RET,
            0x03B => Self::HLT,

            _ => panic!("Unknown Operation. {num}"),
        }
    }
}

pub fn parse_hex(str: &str) -> u8 {
    return u8::from_str_radix(str.strip_prefix("0x").expect("Error stripping hexidecimal prefix."), 16).expect("Error parsing hexadecimal number.");
}