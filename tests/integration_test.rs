use curlxy;

#[tokio::test]
async fn comments() {
    let path = "tests/collection/comments.http";
    let http_res = curlxy::execute(path).await.unwrap();

    assert_eq!(http_res.status, "200 OK");
    assert_eq!(http_res.text, "{\n  \"user-agent\": null\n}\n");
}

#[tokio::test]
async fn simple_get() {
    let path = "tests/collection/simple_get.http";
    let http_res = curlxy::execute(path).await.unwrap();

    assert_eq!(http_res.status, "200 OK");
    assert_eq!(http_res.text, "{\n  \"user-agent\": null\n}\n");
}

#[tokio::test]
async fn whitespace() {
    let path = "tests/collection/whitespace.http";
    let http_res = curlxy::execute(path).await.unwrap();

    assert_eq!(http_res.status, "200 OK");
    assert_eq!(http_res.text, "{\n  \"user-agent\": null\n}\n");
}
