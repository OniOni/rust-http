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

struct HttpResponse {
    // status_code: u8,
    status_line: String,
    headers: HashMap<String, String>,
    body: String
}

impl HttpResponse {

    fn parse(response: String) -> HttpResponse {
        let mut lines = response.lines();

        let status_line = lines.next().unwrap();

        let mut headers: HashMap<String, String> = HashMap::new();
        loop {
            let line = lines.next().unwrap();
            if line.len() == 0 {
                break;
            }

            let mut split = line.split(':');
            headers.insert(
                split.next().unwrap().to_string(),
                split.next().unwrap().to_string()
            );
        }

        let body = lines.next().unwrap();

        return HttpResponse {
            status_line: status_line.to_string(),
            headers: headers,
            body: body.to_string()
        }
    }
}

struct HttpClient {
    host: String,
    port: String
}

impl HttpClient {

    fn get(&self, path: &str) -> Result<HttpResponse, String> {
        let req = HttpRequest::new(String::from("GET ") + path +" HTTP/1.1", "".to_string());

        let constr = String::from("") + &self.host + ":" + &self.port;
        if let Ok(mut stream) = TcpStream::connect(&constr) {
            let _ = stream.write(&req.to_u8buf());

            let mut buf = String::new();
            let _ = stream.read_to_string(&mut buf).unwrap();

            Ok(HttpResponse::parse(buf))
        } else {
            Err(String::from("Could not connect to ") + &constr)
        }
    }
}

fn get_cmd() -> String {
    match env::args().nth(1) {
        Some(cmd) => cmd,
        None => String::from("_no_command")
    }
}

fn main() {
    let client = HttpClient {
        host: "127.0.0.1".to_string(),
        port: "8080".to_string()
    };
    let res = client.get("/response.json").unwrap();

    println!("{}", res.body);
}
