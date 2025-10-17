use crate::github::github_issues::Issue;
use crate::github::github_commits::Commit;
use crate::github::github_forks::ForkRepo;

#[derive(Debug)]
pub struct TopLevelApiCall
{
    items: Vec<TempRepo>
}

// We need a separate api call to obtain the commit count
// We'll store what we can from the /search/repositories endpoint here
#[derive(Debug)]
pub struct TempRepo
{
    pub name: String,
    pub owner: Owner,
    pub html_url: String,
    pub forks_count: u32,
    pub language: String,
    pub open_issues_count: u32,
    pub forks_url: String, // Get the url for now, vectors later
    pub commits_url: String,
    pub issues_url: String,
    pub stargazer_count: u32, // added stargazer cnt
}

// We will construct this FullRepo from TempRepo and an api call to get the commit count
#[derive(Debug)]
pub struct FullRepo
{
    pub name: String,
    pub owner: Owner,
    pub html_url: String,
    pub forks_count: u32,
    pub language: String,
    pub open_issues_count: u32,
    pub forks_list: Vec<ForkRepo>, 
    pub commits_url: Vec<Commit>,
    pub issues: Vec<Issue>,
    pub commit_count: u32,
    pub stargazer_count: u32,
}

#[derive(Debug)]
pub struct Fork
{
    full_name: String,
    html_url: String, // of the fork
    owner: Owner
}

// #[derive(Debug)]
// pub struct Issue
// {
//     title: String,
//     body: Option<String>,
//     state: String,
//     createdAt: String,
//     updatedAt: String
// }

#[derive(Debug)]
pub struct Author
{
    name: String,
    email: String,
    date: String
}

#[derive(Debug)]
pub struct Owner
{
    pub login: String,
    pub id: u64,
    pub html_url: String,
    pub site_admin: bool
}