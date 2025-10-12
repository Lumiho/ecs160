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
    if let Some(start_idx) = json.find(&pattern)
    {
        if let Some(open_brace_idx) = json[start_idx..].find("{")
        {
            if let Some(closing_brace_idx) = json[start_idx + open_brace_idx + 1..].find(("}"))
                {
                    let start = start_idx + open_brace_idx + 1;
                    let end = start_idx + open_brace_idx + closing_brace_idx + 1;
                    return Some(&json[start..end]);
                }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let name = get_values(json, "name");
        assert_eq!(name, Some("rustlings".to_string()));

        let full_name = get_values(json, "full_name");
        assert_eq!(full_name, Some("rust-lang/rustlings".to_string()));

        let language = get_values(json, "language");
        assert_eq!(language, Some("Rust".to_string()));

        let forks = get_values(json, "forks_count");
        assert_eq!(forks, Some("10944".to_string()));

        let archived = get_values(json, "archived");
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

        let owner_block = get_nested_block(json, "owner").unwrap();

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
}


