mod client;
mod bin_wrap;
mod error;
mod types;
mod request;

use std::collections::HashMap;
use serde;
use serde::{Deserialize, Serialize};

pub use bin_wrap::*;
pub use client::*;
pub use error::*;
pub use types::*;
pub use request::*;

#[derive(Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestResponse {
    pub id: String,
    pub session_id: String,
    pub status: u32,
    pub target: String,
    pub used_protocol: Option<String>,
    pub body: Option<String>,
    pub headers: Option<HashMap<String, Vec<String>>>,
    pub cookies: Option<HashMap<String, String>>,
}

impl RequestResponse {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
    
    pub fn get_status(&self) -> u32 {
        self.status
    }
    
    pub fn get_content(&self) -> Option<String> {
        self.body.as_ref().map(|v| v.to_string())
    }
    
    pub fn get_headers(&self) -> Option<HashMap<String, Vec<String>>> {
        self.headers.to_owned()
    }
    
    pub fn get_cookies(&self) -> Option<HashMap<String, String>> {
        self.cookies.to_owned()
    }
}

#[derive(Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    session_id: String,
    url: Option<String>,
    cookies: Option<Vec<String>>,
}

impl Payload {
    pub fn get_cookies(session_id: String, url: String) -> Self {
        Self {
            session_id,
            url: Some(url),
            cookies: None,
        }
    }

    pub fn add_cookies(session_id: String, url: String, cookies: Vec<String>) -> Self {
        Self {
            session_id,
            url: Some(url),
            cookies: Some(cookies),
        }
    }

    pub fn destroy_session(session_id: String) -> Self {
        Self {
            session_id,
            url: None,
            cookies: None,
        }
    }
}
