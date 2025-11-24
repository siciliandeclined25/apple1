use std::io; // Import the standard I/O library
mod apple1;
use apple1::Apple1;

fn main() {
    println!("apple1 rust rewrite -- lucas frias\nplease enter a filename");
    let mut userInput = String::new();
    io::stdin()
        .read_line(&mut userInput)
        .expect("Failed to read line");
    //create an instance of the apple1 class with the parameter of the file
    // in the cwd
    let a1 = apple1::Apple1::new(userInput.trim().to_string());
    a1.runtime();
}
