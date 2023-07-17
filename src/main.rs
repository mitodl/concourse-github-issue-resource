mod github_issue;

#[tokio::main]
async fn main() -> Result<(), ()> {
    // instantiate issue
    let gh_issue = github_issue::Issue::new(None, "mschuchard", "puppet-check", None, None, None, None, Some(1), None);
    gh_issue.main(github_issue::Action::Read).await?;

    Ok(())
}
