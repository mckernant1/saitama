use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::fmt::{Display, Formatter};

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestRecord {
    #[serde_as(as = "serde_with::DurationNanoSeconds<i64>")]
    pub latency: Duration,
    pub response_code: u16,
    pub response_body: String,
    #[serde(with = "ts_milliseconds")]
    pub timestamp: DateTime<Utc>,
}

impl Display for RequestRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).expect("Could not convert json")
        )
    }
}

impl RequestRecord {
    pub fn new(latency: Duration, response_code: u16, response_body: String) -> RequestRecord {
        RequestRecord {
            latency,
            response_code,
            response_body,
            timestamp: Utc::now(),
        }
    }
}
