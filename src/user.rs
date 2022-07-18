use itertools::Itertools;
use serde::{Deserialize, Serialize};

/// A very basic struct of information we can get from OIDC
#[derive(Debug, Serialize, Deserialize)]
pub struct OIDCUser {
    /// The real name of a user
    pub name: Option<String>,
    /// The username of a user
    pub preferred_username: String,
    /// Any groups the user is in
    pub groups: Box<[String]>,
    // TODO: A variable list of any other attributes we're given
}

impl OIDCUser {
    #[must_use]
    pub fn has_group(&self, group_name: &str) -> bool {
        self.groups.iter().contains(&group_name.to_owned())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn user_has_group() {
        let user = super::OIDCUser {
            name: Some(String::from("Testy McTestyFace")),
            preferred_username: String::from("test"),
            groups: Box::new([String::from("member")]),
        };
        assert_eq!(user.has_group("member"), true);
    }

    #[test]
    fn user_missing_group() {
        let user = super::OIDCUser {
            name: Some(String::from("Testy McTestyFace")),
            preferred_username: String::from("test"),
            groups: Box::new([]),
        };
        assert_eq!(user.has_group("missing"), false);
    }
}
