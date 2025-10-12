use crate::github::github_models::{TempRepo, Owner};
pub fn build_temp_repo(json: &str) -> Vec<TempRepo> {
    // array of the json items
    let items: Vec<&str> = parse_items(&json);

    // println!("Begin Debug Print =================================================================================");
    // for (i, item) in items.iter().enumerate() {
    //     println!("Repo: {}, {:#?}", i, item);
    // }
    // println!("End Debug Print =================================================================================");

    // array of TempRepos to build
    let mut temp_repos: Vec<TempRepo> = Vec::new();

    for item in items {
        let repo_name = get_values(item, "name");
        let html_url = get_values(item, "html_url");
        let forks_count = get_values(item, "forks_count");
        let language = get_values(item, "language");
        let open_issues_count = get_values(item, "open_issues_count");
        let forks_url = get_values(item, "forks_url");
        let commits_url = get_values(item, "commits_url");
        let issues_url = get_values(item, "issues_url");

        // build the owner
        let owner = get_nested_block(&item, "owner");

        if owner.is_none() {
            eprintln!("Missing owner block for item:\n{}", item);
        }

        let login = get_values(owner.unwrap_or(""), "login");
        let id = get_values(owner.unwrap_or(""), "id");
        let owner_url = get_values(owner.unwrap_or(""), "html_url");


        let site_admin = get_values(&owner.unwrap_or(""), "site_admin");
        if site_admin.is_none() {
            eprintln!("Missing site_admin block for item:\n{:?}", owner);
        }

        let built_owner = Owner {
            login: login.unwrap(),
            id: id.unwrap().parse::<u64>().unwrap(),
            html_url: owner_url.unwrap(),
            site_admin: site_admin.unwrap().parse::<bool>().unwrap(),
        };

        let built_temp_repo = TempRepo {
            name: repo_name.unwrap(),
            owner: built_owner,
            html_url: html_url.unwrap(),
            forks_count: forks_count.unwrap().parse::<u32>().unwrap(),
            language: language.unwrap(),
            open_issues_count: open_issues_count.unwrap().parse::<u32>().unwrap(),
            forks_url: forks_url.unwrap(),
            commits_url: commits_url.unwrap(),
            issues_url: issues_url.unwrap(),
        };

        temp_repos.push(built_temp_repo);
    }

    temp_repos
}


// These functions only work for the /search/repositories endpoint to meet the assignment specifications.
// This function returns the list of items in the items array at the /search/repositories endpoint
fn parse_items(json: &str) -> Vec<&str> {
    let mut items: Vec<&str> = Vec::new();
    let mut start_idx: usize = 0;
    let mut depth: u16 = 0;
    // flag to track whether the parser is in a string or not, so we don't count '{' '}' within strings.
    let mut within_string = false;


    // Let's go past the '[' character, and on to the first element
    if let Some(array_start_idx) = json.find("[") {
        let mut within_array = true;
        // keep in mind, each byte i is an offset of the position of array_start_idx
        // when building item:
        // add array_start_idx to i to make up for the offset and find the global position of the braces
        for (i, byte) in json[array_start_idx..].bytes().enumerate() {
            if byte == b'"' {
                within_string = !within_string;
            }

            if byte == b'[' && !within_string {
                within_array = true;
            }

            if byte == b']' && !within_string && depth == 0 {
                within_array = false;
                break;
            }

            if byte == b'{' && !within_string {
                if depth == 0 {
                    start_idx = array_start_idx + i;
                }
                depth += 1;
            }

            if byte == b'}' && !within_string  {
                if depth > 0 {
                    depth -= 1;
                }

                if depth == 0 {
                    let end_idx = array_start_idx + i;
                    let item = json[start_idx..=end_idx].trim_matches(|c: char| c == ',' || c.is_whitespace());
                    items.push(item);
                }

            }
        }
    }
    items
}

