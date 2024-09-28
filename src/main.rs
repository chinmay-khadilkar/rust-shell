#[allow(unused_imports)]
use std::io::{self, Write};
let commands: Vec<str> = Vec!["echo", "type", "exit"]
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
        } else if input.trim().contains("echo") {
            println!("{}", input.trim().split(" ").collect::<Vec<&str>>()[1..].join(" "));
        } else if input.trim().contains("type") {
            if commands.iter().any(|e| input.trim().contains(e)) {
                let cmd = input.trim().split(" ").collect::<Vec<&str>>()[1..].join("");
                println!("{} is a builtin command", cmd);
            } else {
                println!("{}: command not found", input.trim());
            }
        } else {
            println!("{}: command not found", input.trim());
        }
        
    }
}
