use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};

fn main() {
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // --- Fix Start ---

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command: Option<Child> = None;

        while let Some(command_str) = commands.next() {
            let mut parts = command_str.trim().split_whitespace();
            
            // 1. Handle empty commands to prevent panics.
            let command = match parts.next() {
                Some(cmd) => cmd,
                None => {
                    // This handles empty input or syntax like "ls | | wc"
                    if previous_command.is_some() {
                        eprintln!("Error: empty command in pipe.");
                    }
                    previous_command = None; // Break the pipe chain
                    continue;
                }
            };
            let args = parts;

            match command {
                "cd" => {
                    // Default to the user's home directory
                    let default_path = env::home_dir().unwrap_or_else(|| Path::new("/").to_path_buf());
                    let new_dir = args.peekable().peek()
                        .map_or(default_path.as_path(), |x| Path::new(*x));
                    
                    if let Err(e) = env::set_current_dir(new_dir) {
                        eprintln!("cd: {}", e);
                    }
                    // `cd` produces no stdout, so it must be the end of a pipe chain
                    previous_command = None;
                }
                "exit" => return,

                command => {
                    // 2. Correctly handle stdin for piped commands.
                    //    We borrow `previous_command` mutably (`ref mut`) instead of moving it.
                    let stdin = if let Some(ref mut child) = previous_command {
                        // Take ownership of the previous command's stdout, leaving `None` in its place.
                        // This `unwrap()` is safe because we set it to `Stdio::piped()` below.
                        Stdio::from(child.stdout.take().unwrap())
                    } else {
                        // First command in a chain gets stdin from the shell
                        Stdio::inherit()
                    };

                    let stdout = if commands.peek().is_some() {
                        // There is another command piped after this one.
                        // Pipe this command's stdout to the next one.
                        Stdio::piped()
                    } else {
                        // This is the last command.
                        // Its stdout should go to the shell.
                        Stdio::inherit()
                    };

                    match Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn()
                    {
                        Ok(new_child) => {
                            // The new child becomes the previous command for the next iteration.
                            previous_command = Some(new_child);
                        }
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}: {}", command, e);
                        }
                    };
                }
            }
        }

        // 3. Wait for the final command of the pipeline to complete.
        if let Some(mut final_command) = previous_command {
            // This prevents the shell from prompting for new input
            // before the background processes have finished.
            final_command.wait().unwrap();
        }
        // --- Fix End ---
    }
}