fn get_values(json: &str, key: &str) -> Option<String> {
    let pattern = format!("\"{}\":", key);
    if let Some(start_index) = json.find(&pattern)
    {
        let after_str = &json[start_index + pattern.len()..].trim_start();

        if after_str.starts_with('"')
        {
            let after_quote = &after_str[1..];
            if let Some(end_idx) = after_quote.find('"')
            {
                return Some(after_quote[..end_idx].to_string());
            }
        }
        else
        {
            // value not a string, could be bool, null, uint. json delimiters: , } ]
            if let Some(end_idx) = after_str.find(|c| c == ']'|| c =='}'|| c == ',') // equivalent to "for c in after_str, find these patterns"
            {
                return Some(after_str[..end_idx].trim().to_string());
            }
        }
    }
    None
}


fn get_nested_block<'a>(json: &'a str, key: &str) -> Option<&'a str> {
    let pattern = format!("\"{}\":", key);

    if let Some(key_idx) = json.find(&pattern)
    {
        let mut within_string = false;
        let mut depth = 0;
        let mut start_idx: usize = 0;
        let mut end_idx: usize = 0;

        for (i , byte) in json[key_idx..].bytes().enumerate() {
            if byte == b'"' {
                within_string = !within_string;
            }

            if byte == b'{' && !within_string {
                if depth == 0 {
                    start_idx = key_idx + i;
                }
                depth += 1;
            }

            if byte == b'}' && !within_string {
                if depth > 0 {
                    depth -= 1;
                }

                if depth == 0 {
                    end_idx = key_idx + i;
                    return Some(json[start_idx..=end_idx].trim());
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{path::PathBuf, env, fs};

    #[test]
    fn test_get_values() {
        let json = r#"
        {
            "name": "rustlings",
            "full_name": "rust-lang/rustlings",
            "language": "Rust",
            "forks_count": 10944,
            "archived": false
        }
        "#;

        let name = get_values(&json.to_string(), "name");
        assert_eq!(name, Some("rustlings".to_string()));

        let full_name = get_values(&json.to_string(), "full_name");
        assert_eq!(full_name, Some("rust-lang/rustlings".to_string()));

        let language = get_values(&json.to_string(), "language");
        assert_eq!(language, Some("Rust".to_string()));

        let forks = get_values(&json.to_string(), "forks_count");
        assert_eq!(forks, Some("10944".to_string()));

        let archived = get_values(&json.to_string(), "archived");
        assert_eq!(archived, Some("false".to_string()));

        println!("Extracted values:");
        println!("name = {:?}", name);
        println!("full_name = {:?}", full_name);
        println!("language = {:?}", language);
        println!("forks_count = {:?}", forks);
        println!("archived = {:?}", archived);
    }

    #[test]
    fn test_get_nested_block() {
        let json = r#"{
        "name": "linux",
        "owner": {
            "login": "torvalds",
            "id": 1024025,
            "html_url": "https://github.com/torvalds",
            "site_admin": false
        }
    }"#;

        let owner_block = get_nested_block(&json, "owner").unwrap();

        let expected = r#""login": "torvalds",
                            "id": 1024025,
                            "html_url": "https://github.com/torvalds",
                            "site_admin": false"#;

        let clean = |s: &str| s.chars().filter(|c| !c.is_whitespace()).collect::<String>();

        // format both strings for consistent formatting (just for debug output). not the neatest to read, but it works.
        let actual_clean = clean(owner_block);
        let expected_clean = clean(expected);

        println!("--- Extracted ---\n{}\n", actual_clean);
        println!("--- Expected ---\n{}\n", expected_clean);

        // println!("--- Extracted ---\n{}\n", owner_block);
        // println!("--- Expected ---\n{}\n", expected);

        if actual_clean != expected_clean {
            eprintln!("Owner block mismatch:\n");
            panic!("Owner block did not match expected output");
        }
    }

    #[test]
    fn test_parse_items_output() {
        // CARGO_MANIFEST_DIR tells cargo to build file path from root directory (directory that contains Cargo.toml)
        // build a path that works on every OS
        let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        path.push("src");
        path.push("sample_data");
        path.push("rust.json");
        //println!("{:?}", path);

        assert!(path.exists(), "File not found at {:?}", path);

        let json_data = fs::read_to_string(&path).expect("Unable to read file");
        let items = parse_items(&json_data);

        for (i, item) in items.iter().enumerate() {
            println!("Item {}: {}", i, item);
        }
    }
}


