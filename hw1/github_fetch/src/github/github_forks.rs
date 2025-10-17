use reqwest::{get, Method};
use reqwest::header::HeaderValue;
use reqwest::header::LINK;
use crate::github::github_models::Owner;
use crate::github::github_client::GithubClient;
use crate::github::github_parser::*;

#[derive(Debug)]
pub struct ForkRepo {
    name: String,
    owner: Owner,
    html_url: String,
    created_at: String,
    new_commits: u32,
}

// This function builds the list of forks for one repository
// TODO: Get the count of commits since the created_at date
pub async fn build_forks(client: &GithubClient, url: &str) -> Result<Vec<ForkRepo>, String>
{
    // Endpoint used: https://api.github.com/repos/{owner}/{repository}/forks?per_page=100&page=1
    // First, serialize the first page of forks from json to structs

    let mut forks: Vec<ForkRepo> = Vec::new();
    let mut page = 1;

    // Append these search parameters to the forks endpoint and get the link header to obtain relative types like "next"
    // Will use "next" to loop through all pages in order to obtain all forks
    let mut process_url = format!("{}?per_page={}", url, page);

    loop {
        let fork_items_resp = client.call_github_api(&process_url, Method::GET).await;
        let mut next: bool = false;

        match(fork_items_resp) {

            Ok(fork_items_body) => {
                // TODO
                let link_header = fork_items_body
                    .headers()
                    .get("link")
                    .and_then(|h| h.to_str().ok())
                    .map(|s| s.to_string())
                    .unwrap();

                let fork_items_string = fork_items_body.text().await.unwrap();
                let fork_items = parse_items(&fork_items_string);

                for item in fork_items {
                    let owner_string = get_owner(&item, "owner").unwrap();
                    let owner = Owner {
                        login: get_values(owner_string, "login").unwrap().parse::<String>().unwrap(),
                        id: get_values(owner_string, "id").unwrap().parse::<u64>().unwrap(),
                        html_url: get_values(owner_string, "html_url").unwrap().parse::<String>().unwrap(),
                        site_admin: get_values(owner_string, "site_admin").unwrap().parse::<bool>().unwrap()
                    };

                    let fork_repo = ForkRepo {
                        name: get_values(item, "name").unwrap(),
                        owner: owner,
                        html_url: get_values(item, "html_url").unwrap(),
                        created_at: get_values(item, "created_at").unwrap(),
                        new_commits: 1
                    };

                    forks.push(fork_repo);
                }
                // Second, we need to determine if a header has a "next" tag
                // <https://api.github.com/repositories/246335987/forks?sort=newest&per_page=20&page=230>; rel="next"
                if link_header.contains("next") {
                    next = true;
                } else{
                    next = false;
                }
            }
            Err(_) => {}
        }

        // If the next tag exists, go to the next page and update the url
        if next {
            page += 1;
            process_url = get_relative_url(url, "next")?; // handle error later
        } else {
            break;
        }
    }
    Ok(forks)
}

#[cfg(test)]
mod tests {
    use super::build_forks;
    use dotenv::dotenv;
    #[tokio::test]
    async fn test_build_forks() {
        dotenv().ok();
    }
}


