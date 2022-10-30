use crate::impls::request_record::RequestRecord;
use crate::traits::config::Config;
use crate::traits::worker::Worker;
use chrono::Utc;
use crossbeam::channel::{Receiver, Sender};
use reqwest::blocking::{Client, Request};

pub trait HttpWorkerConfig: Config {
    fn create_request(&self) -> Request;
}

pub struct HttpWorker;

impl<C> Worker<C, RequestRecord> for HttpWorker
where
    C: HttpWorkerConfig,
{
    fn start(
        work_recv: Receiver<bool>,
        output_send: Sender<Option<RequestRecord>>,
        feedback_send: Sender<bool>,
        config: C,
    ) {
        let c = Client::new();
        let request = config.create_request();

        while work_recv.recv().expect("Could not receive from channel") {
            let start = Utc::now();
            let b = c
                .execute(request.try_clone().expect("Could not clone request"))
                .expect("Http Request failure");
            let latency = Utc::now() - start;
            let rr = RequestRecord::new(latency, b.status().as_u16(), b.text().unwrap());
            output_send.send(Some(rr)).expect("Failed to send output");
            feedback_send.send(true).expect("Failed to send feedback");
        }
    }
}
