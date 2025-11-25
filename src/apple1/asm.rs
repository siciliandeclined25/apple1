use std::collections::HashMap;
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
pub const MAXINSTRUCTIONHEIGHT: u8 = 12;

pub fn get_opcode(namespace: &str, addressingMode: u8) -> u8 {
    //A function to return the opcode given the namespace of the function
    // (command, 3 byte String buffer, borrowed)
    // & (addressingMode, u8, defined by constants)
    match addressingMode {
        IMMEDIATE => match namespace {
            "LDA" => 0xA9,
            "ADC" => 0x69,
            "AND" => 0x29,
            "CMP" => 0xC9,
            "CPX" => 0xE0,
            "CPY" => 0xC0,
            "EOR" => 0x49,
            "LDX" => 0xA2,
            "LDY" => 0xA0,
            "ORA" => 0x09,
            "SBC" => 0xE9,
            _ => 0xFF, //nop
        },
        IMPLIED => match namespace {
            "BRK" => 0x00,
            "CLC" => 0x18,
            "CLD" => 0xD8,
            "CLI" => 0x58,
            "CLV" => 0xB8,
            "DEX" => 0xCA,
            "DEY" => 0x88,
            "INX" => 0xE8,
            "INY" => 0xC8,
            "NOP" => 0xEA,
            "PHA" => 0x48,
            "PHP" => 0x08,
            "PLA" => 0x68,
            "PLP" => 0x28,
            "RTI" => 0x40,
            "RTS" => 0x60,
            "SEC" => 0x38,
            "SED" => 0xF8,
            "SEI" => 0x78,
            "TXA" => 0x8A,
            "TXS" => 0x9A,
            "TYA" => 0x98,
            "TSX" => 0xBA,
            _ => 0xFF, //nop
        },
        ACCUMULATOR => match namespace {
            "ASL" => 0x0A,
            "LSR" => 0x4A,
            "ROL" => 0x2A,
            "ROR" => 0x6A,
            _ => 0xFF, //nop
        },
        RELATIVE => match namespace {
            "BCC" => 0x90,
            "BCS" => 0xB0,
            "BEQ" => 0xF0,
            "BMI" => 0x30,
            "BNE" => 0xD0,
            "BPL" => 0x10,
            "BVC" => 0x50,
            "BVS" => 0x70,
            _ => 0xFF, //nop
        },
        ABSOLUTE => match namespace {
            "ADC" => 0x6D,
            "AND" => 0x2D,
            "ASL" => 0x0E,
            "BIT" => 0x2C,
            "CMP" => 0xCD,
            "CPX" => 0xEC,
            "CPY" => 0xCC,
            "DEC" => 0xCE,
            "EOR" => 0x4D,
            "INC" => 0xEE,
            "JMP" => 0x4C,
            "JSR" => 0x20,
            "LDA" => 0xAD,
            "LDX" => 0xAE,
            "LDY" => 0xAC,
            "LSR" => 0x4E,
            "ORA" => 0x0D,
            "ROL" => 0x2E,
            "ROR" => 0x6E,
            "SBC" => 0xED,
            "STA" => 0x8D,
            "STX" => 0x8E,
            "STY" => 0x8C,
            _ => 0xFF, //nop
        },
        XABSOLUTE => match namespace {
            "ADC" => 0x7D,
            "AND" => 0x3D,
            "ASL" => 0x1E,
            "CMP" => 0xDD,
            "EOR" => 0x5D,
            "INC" => 0xFE,
            "LDA" => 0xBD,
            "LDX" => 0xBE,
            "LDY" => 0xBC,
            "LSR" => 0x5E,
            "ORA" => 0x1D,
            "ROL" => 0x3E,
            "ROR" => 0x7E,
            "SBC" => 0xFD,
            "STA" => 0x9D,
            _ => 0xFF,
        },
        YABSOLUTE => match namespace {
            "ADC" => 0x79,
            "AND" => 0x39,
            "CMP" => 0xD9,
            "EOR" => 0x59,
            "LDA" => 0xB9,
            "LDX" => 0xB8,
            "ORA" => 0x19,
            "SBC" => 0xF9,
            "STA" => 0x99,
            _ => 0xFF,
        },
        ZEROPAGE => match namespace {
            "ADC" => 0x65,
            "AND" => 0x25,
            "ASL" => 0x06,
            "BIT" => 0x24,
            "CMP" => 0xC5,
            "CPX" => 0xE4,
            "CPY" => 0xC4,
            "DEC" => 0xC6,
            "EOR" => 0x45,
            "INC" => 0xE6,
            "LDA" => 0xA5,
            "LDX" => 0xA6,
            "LDY" => 0xA4,
            "LSR" => 0x46,
            "ORA" => 0x05,
            "ROL" => 0x26,
            "ROR" => 0x66,
            "SBC" => 0xE5,
            "STA" => 0x85,
            "STX" => 0x86,
            "STY" => 0x84,
            _ => 0xFF,
        },
        XZEROPAGE => match namespace {
            "ADC" => 0x75,
            "AND" => 0x35,
            "ASL" => 0x16,
            "CMP" => 0xD5,
            "DEC" => 0xD6,
            "EOR" => 0x55,
            "INC" => 0xF6,
            "LDA" => 0xB5,
            "LDY" => 0xB4,
            "LSR" => 0x56,
            "ORA" => 0x15,
            "ROL" => 0x36,
            "ROR" => 0x76,
            "SBC" => 0xF5,
            "STA" => 0x95,
            "STY" => 0x94,
            _ => 0xFF,
        },

        YZEROPAGE => match namespace {
            "LDX" => 0xB6,
            "STX" => 0x96,
            _ => 0xFF,
        },
        INDIRECT => match namespace {
            "JMP" => 0x6c,
            _ => 0xFF,
        },
        XINDIRECT => match namespace {
            "ORA" => 0x01,
            "AND" => 0x21,
            "EOR" => 0x41,
            "ADC" => 0x61,
            "STA" => 0x81,
            "LDA" => 0xA1,
            "CMP" => 0xC1,
            "SBC" => 0xE1,
            _ => 0xFF,
        },
        YINDIRECT => match namespace {
            "ORA" => 0x11,
            "AND" => 0x31,
            "EOR" => 0x51,
            "ADC" => 0x71,
            "STA" => 0x91,
            "LDA" => 0xB1,
            "CMP" => 0xD1,
            "SBC" => 0xF1,
            _ => 0xFF,
        },
        _ => 0xFF, //handle non-numeric assignments
    }
}
pub fn get_address_as_string(addressint: u8) -> &'static str {
    return match addressint {
        0 => "IMM (immediate)",
        1 => "IMP (implied)",
        2 => "ACC (accumulator)",
        3 => "REL (relative)",
        4 => "ABS (absolute)",
        5 => "XABS (absolute + x)",
        6 => "YABS (absolute + y)",
        7 => "ZPG (zeropage)",
        8 => "XZPG (zeropage + x)",
        9 => "YZPG (zeropage + y)",
        10 => "IND (indirect)",
        11 => "XIND (indirect + x)",
        12 => "YIND (indirect + y)",
        _ => "no addressing mode found",
    };
}
pub fn is_ascii_letters_only(char_vector: Vec<char>) -> bool {
    for maybe_letter_char in &char_vector {
        //turn the letter of the ascii_letter_char into
        // an unsigned integer by accessing the pointer value
        // of the protected borrowed vector and interperting
        // it's byte value as an unsigned integer (works only
        // with 1 byte encoded ascii and is not utf-8 compatible
        let ascii_of_letter = *maybe_letter_char as u8;
        //check if it's within the range of valid ascii letters
        // from A-Z then a-z
        if !(ascii_of_letter >= 65
            || ascii_of_letter <= 90
            || ascii_of_letter >= 97
            || ascii_of_letter <= 122)
        {
            return false;
        }
    }
    //if false function would have aleady returned false so now returns true
    return true;
}
pub fn get_addressing_mode(char_of_line: Vec<char>) -> u8 {
    let mut instruction_type = MAXINSTRUCTIONHEIGHT + 1;
    if char_of_line[4] == '#' {
        //now we assign immediate addressing
        //LDA #$03 or (A <-- 3)
        instruction_type = IMMEDIATE;
    } else if char_of_line[4] == '$' {
        //character could be relative or absolute
        if char_of_line[char_of_line.len() - 1] == 'X' && char_of_line.len() == 11 {
            instruction_type = XABSOLUTE;
        }
        if char_of_line[char_of_line.len() - 1] == 'Y' && char_of_line.len() == 11 {
            instruction_type = YABSOLUTE;
        }
        if char_of_line.len() == 9 {
            //absolute addressing (LDA $DO12) or A <-- D012
            instruction_type = ABSOLUTE;
            //well let's see if it is a zeropage (that could be true too)
            if char_of_line[char_of_line.len() - 1] == 'X' {
                instruction_type = XZEROPAGE;
            }
            if char_of_line[char_of_line.len() - 1] == 'Y' {
                instruction_type = YZEROPAGE;
            }
        }
        if char_of_line.len() == 6 {
            //this means it's a simple zeropage instruction
            instruction_type = ZEROPAGE;
        }
    } else if is_ascii_letters_only(char_of_line) {
        //quick cheap trick-- all relative instructions
        //will syntaxically only use upper and lowercase ascii
        //so we can simplify the assembly this way.
        instruction_type = RELATIVE;
    } else {
        instruction_type = MAXINSTRUCTIONHEIGHT + 1; //error instruction
    }
    return instruction_type;
}

