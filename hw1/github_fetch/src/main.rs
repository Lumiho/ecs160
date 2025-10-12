#![allow(warnings)]
mod github;
use github::github_client::GithubClient;
use dotenv::dotenv;


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
            Ok(temp_repos) =>
                for (i, temp_repo) in temp_repos.iter().enumerate() {
                    println!("Repository {}: {:#?}", i + 1, temp_repo);
                }
            Err(e) =>
                {
                    eprintln!("Error fetching repository data: {}", e);
                }
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use crate::github::github_parser::get_values;
    #[test]
    fn analyze_output()
    {
        let data = std::fs::read_to_string("../output_repos.json").expect("Failed to read file");

        // Split JSON objects
        let json_objects: Vec<&str> = data.split("\n{").collect();

        let mut total_repos = 0;
        let mut total_forks = 0;

        for (i, json_str) in json_objects.iter().enumerate() 
        {
            // Re-add '{' if split removed it
            let json_text = if i > 0 { format!("{{{}", json_str) } else { json_str.to_string()};

            for repo_chunk in json_text.split("\"name\":").skip(1) 
            {
                total_repos += 1;

                // reconstruct a minimal JSON slice
                let repo_json = format!("\"name\":{}", repo_chunk);

                // detect forks if the "fork" key is present
                if let Some(fork_val) = get_values(&repo_json, "fork") 
                {
                    if fork_val == "true" 
                    {
                        total_forks += 1;
                    }
                }
            }
        }
        let total_main = total_repos - total_forks;
        println!("Total repositories: {}", total_repos);
        println!("Main/original repos: {}", total_main);
        println!("Forked repos: {}", total_forks);
    }
}

