#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::fs;
use std::process::Command;
fn main() {
    // Uncomment this block to pass the first stage
    let commands: Vec<String> = vec!["echo".to_string(), "type".to_string(), "exit".to_string(), "pwd".to_string(), "cd".to_string()];
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
        if cmd == String::from("cd") {
            let mut change_path = String::from("");
            if args == String::from("~") {
                change_path = env::var("HOME").unwrap();
            } else {
                change_path = args;
            }
            let change_dir_out = env::set_current_dir(&change_path);
            match change_dir_out {
                Ok(change) => continue,
                Err(_) => { 
                    println!("cd: {}: No such file or directory", change_path) 
                }
            }
        } else if cmd == String::from("pwd") {
            let path = env::current_dir();
            match path {
                Ok(pth) => {
                    println!("{}", pth.display());
                }
                Err(_) => ()
            }
        } else if is_cmd_executable {
            let mut full_path = String::from("");
            for each_path in exec_path.iter() {
                let complete_path = format!("{}/{}", each_path, cmd);
                if fs::metadata(&complete_path).is_ok() {
                    full_path = complete_path;
                }
            }
            
            // let complete_cmd = exec_path[exec_path_index];
            let exec_out = Command::new(full_path).args(args.split(" ")).output();
            match exec_out {
                Ok(output) => {
                    if output.status.success() {
                        let console_out = &String::from_utf8_lossy(&output.stdout);
                        let console = console_out.trim_end();
                        println!("{}", console);
                    } else {
                        let console_out = String::from_utf8_lossy(&output.stdout);
                        println!("Command failed with error:\n{}", console_out);
                    }
                }
                Err (_) => {
                    println!("{}: command not found", cmd);
                }
            }
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