use anyhow::Result;

use git_mg::git_emergency;

fn main() -> Result<()> {
    // todo: read input commit message
    git_emergency("this is a test commit")?;
    Ok(())
}
