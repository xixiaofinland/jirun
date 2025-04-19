pub fn truncate_with_ellipsis(text: &str, max_chars: usize) -> String {
    let mut chars = text.chars();
    let truncated: String = chars.by_ref().take(max_chars).collect();

    if chars.next().is_some() {
        format!("{}...", truncated)
    } else {
        truncated
    }
}

pub fn bold_yellow(text: &str) -> String {
    format!("\x1b[1;33m{}\x1b[0m", text)
}

pub fn bold_cyan(text: &str) -> String {
    format!("\x1b[1;36m{}\x1b[0m", text)
}

pub fn bold_white(text: &str) -> String {
    format!("\x1b[1;97m{}\x1b[0m", text)
}
