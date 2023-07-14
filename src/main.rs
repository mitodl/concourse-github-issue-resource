mod github_issue;

#[tokio::main]
async fn main() -> Result<(), ()> {
    github_issue::main(None, "mschuchard", "puppet-check", github_issue::Action::Read).await?;

    Ok(())
}
