mod concourse;
mod github_issue;

struct GithubIssue{}

#[tokio::main]
async fn main() -> Result<(), ()> {
    // instantiate issue
    let gh_issue = github_issue::Issue::new(None, "mschuchard", "puppet-check", None, None, None, None, Some(1), None);
    let issue = gh_issue.main(github_issue::Action::Read).await;
    println!("{issue:#?}");

    Ok(())
}

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
    fn resource_check(
        source: Option<Self::Source>,
        version: Option<Self::Version>,
    ) -> Vec<Self::Version> {
        vec![concourse::Version::new(String::from("Open"))]
    }

    /// Dummies the in step as it performs no functionality.
    fn resource_in(
        _source: Option<Self::Source>,
        _version: Self::Version,
        _params: Option<Self::InParams>,
        _output_path: &str,
    ) -> Result<concourse_resource::InOutput<Self::Version, Self::InMetadata>, Box<dyn std::error::Error>> {
        Ok(concourse_resource::InOutput {
            version: concourse::Version::new(String::from("Open")),
            metadata: None,
        })
    }

    /// Performs the out step for the resource. Creates a new Github issue based on the parameters.
    fn resource_out(
        source: Option<Self::Source>,
        params: Option<Self::OutParams>,
        input_path: &str
    ) -> concourse_resource::OutOutput<Self::Version, Self::OutMetadata> {
        concourse_resource::OutOutput {
            version: concourse::Version::new(String::from("Open")),
            metadata: None,
        }
    }
}

impl GithubIssue {

}

//concourse_resource::create_resource!(GithubIssue);
