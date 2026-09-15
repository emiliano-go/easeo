pub fn build_description_snippet(body_html: Option<&str>, max_len: usize) -> Option<String> {
    let html = body_html?;
    let text = html_to_text(html);
    if text.is_empty() {
        return None;
    }
    let char_count = text.chars().count();
    if char_count <= max_len {
        Some(text)
    } else {
        let truncated: String = text.chars().take(max_len.saturating_sub(3)).collect();
        Some(format!("{}...", truncated))
    }
}

pub fn html_to_text(html: &str) -> String {
    let mut text = String::new();
    let mut chars = html.chars().peekable();
    let mut skip_depth: Option<String> = None;

    while let Some(ch) = chars.next() {
        if ch == '<' {
            // Collect the full tag content (without the leading '<')
            let mut tag = String::new();
            while let Some(&next) = chars.peek() {
                if next == '>' {
                    chars.next(); // consume '>'
                    break;
                }
                tag.push(next);
                chars.next();
            }

            let tag_lower = tag.to_lowercase();

            if let Some(ref skip_tag) = skip_depth {
                // Check if this closes the tag we're skipping
                // Closing tag format: "/script" (without '<' and '>')
                if tag_lower == format!("/{}", skip_tag) {
                    skip_depth = None;
                }
            } else {
                // Check if this opens a script/style/noscript tag
                // Opening tag format: "script" or "script ..." (without '<')
                if tag_lower == "script" || tag_lower == "style" || tag_lower == "noscript" {
                    skip_depth = Some(tag_lower.clone());
                } else {
                    text.push(' ');
                }
            }
        } else if skip_depth.is_some() {
            // Inside script/style/noscript — skip content
        } else {
            text.push(ch);
        }
    }

    // Collapse whitespace
    let mut result = String::new();
    let mut prev_space = false;
    for ch in text.chars() {
        if ch.is_whitespace() {
            if !prev_space {
                result.push(' ');
                prev_space = true;
            }
        } else {
            result.push(ch);
            prev_space = false;
        }
    }

    result.trim().to_string()
}
