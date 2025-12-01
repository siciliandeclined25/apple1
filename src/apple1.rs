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
pub const DEBUG: bool = true;
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

        println!("tokenizing...");
        asm::compile(&cleaned);
        println!("executing...");
        let mut execute_code: Vec<u8> = load_bytes("out.bin");
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
    let mut program_counter: u16 = 0;
    let mut flag_register: u8 = 0b00000000;
    let ram: &mut Vec<u8> = bytes_given; //for somereason there's an unused always high
    // bit value in the flag register
    flag_register = set_flag(flag_register, &UNUSUEDB, true);
    //interrupt so that when the computer is started it
    // is essentially reset
    flag_register = set_flag(flag_register, &INTERRUPTB, true);
    let mut i: u32 = 0;
    while true {
        i += 1;
        let mut program_counter_change: u16 = 0;
        let opcode_given = ram[program_counter as usize];
        let next_byte = ram[(program_counter + 1) as usize];
        let next_next_byte = ram[(program_counter + 2) as usize];
        program_counter_change = execute_opcode(
            opcode_given,
            &mut a,
            &mut x,
            &mut y,
            &mut stack_pointer,
            next_byte,
            next_next_byte,
            ram,
            &mut flag_register,
            &mut program_counter,
        );
        program_counter = program_counter_change;
    }

    return true;
}

pub fn execute_opcode(
    opcode_given: u8,
    a: &mut u8,
    x: &mut u8,
    y: &mut u8,
    stack_pointer: &mut u16,
    next_byte: u8,
    next_next_byte: u8,
    ram: &mut Vec<u8>,
    flag_register: &mut u8,
    program_counter: &mut u16,
) -> u16 {
    let mut push_program_counter: u16 = 0;
    match opcode_given {
        0x00 => {
            if DEBUG {
                println!("BRK")
            }
            //BRK impl
            // first standard behavior, push the pc + 1 to the stack
            let pc_to_push_stack: u16 = *program_counter + 1;
            let lo: u8 = (pc_to_push_stack & 0x00ff) as u8;
            let hi: u8 = (pc_to_push_stack >> 8) as u8;
            push_to_stack(ram, hi, lo, stack_pointer);
            // secondly we push the register of the program
            *flag_register = set_flag(*flag_register, &BREAKB, true);
            *flag_register = set_flag(*flag_register, &INTERRUPTB, true);
            push_to_stack(ram, *flag_register, 0x00, stack_pointer);
            //push to a stack and then let's move the stack pointer back one
            // because this function takes twice the parameters
            *stack_pointer += 1;
            //should be okay, now finally load the BRK interrupt pointer
            let push_program_counter: u16 = u16::from_le_bytes([ram[0xFFFE], ram[0xFFFF]]);
        }
        //STX STY
        0x86 | 0x96 | 0x8E | 0x84 | 0x94 | 0x8C => {
            if DEBUG {
                println!("STX/STY")
            }
            if opcode_given == 0x86 || opcode_given == 0x96 || opcode_given == 0x8E {
                //stx instructions
                let where_to_write_stx: u8 = match opcode_given {
                    0x86 => ram[next_byte as usize],        //zpg
                    0x96 => ram[(next_byte + *y) as usize], //zpg y
                    0x8E => ram[u16::from_le_bytes([next_byte, next_next_byte]) as usize], //abs
                    _ => 0x00,
                };
                //set the location equal to x
                ram[where_to_write_stx as usize] = *x;
            }
            if opcode_given == 0x84 || opcode_given == 0x94 || opcode_given == 0x8C {
                //sty instructions
                let where_to_write_stx: u8 = match opcode_given {
                    0x86 => ram[next_byte as usize],        //zpg
                    0x96 => ram[(next_byte + *x) as usize], //zpg x
                    0x8E => ram[u16::from_le_bytes([next_byte, next_next_byte]) as usize], //abs
                    _ => 0x00,
                };
                //set the location equal to x
                ram[where_to_write_stx as usize] = *y;
            }
            //opcode is abs
            if opcode_given == 0x8C || opcode_given == 0x8E {
                push_program_counter += 3;
            } else {
                push_program_counter += 2;
            }
        }
        //JMP block
        0x4C | 0x6C => {
            if opcode_given == 0x4C {
                push_program_counter = u16::from_be_bytes([next_byte, next_next_byte]);
            } else {
                let jump_indirect_address: u16 = u16::from_le_bytes([next_byte, next_next_byte]);
                let lo: u8 =  ram[jump_indirect_address as usize];
                let hi: u8 = ram[(jump_indirect_address + 1) as usize];
                push_program_counter =;
            }
        }
        //Register control block
        0xAA | 0x8A | 0xCA | 0xE8 | 0xA8 | 0x98 | 0x88 | 0xC8 => {
            if DEBUG {
                println!("register control block")
            }
            let register_ctrl_match: () = match opcode_given {
                //TAX & INX and DEX
                0xAA | 0xCA | 0xE8 => {
                    if opcode_given == 0xAA {
                        *a = *x; //TAX
                    } else {
                        if opcode_given != 0xe8 {
                            *x = x.saturating_sub(1); //INX
                        } else {
                            *x = x.saturating_add(1); //DEX
                        }
                    }
                }
                //TAY & INY and DEY
                0xA8 | 0x88 | 0xC8 => {
                    if opcode_given == 0xAA {
                        *a = *y; //TAY
                    } else {
                        if opcode_given != 0xe8 {
                            *y = y.saturating_sub(1); //INY
                        } else {
                            *y = y.saturating_add(1); //DEY
                        }
                    }
                }
                //TXA & TYA
                0x8A | 0x98 => {
                    if opcode_given == 0x8A {
                        *x = *a
                    } else {
                        *y = *a
                    }
                }
                _ => (),
            };
            //inc the word
            push_program_counter += 1;
        }
        //JSR and RTS
        0x60 | 0x20 => {
            if DEBUG {
                println!("JSR/RTS")
            }
            if 0x20 == opcode_given {
                //JSR
                let [lo, hi] = program_counter.to_le_bytes(); // little-endian order
                push_to_stack(ram, hi, lo, stack_pointer);
                push_program_counter += 3;
            } else {
                //RTS
                let hi: u8 = ram[*program_counter as usize];
                let lo: u8 = ram[(*program_counter - 1) as usize];
                *stack_pointer -= 2;
                let bytes: [u8; 2] = [lo, hi];
                push_program_counter = u16::from_le_bytes(bytes);
            };
        }
        //LDA instruction block
        0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
            //here we will match and find a value for lda that
            // will be outputted in LDA match
            if DEBUG {
                println!("LDA")
            }
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
            panic!("unidentified opcode -- {:#x}", opcode_given)
        }
    }
    return push_program_counter;
}
pub fn push_to_stack(
    ram: &mut Vec<u8>,
    next_byte: u8,
    next_next_byte: u8,
    stack_pointer: &mut u16,
) {
    //gabe forgive me
    if &stack_pointer < &&mut 0x0100 && &stack_pointer > &&mut 0x01FF {
        *stack_pointer -= 2;
    }
    //now we just set the ram at the correct low/high byte position
    ram[(*stack_pointer) as usize] = next_byte;
    ram[(*stack_pointer - 1) as usize] = next_next_byte;
}
