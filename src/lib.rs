use std::{process::{Command, Output}, sync::mpsc::channel, time::Duration, path::Path};

use anyhow::Result;
use chrono::Local;
use notify::{watcher, Watcher, RecursiveMode};

// basic logic
// 1. git add
// 2. git commit
// 3. push to new branch

pub fn git_emergency(commit_message: &str) -> Result<()> {
    // check if current location is in a git repository
    // if not or nothing to commit, just return
    let status = show_git_info(&["status", "--porcelain"])?;
    if status.is_empty() {
        println!("Nothing to commit, working tree clean");
        return Ok(());
    }

    if status.starts_with("fatal: not a git repository") {
        println!("{}", status);
        return Ok(());
    }

    // checkout to a new branch named: emergency/<user.email>-<current_branch>-<current_time>
    let mut email = show_git_info(&["config", "--get", "user.email"])?;
    if email.is_empty() {
        println!("user.email is not configured, use user@emergency.com instead");
        email = String::from("user@emergency.com");
    } else {
        email = email.trim().to_owned();
    }

    let mut current_branch = show_git_info(&["branch", "--show-current"])?;
    current_branch = current_branch.trim().to_string();
    let current_time = Local::now().format("%F-%H_%M_%S").to_string();

    let new_branch = format!("emergency/{email}-{current_branch}-{current_time}");

    println!("These files will be committed to branch: {new_branch}\n\n{status}");

    // checkout to a new branch
    execute_git_command(&["checkout", "-b", &new_branch])?;

    // add files to working area
    execute_git_command(&["add", "--all"])?;

    // commit
    execute_git_command(&["commit", "-m", commit_message])?;
    println!("Changes committed with message: {}", commit_message);

    // push to remote server
    execute_git_command(&["push", "origin", &new_branch])?;
    println!("OK! All the changes are pushed to remote server");

    Ok(())
}

fn execute_git_command(args: &[&str]) -> Result<()> {
    wait_for_git_lock_released()?;
    let mut command = Command::new("git").args(args).spawn()?;
    command.wait()?;
    Ok(())
}

fn show_git_info(args: &[&str]) -> Result<String> {
    let output = Command::new("git").args(args).output()?;
    get_command_output_str(output)
}

fn get_command_output_str(output: Output) -> Result<String> {
    let output_str = String::from_utf8(output.stdout)?;
    Ok(output_str)
}

/// wait for last git command finished
fn wait_for_git_lock_released() -> Result<()> {
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    let repo_root_path = match show_git_info(&["rev-parse", "--top-level"]) {
        Ok(path) => path,
        Err(e) => {return Err(e)}
    };

    let lock_file_path = Path::new(&repo_root_path).join(".git/index.lock");
    if !Path::exists(&lock_file_path) {
        return Ok(())
    }

    watcher.watch(lock_file_path, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => return Ok(()),
            Err(e) => {},
        }
    }
}
