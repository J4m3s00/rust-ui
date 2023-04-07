use std::fmt::{Display, Formatter, Result};

pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl Default for Version {
    fn default() -> Self {
        Self {
            major: 0,
            minor: 0,
            patch: 1,
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl<S> From<S> for Version
where
    S: Into<String>,
{
    fn from(version: S) -> Self {
        let version = version.into();
        let mut version = version.split('.');
        let major = version.next().unwrap().parse().unwrap();
        let minor = version.next().unwrap().parse().unwrap();
        let patch = version.next().unwrap().parse().unwrap();
        Self {
            major,
            minor,
            patch,
        }
    }
}
