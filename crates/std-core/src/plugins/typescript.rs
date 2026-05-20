pub(crate) fn strip_typescript_source(source: &str) -> String {
    let mut output = String::new();
    let mut skipping_type_block = false;
    let mut type_block_depth = 0_i32;
    for line in source.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("type ") || trimmed.starts_with("interface ") {
            let depth = brace_delta(line);
            if depth > 0 {
                skipping_type_block = true;
                type_block_depth = depth;
            }
            continue;
        }
        if skipping_type_block {
            type_block_depth += brace_delta(line);
            if type_block_depth <= 0 {
                skipping_type_block = false;
                type_block_depth = 0;
            }
            continue;
        }
        output.push_str(&strip_typescript_line(line));
        output.push('\n');
    }
    output
}

fn brace_delta(line: &str) -> i32 {
    let mut delta = 0_i32;
    let mut quote: Option<char> = None;
    let chars = line.chars().collect::<Vec<_>>();
    for (index, ch) in chars.iter().enumerate() {
        if let Some(active_quote) = quote {
            if *ch == active_quote && !is_escaped(&chars, index) {
                quote = None;
            }
            continue;
        }
        if *ch == '"' || *ch == '\'' || *ch == '`' {
            quote = Some(*ch);
            continue;
        }
        if *ch == '{' {
            delta += 1;
        } else if *ch == '}' {
            delta -= 1;
        }
    }
    delta
}

fn strip_typescript_line(line: &str) -> String {
    let without_assertions = strip_type_assertions(line);
    strip_type_annotations(&without_assertions)
}

fn strip_type_assertions(line: &str) -> String {
    let mut output = String::new();
    let chars = line.chars().collect::<Vec<_>>();
    let mut i = 0;
    let mut quote: Option<char> = None;
    while i < chars.len() {
        let ch = chars[i];
        if let Some(active_quote) = quote {
            output.push(ch);
            if ch == active_quote && !is_escaped(&chars, i) {
                quote = None;
            }
            i += 1;
            continue;
        }
        if ch == '"' || ch == '\'' || ch == '`' {
            quote = Some(ch);
            output.push(ch);
            i += 1;
            continue;
        }
        if ch == ' ' && starts_word_at(&chars, i + 1, "as") {
            i = skip_type_assertion(&chars, i + 3);
            continue;
        }
        output.push(ch);
        i += 1;
    }
    output
}

fn skip_type_assertion(chars: &[char], mut index: usize) -> usize {
    while index < chars.len() && chars[index].is_whitespace() {
        index += 1;
    }
    while index < chars.len() && is_type_token_char(chars[index]) {
        index += 1;
    }
    index
}

fn strip_type_annotations(line: &str) -> String {
    let mut output = String::new();
    let chars = line.chars().collect::<Vec<_>>();
    let mut i = 0;
    let mut quote: Option<char> = None;
    while i < chars.len() {
        let ch = chars[i];
        if let Some(active_quote) = quote {
            output.push(ch);
            if ch == active_quote && !is_escaped(&chars, i) {
                quote = None;
            }
            i += 1;
            continue;
        }
        if ch == '"' || ch == '\'' || ch == '`' {
            quote = Some(ch);
            output.push(ch);
            i += 1;
            continue;
        }
        if ch == ':' && should_strip_type_annotation(&chars, i) {
            i = skip_type_annotation(&chars, i + 1);
            continue;
        }
        output.push(ch);
        i += 1;
    }
    output
}

fn skip_type_annotation(chars: &[char], mut index: usize) -> usize {
    while index < chars.len() && chars[index].is_whitespace() {
        index += 1;
    }
    while index < chars.len() && is_type_token_char(chars[index]) {
        index += 1;
    }
    index
}

fn should_strip_type_annotation(chars: &[char], colon_index: usize) -> bool {
    let mut left = colon_index;
    while left > 0 && chars[left - 1].is_whitespace() {
        left -= 1;
    }
    if left == 0 || !is_identifier_char(chars[left - 1]) {
        return false;
    }
    let mut right = colon_index + 1;
    while right < chars.len() && chars[right].is_whitespace() {
        right += 1;
    }
    if right >= chars.len() || !is_type_start_char(chars[right]) {
        return false;
    }
    let mut end = right;
    while end < chars.len() && is_type_token_char(chars[end]) {
        end += 1;
    }
    let mut next = end;
    while next < chars.len() && chars[next].is_whitespace() {
        next += 1;
    }
    matches!(
        chars.get(next),
        Some('=' | ',' | ')' | ';' | '{' | '[') | None
    )
}

fn starts_word_at(chars: &[char], index: usize, word: &str) -> bool {
    let word_chars = word.chars().collect::<Vec<_>>();
    if index + word_chars.len() > chars.len() {
        return false;
    }
    for (offset, expected) in word_chars.iter().enumerate() {
        if chars[index + offset] != *expected {
            return false;
        }
    }
    let before_ok = index == 0 || !is_identifier_char(chars[index - 1]);
    let after = index + word_chars.len();
    let after_ok = after >= chars.len() || !is_identifier_char(chars[after]);
    before_ok && after_ok
}

fn is_escaped(chars: &[char], index: usize) -> bool {
    let mut count = 0;
    let mut i = index;
    while i > 0 && chars[i - 1] == '\\' {
        count += 1;
        i -= 1;
    }
    count % 2 == 1
}

fn is_identifier_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || ch == '$'
}

fn is_type_start_char(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_' || ch == '{' || ch == '['
}

fn is_type_token_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric()
        || matches!(
            ch,
            '_' | '$' | '[' | ']' | '<' | '>' | '{' | '}' | '|' | '&' | '?' | '.' | ','
        )
}
