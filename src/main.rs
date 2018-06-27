mod http;
mod optparse;
mod urlparse;

fn main() {
    let cmds = optparse::get_cmd();

    match cmds.iter().nth(1) {
        Some(x) => {
            match x {
                optparse::Arg::Val(x) => {
                    let url = urlparse::urlparse(x);
                    let client = http::HttpClient {
                        host: url.netloc.unwrap(),
                        port: url.port.unwrap_or(String::from("80"))
                    };
                    let res = client.get(&url.path.unwrap_or(String::from("/"))).unwrap();

                    for l in res.body.lines() {
                        println!("{}", l);
                    }
                },
                _ => eprintln!("Looking for value not option.")
            }
        },
        None => {
            eprintln!("Need at leat and url.")
        }
    }
}
