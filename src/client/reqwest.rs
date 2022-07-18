use crate::{user::OIDCUser, OIDCError};
use std::{env, time::Duration};

#[derive(Clone)]
pub struct OIDCClient {
    /// Re-usable HTTP client for making requests
    http_client: reqwest::Client,
    /// The base path for OIDC requests. Should be in the format of `https://sso.example.com/auth/realms/master`.
    base_path: String,
}

impl OIDCClient {
    /// Takes the base path in the format of `https://sso.example.com/auth/realms/master` and returns a re-usable client
    #[must_use]
    pub fn new(base_path: &str) -> Self {
        OIDCClient {
            http_client: reqwest::ClientBuilder::new()
                .user_agent(&format!("oidc-rs/{}", env!("CARGO_PKG_VERSION")))
                .timeout(Duration::from_secs(5))
                .https_only(true)
                .build()
                .expect("Building reqwest client failed"),
            base_path: base_path.to_owned(),
        }
    }

    pub async fn validate_token(&self, token: &str) -> Result<OIDCUser, OIDCError> {
        let formatted_token = if token.starts_with("Bearer") {
            token.to_string()
        } else {
            format!("Bearer {token}")
        };

        let res = self
            .http_client
            .get(&format!(
                "{}/protocol/openid-connect/userinfo",
                self.base_path
            ))
            .header("Authorization", &formatted_token)
            .send()
            .await;

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<OIDCUser>().await {
                        Ok(user) => Ok(user),
                        Err(e) => Err(OIDCError::ReqwestError(Box::new(e))),
                    }
                } else if response.status().is_client_error() {
                    Err(OIDCError::Unauthorized)
                } else {
                    Err(OIDCError::Unknown)
                }
            }
            Err(e) => Err(OIDCError::ReqwestError(Box::new(e))),
        }
    }
}

impl Default for OIDCClient {
    fn default() -> Self {
        Self::new(&env::var("OIDC_BASE_PATH").expect("OIDC_BASE_PATH is not set"))
    }
}

#[cfg(test)]
mod tests {
    use super::OIDCClient;

    #[test]
    fn test_create_client() {
        let _ = OIDCClient::new("https://sso.csh.rit.edu/auth/realms/csh");
    }
}
