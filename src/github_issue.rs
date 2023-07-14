//! # Github Issue
//!
//! `github_issue` is a minimal utility to create and update issues within Github.

use std::error;

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

pub enum Action {
    Create,
    Read,
    Update
}

/// Instantiate a reusable Octocrab issues object with input authentication, and an input owner and repo.
///
/// # Examples
///
/// ```
/// let issues =
/// ```
pub async fn main<'octo>(pat: Option<String>, owner: &str, repo: &str, action: Action) -> Result<(), ()> {
//Result<octocrab::issues::IssueHandler<'octo>, ()> {
    // instantiate client
    let client = match pat {
        Some(pat) => octocrab::Octocrab::builder()
        .personal_token(pat)
        .build()
        .expect("could not authenticate client with Personal Access Token"),
        None => octocrab::Octocrab::default(),
    };
    // execute action
    match action {
        // create an issue
        Action::Create => println!("create is currently unsupported"),
        // read an issue state
        Action::Read => {
            let issues = client.issues(owner, repo);
            match read_state(100, issues).await {
                Ok(state) => println!("{state:#?}"),
                Err(error) => println!("{error}"),
            }
        }
        // update an issue
        Action::Update => println!("update is currently unsupported"),
    }

    Ok(())
}

/// Crate a Github Issue according to configuration.
///
/// # Examples
///
/// ```
/// TODO
/// ```
async fn create<'octo>(config: Config, issues: octocrab::issues::IssueHandler<'octo>) -> Result<(), ()> {
    Ok(())
}

/// Read a Github Issue according to configuration.
///
/// # Examples
///
/// ```
/// TODO
/// ```

async fn read_state<'octo>(num: u64, issues: octocrab::issues::IssueHandler<'octo>) -> Result<octocrab::models::IssueState, &str> {
    // retrieve the issue with the handler
    let issue = match issues.get(num).await {
        Ok(issue) => issue,
        Err(error) => {
            println!("the issue number {num} could not be retrieved");
            println!("{error}");
            return Err("unknown issue state");
        },
    };

    // return the issue state
    Ok(issue.state)
}

/// Update a Github Issue according to configuration.
///
/// # Examples
///
/// ```
/// TODO
/// ```

async fn update<'octo>(config: Config, issues: octocrab::issues::IssueHandler<'octo>) -> Result<(), ()> {
    Ok(())
}


// TODO tests
