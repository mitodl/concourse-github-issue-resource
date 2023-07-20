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

// convert string to IssueState without trait implementations because not allowed TODO error
fn string_to_issue_state(param: &str) -> octocrab::models::IssueState {
    match param {
        "Open" => octocrab::models::IssueState::Open,
        "Closed" => octocrab::models::IssueState::Closed,
        &_ => todo!(),
    }
}

// struct for general interfacing with module
// the types correspond to octocrab when not advantageous otherwise
#[derive(Eq, PartialEq, Debug)]
pub(crate) struct Issue {
    // client: OctocrabBuilder and issues::IssueHandler
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
    /// let gh_issue = Issue::new(None, "my_org", "my_repo", None, None, None, None, Some(100), None);
    /// ```
    pub(crate) fn new(
        pat: Option<String>,
        owner: String,
        repo: String,
        title: Option<String>,
        body: Option<String>,
        labels: Option<Vec<String>>,
        assignees: Option<Vec<String>>,
        number: Option<u64>,
        state_string: Option<&str>,
    ) -> Self {
        // convert state from string to IssueState
        let state = match state_string {
            Some(state_string) => Some(string_to_issue_state(state_string)),
            None => None,
        };
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
    /// let issue = gh_issue.main(Action::Read).await;
    /// ```
    // TODO inconsistent returns
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
    // TODO: could get and then append instead of overwriting
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
                // TODO requires converting Option<Vec<String>> to &'a [String] which is horrendous
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

// TODO tests
