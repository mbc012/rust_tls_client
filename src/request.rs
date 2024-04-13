use std::collections::HashMap;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use crate::bin_wrap::{SHARED_METHODS};
use crate::client::{CustomClient};
use crate::error::TlsClientError;
use crate::RequestResponse;
use crate::types::ClientIdentifier;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestPayload {
    catch_panics:                       Option<bool>,
    certificate_pinning_hosts:          Option<HashMap<String, String>>,
    custom_tls_client:                  Option<CustomClient>,
    transport_options:                  Option<TransportOptions>,
    follow_redirects:                   Option<bool>,
    force_http_1:                       Option<bool>,
    header_order:                       Option<Vec<String>>,
    headers:                            Option<HashMap<String, String>>,
    insecure_skip_verify:               Option<bool>,
    is_byte_request:                    Option<bool>,
    is_byte_response:                   Option<bool>,
    is_rotating_proxy:                  Option<bool>,
    proxy_url:                          Option<String>,
    request_body:                       Option<String>,
    request_cookies:                    Option<Vec<String>>,
    default_headers:                    Option<HashMap<String, Vec<String>>>,
    request_method:                     Option<String>,
    request_url:                        Option<String>,
    #[serde(rename = "disableIPV6")]
    disable_ipv6:                       Option<bool>,
    local_address:                      Option<String>,
    session_id:                         Option<String>,
    server_name_overwrite:              Option<String>,
    stream_output_block_size:           Option<u32>,    // TODO CHECK TYPE
    stream_output_e_o_f_symbol:         Option<String>, // TODO CHECK TYPE
    stream_output_path:                 Option<String>, // TODO CHECK TYPE
    timeout_milliseconds:               Option<u32>,
    timeout_seconds:                    Option<u32>,
    tls_client_identifier:              Option<ClientIdentifier>,
    with_debug:                         Option<bool>,
    with_default_cookie_jar:            Option<bool>,
    without_cookie_jar:                 Option<bool>,
    #[serde(rename = "withRandomTLSExtensionOrder")]
    with_random_tls_extension_order:    Option<bool>,
}


impl RequestPayload {
    fn add_header(&mut self, key: &str, value: &str) {
        if let Some(headers) = &mut self.headers {
            headers.insert(key.to_string(), value.to_string());
        } else {
            let mut h = HashMap::new();
            h.insert(key.to_string(), value.to_string());
            self.headers = Some(h);
        }
    }
    
    pub fn byte_response(&mut self) -> &mut Self {
        self.is_byte_response = Some(true);
        self
    }
    
    pub fn byte_request(&mut self) -> &mut Self {
        self.is_byte_request = Some(true);
        self
    }
    
    pub fn headers(&mut self, header_map: HeaderMap) -> &mut Self {
        let mut h = HashMap::new();
        for (key, value) in header_map.iter() {
            h.insert(key.as_str().to_string(), value.to_str().unwrap().to_string());
        }
        self.headers = Some(h);
        self
    }

    pub fn body<B: Into<String>>(&mut self, body: B) -> &mut Self {
        self.request_body = Some(body.into());
        self
    }

    pub fn json(&mut self, json: serde_json::Value) -> &mut Self {
        self.add_header("Content-Type", "application/json");
        self.request_body = Some(serde_json::to_string(&json).unwrap());
        self
    }

    pub fn send(&self) -> Result<RequestResponse, TlsClientError> {
        Ok(SHARED_METHODS.request(self))
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TransportOptions {
    disable_keep_alives:        bool,
    disable_compression:        bool,
    max_idle_conns:             u32,
    max_idle_conns_per_host:    u32,
    max_conns_per_host:         u32,
    max_response_header_bytes:  u32,
    write_buffer_size:          u32,
    read_buffer_size:           u32,
    idle_conn_timeout:          u32,
}



