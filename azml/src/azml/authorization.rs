//

use std::{
    convert::{self, TryInto},
    result,
};

#[derive(Clone, Debug)]
pub struct Authorization {
    // Within the same process
    pub same_process: AuthorizationSpec,
    // Across processes but within the same realm
    pub same_realm: AuthorizationSpec,
    // Across realms
    pub anywhere: AuthorizationSpec,
}

#[derive(Clone, Debug)]
pub struct AuthorizationSpec {
    pub allow: AuthorizationAllow,
}

#[derive(Clone, Debug)]
pub enum AuthorizationAllow {
    AlwaysDisallow,
    AlwaysAllow,
    OnCondition,
}

impl AuthorizationAllow {
    pub fn is_not_disallow(&self) -> bool {
        match self {
            Self::AlwaysAllow | Self::OnCondition => true,
            Self::AlwaysDisallow => false,
        }
    }
}

impl Default for AuthorizationAllow {
    fn default() -> Self {
        Self::AlwaysDisallow
    }
}

impl convert::TryFrom<String> for AuthorizationAllow {
    type Error = String;

    fn try_from(s: String) -> result::Result<Self, Self::Error> {
        let sr: &str = s.as_ref();
        sr.try_into()
    }
}

impl convert::TryFrom<&String> for AuthorizationAllow {
    type Error = String;

    fn try_from(s: &String) -> result::Result<Self, Self::Error> {
        let sr: &str = s.as_ref();
        sr.try_into()
    }
}

impl convert::TryFrom<&str> for AuthorizationAllow {
    type Error = String;

    fn try_from(s: &str) -> result::Result<Self, Self::Error> {
        match s {
            "" | "always_disallow" | "disallow" => Ok(Self::AlwaysDisallow),
            "always_allow" | "allow" => Ok(Self::AlwaysAllow),
            "on_condition" => Ok(Self::OnCondition),
            _ => Err(format!("Unrecognized AuthorizationAllow value {}", s).to_owned()),
        }
    }
}

impl From<AuthorizationAllow> for String {
    fn from(s: AuthorizationAllow) -> Self {
        (&s).into()
    }
}

impl From<&AuthorizationAllow> for String {
    fn from(s: &AuthorizationAllow) -> Self {
        match s {
            AuthorizationAllow::AlwaysDisallow => "always_disallow".to_owned(),
            AuthorizationAllow::AlwaysAllow => "always_allow".to_owned(),
            AuthorizationAllow::OnCondition => "on_condition".to_owned(),
        }
    }
}
