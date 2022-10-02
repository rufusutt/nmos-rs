use std::{
    fmt,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub struct TaiTime {
    secs: u64,
    nanos: u32,
}

impl TaiTime {
    #[must_use]
    pub fn now() -> TaiTime {
        let tai_offset = Duration::from_secs(37);

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time before Unix epoch")
            + tai_offset;

        TaiTime {
            secs: now.as_secs(),
            nanos: now.subsec_nanos(),
        }
    }
}

impl fmt::Debug for TaiTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.secs, self.nanos)
    }
}

impl fmt::Display for TaiTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.secs, self.nanos)
    }
}
