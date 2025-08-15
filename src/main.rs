use std::time::Instant;

use log::Level;

pub mod client;
pub mod deserializer;
pub mod examples;
pub mod serializer;

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();
    let begin = Instant::now();

    tokio::join!(examples::read_from_file(), examples::fetch_examples());
    println!("Time elapsed: {:?}", Instant::now().duration_since(begin));
}
