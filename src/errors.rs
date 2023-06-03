use core::fmt;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BDError {
    ConnectionError(String),
    ConfigError(String),
}

impl fmt::Display for BDError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
