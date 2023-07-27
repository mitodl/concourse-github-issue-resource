//! # Github Issue
//!
//! `github_issue` is a minimal utility to create and update issues within Github.

// allowed operations for github issue interactions
#[non_exhaustive]
pub(crate) enum Action {
    Create,
    Read,
    Update,
}

// convert string to IssueState without trait implementations because not allowed
fn str_to_issue_state(param: &str) -> octocrab::models::IssueState {
    match param {
        "Open" => octocrab::models::IssueState::Open,
        "Closed" => octocrab::models::IssueState::Closed,
        &_ => panic!("the issue state must be either Open or Closed"),
    }
}

// struct for general interfacing with module
// the types correspond to octocrab when not advantageous otherwise
#[derive(Eq, PartialEq, Debug)]
pub(crate) struct Issue {
    // client and issues: OctocrabBuilder and issues::IssueHandler
    pat: Option<String>,
    owner: String,
    repo: String,
    // create and update (octocrab update expects AsRef<str> instead of String and AsRef<[String]> instead of Vec<String>)
    title: Option<String>,
    body: Option<String>,
    labels: Option<Vec<String>>,
    assignees: Option<Vec<String>>,
    // read and update
    number: Option<u64>,
    // update
    state: Option<octocrab::models::IssueState>,
}

impl Issue {
    /// Constructor for the Config struct. Contains all of the members necessary for instantiating a client and performing an action.
    ///
    /// # Examples
    ///
    /// ```
    /// let gh_issue = Issue::new(None, String::from("my_org"), String::from("my_repo"), None, None, None, None, Some(100), None);
    /// ```
    pub(crate) fn new(
        pat: Option<String>,
        owner: impl Into<String>,
        repo: impl Into<String>,
        title: Option<String>,
        body: Option<String>,
        labels: Option<Vec<String>>,
        assignees: Option<Vec<String>>,
        number: Option<u64>,
        state_str: Option<&str>,
    ) -> Self {
        // convert state from string to IssueState
        let state = match state_str {
            Some(state_str) => Some(str_to_issue_state(state_str)),
            None => None,
        };
        // type conversion traits
        let owner = owner.into();
        let repo = repo.into();
        // return instantiated github issue
        Self {
            pat,
            owner,
            repo,
            title,
            body,
            labels,
            assignees,
            number,
            state,
        }
    }

