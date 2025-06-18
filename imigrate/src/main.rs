mod core_functions;
use core_functions::database_migrate;

use std::path::PathBuf;
use std::error::Error as StdError; // ✅ You forgot to import this

#[allow(dead_code)]
pub struct Cli {
    command: String,
    subcommand: String,
    path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let mut args = std::env::args().skip(1); // Skip the binary name

    let command = args.next().expect("❌ No main command provided (e.g., 'i-migrate')");
    let subcommand = args.next().expect("❌ No subcommand provided (e.g., 'move')");
    let path = args.next().expect("❌ No path provided to config file");

    let cli = Cli {
        command,
        subcommand,
        path: PathBuf::from(path),
    };

    // Handle the command and subcommand
    if cli.command == "i-migrate" && cli.subcommand == "move" {
        match database_migrate::assign_db(&cli.path.to_string_lossy()).await{
            Ok(_) => println!("✅ Migration completed successfully."),
            Err(e) => eprintln!("❌ Migration failed: {}", e),
        }
    } else {
        eprintln!("❌ Unknown command: {} {}", cli.command, cli.subcommand);
        std::process::exit(1);
    }

    Ok(())
}
