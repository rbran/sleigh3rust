use std::borrow::Cow;

use regex::Regex;
use lazy_static::lazy_static;

#[derive(Clone, Copy)]
enum Endian {
    Big,
    Little,
}

fn remove_spaces(parsed: &str) -> Cow<str> {
    lazy_static!{
        static ref RE: Regex = Regex::new(r"\s+").unwrap();
    }
    RE.replace_all(parsed, " ")
}

mod arm;
mod superh4;
mod pic;