    /// Instantiate a reusable Octocrab issues object with input authentication, and an input owner and repo.
    ///
    /// # Examples
    ///
    /// ```
    /// let issue = gh_issue.main(Action::Read).await?;
    /// ```
    #[allow(unreachable_patterns)]
    pub(crate) async fn main<'octo>(
        &self,
        action: Action,
    ) -> Result<octocrab::models::issues::Issue, &str> {
        // instantiate client and issues
        let client = match &self.pat {
            Some(pat) => octocrab::Octocrab::builder()
                .personal_token(pat.to_string())
                .build()
                .expect("could not authenticate client with Personal Access Token"),
            None => octocrab::Octocrab::default(),
        };
        let issues = client.issues(&self.owner, &self.repo);
        // execute action and assign returned issue
        let issue = match action {
            // create an issue
            Action::Create => self.create(issues).await?,
            // read an issue state
            Action::Read => self.read(issues).await?,
            // update an issue
            Action::Update => self.update(issues).await?,
            // invalid action specified somehow
            _ => return Err("invalid/unsupported action specified"),
        };

        Ok(issue)
    }

    // create a github issue according to configuration
    async fn create<'octo>(
        &self,
        issues: octocrab::issues::IssueHandler<'octo>,
    ) -> Result<octocrab::models::issues::Issue, &str> {
        // validate a title was specified
        match &self.title {
            // title specified
            Some(title) => {
                // build the issue
                let mut issue = issues.create(title);
                // ... with optional parameters
                if self.body.is_some() {
                    issue = issue.body(self.body.as_ref().unwrap());
                }
                if self.labels.is_some() {
                    issue = issue.labels(self.labels.clone().unwrap());
                }
                if self.assignees.is_some() {
                    issue = issue.labels(self.assignees.clone().unwrap());
                }
                // send and await the issue
                match issue.send().await {
                    // return created issue
                    Ok(issue) => return Ok(issue),
                    // issue could not be created
                    Err(error) => {
                        println!("the issue could not be created");
                        println!("{error}");
                        return Err("issue not created");
                    }
                }
            }
            // title unspecified
            None => {
                println!("a title was not specified, and so an issue could not be created");
                return Err("title unspecified");
            }
        }
    }

    // read a github issue according to configuration
    async fn read<'octo>(
        &self,
        issues: octocrab::issues::IssueHandler<'octo>,
    ) -> Result<octocrab::models::issues::Issue, &str> {
        // validate an issue number was specified
        match self.number {
            // issue number specified
            Some(number) => {
                // retrieve the issue with the handler
                match issues.get(number).await {
                    Ok(issue) => return Ok(issue),
                    // issue number probably does not exist, or some other error
                    Err(error) => {
                        println!("the issue number {number} could not be retrieved");
                        println!("{error}");
                        return Err("unknown issue state");
                    }
                };
            }
            // issue number unspecified
            None => {
                println!("an issue number was not specified, and so its state cannot be retrieved");
                return Err("issue number unspecified");
            }
        }
    }

    // update a github issue according to configuration
    async fn update<'octo>(
        &self,
        issues: octocrab::issues::IssueHandler<'octo>,
    ) -> Result<octocrab::models::issues::Issue, &str> {
        // validate an issue number was specified
        match self.number {
            // issue number specified
            Some(number) => {
                // build the issue
                let mut issue = issues.update(number);
                // ... with optional parameters
                if self.title.is_some() {
                    issue = issue.title(self.title.as_ref().unwrap());
                }
                if self.body.is_some() {
                    issue = issue.body(self.body.as_ref().unwrap());
                }
                if self.state.is_some() {
                    issue = issue.state(self.state.clone().unwrap());
                }
                // TODO requires converting Option<Vec<String>> to &'e impl AsRef<[String]> + ?Sized which is horrendous
                /*if self.labels.is_some() {
                    let labels = self.labels.clone().unwrap();
                    issue = issue.labels(&labels[..]);
                }
                if self.assignees.is_some() {
                    issue = issue.labels(&self.assignees.unwrap());
                }*/
                // send and await the issue
                match issue.send().await {
                    // return updated issue
                    Ok(issue) => return Ok(issue),
                    // issue number probably does not exist, or some other error
                    Err(error) => {
                        println!("the issue number {number} could not be updated");
                        println!("{error}");
                        return Err("issue not updated");
                    }
                }
            }
            // issue number unspecified
            None => {
                println!("an issue number was not specified, and so an issue could not be updated");
                return Err("issue number unspecified");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_issue_state() {
        // validates issue open and closed conversions
        assert_eq!(
            str_to_issue_state("Open"),
            octocrab::models::IssueState::Open,
            "failed to convert Open str to Open enum"
        );

        assert_eq!(
            str_to_issue_state("Closed"),
            octocrab::models::IssueState::Closed,
            "failed to convert Closed str to Closed enum"
        );
    }

    #[test]
    fn test_issue_new() {
        // validates basic read constructor
        assert_eq!(
            Issue::new(
                None,
                "my_org",
                "my_repo",
                None,
                None,
                None,
                None,
                Some(100),
                None
            ),
            Issue {
                pat: None,
                owner: String::from("my_org"),
                repo: String::from("my_repo"),
                title: None,
                body: None,
                labels: None,
                assignees: None,
                number: Some(100),
                state: None
            },
            "failed to construct Issue for read"
        );

        // validate basic create constructor
        assert_eq!(
            Issue::new(
                None,
                "my_org",
                "my_repo",
                Some(String::from("my issue")),
                Some(String::from("my body")),
                Some(vec![String::from("label")]),
                Some(vec![String::from("assignee")]),
                None,
                None
            ),
            Issue {
                pat: None,
                owner: String::from("my_org"),
                repo: String::from("my_repo"),
                title: Some(String::from("my issue")),
                body: Some(String::from("my body")),
                labels: Some(vec![String::from("label")]),
                assignees: Some(vec![String::from("assignee")]),
                number: None,
                state: None
            },
            "failed to construct Issue for create"
        );
    }

    #[test]
    fn test_issue_main() {
        // validate issue returned when read from main
        let test = async {
            let gh_issue = Issue::new(
                None,
                "mitodl",
                "ol-infrastructure",
                None,
                None,
                None,
                None,
                Some(100),
                None,
            );
            let issue = gh_issue.main(Action::Read).await;
            assert_eq!(
                issue.unwrap().state,
                octocrab::models::IssueState::Closed,
                "first issue from mitodl/ol-infrastructure not read and returned correctly",
            );
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(test);
    }
}
