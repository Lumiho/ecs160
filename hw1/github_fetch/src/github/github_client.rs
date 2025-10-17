// Useful HTTP Methods from reqwest
// Method::GET
// Method::HEAD
use reqwest::{Client, Response, Method, header::{HeaderMap, HeaderValue}};

// for token. do 'cargo add dotenv' in terminal to add it as a dependency
use reqwest::header::{ACCEPT, AUTHORIZATION, LINK, USER_AGENT};
use crate::github::github_models::*;
use crate::github::github_parser::build_temp_repo;

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

    pub async fn call_github_api(&self, url: &str, method: Method) -> Result<Response, reqwest::Error>
    {
        let response = self.client
            .request(method, url)
            // HEADERS. GitHub requires user agent (can be whatever). AUTHORIZATION -- uncaps requests/hour
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(USER_AGENT, "github-api")
            .header(ACCEPT, "application/vnd.github+json")
            .send()
            .await;
        response
    }

    // gets top 10 listings for each language
    // TODO: Eventually this should return Vec<Repo> -> Change naming of FullRepo to Repo
    pub async fn get_top10(&self, url: &str) -> Result<Vec<TempRepo>, reqwest::Error>
    {
        let repo_api_response = self.call_github_api(url, Method::GET).await?;
        let repo_data = repo_api_response.text().await?;
        let temp_repos = build_temp_repo(&repo_data);
        for temp_repo in &temp_repos{
            // let commit_url = temp_repo.commits_url.replace("{/sha}", "?per_page=1");
            // let commit_result = self.get_commits(&commit_url).await;
            let commit_count = self
                .get_commit_count(&temp_repo.owner.login, &temp_repo.name)
                .await
                .unwrap_or(0);
            //println!("Commits: {:?}", commit_result);
            println!("Commit_Count: {:?}", commit_count);

        }
        Ok(temp_repos)
    }

    pub async fn get_commits(&self, url: &str) -> Result<String, reqwest::Error>
    {
        let commit_api_response = self.call_github_api(url, Method::GET).await?;
        let commit_data = commit_api_response.text().await?;
        Ok(commit_data)
    }

    // commit api endpoint: "https://api.github.com/{}/{}/rust/commits?per_page=1"
    // TODO: Gotta refactor this to work with github_parser.rs to build full repo
    // Leo's plan: focus on just getting one commit count, can loop the fxn later
    pub async fn get_commit_count(&self, owner: &str, repo: &str) -> Result<u32, reqwest::Error> {

        let commit_url = format!("https://api.github.com/repos/{}/{}/commits?per_page=1", owner, repo);
        let header_resp = self.call_github_api(&commit_url, Method::HEAD).await;


        match header_resp {
            Ok(response) => {
                if let Some(link_header) = response.headers().get(LINK) {
                    // ---- Parsing logic from your comment ----
                    let link_str = link_header.to_str().unwrap_or("");

                    // Example Link:
                    // <https://api.github.com/...page=2>; rel="next",
                    // <https://api.github.com/...page=307367>; rel="last"
                    let mut commit_count: Option<u32> = None;

                    // Split by commas (each link rel pair)
                    for part in link_str.split(',') {
                        // only look for the "last" page link
                        if part.contains("rel=\"last\"") {
                            // extract the part with "page=###"
                            if let Some(start) = part.find("page=") {
                                let slice = &part[start + 5..];

                                if let Some(end) = slice.find('>') {
                                    let count_slice = &slice[..end].split("=").nth(1);

                                    match  count_slice {
                                        Some(count_string) => {
                                            match count_string.parse::<u32>() {
                                                Ok(count) => commit_count = Some(count),
                                                Err(e) => eprintln!("Error converting count to u32: {}", e),
                                            }
                                        }
                                        None => println!("Error: '=' not found for {}", part)
                                    }
                                }
                            }
                        }
                    }
                    if let Some(count) = commit_count {
                        return Ok(count);
                    }
                }

                // ---- Fallback if no Link header ----
                let fallback_url = format!(
                    "https://api.github.com/repos/{}/{}/commits?per_page=100",
                    owner, repo
                );
                let commits_json = self.call_github_api(&fallback_url, Method::GET).await?;
                let commits_text = commits_json.text().await?;
                let commit_list: Vec<serde_json::Value> =
                    serde_json::from_str(&commits_text).unwrap_or_default();

                let count = commit_list.len() as u32;
                if count == 100 {
                    println!(
                        "⚠️ Repo {}/{} may have >100 commits (no Link header, truncated)",
                        owner, repo
                    );
                } else {
                    println!("Repo {}/{} has {} commits (small repo)", owner, repo, count);
                }
                Ok(count)
            }
            Err(e) => {
                eprintln!("Error getting commit count for {}/{}: {}", owner, repo, e);
                Ok(0)
            }
        }
    }
}

#[cfg(test)]
mod tests
{
    // tests the get_commit_count() function against sample_header_data.json
    #[test]
    fn test_get_commit_count() {

    }
}