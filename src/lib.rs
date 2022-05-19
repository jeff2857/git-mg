use std::process::{Command, Output};

use anyhow::{Result, Ok};
use chrono::Local;

// basic logic
// 1. git add
// 2. git commit
// 3. push to new branch

pub fn git_emergency(_commit_message: &str) -> Result<()> {
    // check if current location is in a git repository
    // if not or nothing to commit, just return
    let status = execute_git_command(&["status", "--porcelain"])?;
    if status.is_empty() {
        println!("Nothing to commit, working tree clean");
        return Ok(());
    }

    if status.starts_with("fatal: not a git repository") {
        println!("{}", status);
        return Ok(());
    }

    // checkout to a new branch named: emergency/<user.email>-<current_branch>-<current_time>
    let mut email = execute_git_command(&["config", "--get", "user.email"])?;
    if email.is_empty() {
        println!("user.email is not configured, use user@emergency.com instead");
        email = String::from("user@emergency.com");
    } else {
        email = email.trim().to_owned();
    }

    let mut current_branch = execute_git_command(&["branch", "--show-current"])?;
    current_branch = current_branch.trim().to_string();
    let current_time = Local::now().format("%F-%X").to_string();

    let new_branch = format!("emergency/{email}-{current_branch}-{current_time}");

    println!("These files will be committed to branch: {new_branch}\n\n{status}");

    // checkout to a new branch
    execute_git_command(&["checkout", "-b", &new_branch])?;

    // add files to working area
    execute_git_command(&["add", "--all"])?;

    Ok(())
}

fn execute_git_command(args: &[&str]) -> Result<String> {
    let output = Command::new("git").args(args).output()?;
    get_command_output_str(output)
}

fn get_command_output_str(output: Output) -> Result<String> {
    let output_str = String::from_utf8(output.stdout)?;
    Ok(output_str)
}
