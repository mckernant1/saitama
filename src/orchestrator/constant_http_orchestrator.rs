use crate::orchestrator::orchestrator::Orchestrator;
use crate::util::channel::CountReceiveChannel;
use chrono::{Duration, Utc};
use crossbeam::channel::{Receiver, Sender};
use log::{debug, info};
use std::thread::sleep;
use std::time;
use crate::model::http_config::HttpRequestConfig;

pub struct ConstantHttpOrchestrator {
    work_send: Sender<bool>,
    feedback_recv: Receiver<bool>,
    thread_count: u16,
    duration: Duration,
    rps: u64,
}

impl ConstantHttpOrchestrator {
    fn get_sleep_between(&self, elapsed_time: Duration, hits: i64) -> Duration {
        let interval = Duration::seconds(1).num_nanoseconds().unwrap() / self.rps as i64;
        let delta = Duration::nanoseconds((hits + 1) * interval);
        return delta - elapsed_time;
    }
}

impl Orchestrator<HttpRequestConfig> for ConstantHttpOrchestrator {
    fn new(
        work_send: Sender<bool>,
        feedback_recv: Receiver<bool>,
        http_config: HttpRequestConfig,
    ) -> ConstantHttpOrchestrator {
        ConstantHttpOrchestrator {
            work_send,
            feedback_recv,
            thread_count: http_config.thread_count,
            duration: http_config.get_duration(),
            rps: http_config.rps,
        }
    }

    fn start(&self) {
        let start = Utc::now();
        info!("Starting at {}", start);
        let mut recv_counter = 0_i64;
        let feedback_recv = self.feedback_recv.clone();

        while Utc::now() < start + self.duration {
            recv_counter += feedback_recv.count_recvs_until_empty();
            sleep(
                self.get_sleep_between(Utc::now() - start, recv_counter)
                    .to_std()
                    .unwrap_or(time::Duration::from_nanos(0)),
            );
            self.work_send
                .send(true)
                .expect("Could not send work to worker");
        }

        debug!("Sending stop signal to threads");
        for _ in 0..self.thread_count {
            self.work_send
                .send(false)
                .expect("Failed to send shutoff signal to workers");
        }
    }
}
