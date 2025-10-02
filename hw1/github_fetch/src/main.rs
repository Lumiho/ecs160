use dotenv::dotenv; // 'cargo add [dependency]' in terminal to add each as a dependency in .toml
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use reqwest::blocking::Client;
use serde_json::Value;
// Also, create your own .env file with your github token in it; not to be shared
// 'cargo add reqwest -F json' and 'cargo add reqwest -F blocking' -- dependencies needed to use reqwest and work with json data
// Useful sources, using rest api and parsing json with serde:
    // https://docs.github.com/en/rest/using-the-rest-api/getting-started-with-the-rest-api?apiVersion=2022-11-28#http-method
    // https://www.slingacademy.com/article/reading-and-writing-json-files-in-rust-with-serde/

const URL_C: &str = "https://api.github.com/search/repositories?q=language:C&sort=stars&order=desc&per_page=10";
const URL_CPP: &str = "https://api.github.com/search/repositories?q=language:C%2B%2B&sort=stars&order=desc&per_page=10";
const URL_JAVA: &str = "https://api.github.com/search/repositories?q=language:Java&sort=stars&order=desc&per_page=10";
const URL_RUST: &str = "https://api.github.com/search/repositories?q=language:Rust&sort=stars&order=desc&per_page=10";


// We need to create a struct that has the JSON fields we want from the json object
struct Data
{
    //ex: 
        // name: String,
        // age: u8,
}

// gets top 10 listings
fn get_top10(url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> // rust told me to add this return type lol
{    
    let token = std::env::var("GITHUB_TOKEN").expect("Expected GITHUB token in .env");
    let client = Client::new(); // create reqwest client
    let response = client
        .get(url)

        // HEADERS. github always requires User_Agent (can be whatever). AUTHORIZATION -- uncaps requests/hour
        .header(AUTHORIZATION, 
            format!("Bearer {}", token))
        .header(USER_AGENT, 
            "rust-github-api")
        .send()? //if successful, returns a json object -- time for parsing. ? tells to unwrap if safe
        .json();

    Ok(())
}

fn main() 
{
    dotenv().ok();
    let list = [URL_C,URL_CPP,URL_JAVA,URL_RUST];
    for item in list{
        get_top10(item);
    }

}