pub fn compile(buf: &str) -> &str {
    //first part, let's split the buffer to
    // a vector based on \n
    let mut tokenList: Vec<u8> = Vec::new();
    let parts: Vec<&str> = buf.lines().collect();
    let mut assembled_code: Vec<u8> = Vec::new();
    //this acts as a pointer which moves through the program
    // and is used for relative address creation/compiling
    let mut program_simulated_pointer: u16 = 0;
    //the u16 type it to represent memory addresses where the namespace is represented
    //this hashmaps contains all places where this is referenced in pointers to the memory
    // address where the assembler should fill back in
    let mut labels_n_references_hashmap: HashMap<&str, u16> = HashMap::new();
    //this hashmap represents the label and it's location and is assembled while being compiled
    let mut labels_n_locations_hashmap: HashMap<&str, Vec<u16>> = HashMap::new();

    //second, let's iterate through the line
    // through each iteration we will append the correct byte(s)
    // to the vector of unsigned integers
    println!("\nat the disco -- apple 1 6502 asm\n");
    println!("starting write at ( {:#x} )", program_simulated_pointer);
    //warning on writing to reserved memory addresses
    if !((0x0800..=0xbfff).contains(&program_simulated_pointer)
        || (0xc000..=0xcfff).contains(&program_simulated_pointer)
        || (0xd000..=0xdfff).contains(&program_simulated_pointer)
        || (0xe000..=0xfeff).contains(&program_simulated_pointer))
    {
        println!(
            "at the disco: warning 1\nyou are writing into space that is typically reserved, unavailable or changed during normal machine states (zpg, wozmon, the stack!!!)\nthis could result in catastrophic program errors. this is not the compilier's fault-- it's yours.\nconsider changing your address to a different starting address in free RAM"
        )
    }
    for line in parts {
        println!("---( {:#x} )----", program_simulated_pointer);
        if line.len() == 0 {
            continue; //no instruction
        } else if line.chars().last() == Some(':') {
            //we found a label boys
            let found_label: &str = &line[..line.len() - 1];
            if !labels_n_references_hashmap.contains_key(found_label) {
                //let's add it to the hashmap
                println!("at the disco: label found ({})", &found_label);

                labels_n_references_hashmap.insert(found_label, program_simulated_pointer);
            }
            //otherwise it does exist and no action is needed
        } else {
            // yes instruction exists
            println!("{}", &line);
            let byte_index_third_char = line.char_indices().nth(3).unwrap().0;
            let opcode_namespace: &str = &line[..byte_index_third_char];
            //the plus one is to offset the character space between the opcode and instruction
            let address_in_string: &str = &line[byte_index_third_char + 1..];
            let char_of_line: Vec<char> = line.chars().collect(); //makes std:str:char iterator
            let mut instruction_type: u8 = get_addressing_mode(char_of_line);
            //let's determine what type of instruction it is
            //all 6502 instructions are 3 chars one space then the addressing
            //addressing is immediate
            //finished determining instructiontype, return it
            let instruction_type_string: &'static str = get_address_as_string(instruction_type);
            println!("using addressing mode : {}", instruction_type_string);
            //now let's use the lookup tables to find the appropriate opcode
            // for the namespace using the addressing mode, if applicable
            //let's use the "namespace" or the first three characters
            // which will always be that combined with the opcode
            let correct_opcode = get_opcode(opcode_namespace, instruction_type);
            println!(
                "correct opcode for addressing mode\n {} ----> {:#x}",
                opcode_namespace, correct_opcode
            );
            //we have an opcode, so let's go +1
            println!(
                "ADDR: {:#x} ----> {:#x}",
                program_simulated_pointer, correct_opcode
            );

            let opcode_pushed: &u8 = &correct_opcode;
            assembled_code.push(correct_opcode);
            program_simulated_pointer += 1;
            //bye-bye correct_opcode hello pushed_opcode (in case this is needed later)
            //this increment sets us at the correct spacing. we've written
            if correct_opcode == 0xFF {
                //handles something going wrong
                // 0xFF isn't valid 6502 opcode silly goose
                panic!(
                    "at the disco: malformed addressing mode
                    -> error 1\nsuitable opcode {} for was not found for addressing mode {}.
                    \nare you sure you're using the right addressing mode?",
                    opcode_namespace, instruction_type_string
                );
            }
            //here's what we have so far
            // the *correct addressing mode
            // the correct opcode for the namepsace with the addresisng mode
            // now let's format the data into the correct data to append to the vector
            // the way i see it there's really three types
            // first is relative data, which is the hardest!!!
            if instruction_type == RELATIVE {
                //we're a relative address
                //we need to append this in the area of bytes to change later!!
                // we expect there to be a label (no one is manually typing in
                // bytes for a label at least not on my assembler)
                // but first regardless of what we have this should be overwritten but
                // we need to push a byte anyways
                assembled_code.push(0x00);
                if labels_n_locations_hashmap.contains_key(&address_in_string) {
                    //weird convention but it's like with as in python
                    // we're getting a mutable reference to the Vec
                    // which i believe means that we allocate memory
                    // that then can be pushed back to the pointer
                    println!(
                        "ADDR: {:#x} ----> {:#x}",
                        program_simulated_pointer,
                        program_simulated_pointer - 1 // we have to use minus two in order to return from the relative addr and opcode associated
                    );
                    if let Some(vector) = labels_n_locations_hashmap.get_mut(&address_in_string) {
                        //vector is a Vec<u16> with every address
                        vector.push(program_simulated_pointer);
                        //we just push this to the vector stored inside
                        //weird conventioning but so memory safe!!!!
                    }
                    program_simulated_pointer += 1;
                } else {
                    println!(
                        "ADDR: {:#x} ----> 0x00\nat the disco -- ^^^^^ warning 2\nthe assembler is using a stopgap. this is not the correct relative address \nthis is a temporary relative address\nafter basic tokenization the assembler will come back and fix this. if your program unexpectedly breaks, then your label is not defined in memory\n the assembler will attempt to fix this for you",
                        program_simulated_pointer
                    );
                    //memory_pointer_vect creates a new memory vect and allow us to store a
                    // new key/value pair into the hashmap
                    // first we'll create it
                    let mut memory_pointer_vect: Vec<u16> = Vec::new();
                    // then we'll push the program pointer (where the program is)
                    memory_pointer_vect.push(program_simulated_pointer);
                    //lastly, we'll append the key/value pair where
                    // key: &str (namespace of label)
                    // value: Vec<u16> (memory_pointer)
                    labels_n_locations_hashmap.insert(&address_in_string, memory_pointer_vect);
                }
            } else if (instruction_type == XABSOLUTE
                || instruction_type == YABSOLUTE
                || instruction_type == ABSOLUTE
                || instruction_type == INDIRECT)
            {
                //this means we have to assemble the text into two parrts
                // and flip the order(little endian) so that it's
                // [opcode] [low] [high]
                // see comments on other addressing for explanation on code
                let hexish: String = address_in_string
                    .chars()
                    .filter(|c| c.is_ascii_digit() || matches!(c, 'a'..='f' | 'A'..='F'))
                    .collect();
                let signed_address = u16::from_str_radix(&hexish, 16).unwrap();
                let [lo, hi] = signed_address.to_le_bytes(); // little-endian order
                println!(
                    "ADDR: {:#x} ----> {:#x}\nADDR: {:#x} ----> {:#x}",
                    program_simulated_pointer,
                    lo,
                    program_simulated_pointer + 1,
                    hi
                );
                program_simulated_pointer += 2;
            //now we must turn the signed_address into two u8 numbers
            } else {
                //we have just a normal value, nothing fancy, single 1 byte unsigned addressing
                //turns and iterates gets if char ought to have filter and then matches on capital/lower
                let hexish: String = address_in_string
                    .chars()
                    .filter(|c| c.is_ascii_digit() || matches!(c, 'a'..='f' | 'A'..='F'))
                    .collect();
                //now converts &hexish reference to unsigned_address
                let unsigned_address = u8::from_str_radix(&hexish, 16).unwrap();
                //let's push it
                assembled_code.push(unsigned_address);
                //increment the program counter
                println!(
                    "ADDR: {:#x} ----> {:#x}",
                    program_simulated_pointer,
                    unsigned_address // we have to use minus two in order to return from the relative addr and opcode associated
                );
                program_simulated_pointer += 1;
            }
        }
    }
    return buf;
}
