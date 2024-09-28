#[allow(unused_imports)]
use std::io::{self, Write};
const commands:Vec<str> = Vec!["echo", "type", "exit"];
fn main() {
    // Uncomment this block to pass the first stage
    let commands: Vec<String> = vec!["echo".to_string(), "type".to_string(), "exit".to_string()];
    loop {
        print!("$ ");
        
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let cmd_args = input.trim().split(" ").collect::<Vec<&str>>();
        let cmd = cmd_args[0];
        let args = cmd_args[1..].join(" ");
        if input.trim() == String::from("exit 0") {
            break;
        }  else if cmd == String::from("type") {
            // let query_cmd = args.join("");
            if  commands.contains(&args) {
                println!("{} is a builtin command", args);
            } else {
                println!("{}: command not found", args);
            }
        } else if cmd == String::from("echo") {
            println!("{}", args);
        } else {
            println!("{}: command not found", input.trim());
        }
        
    }
}
