mod github_client;
use crate::github_client::GithubClient;
use dotenv::dotenv;
use serde_json::{to_string_pretty};

// Also, create your own .env file with your GitHub token in it; not to be shared
// 'cargo add reqwest -F json' -- dependency needed to use reqwest and work with json data
// https://docs.github.com/en/rest/using-the-rest-api/getting-started-with-the-rest-api?apiVersion=2022-11-28#http-method

const URL_C: &str = "https://api.github.com/search/repositories?q=language:C&sort=stars&order=desc&per_page=10";
const URL_CPP: &str = "https://api.github.com/search/repositories?q=language:C%2B%2B&sort=stars&order=desc&per_page=10";
const URL_JAVA: &str = "https://api.github.com/search/repositories?q=language:Java&sort=stars&order=desc&per_page=10";
const URL_RUST: &str = "https://api.github.com/search/repositories?q=language:Rust&sort=stars&order=desc&per_page=10";

#[tokio::main]
async fn main()
{
    dotenv().ok();
    let github_client = GithubClient::new();

    match github_client.get_top10(URL_RUST).await {
        Ok(repo_data) => {
            match to_string_pretty(&repo_data) {
                Ok(json) => println!("{}", json),
                Err(e) => eprintln!("Error serializing json: {}", e)
            }
        }
        Err(e) => eprintln!("Error fetching repository data: {}", e),
    }
}
