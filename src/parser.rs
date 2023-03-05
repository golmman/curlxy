use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::path::Path;

use regex::Regex;

use crate::HttpRequest;

pub struct Parser {
    comment_regex: Regex,
    lines: Lines<BufReader<File>>,
}

impl Parser {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(path).unwrap();
        let lines = BufReader::new(file).lines();
        let comment_regex = Regex::new(r"\s*(#|\\/\\/)").unwrap();

        Self {
            comment_regex,
            lines,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<HttpRequest>, Box<dyn Error>> {
        Ok(Vec::new())
    }

    pub fn parse_request_line(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(String::new())
    }

    pub fn parse_headers(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(String::new())
    }

    pub fn parse_message_body(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(String::new())
    }

    pub fn parse_request_handler(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(String::new())
    }

    pub fn parse_request_ref(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(String::new())
    }
}
