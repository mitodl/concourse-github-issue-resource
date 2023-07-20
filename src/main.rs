use concourse_resource::*;

mod concourse;
mod github_issue;

struct GithubIssue {}

impl concourse_resource::Resource for GithubIssue {
    // implementtations for inputs and outputs
    type Source = concourse::Source;
    type Version = concourse::Version;
    type InParams = concourse_resource::Empty;
    type InMetadata = concourse_resource::Empty;
    type OutParams = concourse::OutParams;
    type OutMetadata = concourse::OutMetadata;

    // implementations for steps
    /// Performs the check step for the resource. Returns a single sized vector of version of state string if the input issue is Open (no trigger), and a two sized vector of version of state string if the input issue is closed (trigger). For convenience and standardization the former return is "Open", and the latter is "Open" and "Closed".
    #[tokio::main]
    async fn resource_check(
        source: Option<Self::Source>,
        _version: Option<Self::Version>,
    ) -> Vec<Self::Version> {
        // validate and re-assign source
        let source = match source {
            Some(source) => source,
            None => panic!("source is required for the Github Issue resource"),
        };

        // if no number is specified in source then this resource should skip check step and cannot trigger
        if source.number().is_none() {
            println!(
                "no issue number was specified in source, and therefore the check step is skipped"
            );
            return vec![concourse::Version::new(String::from("Open"))];
        }

        // construct an issue...
        let gh_issue = github_issue::Issue::new(
            source.pat(),
            source.owner(),
            source.repo(),
            None,
            None,
            None,
            None,
            source.number(),
            None,
        );
        // ...and read the octocrab issue
        let issue = match gh_issue.main(github_issue::Action::Read).await {
            Ok(issue) => issue,
            Err(error) => {
                println!("{error}");
                panic!("the check step was unable to read the specified github issue number");
            }
        };

        // return one sized version vector if issue is open and two sized if closed
        match issue.state {
            octocrab::models::IssueState::Open => vec![concourse::Version::new(String::from("Open"))],
            octocrab::models::IssueState::Closed => vec![concourse::Version::new(String::from("Open")), concourse::Version::new(String::from("Closed"))],
            _ => panic!("expected the github issue state to either be open or closed, and somehow it is something else")
        }
    }

    /// Dummies the in step as it performs no functionality.
    #[tokio::main]
    async fn resource_in(
        _source: Option<Self::Source>,
        _version: Self::Version,
        _params: Option<Self::InParams>,
        _output_path: &str,
    ) -> Result<
        concourse_resource::InOutput<Self::Version, Self::InMetadata>,
        Box<dyn std::error::Error>,
    > {
        Ok(concourse_resource::InOutput {
            version: concourse::Version::new(String::from("Open")),
            metadata: None,
        })
    }

    /// Performs the out step for the resource. Creates a new Github issue based on the parameters.
    #[tokio::main]
    async fn resource_out(
        source: Option<Self::Source>,
        params: Option<Self::OutParams>,
        _input_path: &str,
    ) -> concourse_resource::OutOutput<Self::Version, Self::OutMetadata> {
        // validate source and params
        let source = match source {
            Some(source) => source,
            None => panic!("source is required for the Github Issue resource"),
        };
        let params = match params {
            Some(params) => params,
            None => panic!("params is required for the Github Issue resource out/put step"),
        };

        // construct an issue...
        let gh_issue = github_issue::Issue::new(
            source.pat(),
            source.owner(),
            source.repo(),
            Some(params.title()),
            params.body(),
            params.labels(),
            params.assignees(),
            None,
            None,
        );
        // ...and create the octocrab issue
        let issue = match gh_issue.main(github_issue::Action::Create).await {
            Ok(issue) => issue,
            Err(error) => {
                println!("{error}");
                panic!("the out/put step was unable to create the associated github issue");
            }
        };

        // TODO store issuue number somewhere for subsequent check step

        // return out step output
        concourse_resource::OutOutput {
            version: concourse::Version::new(String::from("Open")),
            metadata: Some(concourse::OutMetadata::new(
                issue.number,
                issue.labels,
                issue.assignees,
            )),
        }
    }
}

// helper functions if we need them
impl GithubIssue {}
concourse_resource::create_resource!(GithubIssue);
