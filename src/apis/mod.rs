use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod access_token_api;
pub mod account_membership_api;
pub mod account_type_api;
pub mod asset_api;
pub mod asset_public_signature_api;
pub mod audit_log_api;
pub mod build_api;
pub mod build_hook_api;
pub mod build_log_msg_api;
pub mod deploy_api;
pub mod deploy_key_api;
pub mod deployed_branch_api;
pub mod dns_zone_api;
pub mod file_api;
pub mod form_api;
pub mod function_api;
pub mod hook_api;
pub mod hook_type_api;
pub mod member_api;
pub mod metadata_api;
pub mod payment_method_api;
pub mod service_api;
pub mod service_instance_api;
pub mod site_api;
pub mod sni_certificate_api;
pub mod snippet_api;
pub mod split_test_api;
pub mod submission_api;
pub mod ticket_api;
pub mod user_api;
pub mod x_internal_api;

pub mod configuration;
