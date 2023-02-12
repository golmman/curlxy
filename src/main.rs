#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    curlxy::execute().await?;

    Ok(())
}
