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
    let mut in_tag = false;

    for ch in html.chars() {
        match ch {
            '<' => {
                in_tag = true;
            }
            '>' => {
                in_tag = false;
                text.push(' ');
            }
            _ if !in_tag => {
                text.push(ch);
            }
            _ => {}
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
