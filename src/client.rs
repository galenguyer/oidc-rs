#[cfg(feature = "reqwest")]
mod reqwest;
#[cfg(feature = "reqwest")]
pub use self::reqwest::OIDCClient;

#[cfg(feature = "ureq")]
mod ureq;
#[cfg(feature = "ureq")]
pub use self::ureq::OIDCClient;
