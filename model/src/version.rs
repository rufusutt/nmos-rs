use std::{
    fmt::{self, Display},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct APIVersion {
    pub major: u8,
    pub minor: u8,
}

impl FromStr for APIVersion {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut versions = s.trim_matches('v').split('.');

        // TODO: Proper error handling
        assert!(versions.clone().count() == 2);

        let major = match versions.next() {
            Some(v) => v.parse::<u8>()?,
            None => panic!("Missing major version"),
        };

        let minor = match versions.next() {
            Some(v) => v.parse::<u8>()?,
            None => panic!("Missing major version"),
        };

        assert!(versions.next().is_none());

        Ok(Self { major, minor })
    }
}

impl Display for APIVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v{}.{}", self.major, self.minor)
    }
}

pub mod is_04 {
    use super::APIVersion;

    pub const V1_0: APIVersion = APIVersion { major: 1, minor: 0 };
}
