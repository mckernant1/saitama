use std::str::FromStr;
use chrono::Duration;
use log::error;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

#[derive(Debug, Clone)]
pub struct HttpRequestConfig {
    /// Target URL
    pub url: String,

    /// Headers on the request
    headers: Vec<String>,

    /// HTTP method
    method: Method,

    /// How many worker threads to start
    pub thread_count: u16,

    /// How many RPS to drive. This can max out on certain devices
    pub rps: u64,

    /// How long to run the test. Used with duration_unit
    duration: u64,

    /// What unit to run with the test
    duration_unit: ChronoUnit,

    /// Body of the request
    pub body: String,
}


impl HttpRequestConfig {
    pub fn get_duration(&self) -> Duration {
        match self.duration_unit {
            ChronoUnit::Second => Duration::seconds(self.duration as i64),
            ChronoUnit::Minute => Duration::minutes(self.duration as i64),
            ChronoUnit::Hour => Duration::hours(self.duration as i64),
        }
    }

    pub fn get_method(&self) -> reqwest::Method {
        match self.method {
            Method::Options => reqwest::Method::OPTIONS,
            Method::Get => reqwest::Method::GET,
            Method::Post => reqwest::Method::POST,
            Method::Put => reqwest::Method::PUT,
            Method::Delete => reqwest::Method::DELETE,
            Method::Head => reqwest::Method::HEAD,
            Method::Trace => reqwest::Method::TRACE,
            Method::Connect => reqwest::Method::CONNECT,
            Method::Patch => reqwest::Method::PATCH,
        }
    }

    pub fn get_header_map<'a>(&self) -> HeaderMap {
        let mut hm = HeaderMap::new();

        for header in &self.headers {
            let header = header
                .chars()
                .filter(|it| !it.is_whitespace())
                .collect::<String>();

            let split = header
                .split(":")
                .map(|it| it.to_string())
                .collect::<Vec<String>>();

            if split.len() != 2 {
                error!("There were more then 2 parts of this header {}", header)
            }
            let key = HeaderName::from_str(split.get(0).unwrap()).expect("Invalid Header name");
            let value = HeaderValue::from_str(split.get(1).unwrap()).expect("Invalid Header value");

            hm.insert(key, value);
        }

        return hm;
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ChronoUnit {
    Second,
    Minute,
    Hour,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Method {
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
}
