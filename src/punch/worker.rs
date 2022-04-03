use std::str::FromStr;
use chrono::Utc;
use crossbeam::channel::{Receiver, Sender};
use reqwest::blocking::Client;
use reqwest::Url;
use crate::Punch;
use crate::punch::request_record::RequestRecord;

pub struct Worker {
    work_recv: Receiver<bool>,
    output_send: Sender<Option<RequestRecord>>,
    punch: Punch
}

impl Worker {
    pub fn new(
        work_recv: Receiver<bool>,
        output_send: Sender<Option<RequestRecord>>,
        punch: Punch
    ) -> Worker {
        Worker {
            work_recv,
            output_send,
            punch
        }
    }

    pub fn start(&self) {
        let c = Client::new();
        let url = Url::from_str(self.punch.url.as_str()).expect("Could not parse Url");
        let r = c
            .request(self.punch.get_method(), url)
            .body(self.punch.body.clone())
            .headers(self.punch.get_header_map())
            .build()
            .expect("Could not create request");

        while self.work_recv.recv().expect("Could not receive from channel") {
            let start = Utc::now();
            let b = c
                .execute(r.try_clone().expect("Could not clone request"))
                .expect("Http Request failure");
            let latency = Utc::now() - start;
            let rr = RequestRecord::new(
                latency,
                b.status().as_u16(),
                b.text().unwrap()
            );
            self.output_send.send(Some(rr)).expect("Failed to send output")
        }
    }

}


