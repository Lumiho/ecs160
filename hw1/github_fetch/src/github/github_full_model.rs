use crate::github::github_client::GithubClient;
use crate::github::github_models::{TempRepo,FullRepo};
use crate::github::github_issues::build_issues;
use crate::github::github_commits::build_commits;
use reqwest::Method;

pub async fn build_full_repo(client: &GithubClient, temp_repo: TempRepo) -> Result<FullRepo, reqwest::Error> 
{
    let mut repo = temp_repo;
    let issues_url = repo.issues_url.clone();

    let issues_json = client
        .call_github_api(&issues_url.replace("{/number}", ""), reqwest::Method::GET)
        .await?
        .text().await?;

    let commit_url = repo.commits_url.replace("{/sha}", "?per_page=50");
    let commit_api_response = client.call_github_api(&commit_url, Method::GET).await?;
    let commit_data = commit_api_response.text().await?;

    let commits = build_commits(&commit_data);
    let issues = build_issues(&issues_json);
    Ok(FullRepo
    {
        name: repo.name,
        owner: repo.owner,
        html_url: repo.html_url,
        forks_count: repo.forks_count,
        language: repo.language,
        open_issues_count: repo.open_issues_count,
        forks_url: repo.forks_url, // For now, get the urls. We will make a list later.
        commits_url: commits,
        commit_count: 0,
        issues,
    })
}