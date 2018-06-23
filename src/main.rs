extern crate colored;
extern crate tempfile;

use std::process::Command;

use colored::Colorize;
use tempfile::tempdir;

/// Application entrypoint.
///
/// This immediately creates a temporary workspace and opens a sub shell.
fn main() {
    // Create the temporary directory
    let dir = tempdir().expect("failed to create temporary directory");

    println!("{}", format!(
        "Starting temporary workspace in {}",
        &dir.as_ref().to_str().unwrap_or("?"),
    ).yellow());

    // Create the sub shell
    shell_command()
            .current_dir(&dir)
            .status()
            .expect("failed to create sub shell");

    // Explicitly close the temporary directory
    dir.close().expect("failed to close temporary directory");

    eprintln!("{}", "Cleaned temporary workspace".green());
}

/// Build the command for the shell to open.
///
/// The shell to be used is determined at runtime,
/// by looking at the SHELL environment variable and
/// the default shell entry in the users passwd entry.
fn shell_command() -> Command {
    // TODO: use the shell from SHELL
    // TODO: fallback to the shell in passwd

    Command::new("fish")
            // .arg("-c")
            // .arg("echo hello")
}
