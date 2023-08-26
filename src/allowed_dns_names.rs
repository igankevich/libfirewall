use std::fmt::Write;

use regex::Regex;

use crate::log;

pub(crate) struct AllowedDnsNames {
    patterns: Vec<Regex>,
}

impl AllowedDnsNames {
    pub(crate) fn new() -> Self {
        let mut patterns: Vec<Regex> = Vec::new();
        if let Ok(string) = std::env::var("LIBFIREWALL_ALLOW") {
            for pattern in string.split(' ') {
                match glob_to_regex(pattern.trim()) {
                    Ok(regex) => patterns.push(regex),
                    Err(e) => {
                        log!("failed to parse glob pattern `{}`: {}", pattern, e);
                    }
                }
            }
        }
        Self { patterns }
    }

    pub(crate) fn contain(&self, name: &str) -> bool {
        for pattern in self.patterns.iter() {
            if pattern.is_match(name) {
                return true;
            }
        }
        false
    }
}

/// Convert glob pattern to regular expression.
fn glob_to_regex(glob: &str) -> Result<Regex, regex::Error> {
    let mut regex = String::with_capacity(glob.len() * 6);
    regex.push('^');
    for ch in glob.chars() {
        match ch {
            '*' => regex.push_str(".*"),
            '?' => regex.push('.'),
            ch => write!(regex, "\\u{{{:x}}}", ch as u32).unwrap(),
        }
    }
    regex.push('$');
    Regex::new(regex.as_str())
}
