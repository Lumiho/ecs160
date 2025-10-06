mod github_client;
use crate::github_client::GithubClient;
use dotenv::dotenv;
use serde_json::{to_string_pretty};
use reqwest::{Method};

// Also, create your own .env file with your GitHub token in it; not to be shared
// 'cargo add reqwest -F json' -- dependency needed to use reqwest and work with json data
// https://docs.github.com/en/rest/using-the-rest-api/getting-started-with-the-rest-api?apiVersion=2022-11-28#http-method

// search api links
const URL_C: &str = "https://api.github.com/search/repositories?q=language:C&sort=stars&order=desc&per_page=10";
const URL_CPP: &str = "https://api.github.com/search/repositories?q=language:C%2B%2B&sort=stars&order=desc&per_page=10";
const URL_JAVA: &str = "https://api.github.com/search/repositories?q=language:Java&sort=stars&order=desc&per_page=10";
const URL_RUST: &str = "https://api.github.com/search/repositories?q=language:Rust&sort=stars&order=desc&per_page=10";

// commit count
//URL_COMMIT: &str = "https://api.github.com/repos/{OWNER}/{REPO}/commits?per_page=1"


#[tokio::main]
async fn main()
{
    dotenv().ok();
    let github_client = GithubClient::new();
    let urls = [URL_C, URL_CPP, URL_JAVA, URL_RUST];

    for url in urls
    {
        let repo_result = github_client.get_top10(url).await;
        
        match repo_result 
        {
            Ok(repo_api_call) => 
            {
                match to_string_pretty(&repo_api_call) 
                {
                    Ok(json_string) => println!("{}", json_string),
                    Err(e) => eprintln!("Error serializing json: {}", e)
                }
                println!()
            }
            Err(e) => 
            {
                eprintln!("Error fetching repository data: {}", e);
            }
        }
    }
}
