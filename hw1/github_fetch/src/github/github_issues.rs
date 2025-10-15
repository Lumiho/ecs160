use crate::github::github_models::Issue;

// no serde 😒😒😒
#[derive(Debug, Clone)]
pub struct Issue 
{
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub html_url: Option<String>,
    pub number: Option<u32>,
    pub user: Option<IssueUser>,
}

#[derive(Debug, Clone)]
pub struct IssueUser 
{
    pub login: String,
    pub id: u64,
    pub html_url: Option<String>,
}

pub fn issue_builder(json: &str) -> Vec<Issue> 
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
        let html_url = get_values(item, "html_url");
        let number = get_values(item, "number");

        let user_block = get_nested_block(item, "user");
        let mut user_login = None;
        let mut user_id = None;
        let mut user_url = None;

        if let Some(block) = user_block 
        {
            user_login = get_values(block, "login");
            user_id = get_values(block, "id");
            user_url = get_values(block, "html_url");
        }

        let issue = Issue
        {
            title: title.unwrap_or_else(|| "Untitled".to_string()), 
            body,
            state: state.unwrap_or_else(|| "unknown".to_string()),
            created_at: created_at.unwrap_or_default(),
            updated_at: updated_at.unwrap_or_default(),
            html_url,
            number: number.and_then(|n| n.parse::<u32>().ok()),
            user: Some(crate::github::github_models::IssueUser //only created if user block exists
            { 
                login: user_login.unwrap_or_else(|| "unknown".to_string()),
                id: user_id.and_then(|id| id.parse::<u64>().ok()).unwrap_or_default(),
                html_url: user_url, 
            })
        };
        issues.push(issue);
    } 
    issues
}

