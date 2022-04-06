use crate::model::request_record::RequestRecord;
use crate::worker::worker::Worker;
use chrono::Utc;
use crossbeam::channel::{Receiver, Sender};
use reqwest::blocking::{Client, Request};

pub struct HttpWorker {
    work_recv: Receiver<bool>,
    output_send: Sender<Option<RequestRecord>>,
    feedback_send: Sender<bool>,
    request: Request,
}

impl Worker<Request, RequestRecord> for HttpWorker {
    fn new(
        work_recv: Receiver<bool>,
        output_send: Sender<Option<RequestRecord>>,
        feedback_send: Sender<bool>,
        request: Request,
    ) -> HttpWorker {
        HttpWorker {
            work_recv,
            output_send,
            feedback_send,
            request,
        }
    }

    fn start(&self) {
        let c = Client::new();

        while self
            .work_recv
            .recv()
            .expect("Could not receive from channel")
        {
            let start = Utc::now();
            let b = c
                .execute(self.request.try_clone().expect("Could not clone request"))
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
