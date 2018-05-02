extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use regex::Regex;

struct Page {
    id: Option<u32>,
    title: Option<String>,
    text: Option<String>,
}

use Tag::*;

enum Tag {
    StartPage,
    Title,
    ID,
    EndPage,
    NoTag,
}

fn tag_of(s: &str) -> Tag {
    let s = s.trim();
    if s.starts_with("<page>") {
        StartPage
    } else if s.starts_with("<title>") {
        Title
    } else if s.starts_with("</page>") {
        EndPage
    } else {
        NoTag
    }
}

fn main() {
    let f = File::open("./jawiki-latest-pages-articles_sample.xml").unwrap();
    let buf = BufReader::new(f);

    for line in buf.lines() {
        let s = line.unwrap();

        match tag_of(s.as_str()) {
            StartPage => println!("This is start page"),
            EndPage => println!("This is end page"),
            Title => {
                if let Some(title) = extract_text(s.as_str(), "title") {
                    println!("{}", title);
                }
            }
            _ => (),
        }
    }
}

fn extract_text(s: &str, tag: &str) -> Option<String> {
    let pattern = format!("<{}.*?>(.*)</{}>", tag, tag);
    let re = Regex::new(pattern.as_str()).unwrap();

    let mat = re.captures(s);
    match mat {
        Some(cap) => {
            let s = cap.get(1).unwrap().as_str();
            Some(String::from(s))
        }
        None => None,
    }
}
