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
                return Some(after_str[..end_idx].to_string());
            }
        }
    }
    None
}