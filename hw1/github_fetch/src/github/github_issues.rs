use crate::github::github_models::{TempRepo};
use crate::github::github_client::GithubClient;
use crate::github::github_parser::{get_values, parse_items};
use reqwest::Method;

// no serde 😒😒😒. Only need title, body, state, created/updated_at for hw1
#[derive(Debug, Clone)]
pub struct Issue
{
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    // pub html_url: Option<String>,
    // pub number: Option<u32>,
    // pub user: Option<IssueUser>,
}

// #[derive(Debug, Clone)]
// pub struct IssueUser 
// {
//     pub login: String,
//     pub id: u64,
//     pub html_url: Option<String>,
// }

// pub async fn attach_issues(client: &GithubClient, repo: &mut TempRepo) -> Result<(), reqwest::Error>
// {
//     let issues_url = repo.issues_url.replace("{/number}", "");
//     let issues_json = client
//         .call_github_api(&issues_url, Method::GET)
//         .await?
//         .text().await?;
//     let issues = build_issues(&issues_json);
//     repo.issues = Some(issues);
//     Ok(())
// }

pub fn build_issues(json: &str) -> Vec<Issue>
{
    let mut issues: Vec<Issue> = Vec::new();

    // Split into top-level objects in the JSON array
    let items = parse_items(json);

    for item in items
    {
        let title = get_values(item, "title");
        let body = get_values(item, "body");
        let state = get_values(item, "state");
        let created_at = get_values(item, "created_at");
        let updated_at = get_values(item, "updated_at");
        // let mut user_login = None;
        // let mut user_id = None;
        // let mut user_url = None;

        let issue = Issue
        {
            title: title.unwrap_or_else(|| "Untitled".to_string()),
            body,
            state: state.unwrap_or_else(|| "unknown".to_string()),
            created_at: created_at.unwrap_or_default(),
            updated_at: updated_at.unwrap_or_default(),
            // html_url,
            // number: number.and_then(|n| n.parse::<u32>().ok()),
            // user: Some(crate::github::github_models::IssueUser //only created if user block exists
            // { 
            //     login: user_login.unwrap_or_else(|| "unknown".to_string()),
            //     id: user_id.and_then(|id| id.parse::<u64>().ok()).unwrap_or_default(),
            //     html_url: user_url, 
            // })
        };
        issues.push(issue);
    }
    issues
}


