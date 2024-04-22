use std::ffi::{CStr, CString};
use std::io::Write;
use std::os::raw::c_char;
use lazy_static::lazy_static;
use libloading::{Library, Symbol};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;
use serde_json::Value;
use crate::error::TlsClientError;
use crate::response::RequestResponse;
use crate::request::{RequestPayload};


lazy_static! {
    pub static ref SHARED_METHODS: TlsClientSharedMethods = TlsClientSharedMethods::new_default().unwrap();
    // TODO: Add threadsafe impl
    // Threadsafe
    // static ref TS_SHARED_METHODS: Mutex<TlsClientSharedMethods> = Mutex::new(TlsClientSharedMethods::new_default().unwrap());
}


#[derive(Debug)]
pub struct TlsClientSharedMethods {
    lib: Library,
}
impl TlsClientSharedMethods {
    pub fn new_default() -> Result<Self, TlsClientError> {
        unsafe {
            let bf = TlsClientBinaryDownloader::new()
                .map_err(TlsClientError::GeneralError)
                .unwrap();
            
            Library::new(bf.path())
                .map(|lib| Self { lib })
                .map_err(|_| TlsClientError::GeneralError("Failed to build TLS Client".to_string()))
        }
    }

    pub fn new(dll_path: &str) -> Result<Self, TlsClientError> {
        unsafe {
            Library::new(dll_path)
                .map(|lib| Self { lib })
                .map_err(|_| TlsClientError::GeneralError("Failed to build TLS Client".to_string()))
        }
    }

    pub fn destroy_all(&self) -> String {
        unsafe {
            let da_func: Symbol<unsafe extern "C" fn() -> *const c_char> = self.lib.get(b"destroyAll").unwrap();
            let char_res = da_func();

            let str_res = CStr::from_ptr(char_res).to_string_lossy().into_owned();
            str_res
        }
    }

    pub fn free_memory(&self, id: String) {
        unsafe {
            let cstring = CString::new(id).unwrap();
            let char_ptr = cstring.as_ptr();

            let fm_func: Symbol<unsafe extern "C" fn(*const c_char)> = self.lib.get(b"freeMemory").unwrap();
            fm_func(char_ptr);
        }
    }

    fn string_to_string(&self, payload: String, identifier: &[u8]) -> String {
        unsafe {
            let cstring = CString::new(payload).unwrap();
            let char_ptr = cstring.as_ptr();

            let request_func: Symbol<unsafe extern "C" fn(*const c_char) -> *const c_char> = self.lib.get(identifier).unwrap();
            let char_res = request_func(char_ptr);

            let str_res = CStr::from_ptr(char_res).to_string_lossy().into_owned();
            str_res
        }
    }

    pub fn request(&self, payload: &RequestPayload) -> RequestResponse {
        let res: RequestResponse = serde_json::from_str(
            self.string_to_string(
                serde_json::to_string(payload).unwrap(),
                b"request",
            ).trim()
        ).unwrap();

        // dealloc res from mem
        self.free_memory(res.get_id());

        res
    }

    pub fn get_cookies_from_session(&self, payload: String) -> String {
        self.string_to_string(payload, b"getCookiesFromSession")
    }

    pub fn add_cookies_to_session(&self, payload: String) -> String {
        self.string_to_string(payload, b"addCookiesToSession")
    }

    pub fn destroy_session(&self, payload: String) -> String {
        self.string_to_string(payload, b"destroySession")
    }
}
impl Default for TlsClientSharedMethods {
    fn default() -> Self {
        TlsClientSharedMethods::new_default().unwrap()
    }
}


struct TlsClientBinaryDownloader(pub String);
impl TlsClientBinaryDownloader {
    pub fn new() -> Result<Self, String> {
        let info = Self::get_latest_info()?;
        let url = Self::parse_for_file(info)?;
        let path = Self::download_save_file(url)?;
        Ok(Self(path))
    }
    
    fn download_save_file(url: String) -> Result<String, String> {
        let cli = reqwest::blocking::Client::new();
        let req = cli.get(url)
            .send()
            .map_err(|_| String::from("Failed to download file."))?
            .bytes()
            .map_err(|_| String::from("Failed to convert to bytes."))?
            .to_vec();

        let filename = "tls_client_binary".to_owned() + match std::env::consts::OS {
            "windows" => ".dll",
            "macos" => ".dylib",
            "linux" => ".so",
            _ => return Err(String::from("Invalid OS detected.")),
        };

        let mut dir = std::env::temp_dir();

        dir.push("rust_tls_client");
        let _ = std::fs::create_dir_all(&dir);

        dir.push(filename);
        let fp = &dir.display().to_string();

        let mut file = std::fs::File::create(fp).unwrap();
        file.write_all(req.as_slice()).unwrap();
        
        Ok(fp.to_string())
    }
    
    fn parse_for_file(info: Vec<AssetEntry>) -> Result<String, String> {
        let os = match std::env::consts::OS {
            "windows" => "-windows",
            "macos" => "-darwin",
            "linux" => "-linux",
            _ => return Err(String::from("Invalid OS detected.")),
        };

        let arch = match std::env::consts::ARCH {
            "x86" => "-32",
            "x86_64" => if os == "-windows" { "-64" } else { "-amd64" },
            "aarch64" => "-arm64",
            "arm" => "-armv7",
            _ => return Err(String::from("Invalid AARCH detected.")),
        };

        let mut url = String::new();
        for item in info {
            let name = item.get_name();
            if name.contains(os) && name.contains(arch) && !name.contains("xgo") {
                url = item.get_browser_url();
            }
        }
        
        if url.is_empty() {
            return Err(String::from("No file detected."))
        }
        
        Ok(url)
    }
    
    fn get_latest_info() -> Result<Vec<AssetEntry>, String> {
        let mut header_map = HeaderMap::new();
        header_map.insert("host", HeaderValue::from_str("api.github.com").unwrap());
        header_map.insert("user-agent", HeaderValue::from_str("rust_tls_client").unwrap());
        
        let client = reqwest::blocking::Client::new();
        
        let request = client.get("https://api.github.com/repos/bogdanfinn/tls-client/releases/latest")
            .headers(header_map)
            .send()
            .map_err(|_| String::from("Failed to retrieve latest release."))
            .unwrap();
        
        let build: Value = request
            .json()
            .map_err(|_| String::from("Failed to convert response to `Value`."))
            .unwrap();
        
        let assets = match build.get("assets") {
            Some(val) => val.to_owned(),
            None => return Err(String::from("Failed to extract asset data."))
        };
        
        let object: Vec<AssetEntry> = serde_json::from_value(assets)
            .map_err(|_| String::from("Failed to deserialize."))
            .unwrap();
        
        Ok(object)
    }
    
    pub fn path(&self) -> String {
        self.0.clone()
    }
}


#[derive(Deserialize)]
struct AssetEntry {
    name: String,
    browser_download_url: String,
}
impl AssetEntry {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    
    pub fn get_browser_url(&self) -> String {
        self.browser_download_url.clone()
    }
}


// 
// mod tests {
//     use super::*;
// 
//     #[test]
//     fn test_get_latest() {
//         let tcbd = TlsClientBinaryDownloader::new();
//         
//     }
//     
//     
// 
// 
// }
