use chrono::{DateTime, Duration, Utc};

#[derive(Clone)]
pub struct RequestRecord {
    pub latency: Duration,
    pub response_code: u16,
    pub response_body: String,
    pub timestamp: DateTime<Utc>,
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

    pub fn to_csv_string(&self) -> String {
        format!(
            "timestamp='{:?}',latency='{:?}',status='{}',body='{}'",
            self.timestamp.timestamp(),
            self.latency.num_nanoseconds(),
            self.response_code,
            self.response_body
        )
    }
}
