use std::fs::File;
use std::io::{Read, Write};
mod asm;
use asm::compile;

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
        println!("found!");
        println!("{}", &buf);
        println!("tokenizing...");
        asm::compile(&buf);
    }
}
