use crate::deserializer;
use crate::serializer::{FromJsonVal, JsonVal};
use parser_derive::FromJsonVal;
use std::io::{Read, Write};
use std::net::TcpStream;

const OTHER_URL: &str = "http://jsonplaceholder.typicode.com/todos/1";

#[derive(Debug, FromJsonVal, Default)]
pub struct User {
    #[json(rename = "user_id")]
    pub user_id: usize,
    pub id: usize,
    pub title: String,
    pub completed: bool,
}

impl User {
    pub fn print(&self) {
        println!(
            "UserId: {}, Id: {}, Title: {}, Completed: {}",
            self.user_id, self.id, self.title, self.completed
        );
    }
}

pub fn req_json(url: Option<String>) -> std::io::Result<User> {
    let url = url.unwrap_or_else(|| OTHER_URL.to_string());

    let no_scheme = url.strip_prefix("http://").unwrap_or(&url);
    let mut parts = no_scheme.splitn(2, '/');
    let host = parts.next().unwrap();
    let path = format!("/{}", parts.next().unwrap_or(""));

    let mut stream = TcpStream::connect(format!("{}:80", host))?;
    let req = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        path, host
    );

    stream.write_all(req.as_bytes())?;

    let mut res = String::new();
    stream.read_to_string(&mut res)?;

    if let Some(pos) = res.find("\r\n\r\n") {
        let body = &res[pos + 4..];
        let mut parser = deserializer::Parser::new(body);
        let parsed = parser.parse_value().unwrap();

        let user = User::from_json(&parsed).unwrap();
        return Ok(user);
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "No HTTP body found",
    ))
}
