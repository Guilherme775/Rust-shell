use std::{env, io::{BufRead, Write, stdin, stdout}, path::Path, process::Command};

fn main() {
    loop {
        print!("> ");
        stdout().flush().unwrap();
        
        let input = stdin().lock().lines().next().unwrap().expect("IO error");

        let parts = input.trim().split_whitespace().collect::<Vec<&str>>();
        let command = &parts.first().expect("invalid input");
        let args = &parts[1..parts.len()];

        match command.as_ref() {
            "cd" => {
                let path = Path::new(args[0]);
                env::set_current_dir(path).expect("invalid path");
            },
            command => {
                let child = Command::new(command).args(args).spawn();
                
                match child {
                    Ok(mut child) => { 
                        child.wait().expect("invalid command"); 
                    },
                    Err(e) => panic!("error: {:?}", e),
                };
            }
        }
    }
}
