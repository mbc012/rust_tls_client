use std::collections::HashMap;
use serde::{Serialize, Deserialize};

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