use std::collections::HashMap;
use std::ops::Deref;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::request::{RequestPayload};
use crate::types::{AeadId, ClientIdentifier, DelegatedCredential, H2Setting, KdfId, KeyShareCurve, SignatureAlgorithm, SupportedVersion};


#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TlsClient {
    // Session Parameters
    // |- Can NOT be changed after first request
    session_id:                         String,
    insecure_skip_verify:               bool,
    timeout_seconds:                    u32,

    // |- Can be changed at anytime
    follow_redirects:                   bool,
    proxy_url:                          Option<String>,

    // Profile
    tls_client_identifier:              Option<ClientIdentifier>,
    with_random_tls_extension_order:    Option<bool>,
    custom_tls_client:                  Option<CustomClient>,

    // Request parameters
    request_url:                        Option<String>,
    request_method:                     Option<String>,
}



impl TlsClient {
    fn build_for_request(&self, method: String, url: String) -> RequestPayload {
        let tc = TlsClient {
            session_id: self.session_id.clone(),
            insecure_skip_verify: self.insecure_skip_verify,
            timeout_seconds: self.timeout_seconds,
            follow_redirects: self.follow_redirects,
            proxy_url: self.proxy_url.clone(),
            tls_client_identifier: self.tls_client_identifier.clone(),
            with_random_tls_extension_order: self.with_random_tls_extension_order,
            custom_tls_client: self.custom_tls_client.clone(),
            request_url: Some(url),
            request_method: Some(method),
        };
        serde_json::from_value(serde_json::to_value(&tc).unwrap()).unwrap()
    }

    pub fn default() -> Self {
        Self::new(ClientIdentifier::Chrome120, false)
    }

    pub fn new(client_identifier: ClientIdentifier, random_tls_order: bool) -> TlsClient {
        TlsClient {
            session_id: Uuid::new_v4().to_string(),
            insecure_skip_verify: false,
            timeout_seconds: 30,
            follow_redirects: true,
            tls_client_identifier: Some(client_identifier),
            with_random_tls_extension_order: Some(random_tls_order),
            custom_tls_client: None,
            proxy_url: None,
            request_url: None,
            request_method: None,
        }
    }

    pub fn new_custom(custom_profile: CustomClient) -> TlsClient {
        TlsClient {
            session_id: Uuid::new_v4().to_string(),
            insecure_skip_verify: false,
            timeout_seconds: 30,
            follow_redirects: true,
            tls_client_identifier: None,
            with_random_tls_extension_order: Some(false),
            custom_tls_client: Some(custom_profile),
            proxy_url: None,
            request_url: None,
            request_method: None,
        }
    }

    pub fn set_insecure_skip_verify(mut self, skip: bool) -> Self {
        self.insecure_skip_verify = skip;
        self
    }

    pub fn set_timeout_seconds(mut self, seconds: u32) -> Self {
        self.timeout_seconds = seconds;
        self
    }

    pub fn set_follow_redirects(mut self, follow: bool) -> Self {
        self.follow_redirects = follow;
        self
    }

    pub fn set_proxy_url(mut self, url: String) -> Self {
        self.proxy_url = Some(url);
        self
    }

    pub fn remove_proxy_url(mut self) -> Self {
        self.proxy_url = None;
        self
    }

    pub fn get(&self, url: &str) -> RequestPayload {
        self.build_for_request("GET".to_string(), url.to_string())
    }

    pub fn post(&self, url: &str) -> RequestPayload {
        self.build_for_request("POST".to_string(), url.to_string())
    }

}

/// Simple builder client for generating custom profiles for the `TlsClient`
///
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomClient {
    cert_compression_algo:                      Option<String>,
    connection_flow:                            Option<u32>,
    h2_settings:                                Option<HashMap<H2Setting, u32>>,
    h2_settings_order:                          Option<Vec<H2Setting>>,
    header_priority:                            Option<PriorityParam>,
    ja3_string:                                 Option<String>,
    key_share_curves:                           Option<Vec<KeyShareCurve>>,
    priority_frames:                            Option<Vec<PriorityFrames>>,
    alpn_protocols:                             Option<Vec<String>>,
    alps_protocols:                             Option<Vec<String>>,
    #[serde(rename = "ECHCandidateCipherSuites")]
    ech_candidate_cipher_suites:                Option<Vec<CandidateCipherSuite>>,
    #[serde(rename = "ECHCandidatePayloads")]
    ech_candidate_payloads:                     Option<u16>,
    pseudo_header_order:                        Option<Vec<String>>,
    supported_delegated_credentials_algorithms: Option<Vec<DelegatedCredential>>,
    supported_signature_algorithms:             Option<Vec<SignatureAlgorithm>>,
    supported_versions:                         Option<Vec<SupportedVersion>>,
}

impl CustomClient {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_cert_compression_algo(mut self, algo: String) -> Self {
        self.cert_compression_algo = Some(algo);
        self
    }

    pub fn with_connection_flow(mut self, flow: u32) -> Self {
        self.connection_flow = Some(flow);
        self
    }

    pub fn with_h2_settings(mut self, settings: HashMap<H2Setting, u32>) -> Self {
        self.h2_settings = Some(settings);
        self
    }

