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

    async fn call_github_api(&self, url: &str, method: Method) -> Result<Response, reqwest::Error>
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
            let commit_url = temp_repo.commits_url.replace("{/sha}", "?per_page=1");
            let commit_result = self.get_commits(&commit_url).await;
            println!("Commits: {:?}", commit_result)
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
    // pub async fn get_commit_count(&self, top_level_json: &TopLevelApiCall) -> Vec<u32> {
    //     // From TopLevelApiCall. We need TempRepo.name. TempRepo.Owner.login
    //     let repo_data = top_level_json;
    //     let mut commit_count: Vec<u32> = Vec::new();
    // 
    //     for (i, repo) in repo_data.items.iter().enumerate() {
    //         let commit_url = format!("https://api.github.com/repos/{}/{}/commits?per_page=1", repo.owner.login, repo.name);
    //         let header_data = self.call_github_api(&commit_url, Method::HEAD).await;
    // 
    //         match header_data {
    //             Ok(response) => {
    //                 if let Some(link_header) = response.headers().get(LINK) {
    //                     // Having super verbose comments to remember what the code does for now. Will delete these comments later.
    //                     // Parsing Logic for the link response.
    // 
    //                     // We get 10 of these lines. One for each repository:
    //                     // <https://api.github.com/repositories/724712/commits?per_page=1&page=2>; rel="next", <https://api.github.com/repositories/724712/commits?per_page=1&page=307367>; rel="last"
    //                     // Split them by newline and put them in a vector
    //                     let link_response_vector: Vec<&str> = link_header.to_str().unwrap().split('\n').collect();
    // 
    //                     // After splitting the header by whitespace, we get the 3rd substring, which is the url that contains the commit count
    //                     // Example Result: "<https://api.github.com/repositories/724712/commits?per_page=1&page=307367>"
    //                     let mut commit_count_url: Vec<&str> = Vec::new();
    //                     for link_header in link_response_vector {
    //                         commit_count_url.extend(link_header.split_whitespace().nth(2));
    //                     }
    // 
    //                     // Split the string by page= and the 3rd substring is the commit count followed by a '>'.
    //                     // Example Result: "307367>"
    //                     let mut commit_data: Vec<&str> = Vec::new();
    //                     for data in commit_count_url {
    //                         commit_data.extend(data.split("page=").nth(2));
    //                     }
    // 
    //                     // Lastly, split the string about the '>' character and obtain the commit count
    //                     // Example Result: "307367"
    //                     let mut commit_count_strings: Vec<&str> = Vec::new();
    //                     for count in commit_data {
    //                         commit_count_strings.extend(count.split(">").nth(0))
    //                     }
    // 
    //                     // Convert those counts to a u32 integer and extend them onto our commit_count vector
    //                     commit_count.extend(commit_count_strings.iter().map(|s| s.parse::<u32>().unwrap()));
    //                 }
    //             }
    //             Err(e) => {
    //                 eprintln!("{}", e);
    //             }
    //         }
    //     }
    //     commit_count
    // }
}
