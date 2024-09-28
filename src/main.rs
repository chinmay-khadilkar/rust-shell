#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::fs;
use std::process::Command;
fn main() {
    // Uncomment this block to pass the first stage
    let commands: Vec<String> = vec!["echo".to_string(), "type".to_string(), "exit".to_string()];
    let raw_path = env::var("PATH").unwrap();
    // let exe_path: Vec<String> = env::args().collect();
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
        let is_args_builtin = commands.iter().any(|cmd| *cmd == args);
        let exec_path: Vec<&str> = raw_path.split(":").collect();
        let is_cmd_executable = exec_path.iter().any(|&dir| {
            let full_path = format!("{}/{}", dir, cmd);
            fs::metadata(full_path).is_ok()
        });
        if is_cmd_executable {
            
            let exec_path_index = exec_path.iter().position(|&dir| {
                let full_path = format!("{}/{}", dir, cmd);
                fs::metadata(full_path).is_ok()
            }).unwrap();
            let complete_cmd = exec_path[exec_path_index];
            Command::new(complete_cmd).args(args).status().expect("failed to execute process");
        } else if input.trim() == String::from("exit 0") {
            break;
        } else if cmd == String::from("type") && is_args_builtin == false {
            // let query_cmd = args.join("");
            let path: Vec<&str> = raw_path.split(":").collect();
            let path_found = path.iter().any(|&dir| {
                let full_path = format!("{}/{}", dir, args);
                fs::metadata(full_path).is_ok()
            });
            if path_found {
                let path_index = path.iter().position(|&dir| {
                    let full_path = format!("{}/{}", dir, args);
                    fs::metadata(full_path).is_ok()
                }).unwrap();
                println!("{} is {}/{}", args, path[path_index], args)
            } else {
                println!("{}: not found", args)
            }
        } else if cmd == String::from("type") && is_args_builtin == true {
            // let pos = commands.iter().position(|&cmd| cmd == args);
            println!("{} is a shell builtin", args);
        } else if cmd == String::from("echo") || cmd.trim() == String::from("") {
            println!("{}", args);
        } else {
            println!("{}: not found", input.trim());
        }
        
    }
}