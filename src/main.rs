use std::io::{self, Write};
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

fn find_exec(exec_name: &str) -> Option<String> {
    let path_val = env::var("PATH").unwrap();
    let paths = path_val.split(":");
    for path in paths {
        let files = fs::read_dir(path);
        if let Ok(files) = files {
            for file in files {
                if let Ok(file_path_dir_entry) = file {
                    let file_path_bind = file_path_dir_entry.path();
                    let file_path = file_path_bind.to_str().unwrap();
                    let file_name = file_path.split("/").last().unwrap();

                    if file_name == exec_name {
                        let permissions = file_path_dir_entry.metadata().unwrap().permissions().mode();
                        if permissions == 33261 { // many exec files returned this.
                            return Some(file_path.to_string());
                        }
                    }
                }
            }
        }
    }
    None
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut ipt = String::new();
        let _ = io::stdin().read_line(&mut ipt);
        let _ = ipt.trim_end();
        ipt.pop(); // \n

        let args = ipt.split(" ").collect::<Vec<_>>();
        
        let valid_cmds = vec!["exit", "echo", "type"];

        match args[0] {
            "exit" => break,
            "echo" => println!("{}", ipt.replacen(args[0], "", 1).trim_start()),
            "type" => {
                let cmd = args[1]; // TODO: what if user doesn't provide
                if valid_cmds.iter().find(|&&x| x==cmd).is_some() {
                    println!("{} is a shell builtin", cmd);
                } else {
                    match find_exec(cmd){
                        Some(file_path) => println!("{cmd} is {file_path}"),
                        None => eprintln!("{}: not found", cmd)
                    }
                }
            },
            _ => {
                let exec = args[0];
                match find_exec(exec) {
                    Some(file_path) => {
                        // run exec
                        let mut args = args.clone();
                        args.remove(0);
                        let cmd_out = Command::new(file_path).args(args).output().expect("failed to execute");
                        let mut out = String::from_utf8(cmd_out.stdout).unwrap();
                        if exec != "clear" {
                            // rmvs \n which is useless but useful when does clear..
                            out.pop();
                        }
                        println!("{}", out)
                    }
                    None => eprintln!("{}: command not found", args[0])
                }
            }
        };
    }
}
