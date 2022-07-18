cfg_if::cfg_if! {
    if #[cfg(all(feature = "ureq", feature = "reqwest"))] {
        compile_error!("Features \"ureq\" and \"reqwest\" are incompatible. Please select only one");
    } else if #[cfg(not(any(feature = "ureq", feature = "reqwest")))] {
        compile_error!("An HTTP backend must be specified (one of \"ureq\", \"reqwest\")");
    }
}

#[forbid(unsafe_code)]
mod client;
pub use client::OIDCClient;

pub mod user;

use std::error::Error;

#[derive(Debug)]
pub enum OIDCError {
    Unauthorized,
    #[cfg(feature = "ureq")]
    UreqError(Box<ureq::Error>),
    #[cfg(feature = "reqwest")]
    ReqwestError(Box<reqwest::Error>),
    Unknown,
}

impl Error for OIDCError {}

impl std::fmt::Display for OIDCError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OIDCError::Unauthorized => write!(f, "OIDC returned Unauthorized"),
            #[cfg(feature = "ureq")]
            OIDCError::UreqError(ue) => write!(f, "Ureq Error: {ue}"),
            #[cfg(feature = "reqwest")]
            OIDCError::ReqwestError(re) => write!(f, "Reqwest Error: {re}"),
            &OIDCError::Unknown => write!(f, "Unknown OIDC Error"),
        }
    }
}
