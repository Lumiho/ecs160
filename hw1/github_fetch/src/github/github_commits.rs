use crate::github::github_models::{TempRepo};
use crate::github::github_parser::{get_values, parse_items};
use reqwest::Method;

#[derive(Debug, Clone)]
pub struct Commit
{
    pub sha: String,
    pub message: String,
    pub name: String,
    pub email: String,
    pub date: String,
}

pub fn build_commits(json: &str) -> Vec<Commit>{
    let items = parse_items(&json);

    let mut commits: Vec<Commit> = Vec::new();

    for item in items{
        let commit_sha = get_values(item, "sha");
        let commit_message = get_values(item, "message");
        let commit_name = get_values(item, "name");
        let commit_email = get_values(item, "email");
        let commit_date = get_values(item, "date");

        let commit_info = Commit {
            sha: commit_sha.clone().unwrap(),
            message: commit_message.clone().unwrap(),
            name: commit_message.clone().unwrap(),
            email: commit_message.clone().unwrap(),
            date: commit_message.clone().unwrap(),
        };
        commits.push(commit_info);
    }
    commits
}