use std::ffi::{CStr, CString};
use std::io::Write;
use std::os::raw::c_char;
use lazy_static::lazy_static;
use libloading::{Library, Symbol};
use crate::error::TlsClientError;
use crate::{RequestResponse};
use crate::request::{RequestPayload};

const GITHUB_API_URL: &'static str = "https://api.github.com/repos/bogdanfinn/tls-client/releases/latest";

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
const BINARY_FILE: &'static [u8] = include_bytes!("../bin/tls-client-windows-64-v1.7.2.dll");

#[cfg(all(target_os = "windows", target_arch = "x86"))]
const BINARY_FILE: &'static [u8] = include_bytes!("../bin/tls-client-windows-32-v1.7.2.dll");

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
const BINARY_FILE: &'static [u8] = include_bytes!("../bin/tls-client-darwin-amd64-v1.7.2.dylib");

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
const BINARY_FILE: &'static [u8] = include_bytes!("../bin/tls-client-darwin-arm64-v1.7.2.dylib");

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
const BINARY_FILE: &'static [u8] = include_bytes!("../bin/tls-client-linux-arm64-v1.7.2.so");

#[cfg(all(target_os = "linux", target_arch = "arm"))]
const BINARY_FILE: &'static [u8] = include_bytes!("../bin/tls-client-linux-armv7-v1.7.2.so");

#[cfg(all(target_os = "linux", target_arch = "x86_64", feature = "alpine"))]
const BINARY_FILE: &'static [u8] = include_bytes!("../bin/tls-client-linux-alpine-amd64-v1.7.2.so");

#[cfg(all(target_os = "linux", target_arch = "x86_64", feature = "ubuntu"))]
const BINARY_FILE: &'static [u8] = include_bytes!("../bin/tls-client-linux-ubuntu-amd64-v1.7.2.so");


lazy_static! {
    pub static ref SHARED_METHODS: TlsClientSharedMethods = TlsClientSharedMethods::new_default().unwrap();
    // Threadsafe
    // static ref TS_SHARED_METHODS: Mutex<TlsClientSharedMethods> = Mutex::new(TlsClientSharedMethods::new_default().unwrap());
}



#[derive(Debug)]
pub struct TlsClientSharedMethods {
    lib: Library,
    f: TlsClientBinaryFile,
}

impl TlsClientSharedMethods {
    pub fn new_default() -> Result<Self, TlsClientError> {
        unsafe {
            let bf = TlsClientBinaryFile::new();
            Library::new(bf.path())
                .map(|lib| Self { lib, f: bf })
                .map_err(|_| TlsClientError::GeneralError("Failed to build TLS Client".to_string()))
        }
    }

    pub fn new(dll_path: &str) -> Result<Self, TlsClientError> {
        unsafe {
            Library::new(dll_path)
                .map(|lib| Self { lib, f: TlsClientBinaryFile::from(dll_path) })
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

#[derive(Debug)]
struct TlsClientBinaryFile(pub String);

impl TlsClientBinaryFile {
    pub fn new() -> Self {
        let filename = "tls_client_binary".to_owned() + match std::env::consts::OS {
            "windows" => ".dll",
            "macos" => ".dylib",
            "linux" => ".so",
            _ => panic!("Unsupported OS")
        };
        
        let mut dir = std::env::temp_dir();
        
        dir.push("rust_tls_client");
        let _ = std::fs::create_dir_all(&dir);
        
        dir.push(filename);
        let fp = &dir.display().to_string();
        
        if !dir.exists() {
            let mut file = std::fs::File::create(fp).unwrap();
            file.write_all(BINARY_FILE).unwrap();
        }
        
        Self(fp.to_owned())
    }
    
    pub fn from(v: &str) -> Self {
        Self(v.to_string())
    }
    
    pub fn path(&self) -> &String {
        &self.0
    }
}


mod tests {
    use super::*;

    // #[test]
    // fn test_bin_path() {
    //     let bin_name = BinPath::build_binary_name();
    //     println!("{}", bin_name);
    //     let file_names = BinPath::get_file_names();
    //     assert_eq!(file_names, vec!["win64.dll".to_string()]);
    // }


}
