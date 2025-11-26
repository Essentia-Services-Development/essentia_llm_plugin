use std::{collections::HashMap, fmt, str::FromStr};

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl FromStr for Value {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_value(s.trim())
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "\"{}\"", s.replace("\"", "\\\"")),
            Value::Array(a) => {
                write!(f, "[")?;
                for (i, v) in a.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            },
            Value::Object(o) => {
                write!(f, "{{")?;
                for (i, (k, v)) in o.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "\"{}\":{}", k, v)?;
                }
                write!(f, "}}")
            },
        }
    }
}

fn parse_value(s: &str) -> Result<Value, &'static str> {
    if let Some(stripped) = s.strip_prefix('"') {
        let end = stripped.find('"').ok_or("Unclosed string")? + 1;
        let content = &stripped[..end - 1];
        Ok(Value::String(content.to_string()))
    } else if s == "null" {
        Ok(Value::Null)
    } else if s == "true" {
        Ok(Value::Bool(true))
    } else if s == "false" {
        Ok(Value::Bool(false))
    } else if s.starts_with('{') {
        parse_object(s)
    } else if s.starts_with('[') {
        parse_array(s)
    } else {
        s.parse::<f64>().map(Value::Number).map_err(|_| "Invalid number")
    }
}

fn parse_object(s: &str) -> Result<Value, &'static str> {
    let mut map = HashMap::new();
    let inner = &s[1..s.len() - 1];
    let mut parts = Vec::new();
    let mut start = 0;
    let mut brace_count = 0;
    for (i, c) in inner.char_indices() {
        match c {
            '{' | '[' => brace_count += 1,
            '}' | ']' => brace_count -= 1,
            ',' if brace_count == 0 => {
                parts.push(&inner[start..i]);
                start = i + 1;
            },
            _ => {},
        }
    }
    if start < inner.len() {
        parts.push(&inner[start..]);
    }
    for part in parts {
        let colon = part.find(':').ok_or("Invalid object")?;
        let key = parse_value(&part[..colon])?;
        let Value::String(k) = key else {
            return Err("Key not string");
        };
        let value = parse_value(&part[colon + 1..])?;
        map.insert(k, value);
    }
    Ok(Value::Object(map))
}

fn parse_array(s: &str) -> Result<Value, &'static str> {
    let mut arr = Vec::new();
    let inner = &s[1..s.len() - 1];
    let mut parts = Vec::new();
    let mut start = 0;
    let mut brace_count = 0;
    for (i, c) in inner.char_indices() {
        match c {
            '{' | '[' => brace_count += 1,
            '}' | ']' => brace_count -= 1,
            ',' if brace_count == 0 => {
                parts.push(&inner[start..i]);
                start = i + 1;
            },
            _ => {},
        }
    }
    if start < inner.len() {
        parts.push(&inner[start..]);
    }
    for part in parts {
        arr.push(parse_value(part)?);
    }
    Ok(Value::Array(arr))
}

pub fn parse(json_str: &str) -> Result<Value, &'static str> {
    json_str.parse()
}

impl Value {
    pub fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Object(obj) => obj.get(key),
            _ => None,
        }
    }

    pub fn get_index(&self, index: usize) -> Option<&Value> {
        match self {
            Value::Array(arr) => arr.get(index),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
}

pub fn to_json_string(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(b) => {
            if *b {
                "true".to_string()
            } else {
                "false".to_string()
            }
        },
        Value::Number(n) => n.to_string(),
        Value::String(s) => format!(
            "\"{}\"",
            s.replace("\"", "\\\"")
                .replace("\n", "\\n")
                .replace("\r", "\\r")
                .replace("\t", "\\t")
        ),
        Value::Array(a) => {
            let items: Vec<String> = a.iter().map(to_json_string).collect();
            format!("[{}]", items.join(","))
        },
        Value::Object(o) => {
            let items: Vec<String> =
                o.iter().map(|(k, v)| format!("\"{}\":{}", k, to_json_string(v))).collect();
            format!("{{{}}}", items.join(","))
        },
    }
}
