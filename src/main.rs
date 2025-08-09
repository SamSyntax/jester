use crate::serializer::{FromJsonVal, GitBlob, JsonVal};
use std::{fs, time::Instant};
pub mod deserializer;
pub mod serializer;

fn main() {
    let path = std::env::args().nth(1).expect("no path to json file given");
    let begin = Instant::now();
    use std::collections::HashMap;
    let mut obj = HashMap::new();
    obj.insert(
        "name".to_string(),
        serializer::JsonVal::String("Alice".to_string()),
    );
    obj.insert("age".to_string(), serializer::JsonVal::Number(30));
    obj.insert(
        "ratio".to_string(),
        serializer::JsonVal::Float(1.32141512125),
    );
    let json = serializer::JsonVal::Object(obj);

    let contents = fs::read_to_string(path).expect("Should read");
    let mut parser = deserializer::Parser::new(&contents);
    let parsed = parser.parse_value().unwrap();

    for (i, blob) in parse_blobs(&parsed).iter().enumerate() {
        println!(
            "Num: {}, Id: {}, Action: {}, Commit Sha: {}, repo: {:?}",
            i, blob.id, blob.action, blob.merge_commit_sha, blob.repo
        );
    }

    let end = Instant::now();
    println!("Serializer: {}\n", json);
    println!("Time elapsed: {:?}", end.duration_since(begin));
}

fn parse_blobs(val: &JsonVal) -> Vec<GitBlob> {
    if let JsonVal::Array(arr) = val {
        arr.iter()
            .filter_map(|item| GitBlob::from_json(item).ok())
            .collect()
    } else {
        vec![]
    }
}
