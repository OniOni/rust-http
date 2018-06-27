use std::env;
use std::collections::LinkedList;

#[derive(Debug)]
pub enum Arg {
    Opt(String),
    Val(String)
}

pub fn get_cmd() -> LinkedList<Arg> {

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
