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
                let cmd = args[1]; // TODO: what if user don't provide
                if valid_cmds.iter().find(|&&x| x==cmd).is_some() {
                    println!("{} is a shell builtin", cmd);
                } else {
                    let path_val = env::var("PATH").unwrap();
                    let paths = path_val.split(":");
                    let mut exists = false;
                    'main: for path in paths {
                        let files = fs::read_dir(path);
                        if let Ok(files) = files {
                            for file in files {
                                if let Ok(file_path_dir_entry) = file {
                                    let file_path_bind = file_path_dir_entry.path();
                                    let file_path = file_path_bind.to_str().unwrap();
                                    let file_name = file_path.split("/").last().unwrap();
                                                
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
                        }
                    }
                    if !exists {
                        println!("{}: not found", cmd);
                    }
                }
            },
            _ => eprintln!("{}: command not found",args[0])
        };
    }
}
