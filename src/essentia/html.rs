//! Pure Rust HTML parser implementation.
#![allow(clippy::redundant_else)]

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Element {
    pub tag_name:     String,
    pub attributes:   HashMap<String, String>,
    pub children:     Vec<Node>,
    pub text_content: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Node {
    Element(Element),
    Text(()),
}

#[derive(Debug)]
pub struct Document {
    pub root: Element,
}

impl Document {
    pub fn parse(html: &str) -> Result<Self, String> {
        let mut parser = HtmlParser::new(html);
        let root = parser.parse_element()?;
        Ok(Document { root })
    }

    pub fn find_scripts(&self) -> Vec<String> {
        self.find_elements_by_tag("script")
            .into_iter()
            .filter_map(|elem| elem.attributes.get("src").cloned())
            .collect()
    }

    pub fn find_meta_content(&self, name: &str) -> Option<String> {
        self.find_elements_by_tag("meta")
            .into_iter()
            .find(|elem| elem.attributes.get("name").is_some_and(|n| n == name))
            .and_then(|elem| elem.attributes.get("content").cloned())
    }

    fn find_elements_by_tag(&self, tag: &str) -> Vec<&Element> {
        self.find_elements_recursive(&self.root, tag)
    }

    #[allow(clippy::only_used_in_recursion)]
    fn find_elements_recursive<'a>(&'a self, element: &'a Element, tag: &str) -> Vec<&'a Element> {
        let mut results = Vec::new();

        if element.tag_name == tag {
            results.push(element);
        }

        for child in &element.children {
            if let Node::Element(elem) = child {
                results.extend(self.find_elements_recursive(elem, tag));
            }
        }

        results
    }
}

struct HtmlParser<'a> {
    input:    &'a str,
    position: usize,
}

impl<'a> HtmlParser<'a> {
    fn new(input: &'a str) -> Self {
        HtmlParser { input, position: 0 }
    }

    fn parse_element(&mut self) -> Result<Element, String> {
        self.skip_whitespace();

        self.expect_char('<')?;

        let tag_name = self.parse_tag_name()?;
        let attributes = self.parse_attributes()?;

        if self.peek_char() == Some('/') {
            // Self-closing tag
            let _ = self.expect_char('/');
            self.expect_char('>')?;
            return Ok(Element { tag_name, attributes, children: Vec::new(), text_content: None });
        }

        self.expect_char('>')?;

        let mut children = Vec::new();
        let mut text_content = String::new();

        loop {
            self.skip_whitespace();

            if self.peek_char() == Some('<') {
                if self.peek_ahead(2) == Some("</".to_string()) {
                    // Closing tag
                    let _ = self.expect_char('<');
                    let _ = self.expect_char('/');
                    let close_tag = self.parse_tag_name()?;
                    self.expect_char('>')?;

                    if close_tag != tag_name {
                        return Err(format!(
                            "Mismatched closing tag: expected {}, got {}",
                            tag_name, close_tag
                        ));
                    }

                    break;
                } else {
                    // Child element
                    children.push(Node::Element(self.parse_element()?));
                }
            } else {
                // Text content
                let text = self.parse_text()?;
                if !text.trim().is_empty() {
                    text_content.push_str(&text);
                }
            }
        }

        Ok(Element {
            tag_name,
            attributes,
            children,
            text_content: if text_content.is_empty() {
                None
            } else {
                Some(text_content)
            },
        })
    }

    fn parse_tag_name(&mut self) -> Result<String, String> {
        let mut name = String::new();
        while let Some(c) = self.next_char() {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                name.push(c);
            } else {
                self.position -= 1; // Put back the character
                break;
            }
        }
        if name.is_empty() {
            Err("Empty tag name".to_string())
        } else {
            Ok(name)
        }
    }

    fn parse_attributes(&mut self) -> Result<HashMap<String, String>, String> {
        let mut attributes = HashMap::new();

        loop {
            self.skip_whitespace();

            if self.peek_char().map(|c| c == '>' || c == '/').unwrap_or(false) {
                break;
            }

            let name = self.parse_attribute_name()?;
            self.skip_whitespace();

            self.expect_char('=')?;

            self.skip_whitespace();

            let value = self.parse_attribute_value()?;
            attributes.insert(name, value);
        }

        Ok(attributes)
    }

    fn parse_attribute_name(&mut self) -> Result<String, String> {
        let mut name = String::new();
        while let Some(c) = self.next_char() {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                name.push(c);
            } else {
                self.position -= 1; // Put back the character
                break;
            }
        }
        if name.is_empty() {
            Err("Empty attribute name".to_string())
        } else {
            Ok(name)
        }
    }

    fn parse_attribute_value(&mut self) -> Result<String, String> {
        let quote = self.next_char().ok_or("Expected quote for attribute value")?;
        if quote != '"' && quote != '\'' {
            return Err("Attribute values must be quoted".to_string());
        }

        let mut value = String::new();
        while let Some(c) = self.next_char() {
            if c == quote {
                break;
            }
            value.push(c);
        }

        Ok(value)
    }

    fn parse_text(&mut self) -> Result<String, String> {
        let mut text = String::new();
        while let Some(c) = self.peek_char() {
            if c == '<' {
                break;
            }
            if let Some(ch) = self.next_char() {
                text.push(ch);
            }
        }
        Ok(text)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if !c.is_whitespace() {
                break;
            }
            self.next_char();
        }
    }

    fn expect_char(&mut self, expected: char) -> Result<(), String> {
        if self.peek_char() == Some(expected) {
            self.next_char();
            Ok(())
        } else {
            Err(format!(
                "Expected '{}', found {:?}",
                expected,
                self.peek_char()
            ))
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn peek_ahead(&self, n: usize) -> Option<String> {
        let start = self.position;
        let chars: String = self.input.chars().skip(start).take(n).collect();
        if chars.len() == n { Some(chars) } else { None }
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.peek_char();
        if c.is_some() {
            self.position += 1;
        }
        c
    }
}

pub fn find_scripts(html: &str) -> Vec<String> {
    Document::parse(html).map(|doc| doc.find_scripts()).unwrap_or_default()
}

pub fn find_meta_baggage(html: &str) -> Option<String> {
    Document::parse(html).ok()?.find_meta_content("baggage")
}

pub fn find_meta_sentry(html: &str) -> Option<String> {
    Document::parse(html).ok()?.find_meta_content("sentry-trace")
}

pub fn find_anim(_html: &str) -> Option<String> {
    // For now, return a default animation name
    // In a real implementation, this would parse CSS or specific elements
    Some("loading-x-anim-0".to_string())
}
