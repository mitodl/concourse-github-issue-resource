//! # Github Issue
//!
//! `github_issue` is a minimal utility to create and update issues within Github.

pub enum Action {
    Create,
    Read,
    Update
}

#[derive(Eq, PartialEq, Debug)]
pub struct Issue<'issue> {
    // client
    pat: Option<&'issue str>,
    owner: &'issue str,
    repo: &'issue str,
    // create and update (create expects String instead of &str, and update expects &[String] instead of Vec<>)
    title: Option<&'issue str>,
    body: Option<&'issue str>,
    labels: Option<Vec<String>>,
    assignees: Option<Vec<String>>,
    // read and update
    number: Option<u64>,
    // update
    state: Option<octocrab::models::IssueState>
}

impl<'issue> Issue<'issue> {
    /// Constructor for the Config struct. Contains all of the members necessary for instantiating a client and performing an action.
    ///
    /// # Examples
    ///
    /// ```
    /// let gh_issue = Issue::new(None, "my_org", "my_repo", None, None, None, None, Some(100), None);
    /// ```
    pub fn new(pat: Option<&'issue str>, owner: &'issue str, repo: &'issue str, title: Option<&'issue str>, body: Option<&'issue str>, labels: Option<Vec<String>>, assignees: Option<Vec<String>>, number: Option<u64>, state: Option<octocrab::models::IssueState>) -> Self {
        // return instantiated github issue
        return Self { pat, owner, repo, title, body, labels, assignees, number, state }
    }

    /// Instantiate a reusable Octocrab issues object with input authentication, and an input owner and repo.
    ///
    /// # Examples
    ///
    /// ```
    /// let issues =
    /// ```
    pub async fn main<'octo>(&self, action: Action) -> Result<(), ()> {
        // instantiate client and issues
        let client = match self.pat {
            Some(pat) => octocrab::Octocrab::builder()
            .personal_token(pat.to_string())
            .build()
            .expect("could not authenticate client with Personal Access Token"),
            None => octocrab::Octocrab::default(),
        };
        let issues = client.issues(self.owner, self.repo);
        // execute action
        match action {
            // create an issue
            Action::Create => {
                match self.create(issues).await {
                    Ok(issue) => println!("{issue:#?}"),
                    Err(error) => println!("{error}"),
                }
            },
            // read an issue state
            Action::Read => {
                match self.read_state(issues).await {
                    Ok(state) => println!("{state:#?}"),
                    Err(error) => println!("{error}"),
                }
            },
            // update an issue
            Action::Update => {
                match self.update(issues).await {
                    Ok(issue) => println!("{issue:#?}"),
                    Err(error) => println!("{error}"),
                }
            },
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
    async fn create<'octo>(&self, issues: octocrab::issues::IssueHandler<'octo>) -> Result<octocrab::models::issues::Issue, &str> {
        // validate a title was specified
        match self.title {
            // title specified
            Some(title) => {
                // build the issue
                let mut issue = issues.create(title);
                // ... with optional parameters
                if self.body.is_some() {
                    issue = issue.body(self.body.unwrap());
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
                    },
                }
            },
            // title unspecified
            None => {
                println!("a title was not specified, and so an issue could not be created");
                return Err("title unspecified");
            },
        }
    }

    /// Read a Github Issue according to configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// TODO
    /// ```

    async fn read_state<'octo>(&self, issues: octocrab::issues::IssueHandler<'octo>) -> Result<octocrab::models::IssueState, &str> {
        // validate an issue number was specified
        match self.number {
            // issue number specified
            Some(number) => {
                // retrieve the issue with the handler
                let issue = match issues.get(number).await {
                    Ok(issue) => issue,
                    // issue number probably does not exist, or some other error
                    Err(error) => {
                        println!("the issue number {number} could not be retrieved");
                        println!("{error}");
                        return Err("unknown issue state");
                    },
                };
                // return the issue state
                return Ok(issue.state);
            }
            // issue number unspecified
            None => {
                println!("an issue number was not specified, and so its state cannot be retrieved");
                return Err("issue number unspecified");
            }
        }
    }

    /// Update a Github Issue according to configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// TODO
    /// ```

    // TODO: could get and then append instead of overwriting
    async fn update<'octo>(&self, issues: octocrab::issues::IssueHandler<'octo>) -> Result<octocrab::models::issues::Issue, &str> {
        // validate an issue number was specified
        match self.number {
            // issue number specified
            Some(number) => {
                // build the issue
                let mut issue = issues.update(number);
                // ... with optional parameters
                if self.title.is_some() {
                    issue = issue.title(self.title.unwrap());
                }
                if self.body.is_some() {
                    issue = issue.body(self.body.unwrap());
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
                    },
                }
            },
            // issue number unspecified
            None => {
                println!("an issue number was not specified, and so an issue could not be updated");
                return Err("issue number unspecified");
            },
        }
    }
}


// TODO tests
