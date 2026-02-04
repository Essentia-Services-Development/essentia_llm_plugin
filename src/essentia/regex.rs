use std::collections::HashSet;

#[derive(Debug)]
pub struct Regex {
    pattern: String,
}

impl Regex {
    pub fn new(pattern: &str) -> Result<Self, String> {
        Ok(Regex { pattern: pattern.to_string() })
    }

    pub fn is_match(&self, text: &str) -> bool {
        self.find(text).is_some()
    }

    pub fn find(&self, text: &str) -> Option<(usize, usize)> {
        for i in 0..text.len() {
            if let Some(len) = self.match_at(&text[i..]) {
                return Some((i, i + len));
            }
        }
        None
    }

    pub fn find_iter<'a>(&'a self, text: &'a str) -> FindIter<'a, 'a> {
        FindIter { regex: self, text, position: 0 }
    }

    fn match_at(&self, text: &str) -> Option<usize> {
        self.match_pattern(&self.pattern, text, 0, 0)
    }

    fn match_pattern(
        &self, pattern: &str, text: &str, pat_idx: usize, text_idx: usize,
    ) -> Option<usize> {
        if pat_idx == pattern.len() {
            return Some(text_idx);
        }

        let pat_chars: Vec<char> = pattern.chars().collect();
        let text_chars: Vec<char> = text.chars().collect();

        if pat_idx >= pat_chars.len() {
            return Some(text_idx);
        }

        match pat_chars[pat_idx] {
            '.' => {
                if text_idx < text_chars.len() {
                    self.match_pattern(pattern, text, pat_idx + 1, text_idx + 1)
                } else {
                    None
                }
            },
            '*' => {
                // Handle * (zero or more)
                if pat_idx == 0 {
                    return None; // Invalid pattern
                }
                let prev = pat_chars[pat_idx - 1];

                // Try zero occurrences
                if let Some(len) = self.match_pattern(pattern, text, pat_idx + 1, text_idx) {
                    return Some(len);
                }

                // Try one or more occurrences
                let mut current_idx = text_idx;
                while current_idx < text_chars.len()
                    && self.matches_char(prev, text_chars[current_idx])
                {
                    if let Some(len) =
                        self.match_pattern(pattern, text, pat_idx + 1, current_idx + 1)
                    {
                        return Some(len);
                    }
                    current_idx += 1;
                }
                None
            },
            '+' => {
                // Handle + (one or more)
                if pat_idx == 0 {
                    return None; // Invalid pattern
                }
                let prev = pat_chars[pat_idx - 1];
                if text_idx >= text_chars.len() || !self.matches_char(prev, text_chars[text_idx]) {
                    return None;
                }

                let mut current_idx = text_idx;
                while current_idx < text_chars.len()
                    && self.matches_char(prev, text_chars[current_idx])
                {
                    if let Some(len) =
                        self.match_pattern(pattern, text, pat_idx + 1, current_idx + 1)
                    {
                        return Some(len);
                    }
                    current_idx += 1;
                }
                None
            },
            '?' => {
                // Handle ? (zero or one)
                if pat_idx == 0 {
                    return None; // Invalid pattern
                }
                let prev = pat_chars[pat_idx - 1];

                // Try zero occurrences
                if let Some(len) = self.match_pattern(pattern, text, pat_idx + 1, text_idx) {
                    return Some(len);
                }

                // Try one occurrence
                if text_idx < text_chars.len() && self.matches_char(prev, text_chars[text_idx]) {
                    self.match_pattern(pattern, text, pat_idx + 1, text_idx + 1)
                } else {
                    None
                }
            },
            '[' => {
                // Handle character classes
                if let Some(end_idx) = pattern[pat_idx..].find(']') {
                    let class = &pattern[pat_idx + 1..pat_idx + end_idx];
                    let negate = class.starts_with('^');
                    let chars: HashSet<char> = if negate {
                        class[1..].chars().collect()
                    } else {
                        class.chars().collect()
                    };

                    if text_idx >= text_chars.len() {
                        return None;
                    }

                    let matches = if negate {
                        !chars.contains(&text_chars[text_idx])
                    } else {
                        chars.contains(&text_chars[text_idx])
                    };

                    if matches {
                        self.match_pattern(pattern, text, pat_idx + end_idx + 1, text_idx + 1)
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            '^' => {
                // Start of string
                if text_idx == 0 {
                    self.match_pattern(pattern, text, pat_idx + 1, text_idx)
                } else {
                    None
                }
            },
            '$' => {
                // End of string
                if text_idx == text_chars.len() {
                    Some(text_idx)
                } else {
                    None
                }
            },
            '\\' => {
                // Escape sequences
                if pat_idx + 1 < pat_chars.len() {
                    let escaped = pat_chars[pat_idx + 1];
                    if text_idx < text_chars.len()
                        && self.matches_escaped(escaped, text_chars[text_idx])
                    {
                        self.match_pattern(pattern, text, pat_idx + 2, text_idx + 1)
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            c => {
                if text_idx < text_chars.len() && self.matches_char(c, text_chars[text_idx]) {
                    self.match_pattern(pattern, text, pat_idx + 1, text_idx + 1)
                } else {
                    None
                }
            },
        }
    }

    fn matches_char(&self, pattern_char: char, text_char: char) -> bool {
        pattern_char == text_char
    }

    fn matches_escaped(&self, escaped: char, text_char: char) -> bool {
        match escaped {
            'd' => text_char.is_ascii_digit(),
            'w' => text_char.is_ascii_alphanumeric() || text_char == '_',
            's' => text_char.is_whitespace(),
            'D' => !text_char.is_ascii_digit(),
            'W' => !(text_char.is_ascii_alphanumeric() || text_char == '_'),
            'S' => !text_char.is_whitespace(),
            _ => escaped == text_char,
        }
    }
}

pub struct FindIter<'r, 't> {
    regex:    &'r Regex,
    text:     &'t str,
    position: usize,
}

impl<'r, 't> Iterator for FindIter<'r, 't> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.text.len() {
            return None;
        }

        if let Some((start, end)) = self.regex.find(&self.text[self.position..]) {
            let actual_start = self.position + start;
            let actual_end = self.position + end;
            self.position = actual_end;
            Some((actual_start, actual_end))
        } else {
            None
        }
    }
}

pub fn search(pattern: &str, text: &str) -> Option<String> {
    let regex = Regex::new(pattern).ok()?;
    regex.find(text).map(|(start, end)| text[start..end].to_string())
}
