use chrono::Duration;




#[derive(Debug, Clone)]
pub struct HttpLoadConfig {
    /// How many worker threads to start
    pub thread_count: u16,

    /// How many RPS to drive. This can max out on certain devices
    pub rps: u64,

    /// How long to run the test. Used with duration_unit
    duration: u64,

    /// What unit to run with the test
    duration_unit: ChronoUnit,
}

impl HttpLoadConfig {
    pub fn new(thread_count: u16, rps: u64, duration: u64, duration_unit: ChronoUnit) -> Self {
        Self {
            thread_count,
            rps,
            duration,
            duration_unit,
        }
    }

    pub fn get_chrono_duration(&self) -> Duration {
        match self.duration_unit {
            ChronoUnit::Second => Duration::seconds(self.duration as i64),
            ChronoUnit::Minute => Duration::minutes(self.duration as i64),
            ChronoUnit::Hour => Duration::hours(self.duration as i64),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ChronoUnit {
    Second,
    Minute,
    Hour,
}
