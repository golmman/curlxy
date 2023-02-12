use reqwest::Response;

pub struct HttpRequest {
    pub method: String,
    pub url: String,
}

#[derive(Debug)]
pub struct HttpResponse {
    pub status: String,
    pub text: String,
}

impl HttpResponse {
    pub async fn from(res: Response) -> Result<Self, Box<dyn std::error::Error>> {
        let status = res.status().to_string();
        let text = res.text().await?;

        Ok(Self { status, text })
    }
}

pub fn parse() -> HttpRequest {
    HttpRequest {
        method: "GET".to_string(),
        url: "https://httpbin.org/ip".to_string(),
    }
}

pub async fn request(http_req: HttpRequest) -> Result<HttpResponse, Box<dyn std::error::Error>>{
    let res = reqwest::get("https://httpbin.org/ip").await?;
    let http_res = HttpResponse::from(res).await?;

    Ok(http_res)
}



pub async fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::get("https://httpbin.org/ip").await?;

    let http_res = HttpResponse::from(res).await?;

    println!("{:?}", http_res);

    Ok(())
}
