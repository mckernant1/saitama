use chrono::Duration;

#[derive(Debug, Clone)]
pub struct HttpLoadConfig {
    /// How many worker threads to start
    pub thread_count: u16,

    /// How many RPS to drive. This can max out on certain devices
    pub rps: u64,

    /// How long to run the test.
    pub duration: Duration,
}

impl HttpLoadConfig {
    pub fn new(thread_count: u16, rps: u64, duration: Duration) -> Self {
        Self {
            thread_count,
            rps,
            duration,
        }
    }
}
