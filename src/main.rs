use std::io::{self, Write};
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::path::{Path, PathBuf};
use regex::Regex;

fn pathbuf_to_string(pathbuf: PathBuf) -> String {
    pathbuf.into_os_string().into_string().unwrap()
}

fn find_exec(exec_name: &str) -> Option<String> {
    let path_val = env::var("PATH").unwrap();
    let paths = path_val.split(":");
    for path in paths {
        let files = fs::read_dir(path);
        if let Ok(files) = files {
            for file in files {
                if let Ok(file_path_dir_entry) = file {
                    let file_path = pathbuf_to_string(file_path_dir_entry.path());
                    let file_name = file_path.split("/").last().unwrap();

                    if file_name == exec_name {
                        let permissions = file_path_dir_entry.metadata().unwrap().permissions().mode();
                        if permissions == 33261 { // many exec files returned 33261
                            return Some(file_path.to_string());
                        }
                    }
                }
            }
        }
    }
    None
}

fn split_args(args_str: &str) -> Vec<String> {
    let char_re = r"a-zA-Z0-9\/~\.\-_";
    let re = Regex::new(format!(r#"("[{char_re} ']*"|'[{char_re} ]*'|[{char_re}]*)* *"#).as_str()).unwrap();

    let args_iter = re.find_iter(args_str);
    let mut args: Vec<String> = Vec::new();

    for arg in args_iter {
        let arg_str = arg.as_str(); // match to str
        if arg_str.contains("\"") { // most prioritized
            let mut arg_string = arg_str.replace("\"", "");
            if arg_string.len() > 0 {
                loop { // running a loop here bcz "something   "    "hey" would be returned as "\"something   \"   " which is wrong as there are extra whitespaces that we don't need bt through ts loop we are removing all right side whitespaces, but problem is, it also rmvs whitespaces which is inside " TODO.. btw don't we need this in ' ?
                    let last_idx = arg_string.len() - 1;
                    if arg_string.as_bytes()[last_idx] == 32 { // if last substr contains space
                        arg_string.remove(last_idx);
                    } else {
                        break
                    }
                }
                args.push(arg_string)
            }
        } else if arg_str.contains("'") {
            let mut arg_string = arg_str.replace("'", "");
            if arg_string.len() > 0 {
                let last_idx = arg_string.len() - 1;
                if arg_string.as_bytes()[last_idx] == 32 { // if last substr contains space
                    arg_string.remove(last_idx);
                }
                args.push(arg_string)
            }
        } else {
            args.push(arg_str.replace(" ", ""));
        }
    }
    args
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut ipt = String::new();
        let _ = io::stdin().read_line(&mut ipt);
        let _ = ipt.trim_end();
        ipt.pop(); // \n

        let args_bind = split_args(&ipt);
        let mut args = args_bind.iter().map(|c|c.as_str()).collect::<Vec<&str>>();
        //println!("{:?}", args);
        
        let valid_cmds = vec!["exit", "echo", "type", "pwd", "cd"];

        // TODO: its panicking if user doesn't provide enough args
        match args[0] {
            "exit" => break,
            "echo" => {
                args.remove(0);
                println!("{}", args.join(" "));
            },
            "type" => {
                let cmd = args[1]; 
                if valid_cmds.iter().find(|&&x| x==cmd).is_some() {
                    println!("{} is a shell builtin", cmd);
                } else {
                    match find_exec(cmd){
                        Some(file_path) => println!("{cmd} is {file_path}"),
                        None => eprintln!("{}: not found", cmd)
                    }
                }
            },
            "pwd" => {
                let current_path = pathbuf_to_string(env::current_dir().unwrap());
                println!("{current_path}")
            },
            "cd" => {
                // for absolute and relative(set_current_dir automatically supports it) path
                let home_dir = pathbuf_to_string(env::home_dir().unwrap());
                let new_path_str = args[1].replacen("~", &home_dir, 1);
                let new_path = Path::new(&new_path_str);
                let moved_dir = env::set_current_dir(&new_path).is_ok();
                if !moved_dir {
                    eprintln!("cd: {}: No such file or directory", new_path_str);
                }
            }
            _ => {
                let exec = args[0];
                match find_exec(exec) {
                    Some(_file_path) => {
                        // run exec
                        args.remove(0);
                        let cmd_out = Command::new(exec).args(args).output().expect("failed to execute");
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
