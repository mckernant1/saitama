use std::thread::sleep;
use chrono::{Duration, Utc};
use crossbeam::channel::{Receiver, Sender};
use log::info;
use crate::Punch;


pub struct Orchestrator {
    work_send: Sender<bool>,
    feedback_recv: Receiver<bool>,
    delay: Duration,
    thread_count: u16,
    duration: Duration
}

impl Orchestrator {
    pub fn new(
        work_send: Sender<bool>,
        feedback_recv: Receiver<bool>,
        punch: Punch
    ) -> Orchestrator {
        let delay = Duration::nanoseconds(((1_f64 / punch.rps as f64) * 1e9) as i64);
        Orchestrator {
            work_send,
            feedback_recv,
            delay,
            thread_count: punch.thread_count,
            duration: punch.get_duration()
        }
    }


    pub fn start(&self) {
        let start = Utc::now();
        info!("Computed delay to be {:?}", self.delay);
        while Utc::now() < start + self.duration {
            sleep(self.delay.to_std().unwrap());
            self.work_send.send(true).expect("Could not send work to worker");
        }

        for _ in 0..self.thread_count {
            self.work_send
                .send(false)
                .expect("Failed to send shutoff signal to workers");
        }
    }

}


