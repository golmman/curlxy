use curlxy;

#[tokio::test]
async fn simple_get() {
    let path = "tests/collection/simple_get.http";
    let x = curlxy::execute(path).await.unwrap();

    assert_eq!(x.status, "200 OK");
    assert_eq!(x.text, "{\n  \"user-agent\": null\n}\n");
}

#[tokio::test]
async fn whitespace() {
    let path = "tests/collection/whitespace.http";
    let x = curlxy::execute(path).await.unwrap();

    assert_eq!(x.status, "200 OK");
    assert_eq!(x.text, "{\n  \"user-agent\": null\n}\n");
}
