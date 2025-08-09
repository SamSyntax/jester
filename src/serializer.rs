use parser_derive::FromJsonVal;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum JsonVal {
    Null,
    Bool(bool),
    Number(usize),
    Float(f64),
    String(String),
    Array(Vec<JsonVal>),
    Object(std::collections::HashMap<String, JsonVal>),
}

impl fmt::Display for JsonVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonVal::Null => write!(f, "null"),
            JsonVal::Bool(b) => write!(f, "{}", b),
            JsonVal::Number(n) => write!(f, "{}", n),
            JsonVal::Float(fl) => write!(f, "{:.5}", fl),
            JsonVal::String(s) => write!(f, "\"{}\"", s.replace('"', "\\\"")),
            JsonVal::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", elements.join(","))
            }
            JsonVal::Object(obj) => {
                let members: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("\"{}\":{}", k, v))
                    .collect();
                write!(f, "{{{}}}", members.join(","))
            }
        }
    }
}

impl JsonVal {
    pub fn get(&self, key: &str) -> Option<&JsonVal> {
        if let JsonVal::Object(map) = self {
            map.get(key)
        } else {
            None
        }
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        if let JsonVal::Object(map) = self {
            if let Some(JsonVal::String(s)) = map.get(key) {
                return Some(s.clone());
            }
        }
        None
    }

    pub fn get_number(&self, key: &str) -> Option<usize> {
        if let JsonVal::Object(map) = self {
            if let Some(JsonVal::Number(n)) = map.get(key) {
                return Some(*n);
            }
        }
        None
    }

    pub fn get_float(&self, key: &str) -> Option<f64> {
        if let JsonVal::Object(map) = self {
            if let Some(JsonVal::Number(f)) = map.get(key) {
                return Some(*f as f64);
            }
        }
        None
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        if let JsonVal::Object(map) = self {
            if let Some(JsonVal::Bool(b)) = map.get(key) {
                return Some(*b);
            }
        }
        None
    }
}

pub trait FromJsonVal: Sized {
    fn from_json(val: &JsonVal) -> Result<Self, String>;
}

#[derive(Debug, FromJsonVal)]
pub struct GitBlob {
    pub id: String,
    #[json(rename = "type")]
    pub action: String,
    pub merge_commit_sha: String,
    pub repo: Repo,
}

#[derive(Debug, FromJsonVal, Default)]
pub struct Repo {
    pub id: usize,
    pub name: String,
    pub url: String,
}
