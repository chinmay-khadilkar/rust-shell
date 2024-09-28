#[allow(unused_imports)]
use std::io::{self, Write};
fn main() {
    // Uncomment this block to pass the first stage
    loop {
        print!("$ ");
        
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        if input.trim() == String::from("exit 0") {
            break;
        } else if input.contains("echo") {
            println!(input.trim().split(" ").collect::<Vec<&str>>()[1..].join(" "));
        } else {
            println!("{}: command not found", input.trim());
        }
        
    }
}
