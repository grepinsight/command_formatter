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


## License

This tool is released under the [MIT License](LICENSE).
