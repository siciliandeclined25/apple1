use std::fs::File;
use std::io::{Read, Write};
mod asm;
use asm::compile;

//6502 constants
/*
* 7  n   negative
6  v   overflow
5  –   always 1 (unused)
4  b   break
3  d   decimal
2  i   interrupt disable
1  z   zero
0  c   carry
*/
pub const CARRYB: u8 = 0;
pub const ZEROB: u8 = 1;
pub const INTERRUPTB: u8 = 2;
pub const DECIMALB: u8 = 3;
pub const BREAKB: u8 = 4;
pub const UNUSUEDB: u8 = 5;
pub const OVERFLOWB: u8 = 6;
pub const NEGATIVEB: u8 = 7;
// pre-defined architecture registers
pub const RESETVECLOW: u8 = 0xf0;
pub const RESETVECHIGH: u8 = 0xff;
pub const INTERRUPVECTLOW: u8 = RESETVECLOW;
pub const INTERRUPTVECHIGH: u8 = RESETVECHIGH;

pub struct Apple1 {
    pub fname: String,
}

impl Apple1 {
    pub fn new(fname: String) -> Self {
        Apple1 { fname }
    }

    pub fn runtime(&self) {
        println!("apple1");
        println!("attempting to open {}...", self.fname);
        let mut file = match File::open(&self.fname) {
            Err(why) => panic!("ERR: {}", why),
            Ok(file) => file,
        };
        let mut buf = String::new(); // allocate string buffer
        file.read_to_string(&mut buf); // fill buffer
        let cleaned = buf
            .lines()
            .map(|l| l.trim_start())
            .collect::<Vec<_>>()
            .join("\n");
        println!("found!");
        println!("{}", cleaned);

        println!("tokenizing...");
        asm::compile(&cleaned);
        println!("executing...");
        let mut execute_code: Vec<u8> = load_bytes("mem.b");
        run_file(&mut execute_code);
    }
}
pub fn load_bytes(path: &str) -> Vec<u8> {
    let mut f = File::open(path).unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap();
    return buf;
}
pub fn set_flag(mut flag_register: u8, flag_name: &u8, value: bool) -> u8 {
    //if the write value is high write 1
    if value {
        flag_register |= 1 << flag_name;
    } else {
        //else write zero
        flag_register |= 0 << flag_name;
    }
    //return the register
    return flag_register;
}

pub fn run_file(bytes_given: &mut Vec<u8>) -> bool {
    println!("--------------");
    //emulation variables
    let mut a: u8 = 0;
    let mut x: u8 = 0;
    let mut y: u8 = 0;
    let mut stack_pointer: u16 = 0xfd;
    let mut program_counter: u8 = 0;
    let mut flag_register: u8 = 0b00000000;
    let ram: &mut Vec<u8> = bytes_given; //for somereason there's an unused always high
    // bit value in the flag register
    flag_register = set_flag(flag_register, &UNUSUEDB, true);
    //interrupt so that when the computer is started it
    // is essentially reset
    flag_register = set_flag(flag_register, &INTERRUPTB, true);
    println!("{}", flag_register);
    while true {
        let mut program_counter_change: u8 = 0;
        let opcode_given = ram[program_counter as usize];
        let next_byte = ram[(program_counter + 1) as usize];
        let next_next_byte = ram[(program_counter + 2) as usize];
        program_counter_change = execute_opcode(
            opcode_given,
            &mut a,
            &mut x,
            &mut y,
            stack_pointer,
            next_byte,
            next_next_byte,
            ram,
            &mut flag_register,
        );
        println!("{}", a);
        program_counter += program_counter_change;
    }

    return true;
}

pub fn execute_opcode(
    opcode_given: u8,
    a: &mut u8,
    x: &mut u8,
    y: &mut u8,
    stack_pointer: u16,
    next_byte: u8,
    next_next_byte: u8,
    ram: &mut Vec<u8>,
    flag_register: &mut u8,
) -> u8 {
    let mut push_program_counter: u8 = 0;
    match opcode_given {
        //LDA instruction block
        0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
            //here we will match and find a value for lda that
            // will be outputted in LDA match
            let lda_match: u8 = match opcode_given {
                0xA9 => next_byte,                                                     //LDA imm
                0xA5 => ram[next_byte as usize],                                       //LDA zpg
                0xB5 => ram[(next_byte + *x) as usize],                                //LDA zpgx
                0xAD => ram[u16::from_le_bytes([next_byte, next_next_byte]) as usize], //LDA abs
                0xBD => ram[(u16::from_le_bytes([next_byte, next_next_byte]) + *x as u16) as usize], //LDA absx
                0xB9 => ram[(u16::from_le_bytes([next_byte, next_next_byte]) + *y as u16) as usize], //LDA absy
                0xA1 => {
                    ram[ram[(u16::from_le_bytes([next_byte, next_next_byte]) + *x as u16) as usize]
                        as usize] //LDA indx
                }

                0xB1 => {
                    ram[ram[(u16::from_le_bytes([next_byte, next_next_byte]) + *x as u16) as usize]
                        as usize] //LDA indy
                }
                _ => 0x00,
            };
            //set the flags according
            if lda_match == 0 {
                //lda match's value is 0 so set the flag to zero
                _ = set_flag(*flag_register, &ZEROB, true);
            } else {
                _ = set_flag(*flag_register, &ZEROB, false);
            }
            if lda_match >= 0x80 {
                //bit seven is on, so it's a two bit comp neg number
                // and let's set the register to be 1
                _ = set_flag(*flag_register, &NEGATIVEB, true);
            } else {
                _ = set_flag(*flag_register, &NEGATIVEB, false);
            }
            *a = lda_match;
            push_program_counter += 2;
            if opcode_given == 0xAD || opcode_given == 0xB9 || opcode_given == 0xBD {
                push_program_counter += 1;
            }
        }
        _ => {
            panic!("unidentified opcode!")
        }
    }
    return push_program_counter;
}
