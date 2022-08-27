use std::{
    fmt::{self, Display},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct APIVersion {
    major: u8,
    minor: u8,
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

        assert!(versions.next() == None);

        Ok(Self { major, minor })
    }
}

impl Display for APIVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v{}.{}", self.major, self.minor)
    }
}
