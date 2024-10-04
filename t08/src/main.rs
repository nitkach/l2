use std::{io::Write, process::Stdio};

fn main() {
    loop {
        let input = get_input();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        // pipelines
        let commands = input.split('|').map(str::trim).collect::<Vec<_>>();

        if commands.len() > 1 {
            handle_pipeline(&commands);
        } else {
            handle_command(input);
        }
    }
}

fn get_input() -> String {
    let current_directory = std::env::current_dir().unwrap_or("".into());
    print!("{}> ", current_directory.display());
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn handle_command(command: &str) {
    let args = command.split_whitespace().collect::<Vec<_>>();
    match args.as_slice() {
        ["cd", args @ ..] => {
            let Some(first) = args.first() else {
                eprintln!("cd: missing argument");
                return;
            };

            if let Err(err) = std::env::set_current_dir(first) {
                eprintln!("cd: {first}: {err}");
            };
        }
        ["pwd", ..] => match std::env::current_dir() {
            Ok(path) => println!("{}", path.display()),
            Err(err) => eprintln!("pwd: {err}"),
        },
        ["echo", args @ ..] => {
            let output = args.join(" ");
            println!("{output}");
        }
        ["kill", pid, ..] => {
            let Ok(pid) = pid.parse::<u32>() else {
                eprintln!("kill: invalid PID");
                return;
            };
            match std::process::Command::new("kill")
                .arg(pid.to_string())
                .status()
            {
                Ok(status) => println!("kill: {status}"),
                Err(err) => eprintln!("kill: {err}"),
            }
        }
        ["ps", ..] => {
            let output = match std::process::Command::new("ps")
                .arg("-eo")
                .arg("pid,comm,etime")
                .output()
            {
                Ok(output) => output,
                Err(err) => {
                    eprintln!("ps: {err}");
                    return;
                }
            };
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        [command, args @ ..] => {
            // fork and exec for other commands
            let mut child = match std::process::Command::new(command).args(args).spawn() {
                Ok(child) => child,
                Err(err) => {
                    eprintln!("{command}: {err}");
                    return;
                }
            };
            match child.wait() {
                Ok(status) => println!("{command}: {status}"),
                Err(err) => println!("{command}: {err}"),
            };
        }
        [] => unreachable!(),
    }
}

fn handle_pipeline(commands: &[&str]) {
    let mut previous_command = None;

    for command in commands {
        let args = command.split_whitespace().collect::<Vec<_>>();
        let (command, args) = args.split_first().unwrap();

        let mut child = std::process::Command::new(command)
            .args(args)
            .stdin(
                previous_command
                    .take()
                    .map(Stdio::from)
                    .unwrap_or_else(std::process::Stdio::inherit),
            )
            .stdout(if *command == *commands.last().unwrap() {
                std::process::Stdio::inherit()
            } else {
                std::process::Stdio::piped()
            })
            .spawn()
            .expect("Failed to execute command");

        previous_command = child.stdout.take();

        child.wait().unwrap();
    }
}
