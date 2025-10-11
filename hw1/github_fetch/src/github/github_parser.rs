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
}

