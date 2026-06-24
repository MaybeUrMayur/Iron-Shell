#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub program: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Pipeline {
    pub commands: Vec<Command>,
    pub stdin_file: Option<String>,
    pub stdout_file: Option<String>,
}

pub fn parse(input: &str) -> Option<Pipeline> {
    let mut pipeline = Pipeline::default();
    let mut current_command = Command {
        program: String::new(),
        args: Vec::new(),
    };
    let mut current_token = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut expecting_stdin = false;
    let mut expecting_stdout = false;
    let mut escape_next = false;

    let mut chars = input.chars().peekable();

    let push_token = |token: &mut String, cmd: &mut Command| {
        if !token.is_empty() {
            if cmd.program.is_empty() {
                cmd.program = token.clone();
            } else {
                cmd.args.push(token.clone());
            }
            token.clear();
        }
    };

    while let Some(c) = chars.next() {
        if escape_next {
            current_token.push(c);
            escape_next = false;
            continue;
        }

        match c {
            '\\' if !in_single_quote => {
                escape_next = true;
            }
            '\'' if !in_double_quote => {
                in_single_quote = !in_single_quote;
            }
            '"' if !in_single_quote => {
                in_double_quote = !in_double_quote;
            }
            ' ' | '\t' if !in_single_quote && !in_double_quote => {
                if expecting_stdin {
                    if !current_token.is_empty() {
                        pipeline.stdin_file = Some(current_token.clone());
                        current_token.clear();
                        expecting_stdin = false;
                    }
                } else if expecting_stdout {
                    if !current_token.is_empty() {
                        pipeline.stdout_file = Some(current_token.clone());
                        current_token.clear();
                        expecting_stdout = false;
                    }
                } else {
                    push_token(&mut current_token, &mut current_command);
                }
            }
            '|' if !in_single_quote && !in_double_quote => {
                push_token(&mut current_token, &mut current_command);
                if !current_command.program.is_empty() {
                    pipeline.commands.push(current_command.clone());
                    current_command = Command {
                        program: String::new(),
                        args: Vec::new(),
                    };
                }
            }
            '<' if !in_single_quote && !in_double_quote => {
                push_token(&mut current_token, &mut current_command);
                expecting_stdin = true;
            }
            '>' if !in_single_quote && !in_double_quote => {
                push_token(&mut current_token, &mut current_command);
                expecting_stdout = true;
            }
            _ => {
                current_token.push(c);
            }
        }
    }

    if expecting_stdin {
        if !current_token.is_empty() {
            pipeline.stdin_file = Some(current_token.clone());
            current_token.clear();
        }
    } else if expecting_stdout {
        if !current_token.is_empty() {
            pipeline.stdout_file = Some(current_token.clone());
            current_token.clear();
        }
    } else {
        push_token(&mut current_token, &mut current_command);
    }

    if !current_command.program.is_empty() {
        pipeline.commands.push(current_command);
    }

    if pipeline.commands.is_empty() {
        None
    } else {
        Some(pipeline)
    }
}
