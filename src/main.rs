use rust_tls_client::{TlsClient, ClientIdentifier};

fn main() {
    let client = TlsClient::new(
        ClientIdentifier::Chrome110, true
    );
    let res = client.get("https://www.google.com")
        .send()
        .unwrap();
    println!("{}", serde_json::to_string(&res).unwrap());
}