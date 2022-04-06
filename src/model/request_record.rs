use chrono::{DateTime, Duration, Utc};

#[derive(Clone)]
pub struct RequestRecord {
    pub latency: Duration,
    pub response_code: u16,
    pub response_body: String,
    pub timestamp: DateTime<Utc>,
}

impl Into<String> for RequestRecord {
    fn into(self) -> String {
        format!(
            "timestamp='{:?}',latency='{:?}',status='{}',body='{}'",
            self.timestamp.timestamp(),
            self.latency.num_nanoseconds(),
            self.response_code,
            self.response_body
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
