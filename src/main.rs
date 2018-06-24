use std::io::prelude::*;
use std::env;
use std::net::TcpStream;
use std::collections::HashMap;
use std::collections::LinkedList;

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

            let mut split = line.split(": ");
            headers.insert(
                split.next().unwrap().to_string(),
                split.next().unwrap().to_string()
            );
        }

        let mut body = String::new();
        for l in lines {
            body += l;
        }

        return HttpResponse {
            status_line: status_line.to_string(),
            headers: headers,
            body: body
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

#[derive(Debug)]
struct Url {
    scheme: Option<String>,
    netloc: Option<String>,
    port: Option<String>,
    path: Option<String>,
}

impl Url {
    fn new() -> Url {
        Url { scheme: None, netloc: None, port: None, path: None}
    }
}

enum UrlState {
    Scheme,
    NetLoc,
    Port,
    Path,
}

fn urlparse(url: &String) -> Url {
    let mut state = UrlState::Scheme;
    let mut url_struct = Url::new();
    let mut buf = String::new();
    let mut end = false;

    for c in url.chars() {
        match state {
            UrlState::Scheme => {
                if c == ':' {
                    continue;
                } else if c == '/' {
                    if end {
                        url_struct.scheme = Some(buf);
                        state = UrlState::NetLoc;
                        buf = String::new();
                    } else {
                        end = true
                    }
                } else {
                    buf.push(c);
                }
            },
            UrlState::NetLoc => {
                if c == ':' {
                    url_struct.netloc = Some(buf);
                    state = UrlState::Port;
                    buf = String::new();
                } else if c == '/' {
                    url_struct.netloc = Some(buf);
                    state = UrlState::Path;
                    buf = String::new();
                } else {
                    buf.push(c);
                }
            },
            UrlState::Port => {
                if c == '/' {
                    url_struct.port = Some(buf);
                    state = UrlState::Path;
                    buf = String::new();
                } else {
                    buf.push(c);
                }
            },
            UrlState::Path => {
                buf.push(c);
            }
        };
    }

    if buf.len() > 0 {
        url_struct.path = Some(buf);
    }

    return url_struct;
}

#[derive(Debug)]
enum Arg {
    Opt(String),
    Val(String)
}

fn get_cmd() -> LinkedList<Arg> {

    let mut args: LinkedList<Arg> = LinkedList::new();

    for a in env::args() {
        let arg = if a.starts_with('-') {
            Arg::Opt(a)
        } else {
            Arg::Val(a)
        };

        args.push_back(arg);
    }

    return args;
}

fn main() {
    let client = HttpClient {
        host: "127.0.0.1".to_string(),
        port: "8080".to_string()
    };
    let res = client.get("/response.json").unwrap();

    println!("{}", res.body);

    let cmds = get_cmd();
    println!("{:?}", cmds);
}