    pub fn with_h2_settings_order(mut self, order: Vec<H2Setting>) -> Self {
        self.h2_settings_order = Some(order);
        self
    }

    pub fn with_header_priority(mut self, priority: PriorityParam) -> Self {
        self.header_priority = Some(priority);
        self
    }

    pub fn with_ja3_string(mut self, ja3: String) -> Self {
        self.ja3_string = Some(ja3);
        self
    }

    pub fn with_key_share_curves(mut self, curves: Vec<KeyShareCurve>) -> Self {
        self.key_share_curves = Some(curves);
        self
    }

    pub fn with_priority_frames(mut self, frames: Vec<PriorityFrames>) -> Self {
        self.priority_frames = Some(frames);
        self
    }

    pub fn with_alpn_protocols(mut self, protocols: Vec<String>) -> Self {
        self.alpn_protocols = Some(protocols);
        self
    }

    pub fn with_alps_protocols(mut self, protocols: Vec<String>) -> Self {
        self.alps_protocols = Some(protocols);
        self
    }

    pub fn with_ech_candidate_cipher_suites(mut self, suites: Vec<CandidateCipherSuite>) -> Self {
        self.ech_candidate_cipher_suites = Some(suites);
        self
    }

    pub fn with_ech_candidate_payloads(mut self, payloads: u16) -> Self {
        self.ech_candidate_payloads = Some(payloads);
        self
    }

    pub fn with_pseudo_header_order(mut self, order: Vec<String>) -> Self {
        self.pseudo_header_order = Some(order);
        self
    }

    pub fn with_supported_delegated_credentials_algorithms(mut self, algorithms: Vec<DelegatedCredential>) -> Self {
        self.supported_delegated_credentials_algorithms = Some(algorithms);
        self
    }

    pub fn with_supported_signature_algorithms(mut self, algorithms: Vec<SignatureAlgorithm>) -> Self {
        self.supported_signature_algorithms = Some(algorithms);
        self
    }

    pub fn with_supported_versions(mut self, versions: Vec<SupportedVersion>) -> Self {
        self.supported_versions = Some(versions);
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CandidateCipherSuite {
    kdf_id: KdfId,
    aead_id: AeadId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriorityFrames {
    #[serde(rename = "streamID")]
    stream_id: u32,
    frame_payload: PriorityParam,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriorityParam {
    stream_dep: u32,
    exclusive: bool,
    weight: u32,
}


mod tests {
    use std::io::Read;
    use reqwest::header::{ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONTENT_TYPE, HeaderValue, USER_AGENT};
    use serde_json::Value::String;
    use crate::types::ClientIdentifier;
    use super::*;

    #[test]
    fn test_get() {
        let mut hm = HeaderMap::new();
        hm.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"));
        hm.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36"));
        hm.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br"));
        hm.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("de-DE,de;q=0.9,en-US;q=0.8,en;q=0.7"));

        let client = TlsClient::new(ClientIdentifier::Chrome105, false);

        let req = &client.get("https://microsoft.com")
            .headers(hm)
            .send()
            .unwrap();

        assert_eq!(req.get_status(), 200)
    }
    
    #[test]
    fn test_post() {
        let mut hm = HeaderMap::new();
        hm.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"));
        hm.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36"));
        hm.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br"));
        hm.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
        hm.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("de-DE,de;q=0.9,en-US;q=0.8,en;q=0.7"));

        let client = TlsClient::new(ClientIdentifier::Chrome105, false);
        
        let req = client.post("https://www.toptal.com/developers/postbin/1711461823032-1978890220634")
            .headers(hm)
            .body("foo=bar&baz=foo")
            .send()
            .unwrap();
        
        assert_eq!(req.get_status(), 200)
    }
    
    #[test]
    fn test_get_image() {
        let mut hm = HeaderMap::new();
        hm.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"));
        hm.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36"));
        hm.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br"));
        hm.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("de-DE,de;q=0.9,en-US;q=0.8,en;q=0.7"));

        let client = TlsClient::new(ClientIdentifier::Chrome105, false);
        
        let req = client.get("https://avatars.githubusercontent.com/u/17678241?v=4")
            .headers(hm)
            .byte_response() // Set response type to bytes
            .send()
            .unwrap();



        assert_eq!(req.get_status(), 200)
    }

    #[test]
    fn test_post_image() {
        let mut hm = HeaderMap::new();
        hm.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"));
        hm.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36"));
        hm.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br"));
        hm.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("de-DE,de;q=0.9,en-US;q=0.8,en;q=0.7"));

        let client = TlsClient::new(ClientIdentifier::Chrome105, false);
        
        // Load image and convert to base64 string
        let mut img_file = std::fs::File::open("src/cb_example.png").unwrap();
        let mut buffer = Vec::new();
        img_file.read_to_end(&mut buffer).unwrap();
        let b64_file = base64::encode(&buffer);
        println!("{}", b64_file);

        let req = client.post("https://www.toptal.com/developers/postbin/1711492583368-7330834681633")
            .headers(hm)
            .body(b64_file)
            .byte_request()
            .send()
            .unwrap();
        
        assert_eq!(serde_json::to_string(&req).unwrap(), "xx".to_string())

    }

    #[test]
    fn make_custom_client() {

    }

}

