mod github_issue;

fn main() -> Result<(), ()> {
    let config = github_issue::Config{
        owner: String::from("mschuchard"),
        repo: String::from("dummy/dummy"),
        title: Some(String::from("testing")),
        body: Some(String::from("go approve your Concourse manual step!")),
        milestone: Some(1000),
        labels: Some(vec![String::from("")]),
        assignees: Some(vec![String::from("")]),
        number: None,
        state: None,
    };

    let client = octocrab::Octocrab::default();

    match github_issue::create(config, client) {
        Ok(_) => println!("success"),
        Err(_) => eprintln!("failure"),
    }

    Ok(())
}
