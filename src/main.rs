use std::io::{self, Write};

#[derive(Debug)]
struct Command {
    name: String,
    description: String
}
fn main() {
    let commands:Vec<Command> = vec![
        Command {
            name: String::from("man"),
            description: String::from("Provides built in manual for most of the linux programs that are installed on the machine")
        },
        Command {
            name: String::from("cd"),
            description: String::from("Built in command that helps to change directory")
        }
    ];

    print!("$ ");
    io::stdout().flush().unwrap();

    let mut ipt = String::new();
    let _ = io::stdin().read_line(&mut ipt);
    ipt.trim_end();
    ipt.pop(); // \n
    // println!("You typed {:?}", ipt);

    let cmd_res = commands.iter().find(|x| &x.name==&ipt);
    if let Some(cmd) = cmd_res {
        println!("{}: {}", cmd.name, cmd.description)
    } else {
        eprintln!("{ipt}: command not found");
    }
}
