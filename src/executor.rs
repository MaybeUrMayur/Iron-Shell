use crate::parser::Pipeline;
use crate::builtins;
use std::process::{Command, Stdio, Child};
use std::fs::File;
use std::io::Read;

pub fn execute(pipeline: Pipeline) -> (bool, String) {
    if pipeline.commands.is_empty() {
        return (true, String::new());
    }

    // Handle built-ins first if it's a single command
    if pipeline.commands.len() == 1 {
        let cmd = &pipeline.commands[0];
        match cmd.program.as_str() {
            "cd" => return builtins::cd(&cmd.args),
            "exit" => return builtins::exit(&cmd.args),
            "create" => return builtins::create(&cmd.args),
            "delete" => return builtins::delete(&cmd.args),
            "list" => return builtins::list(&cmd.args),
            "read" => return builtins::read(&cmd.args),
            "copy" => return builtins::copy(&cmd.args),
            "move" => return builtins::move_cmd(&cmd.args),
            "rename" => return builtins::rename(&cmd.args),
            "hi" => return builtins::hi(&cmd.args),
            _ => {} // Not a built-in
        }
    }

    let mut previous_command: Option<Child> = None;
    let num_commands = pipeline.commands.len();
    let mut final_output = String::new();

    for (i, cmd) in pipeline.commands.iter().enumerate() {
        let mut command = Command::new(&cmd.program);
        command.args(&cmd.args);

        // Handle stdin
        if i == 0 {
            if let Some(ref stdin_file) = pipeline.stdin_file {
                match File::open(stdin_file) {
                    Ok(file) => {
                        command.stdin(Stdio::from(file));
                    }
                    Err(e) => {
                        return (true, format!("iron_shell: {}: {}", stdin_file, e));
                    }
                }
            } else {
                // To avoid hanging on input if command expects it, we probably want null for graphical window.
                // But let's leave it piped or inherit and hope the user doesn't run interactive programs.
                command.stdin(Stdio::null());
            }
        } else {
            if let Some(mut previous_child) = previous_command.take() {
                if let Some(stdout) = previous_child.stdout.take() {
                    command.stdin(Stdio::from(stdout));
                }
            }
        }

        // Handle stdout
        if i == num_commands - 1 {
            if let Some(ref stdout_file) = pipeline.stdout_file {
                match File::create(stdout_file) {
                    Ok(file) => {
                        command.stdout(Stdio::from(file));
                    }
                    Err(e) => {
                        return (true, format!("iron_shell: {}: {}", stdout_file, e));
                    }
                }
            } else {
                command.stdout(Stdio::piped());
            }
        } else {
            command.stdout(Stdio::piped());
        }

        command.stderr(Stdio::piped());

        match command.spawn() {
            Ok(child) => {
                previous_command = Some(child);
            }
            Err(e) => {
                return (true, format!("iron_shell: {}: {}", cmd.program, e));
            }
        }
    }

    // Wait for the last command to finish and read output
    if let Some(mut last_child) = previous_command {
        if let Some(mut stdout) = last_child.stdout.take() {
            let _ = stdout.read_to_string(&mut final_output);
        }
        if let Some(mut stderr) = last_child.stderr.take() {
            let mut err_str = String::new();
            if stderr.read_to_string(&mut err_str).is_ok() && !err_str.is_empty() {
                final_output.push_str(&err_str);
            }
        }
        if let Err(e) = last_child.wait() {
            final_output.push_str(&format!("\niron_shell: wait failed: {}", e));
        }
    }

    (true, final_output)
}
