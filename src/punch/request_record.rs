use std::time::Duration;

#[derive(Clone)]
pub struct RequestRecord {
    pub latency: Duration,
    pub response_code: u8,
    pub response_body: String
}


impl RequestRecord {
    pub fn new(
        latency: Duration,
        response_code: u8,
        response_body: String
    ) -> RequestRecord {

        RequestRecord {
            latency,
            response_code,
            response_body
        }
    }
}



