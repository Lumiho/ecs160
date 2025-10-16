use reqwest::Method;
use reqwest::header::LINK;
use crate::github::github_models::Owner;
use crate::github::github_client::GithubClient;

pub struct ForkRepo {
    name: String,
    owner: Owner,
    html_url: String,
    new_commits: u32,
}

// This is just a function to test functionality. Will call a build_forks function that uses a github client
// that exists already in github_full_model
// pub async fn build_forks() -> Result<ForkRepo, reqwest::Error>
// {
//     let client = GithubClient::new();
//     let url = "https://api.github.com/repos/ventoy/Ventoy/forks?per_page=100&page=1"

    // First, we need to determine if a header has a "next" tag
    // https://api.github.com/repos/{owner}/{repository}/forks?per_page=100&page=1
    // We'll use this endpoint for testing.: "https://api.github.com/repos/ventoy/Ventoy/forks?per_page=100&page=1"
    // And we'll get the headers first and find the Link header:
    // https://api.github.com/repos/ventoy/Ventoy/forks?per_page=100&page=1
    // <https://api.github.com/repositories/246335987/forks?sort=newest&per_page=20&page=230>; rel="next"

//     let forks_header_response = client.call_github_api(url, Method::HEAD).await;
//
//     match(forks_header_response) {
//         Ok(resp) => {
//             if let Some(link_header) = resp.headers().get(LINK) {
//                 let header_str = link_header.to_str().unwrap();
//
//                 // We specifically search for "next". If there is no "next" tag, we are at the last page.
//                 if header_str.contains("/next/") {
//
//                 }
//                 else {
//
//                 }
//             }
//         }
//         Err(e) => {eprintln!("Error getting header: {}", e);}
//     }
//
//     // Parsing logic to find "next"
//
//
//     Ok(fork_repo)
// }