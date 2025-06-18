// This brings the `io` module from the Rust standard library into scope,
// allowing access to functions for reading/writing input/output.
// `self` means we can use `io` itself (like `io::stdin()`),
// and `Write` is a trait required for `flush()` on `stdout()`.
use std::io::{self, Write};

// This declares the use of a custom module named `core_functions`.
// It's assumed this module has functions (like `get_commands()`) and types (like `Command`).
mod core_functions;

fn main() {
    // Get available commands from the core_functions module.
    // `get_commands()` is probably returning a list of command structs.
    let commands = core_functions::get_commands();

    // Prompt user to enter a command.
    print!("enter command: ");
    
    // Flush the output buffer to ensure the prompt appears before input.
    // Without flush, the text might not appear until after input.
    io::stdout().flush().unwrap();

    // Create a mutable string to hold the user input.
    let mut input = String::new();

    // Read a line from standard input and store it in `input`.
    io::stdin().read_line(&mut input).unwrap();

    // Remove any trailing newline or whitespace.
    let input = input.trim();

    // Try to find a command in the list where the prompt matches user input.
    match commands.iter().find(|cmd| cmd.prompt == input) {
        // If a match is found, run its associated action.
        Some(cmd) => (cmd.action)(),

        // If no command matches, print an error message.
        // NOTE: there is a typo here: `printl!` should be `println!`
        None => println!("Command not recognized. use 'i-migrate -help' to see all commands"),
    }

    // This always runs after the match, regardless of the result.
    println!("Hello, world!");

    // This looks like an incomplete or misplaced match statement.
    // You might be planning to use `cli.command` to match subcommands.
    // But `cli` is not defined here, so this part should probably be removed or fixed.
    match cli.command {
        // Placeholder: fill in command handling logic here.
    }
}
