// Testing out the Github API without a key for now
// use dotenv::dotenv; // for token. do 'cargo add dotenv' in terminal to add it as a dependency

use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::{to_string, to_string_pretty};

// Also, create your own .env file with your github token in it; not to be shared
// 'cargo add reqwest -F json' -- dependency needed to use reqwest and work with json data
// https://docs.github.com/en/rest/using-the-rest-api/getting-started-with-the-rest-api?apiVersion=2022-11-28#http-method

const URL_C: &str = "https://api.github.com/search/repositories?q=language:C&sort=stars&order=desc&per_page=10";
const URL_CPP: &str = "https://api.github.com/search/repositories?q=language:C%2B%2B&sort=stars&order=desc&per_page=10";
const URL_JAVA: &str = "https://api.github.com/search/repositories?q=language:Java&sort=stars&order=desc&per_page=10";
const URL_RUST: &str = "https://api.github.com/search/repositories?q=language:Rust&sort=stars&order=desc&per_page=10";

#[derive(Serialize, Deserialize, Debug)]
struct TopLevelApiCall {
    items: Vec<TempRepo>
}

// We need a separate api call to obtain the commit count
// We'll store what we can from the /search/repositories endpoint here
#[derive(Serialize, Deserialize, Debug)]
struct TempRepo {
    name: String,
    owner: Owner,
    html_url: String,
    forks_count: u32,
    language: String,
    open_issues_count: u32,
    forks_url: String, // Get the url for now
    commits_url: String,
    issues_url: String,
}

// We will construct this FullRepo from TempRepo and an api call to get the commit count
struct FullRepo {
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
struct Owner {
    login: String,
    id: u64,
    html_url: String,
    site_admin: bool
}
// gets top 10 listings
async fn get_top10(url: &str) -> Vec<()>
{    
    // let token = std::env::var("GITHUB_TOKEN").expect("Expected GITHUB token in .env"); // expect throws error if it doesn't work
    let client = reqwest::Client::new(); // create reqwest client
    let response = client
        .get(url)
        // need HEADERS. github always requires user agent (can be whatever). AUTHORIZATION -- uncaps requests/hour
        // .header(
        //     AUTHORIZATION,
        //     format!("Bearer {}", token)
        // )
        .header(USER_AGENT, "rust-github-api")
        .header(ACCEPT, "application/vnd.github+json")
        .send()
        .await;
    Vec::new()
}

async fn search_repo(url: &str) -> Result<TopLevelApiCall, reqwest::Error> {
    let client = reqwest::Client::new(); // create reqwest client
    let response = client
        .get(url)
        .header(USER_AGENT, "rust-github-api")
        .header(ACCEPT, "application/vnd.github+json")
        .send()
        // ? at the end of await gives us shorthand for match statement for errors
        .await?;

    let repo = response
        .json::<TopLevelApiCall>()
        .await
        .expect("Something went wrong while parsing");

    Ok(repo)
}

#[tokio::main]
async fn main()
{
    // dotenv().ok();
    // let list = [URL_C,URL_CPP,URL_JAVA,URL_RUST];
    // for item in list{
    //     get_top10(item);
    // }
    let test_rust_repo_result  = search_repo(URL_RUST).await;

    match test_rust_repo_result {
        Ok(repo_api_call) => {
            match to_string_pretty(&repo_api_call) {
                Ok(pretty_json_string) => println!("{}", pretty_json_string),
                Err(e) => eprintln!("Error: {}", e)
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
