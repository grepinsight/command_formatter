# Command Formatter

A command-line tool to format a command string with specific rules.



## Usage


1. Run the tool and provide the command string as input:

   ```
   echo "my_command  positional-arg   --flag 1    --flag2     --flag  3" | ./command_formatter
   ```

   The formatted command string will be output to standard output.

## Formatting Rules

The tool applies the following formatting rules to the command string:

- Adds a backslash (`\`) after each positional argument.
- Adds a backslash (`\`) after each flag argument, except for boolean flags.

## Examples

Input:
```
my_command  positional-arg   --flag 1    --flag2     --flag  3
```

Output:
```
my_command \
positional-arg \
--flag 1 \
--flag2 \
--flag 3
```

## Installation

### Prerequisites

- Rust (version 1.x or later)

### Steps

1. Clone the repository:
   ```
   git clone https://github.com/grepinsight/command-formatter.git
   ```

2. Navigate to the project directory:
   ```
   cd command-formatter
   ```

3. Install the project using Cargo:
   ```
   cargo install --path .
   ```

   This command will compile the code and install the `command_formatter` executable in your Cargo binary directory (usually `$HOME/.cargo/bin`).

   Note: Make sure that your Cargo binary directory is in your system's `PATH` environment variable. This allows you to run the `command_formatter` command from anywhere in your terminal.

   If the Cargo binary directory is not in your `PATH`, you can add it by modifying your shell configuration file (e.g., `~/.bashrc`, `~/.zshrc`, etc.) and adding the following line:
   ```
   export PATH="$HOME/.cargo/bin:$PATH"
   ```

   After modifying the configuration file, restart your terminal or run `source ~/.bashrc` (or the appropriate command for your shell) to apply the changes.


## License

This tool is released under the [MIT License](LICENSE).
