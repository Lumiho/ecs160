use dotenv::dotenv; // for token. do 'cargo add dotenv' in terminal to add it as a dependency
use reqwest::header::{AUTHORIZATION, USER_AGENT};
// Also, create your own .env file with your github token in it; not to be shared
// 'cargo add reqwest -F json' -- dependency needed to use reqwest and work with json data
// https://docs.github.com/en/rest/using-the-rest-api/getting-started-with-the-rest-api?apiVersion=2022-11-28#http-method

const URL_C: &str = "https://api.github.com/search/repositories?q=language:C&sort=stars&order=desc&per_page=10";
const URL_CPP: &str = "https://api.github.com/search/repositories?q=language:C%2B%2B&sort=stars&order=desc&per_page=10";
const URL_JAVA: &str = "https://api.github.com/search/repositories?q=language:Java&sort=stars&order=desc&per_page=10";
const URL_RUST: &str = "https://api.github.com/search/repositories?q=language:Rust&sort=stars&order=desc&per_page=10";

// gets top 10 listings
fn get_top10(url: &str)
{    
    let token = std::env::var("GITHUB_TOKEN").expect("Expected GITHUB token in .env"); // expect throws error if it doesn't work
    let client = reqwest::Client::new(); // create reqwest client
    let response = client
        .get(url)
        // need HEADERS. github always requires user agent (can be whatever). AUTHORIZATION -- uncaps requests/hour
        .header(
            AUTHORIZATION, 
            format!("Bearer {}", token)
        )
        .header(
            USER_AGENT, "rust-github-api"
        )
        .send()
        .await;
}

fn main() 
{
    dotenv().ok();
    let list = [URL_C,URL_CPP,URL_JAVA,URL_RUST];
    for item in list{
        get_top10(item);
    }

}
