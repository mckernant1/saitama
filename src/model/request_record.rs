use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestRecord {
    #[serde_as(as = "serde_with::DurationNanoSeconds<i64>")]
    pub latency: Duration,
    pub response_code: u16,
    pub response_body: String,
    #[serde(with = "ts_milliseconds")]
    pub timestamp: DateTime<Utc>,
}

impl From<RequestRecord> for String {
    fn from(r: RequestRecord) -> Self {
        serde_json::to_string(&r).expect("Could not convert json")
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
