use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CookieJar {
    cookies: HashMap<String, String>,
}

impl CookieJar {
    pub fn new() -> Self {
        Self { cookies: HashMap::new() }
    }

    pub fn set(&mut self, name: &str, value: &str) {
        self.cookies.insert(name.to_string(), value.to_string());
    }

    pub fn get(&self, name: &str) -> Option<&String> {
        self.cookies.get(name)
    }

    pub fn get_dict(&self) -> HashMap<String, String> {
        self.cookies.clone()
    }

    pub fn update(&mut self, other: &HashMap<String, String>) {
        for (k, v) in other {
            self.cookies.insert(k.clone(), v.clone());
        }
    }
}

impl Default for CookieJar {
    fn default() -> Self {
        Self::new()
    }
}
