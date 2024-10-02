use std::env;
use std::ffi::{CStr, CString};
use std::io::{self, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let commands = input
            .split('|')
            .map(|cmd| cmd.trim().to_owned())
            .collect::<Vec<String>>();

        for command in commands {
            let parts: Vec<&str> = command.trim().split_whitespace().collect();
            match parts.get(0) {
                Some(&"cd") => {
                    let path = parts.get(1).map_or("/", |s| *s);
                    chdir(path)?;
                }
                Some(&"pwd") => {
                    println!("{}", env::current_dir()?.display());
                }
                Some(&"echo") => {
                    println!("{}", parts[1..].join(" "));
                }
                Some(&"ps") => {
                    // Dummy implementation for simplicity
                    println!("PID: {}, CMD: example, TIME: 0 ms", getpid());
                }
                Some(&"kill") => {
                    // The 'kill' command is not fully implemented in this example
                    println!("'kill' command is not implemented in this shell.");
                }
                Some(&"exit") => {
                    return Ok(());
                }
                Some(_) => {
                    match fork()? {
                        ForkResult::Parent { .. } => {
                            // Parent process continues here.
                        }
                        ForkResult::Child => {
                            let args: Vec<CString> =
                                parts.iter().map(|s| CString::new(*s).unwrap()).collect();
                            execvp(&args[0], &args)?;
                        }
                    }
                }
                None => {}
            }
        }
    }
}
