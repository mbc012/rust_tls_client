
# rust_tls_client

A rust tls client based on the cffi distributions of bogdanfinn/tls-client.


## Usage/Examples

```rust
// reqwest header map
let mut hm = HeaderMap::new();
hm.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"));
hm.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36"));
hm.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br"));
hm.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("de-DE,de;q=0.9,en-US;q=0.8,en;q=0.7"));

// create TlsClient instance
let client = TlsClient::new(ClientIdentifier::Chrome105, false);

// make request, in similar syntax to reqwest
let req = &client.get("https://microsoft.com")
    .headers(hm)
    .send()
    .unwrap();

println!("{}", serde_json::to_string(&req).unwrap());
```

