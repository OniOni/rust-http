use std::io::prelude::*;
use std::env;
use std::net::TcpStream;
use std::collections::HashMap;

struct HttpRequest {
    request: String,
    headers: HashMap<String, String>,
    body: String
}

fn append(buf: &mut Vec<u8>, str: String) {
    for c in str.bytes() {
        buf.push(c);
    }
}

impl HttpRequest {
    fn new(request: String, body: String) -> HttpRequest {
        let headers: HashMap<String, String> = HashMap::new();
        let mut http = HttpRequest { request: request, headers: headers, body: body };

        http.add_header("User-Agent", "rust-http");

        return http
    }

    fn add_header(&mut self, header: &str, value: &str) {
        self.headers.insert(header.to_string(), value.to_string());
    }

    fn to_u8buf(self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        append(&mut buf, self.request + "\r\n");

        for (key, val) in self.headers.iter() {
            append(&mut buf, key.to_string() + ": ");
            append(&mut buf, val.to_string() + "\r\n");
        }

        append(&mut buf, "\r\n".to_string());

        if self.body.len() > 0 {
            append(&mut buf, self.body + "\r\n")
        }

        return buf;
    }
}


fn get_cmd() -> String {
    match env::args().nth(1) {
        Some(cmd) => cmd,
        None => String::from("_no_command")
    }
}

fn main() {
    let con_str = "127.0.0.1:8080";
    let http = HttpRequest::new("GET /response.json HTTP/1.1".to_string(), "".to_string());

    let req = http.to_u8buf();

    if let Ok(mut stream) = TcpStream::connect(con_str) {
        let _ = stream.write(&req);
        let mut buf = String::new();
        let n_read = stream.read_to_string(&mut buf).unwrap();
        println!("{}, {}", buf, n_read);
    } else {
        println!("Could not connect to {}", con_str);
    }
}
