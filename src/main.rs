use std::env;

use anyhow::Result;

use git_mg::git_emergency;

fn main() -> Result<()> {
    let mut commit_message = String::from("emergency commit!!!");

    let mut args = env::args();
    args.next();

    match args.next() {
        Some(message) => {
            commit_message = message;
        },
        None => {},
    }

    //println!("{}", commit_message);
    git_emergency(&commit_message)?;
    Ok(())
}
