//! # Github Issue
//!
//! `github_issue` is a minimal utility to create and update issues within Github.

#[derive(Eq, PartialEq, Debug)]
pub struct Config {
    // create and update
    pub title: Option<String>,
    pub body: Option<String>,
    pub milestone: Option<u64>,
    pub labels: Option<Vec<String>>,
    pub assignees: Option<Vec<String>>,
    // read and update
    pub number: Option<u64>,
    // update
    pub state: Option<octocrab::models::IssueState>
}

/// Instantiate a reusable Octocrab issues object with input authentication, and an input owner and repo.
///
/// # Examples
///
/// ```
/// let issues =
/// ```
pub fn new_issues<'octo>(pat: Option<String>, owner: &str, repo: &str) -> Result<(), ()> {
//Result<octocrab::issues::IssueHandler<'octo>, ()> {
    // instantiate client
    let client = match pat {
        Some(pat) => octocrab::Octocrab::builder()
        .personal_token(pat)
        .build()
        .unwrap(),
        None => octocrab::Octocrab::default(),
    };
    // initalize and return issues
    let issues = client.issues(owner, repo);
    let issue = read(1, issues);

    println!("{:#?}", issue);

    return Ok(())
}

/// Crate a Github Issue according to configuration.
///
/// # Examples
///
/// ```
/// TODO
/// ```
pub fn create(config: Config, issues: octocrab::issues::IssueHandler) -> Result<(), ()> {
    Ok(())
}

/// Read a Github Issue according to configuration.
///
/// # Examples
///
/// ```
/// TODO
/// ```

pub async fn read<'octo>(num: u64, issues: octocrab::issues::IssueHandler<'octo>) -> Result<octocrab::models::issues::Issue, ()> {
    // read issue
    let issue = issues.get(num).await;

    Ok(issue)
}

/// Update a Github Issue according to configuration.
///
/// # Examples
///
/// ```
/// TODO
/// ```

pub fn update(config: Config, issues: octocrab::issues::IssueHandler) -> Result<(), ()> {
    Ok(())
}
