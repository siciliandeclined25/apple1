use std::io;

pub const IMMEDIATE: u8 = 0;
pub const IMPLIED: u8 = 1;
pub const ACCUMULATOR: u8 = 2;
pub const RELATIVE: u8 = 3;
pub const ABSOLUTE: u8 = 4;
pub const XABSOLUTE: u8 = 5;
pub const YABSOLUTE: u8 = 6;
pub const ZEROPAGE: u8 = 7;
pub const XZEROPAGE: u8 = 8;
pub const YZEROPAGE: u8 = 9;
pub const INDIRECT: u8 = 10;
pub const XINDIRECT: u8 = 11;
pub const YINDIRECT: u8 = 12;

pub fn getOpcode(namespace: &str, addressingMode: u8) -> Option<u32> {
    //A function to return the opcode given the namespace of the function
    // (command, 3 byte String buffer, borrowed)
    // & (addressingMode, u8, defined by constants)
    match addressingMode {
        IMMEDIATE => match namespace {
            "LDA" => Some(0x1a2b),
            _ => Some(0xEA), //nop
        },
        _ => Some(0xEA), //handle non-numeric assignments
    }
}

pub fn compile(buf: &str) -> &str {
    //first part, let's split the buffer to
    // a vector based on \n
    let mut tokenList: Vec<u8> = Vec::new();
    let parts: Vec<&str> = buf.lines().collect();
    let mut assembledCode: Vec<&u8> = Vec::new();
    //second, let's iterate through the line
    for line in parts {
        if line.len() == 0 {
            continue; //no instruction
        } else {
            // yes instruction exists
            println!("{}", &line);
            let char_of_line: Vec<char> = line.chars().collect(); //makes std:str:char iterator

            let mut instructionType: u8 = 255;
            //let's determine what type of instruction it is
            //all 6502 instructions are 3 chars one space then the addressing
            print!("{}", char_of_line[4]);
            if char_of_line[4] == '#' {
                instructionType = IMMEDIATE;
            }
            if char_of_line[4] == '$'{
                instruction
            }

        }
    }
    return buf;
}
