# oidc-rs
Hopefully simple OIDC interactions. Designed for Computer Science House but hopefully useful for other people as well.

Currently, valid HTTPS is required for any OIDC requests. Insecure connections may be added behind a feature gate.

## Configuration
The only configuration value is the base path for OIDC lookups. This can either be passed into `OidcClient::new(base_path: &str)` or provided in the environment as `OIDC_BASE_PATH`. The format of this should be akin to `https://sso.example.com/auth/realms/master`. Failure to set this will result in a panic at runtime.
