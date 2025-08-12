use std::{fs, time::Instant};

pub mod client;
pub mod deserializer;
pub mod serializer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).expect("no path to json file given");
    let begin = Instant::now();

    let contents = fs::read_to_string(path).expect("Should read");
    let mut parser = deserializer::Parser::new(&contents);
    let parsed: serializer::JsonVal = parser.parse_value().unwrap();

    for blob in serializer::parse_blobs::<serializer::GitBlob>(&parsed).iter() {
        blob.print();
    }

    let end = Instant::now();

    let user = client::req_json(None)?;
    user.print();
    println!("Time elapsed: {:?}", end.duration_since(begin));

    Ok(())
}
