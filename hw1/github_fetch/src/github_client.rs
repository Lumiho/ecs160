use reqwest::{Client, Response};
// for token. do 'cargo add dotenv' in terminal to add it as a dependency
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};


pub struct GithubClient 
{
    client: Client,
    token: String
}

impl GithubClient {
    pub fn new() -> Self 
    {
        Self 
        {
            client: Client::new(),
            token: std::env::var("GITHUB_TOKEN").expect("Expected GITHUB token in .env")
        }
    }

    async fn call_github_api(&self, url: &str) -> Result<Response, reqwest::Error> 
    {
        let response = self.client
            .get(url)
            // HEADERS. GitHub always requires user agent (can be whatever). AUTHORIZATION -- uncaps requests/hour
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(USER_AGENT, "github-api")
            .header(ACCEPT, "application/vnd.github+json")
            .send()
            .await;

        response
    }

    // gets top 10 listings for each language
    pub async fn get_top10(&self, url: &str) -> Result<TopLevelApiCall, reqwest::Error> 
    {
        let repo_api_response = self.call_github_api(url).await?;
        let repo_data = repo_api_response.json::<TopLevelApiCall>().await?;
        Ok(repo_data)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopLevelApiCall 
{
    items: Vec<TempRepo>
}

// We need a separate api call to obtain the commit count
// We'll store what we can from the /search/repositories endpoint here
#[derive(Serialize, Deserialize, Debug)]
pub struct TempRepo 
{
    name: String,
    owner: Owner,
    html_url: String,
    forks_count: u32,
    language: String,
    open_issues_count: u32,
    forks_url: String, // Get the url for now, vectors later
    commits_url: String,
    issues_url: String,
}

// We will construct this FullRepo from TempRepo and an api call to get the commit count
pub struct FullRepo 
{
    name: String,
    owner: Owner,
    html_url: String,
    forks_count: u32,
    language: String,
    open_issues_count: u32,
    forks_url: String, // For now, get the urls. We will make a list later.
    commits_url: String,
    issues_url: String,
    commit_count: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Owner 
{
    login: String,
    id: u64,
    html_url: String,
    site_admin: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fork 
{
    full_name: String,
    html_url: String, // of the fork
    owner: Owner
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Commit 
{
    sha: String,
    message: String,
    author: Author
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Issue 
{
    title: String,
    body: <Option>String,
    state: String,
    createdAt: String,
    updatedAt: String
}

pub struct Author
{
    name: String,
    email: String,
    date: String
}

pub struct Owner
{
    login: String,
    id: u64,
    html_url: String,
    site_admin: bool
}
