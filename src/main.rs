use serde_json::Value;
use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;
use std::io::Result;
use std::process::exit;
use std::process::Command;

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();

    // Load aliases.
    let data: String = read_to_string("aliases.json").expect("Failed to read aliases.json.");
    let aliases: HashMap<String, Value> = serde_json::from_str(&data)?;

    /* Make sure there is only one argument.
    The program only accepts the following form:
    alias.exe [alias] */
    if args.len() != 2 {
        println!("alias.exe [alias]");
        println!("Avaliable aliases:");

        // Print all aliases to user to see.
        for (alias, _) in aliases {
            println!("{}", alias);
        }

        exit(0);
    }

    /* If we don't find the alias or command, then just
    return a string. We will handle this later. */
    let alias: &str = &args[1];
    let command = aliases[alias].as_str().unwrap_or("");

    /* If the command is empty, exit and tell user we couldn't
    find the alis.*/
    if command.is_empty() {
        println!("Unexpected alias '{}'.", &alias);
        exit(0);
    }

    // Run the command in powershell.
    Command::new("powershell.exe")
        .arg(command)
        .spawn()?
        .wait()?;

    Ok(())
}
