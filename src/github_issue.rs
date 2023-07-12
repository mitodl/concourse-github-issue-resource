//! # Github Issue
//!
//! `github_issue` is a minimal utility to create and update issues within Github.

// TODO: octocrab::issue with owner and repo as param instead of client
#[derive(Eq, PartialEq, Debug)]
pub struct Config {
    pub owner: String,
    pub repo: String,
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

/// Crate a Github Issue according to configuration.
///
/// # Examples
///
/// ```
/// TODO
/// ```
pub fn create(config: Config, client: octocrab::Octocrab) -> Result<octocrab::Result<()>, ()> {
    // create issue and assign octocrab result
    let issue = client.issues(config.owner, config.repo).create(config.title)
        .body(config.body)
        .milestone(config.milestone)
        .labels(config.labels)
        .assignees(config.assignees)
        // send the request
        .send();

    Ok(issue, )
}

/// Read a Github Issue according to configuration.
///
/// # Examples
///
/// ```
/// TODO
/// ```

pub fn read(config: Config, client: octocrab::Octocrab) -> Result<(), ()> {
    // read issue
    let issue = client.issues(config.owner, config.repo).get(config.number);

    Ok(())
}

/// Update a Github Issue according to configuration.
///
/// # Examples
///
/// ```
/// TODO
/// ```

pub fn update(config: Config, client: octocrab::Octocrab) -> Result<(), ()> {
    let issue = client.issues(config.owner, config.repo).update(config.number)
        .body(config.body)
        .state(config.state)
        .milestone(config.milestone)
        .labels(config.labels)
        .assignees(config.assignees)
        // send the request
        .send();

    Ok(())
}
