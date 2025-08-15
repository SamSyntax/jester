use std::env;
use std::path::PathBuf;

use futures::future::join_all;

use crate::client;
use crate::deserializer;
use crate::serializer;

pub async fn read_file_cli(file: String) -> Result<String, String> {
    let default_path = PathBuf::from(file);
    let path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or(default_path);

    tokio::fs::read_to_string(&path)
        .await
        .map_err(|e| format!("Couldn't read file {}: {}", path.display(), e))
}

pub async fn to_json_from_file(path: String) -> serializer::JsonVal {
    let contents = read_file_cli(path).await.expect("Failed to read file");
    tokio::task::spawn_blocking(move || {
        let mut parser = deserializer::Parser::new(&contents);
        parser.parse_value().unwrap()
    })
    .await
    .expect("Parsing task panicked")
}

pub async fn fetch_examples() {
    let futures = (1..=10).map(|i| {
        client::req_json(Some(format!(
            "http://jsonplaceholder.typicode.com/todos/{}",
            i
        )))
    });
    let results = join_all(futures).await;
    for res in results {
        match res {
            Ok(user) => user.print(),
            Err(e) => eprintln!("Request failed: {}", e),
        }
    }
}

pub async fn read_from_file() {
    // getting home dir
    let home_dir = env::home_dir().expect("Can't get home_dir");
    let json_body = to_json_from_file(
        home_dir
            .join("dev/rust/parser/large-file.json")
            .to_string_lossy()
            .to_string(),
    )
    .await;
    for blob in serializer::parse_blobs::<serializer::GitBlob>(&json_body).iter() {
        blob.print();
    }
}
