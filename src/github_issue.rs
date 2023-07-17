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
    // create and update
    title: Option<String>,
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
    /// let gh_issue = Issue::new(None, String::from("my_org"), String::from("my_repo"), None, None, None, None, Some(100), None);
    /// ```
    pub fn new(pat: Option<&'issue str>, owner: &'issue str, repo: &'issue str, title: Option<String>, body: Option<&'issue str>, labels: Option<Vec<String>>, assignees: Option<Vec<String>>, number: Option<u64>, state: Option<octocrab::models::IssueState>) -> Self {
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
            Action::Create => println!("create is currently unsupported"),
            // read an issue state
            Action::Read => {
                match self.read_state(issues).await {
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
    async fn create<'octo>(&self, issues: octocrab::issues::IssueHandler<'octo>) -> Result<octocrab::models::issues::Issue, &str> {
        // validate a title was specified
        match self.title {
            // title specified
            Some(title) => {
                // build the issue
                match issues.create(title)
                    // ... with optional parameters
                    .body(self.body)
                    .labels(self.labels)
                    .assignees(self.assignees)
                    // send and await the issue
                    .send()
                    .await
                {
                    // return created issue
                    Ok(issue) => return Ok(issue),
                    // issue could not be created
                    Err(error) => {
                        println!("the issue could not be created");
                        println!("{error}");
                        return Err("issue uncreated");
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

    /*/// Update a Github Issue according to configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// TODO
    /// ```

    async fn update<'octo>(&self, issues: octocrab::issues::IssueHandler<'octo>) -> Result<(), ()> {
        let issue = issues
        .update(1234u64)
        // Optional Parameters
        .title("Updated title")
        .body("New body")
        .state(models::IssueState::Closed)
        .assignees(&[String::from("ferris")])
        .labels(&[String::from("help wanted"), String::from("good first issue")])
        // Send the request
        .send()
        .await?;

        Ok(())
    }*/
}


// TODO tests
