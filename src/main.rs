use std::{
    env::{self},
    fs,
    path::PathBuf,
    time::Instant,
};

use crate::serializer::JsonVal;

pub mod client;
pub mod deserializer;
pub mod serializer;

fn read_file_cli(file: String) -> Result<String, String> {
    let default_path = PathBuf::from(file);
    let path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or(default_path);

    let contents = fs::read_to_string(&path)
        .map_err(|e| format!("Couldn't read file {}: {}", path.display(), e))?;
    Ok(contents)
}

fn to_json_from_file(path: String) -> JsonVal {
    let contents = read_file_cli(path).expect("Failed to read file");
    let mut parser = deserializer::Parser::new(&contents);
    parser.parse_value().unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let begin = Instant::now();
    let home_dir = env::home_dir().expect("Can't get home_dir");
    let json_body = to_json_from_file(
        home_dir
            .join("dev/rust/parser/large-file.json")
            .to_string_lossy()
            .to_string(),
    );
    for blob in serializer::parse_blobs::<serializer::GitBlob>(&json_body).iter() {
        blob.print();
    }

    for i in 1..11 {
        let user = client::req_json(Some(
            format!("http://jsonplaceholder.typicode.com/todos/{}", i).to_string(),
        ))?;
        user.print();
    }
    let end = Instant::now();
    println!("Time elapsed: {:?}", end.duration_since(begin));

    Ok(())
}
