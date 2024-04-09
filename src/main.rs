//! A command-line tool to format a command string with specific rules.
//!
//! The tool reads a command string from standard input, formats it according to the following rules:
//! - Adds a backslash (`\`) after each positional argument.
//! - Adds a backslash (`\`) after each flag argument, except for boolean flags.
//! - Outputs the formatted command string to standard output.
use std::env;
use std::io::{self, BufRead};
use std::process;

/// Formats a command string according to the specified rules.
///
/// # Arguments
///
/// * `input` - A string slice containing the command and its arguments.
///
/// # Returns
///
/// A formatted `String` with the command and its arguments separated by backslashes.
fn format_command(input: &str) -> String {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let mut output = String::new();
    for (i, token) in tokens.iter().enumerate() {
        output.push_str(token);
        if i < tokens.len() - 1 {
            if token.starts_with("--") {
                if let Some(next_token) = tokens.get(i + 1) {
                    if !next_token.starts_with("--") {
                        output.push(' ');
                        continue;
                    }
                }
            }
            output.push_str(" \\");
        }
        output.push('\n');
    }
    output
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "-h" {
        println!("Usage: command_formatter");
        println!("Reads a command string from standard input and formats it according to specific rules.");
        println!("Options:");
        println!("  -h  Display this help message");
        process::exit(0);
    }

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    if let Some(Ok(line)) = lines.next() {
        let output = format_command(&line);
        print!("{}", output);
    }
}

#[test]
fn test_format_command() {
    let input = "my_command  positional-arg   --flag 1    --flag2     --flag  3";
    let expected_output = "my_command \\
positional-arg \\
--flag 1 \\
--flag2 \\
--flag 3
";
    assert_eq!(format_command(input), expected_output);
}

#[test]
fn test_format_command_whitespace() {
    let input = "my_command  positional-arg         --flag 1        --flag2     --flag  3";
    let expected_output = "my_command \\
positional-arg \\
--flag 1 \\
--flag2 \\
--flag 3
";
    assert_eq!(format_command(input), expected_output);
}
