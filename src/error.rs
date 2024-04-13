use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum TlsClientError {
    GeneralError(String),
}


impl Display for TlsClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self { 
            TlsClientError::GeneralError(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for TlsClientError {}





