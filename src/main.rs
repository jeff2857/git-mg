use anyhow::Result;

use git_mg::git_emergency;

fn main() -> Result<()> {
    git_emergency("hello")?;
    Ok(())
}
