#[derive(Debug)]
pub struct Url {
    pub scheme: Option<String>,
    pub netloc: Option<String>,
    pub port: Option<String>,
    pub path: Option<String>,
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

pub fn urlparse(url: &String) -> Url {
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
