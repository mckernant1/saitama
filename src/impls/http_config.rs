use crate::impls::constant_rate_orchestrator::ConstantRateOrchestratorConfig;
use crate::impls::http_worker::HttpWorkerConfig;
use crate::traits::config::Config;
use chrono::Duration;
use reqwest::blocking::{Client, Request};
use reqwest::Method;

#[derive(Debug, Clone)]
pub struct HttpLoadConfig {
    /// How many worker threads to start
    pub thread_count: u16,

    /// How many RPS to drive. This can max out on certain devices
    pub rps: u64,

    /// How long to run the test.
    pub duration: Duration,
}

impl Config for HttpLoadConfig {}

impl ConstantRateOrchestratorConfig for HttpLoadConfig {
    fn get_duration(&self) -> Duration {
        self.duration
    }

    fn get_rps(&self) -> u64 {
        self.rps
    }

    fn get_thread_count(&self) -> u16 {
        self.thread_count
    }
}

impl HttpWorkerConfig for HttpLoadConfig {
    fn create_request(&self) -> Request {
        Client::new()
            .request(Method::GET, "".to_string())
            .build()
            .expect("Could not build message")
    }
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
