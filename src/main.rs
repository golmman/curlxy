#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "tests/collection/simple_get.http";
    let http_res = curlxy::execute(path).await?;
    curlxy::display(http_res);

    Ok(())
}
