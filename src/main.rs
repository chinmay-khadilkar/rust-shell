#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
fn main() {
    // Uncomment this block to pass the first stage
    // let commands: Vec<String> = vec!["echo".to_string(), "type".to_string(), "exit".to_string()];
    let raw_path = env::var("PATH").unwrap();
    let exe_path = env::args().collect();
    println!("{}", exe_path);
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
        } else if cmd == String::from("type") {
            // let query_cmd = args.join("");
            let path: Vec<&str> = raw_path.split(":").collect::<Vec<&str>>();
            let path_index = path.clone().iter().position(|&r| r.contains(&args));
            match path_index {
                Some(pos) => println!("{} is {}", args, path[pos]),
                None => println!("{}: not found", args)
            }
        } else if cmd == String::from("echo") || cmd.trim() == String::from("") {
            println!("{}", args);
        } else {
            println!("{}: not found", input.trim());
        }
        
    }
}