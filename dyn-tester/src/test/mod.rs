use std::borrow::Cow;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone, Copy)]
enum Endian {
    Big,
    Little,
}

fn remove_spaces(parsed: &str) -> Cow<str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\s+").unwrap();
    }
    RE.replace_all(parsed, " ")
}

mod arm;
mod aarch64;
mod pic;
mod superh4;
mod z80;
mod v850;
//TODO mod tricore;
mod sparcv9_32;
mod sparcv9_64;
mod x86;
