mod github_issue;

fn main() -> Result<(), ()> {
    let config = github_issue::Config{
        title: Some(String::from("testing")),
        body: Some(String::from("go approve your Concourse manual step!")),
        milestone: Some(1000),
        labels: Some(vec![String::from("")]),
        assignees: Some(vec![String::from("")]),
        number: None,
        state: None,
    };

    match github_issue::read(1, issues) {
        Ok(_) => println!("success"),
        Err(_) => eprintln!("failure"),
    }

    async fn run() -> octocrab::Result<()> {
        let octocrab = octocrab::Octocrab::default();
        let issue = octocrab.issues("mschuchard", "puppet-check").get(3).await?;
        println!("{:#?}", issue);
        Ok(())
    }
    let octocrab = octocrab::Octocrab::default();
    let issue = octocrab.issues("mschuchard", "puppet-check").list().send();

    Ok(())
}
