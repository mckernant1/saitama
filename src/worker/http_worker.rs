use crate::model::request_record::RequestRecord;
use crate::worker::worker::Worker;
use chrono::Utc;
use crossbeam::channel::{Receiver, Sender};
use reqwest::blocking::Client;
use reqwest::Url;
use std::str::FromStr;
use crate::model::http_config::HttpRequestConfig;

pub struct HttpWorker {
    work_recv: Receiver<bool>,
    output_send: Sender<Option<RequestRecord>>,
    feedback_send: Sender<bool>,
    http_config: HttpRequestConfig,
}

impl Worker<HttpRequestConfig, RequestRecord> for HttpWorker {
    fn new(
        work_recv: Receiver<bool>,
        output_send: Sender<Option<RequestRecord>>,
        feedback_send: Sender<bool>,
        http_config: HttpRequestConfig,
    ) -> HttpWorker {
        HttpWorker {
            work_recv,
            output_send,
            feedback_send,
            http_config,
        }
    }

    fn start(&self) {
        let c = Client::new();
        let url = Url::from_str(self.http_config.url.as_str()).expect("Could not parse Url");
        let r = c
            .request(self.http_config.get_method(), url)
            .body(self.http_config.body.clone())
            .headers(self.http_config.get_header_map())
            .build()
            .expect("Could not create request");

        while self
            .work_recv
            .recv()
            .expect("Could not receive from channel")
        {
            let start = Utc::now();
            let b = c
                .execute(r.try_clone().expect("Could not clone request"))
                .expect("Http Request failure");
            let latency = Utc::now() - start;
            let rr = RequestRecord::new(latency, b.status().as_u16(), b.text().unwrap());
            self.output_send
                .send(Some(rr))
                .expect("Failed to send output");
            self.feedback_send
                .send(true)
                .expect("Failed to send feedback");
        }
    }
}
