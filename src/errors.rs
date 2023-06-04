use core::fmt;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BDError {
    ConnectionError(&'static str),
    ConfigError(&'static str),
    SystemError(&'static str),
}

impl fmt::Display for BDError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BDError::ConnectionError(msg) => write!(f, "Connection Error: {msg}"),
            BDError::ConfigError(msg) => write!(f, "Configuration Error: {msg}"),
            BDError::SystemError(msg) => write!(f, "System Error: {msg}"),
        }
    }
}
