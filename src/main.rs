use std::io::{self, Write};
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut ipt = String::new();
        let _ = io::stdin().read_line(&mut ipt);
        ipt.trim_end();
        ipt.pop(); // \n

        let args = ipt.split(" ").collect::<Vec<_>>();
        
        let valid_cmds = vec!["exit", "echo", "type"];

        match args[0] {
            "exit" => break,
            "echo" => println!("{}", ipt.replacen(args[0], "", 1).trim_start()),
            "type" => {
                let cmd = args[1];
                if valid_cmds.iter().find(|&&x| x==cmd).is_some() {
                    println!("{} is a shell builtin", cmd);
                } else {
                    let path_var = env::var("PATH");
                    match path_var {
                        Ok(path_val) => {
                            // println!("{:?}", path);
                            let paths = path_val.split(":");
                            let mut exists = false;
                            'main: for path in paths {
                                let files = fs::read_dir(path);
                                match files {
                                    Ok(files) => {
                                        // println!("{:?}", files)
                                        for file in files {
                                            if let Ok(file_path_dir_entry) = file {
                                                let file_path_bind = file_path_dir_entry.path();
                                                let file_path = file_path_bind.to_str().unwrap();
                                                let file_name = file_path.split("/").last().unwrap();
                                                // println!("{}", file_name)
                                                
                                                if file_name == cmd {
                                                    let permissions = file_path_dir_entry.metadata().unwrap().permissions().mode();
                                                    if permissions == 33261 { // many exec files returned this.
                                                        println!("{cmd} is {file_path}");
                                                        exists = true;
                                                        break 'main; // breaking so that it marks only one path
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    _ => ()
                                }
                            }
                            if !exists {
                                println!("{}: not found", cmd);
                            }
                        },
                        _ => eprintln!("{}: not found", cmd)
                    }
                }
            },
            _ => eprintln!("{}: command not found",args[0])
        };
    }
}
