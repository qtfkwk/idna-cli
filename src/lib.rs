use serde::Serialize;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html#a-more-efficient-approach

pub fn to_unicode(domain: &str) -> (String, String) {
    match idna::domain_to_unicode(&domain) {
        (s, Ok(_)) => (s, String::new()),
        (s, Err(e)) => (s, e.to_string()),
    }
}

pub fn to_ascii(domain: &str) -> (String, String) {
    match idna::domain_to_ascii(&domain) {
        Ok(s) => (s, String::new()),
        Err(e) => (String::new(), e.to_string()),
    }
}

pub fn process(domain: &str, decode: bool) -> (String, String) {
    if decode {
        to_unicode(domain)
    } else {
        to_ascii(domain)
    }
}

pub fn print_csv_result(domain: &str, decode: bool) {
    let (converted, errors) = process(domain, decode);
    println!("{domain:?},{converted:?},{errors:?}");
}

#[derive(Debug, Serialize)]
pub struct Domain {
    ascii: String,
    unicode: String,
    errors: String,
}

impl Domain {
    pub fn new(domain: &str, decode: bool) -> Domain {
        let (converted, errors) = process(domain, decode);
        if decode {
            Domain {
                ascii: domain.to_string(),
                unicode: converted,
                errors,
            }
        } else {
            Domain {
                ascii: converted,
                unicode: domain.to_string(),
                errors,
            }
        }
    }
}
