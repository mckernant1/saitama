use chrono::{DateTime, Duration, Utc};

#[derive(Clone)]
pub struct RequestRecord {
    pub latency: Duration,
    pub response_code: u16,
    pub response_body: String,
    pub timestamp: DateTime<Utc>,
}

impl From<RequestRecord> for String {
    fn from(r: RequestRecord) -> Self {
        format!(
            "timestamp='{:?}',latency='{:?}',status='{}',body='{}'",
            r.timestamp.timestamp(),
            r.latency.num_nanoseconds(),
            r.response_code,
            r.response_body
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
