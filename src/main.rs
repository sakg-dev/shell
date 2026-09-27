use std::io::{self, Write};

fn main() {
    while true {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut ipt = String::new();
        let _ = io::stdin().read_line(&mut ipt);
        ipt.trim_end();
        ipt.pop(); // \n

        let args = ipt.split(" ").collect::<Vec<_>>();

        match args[0] {
            "exit" => break,
            "echo" => println!("{}", ipt.replacen(args[0], "", 1).trim_start()),
            _ => eprintln!("{}: command not found",args[0]),
        }
    }
}
