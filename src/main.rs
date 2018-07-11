extern crate colored;
extern crate tempfile;

use std::env;
use std::fs;
use std::process::Command;
use std::str::from_utf8 as str_from_utf8;

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
    // Use the shell as specified in the SHELL environment variable
    if let Ok(shell) = env::var("SHELL") {
        if let Some(shell) = try_shell_file(&shell) {
            return shell;
        }
    }

    // Query the system for the default shell of the current user
    let shell = Command::new("sh")
        .arg("-c")
        .arg("getent passwd $LOGNAME | cut -d: -f7")
        .output()
        .expect("failed to determine default shell of user");
    if shell.status.success() {
        if let Ok(shell) = str_from_utf8(&shell.stdout).map(|s| s.trim()) {
            if let Some(shell) = try_shell_file(shell) {
                return shell;
            }
        }
    }

    // Use sh as default
    Command::new("sh")
}

/// Given the path to a shell binary.
///
/// If it is an existing executable binary, return a command to invoke it.
/// Otherwise return `None`.
fn try_shell_file(shell: &str) -> Option<Command> {
    // Check whether the shell file is valid, return it as command if it is
    if let Ok(metadata) = fs::metadata(&shell) {
        if metadata.is_file() {
            return Some(Command::new(&shell));
        }
    }

    // The shell file was not valid, return None
    None
}
