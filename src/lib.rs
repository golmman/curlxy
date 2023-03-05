use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use regex::Regex;
use reqwest::Response;

mod parser;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Option<String>,
    pub url: Option<String>,
}

impl HttpRequest {
    pub fn new() -> Self {
        Self {
            method: None,
            url: None,
        }
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    pub status: String,
    pub text: String,
}

impl HttpResponse {
    pub async fn from(res: Response) -> Result<Self, Box<dyn Error>> {
        let status = res.status().to_string();
        let text = res.text().await?;

        Ok(Self { status, text })
    }
}

pub fn parse<P: AsRef<Path>>(path: P) -> Result<HttpRequest, Box<dyn Error>> {
    let file = File::open(path)?;
    let lines = BufReader::new(file).lines();

    let mut http_req = HttpRequest::new();
    let request_regex = Regex::new(r"\s*(\S+)\s*(\S+)").unwrap();
    let comment_regex = Regex::new(r"\s*(#|\\/\\/)").unwrap();
    let header_regex = Regex::new(r"\s*(\S+):\s*(\S*)").unwrap();

    for line in lines {
        let line = line.unwrap();

        if comment_regex.is_match(&line) {
            continue;
        }

        if let Some(cap) = request_regex.captures_iter(&line).next() {
            http_req.method = Some(cap[1].to_string());
            http_req.url = Some(cap[2].to_string());
            continue;
        }
    }

    Ok(http_req)
}

pub async fn request(http_req: HttpRequest) -> Result<HttpResponse, Box<dyn Error>> {
    let method = reqwest::Method::from_bytes(http_req.method.unwrap().as_bytes())?;
    let url = http_req.url.unwrap();

    let client = reqwest::Client::new();
    let res = client.request(method, url).send().await?;

    let http_res = HttpResponse::from(res).await?;

    Ok(http_res)
}

pub async fn execute<P: AsRef<Path>>(path: P) -> Result<HttpResponse, Box<dyn Error>> {
    let http_req = parse(path)?;
    println!("{http_req:?}");
    let http_res = request(http_req).await?;

    Ok(http_res)
}

pub fn display(http_res: HttpResponse) {
    println!("{:?}", http_res);
}
