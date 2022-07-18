use crate::{user::OIDCUser, OIDCError};
use std::{env, time::Duration};

#[derive(Clone)]
pub struct OIDCClient {
    /// Re-usable HTTP client for making requests
    http_client: ureq::Agent,
    /// The base path for OIDC requests. Should be in the format of `https://sso.example.com/auth/realms/master`.
    base_path: String,
}

impl OIDCClient {
    /// Takes the base path in the format of `https://sso.example.com/auth/realms/master` and returns a re-usable client
    #[must_use]
    pub fn new(base_path: &str) -> Self {
        OIDCClient {
            http_client: ureq::AgentBuilder::new()
                .timeout_connect(Duration::from_secs(5))
                .timeout_read(Duration::from_secs(5))
                .user_agent(&format!("oidc-rs/{}", env!("CARGO_PKG_VERSION")))
                .https_only(true)
                .build(),
            base_path: base_path.to_owned(),
        }
    }

    pub fn validate_token(&self, token: &str) -> Result<OIDCUser, OIDCError> {
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
            .set("Authorization", &formatted_token)
            .call();

        match res {
            Ok(response) => {
                if response.status() >= 200 && response.status() <= 300 {
                    match response.into_json::<serde_json::Value>() {
                        Ok(value) => match serde_json::from_value::<OIDCUser>(value.clone()) {
                            Ok(user) => Ok(OIDCUser {
                                base: value,
                                ..user
                            }),
                            Err(_e) => Err(OIDCError::Unknown),
                        },
                        // TODO: need a better error here... but ureq and reqwest do different types
                        Err(_e) => Err(OIDCError::Unknown),
                    }
                } else if response.status() == 401 || response.status() == 403 {
                    Err(OIDCError::Unauthorized)
                } else {
                    Err(OIDCError::Unknown)
                }
            }
            Err(e) => Err(OIDCError::UreqError(Box::new(e))),
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
