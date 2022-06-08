use crate::model::http_config::HttpLoadConfig;
use crate::orchestrator::orchestrator::Orchestrator;
use chrono::{Duration, Utc};
use crossbeam::channel::{Receiver, Sender};
use log::{debug, info};
use mckernant1_tools::crossbeam::collect::Collectable;
use std::thread::sleep;
use std::time;

pub struct ConstantHttpOrchestrator;

impl ConstantHttpOrchestrator {
    fn get_sleep_between(rps: u64, elapsed_time: Duration, hits: i64) -> Duration {
        let interval = Duration::seconds(1).num_nanoseconds().unwrap() / rps as i64;
        let delta = Duration::nanoseconds((hits + 1) * interval);
        return delta - elapsed_time;
    }
}

impl Orchestrator<HttpLoadConfig> for ConstantHttpOrchestrator {
    fn start(work_send: Sender<bool>, feedback_recv: Receiver<bool>, http_config: HttpLoadConfig) {
        let start = Utc::now();
        info!("Starting at {}", start);
        let mut recv_counter = 0_i64;
        let feedback_recv = feedback_recv.clone();

        while Utc::now() < start + http_config.duration {
            recv_counter += feedback_recv.count_until_empty();
            sleep(
                ConstantHttpOrchestrator::get_sleep_between(
                    http_config.rps,
                    Utc::now() - start,
                    recv_counter,
                )
                .to_std()
                .unwrap_or(time::Duration::from_nanos(0)),
            );
            work_send.send(true).expect("Could not send work to worker");
        }

        debug!("Sending stop signal to threads");
        for _ in 0..http_config.thread_count {
            work_send
                .send(false)
                .expect("Failed to send shutoff signal to workers");
        }
    }
}
