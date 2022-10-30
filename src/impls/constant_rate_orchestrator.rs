use crate::traits::config::Config;
use crate::traits::orchestrator::Orchestrator;
use chrono::{Duration, Utc};
use crossbeam::channel::{Receiver, Sender};
use log::{debug, info};
use mckernant1_tools::crossbeam::collect::Collectable;
use std::thread::sleep;
use std::time;

pub trait ConstantRateOrchestratorConfig: Config {
    fn get_duration(&self) -> Duration;

    fn get_rps(&self) -> u64;

    fn get_thread_count(&self) -> u16;
}

pub struct ConstantRateOrchestrator;

impl ConstantRateOrchestrator {
    fn get_sleep_between(rps: u64, elapsed_time: Duration, hits: i64) -> Duration {
        let interval = Duration::seconds(1).num_nanoseconds().unwrap() / rps as i64;
        let delta = Duration::nanoseconds((hits + 1) * interval);
        return delta - elapsed_time;
    }
}

impl<C> Orchestrator<C> for ConstantRateOrchestrator
where
    C: ConstantRateOrchestratorConfig,
{
    fn start(work_send: Sender<bool>, feedback_recv: Receiver<bool>, config: C) {
        let start = Utc::now();
        info!("Starting at {}", start);
        let mut recv_counter = 0_i64;
        let feedback_recv = feedback_recv.clone();

        while Utc::now() < start + config.get_duration() {
            recv_counter += feedback_recv.count_until_empty();
            sleep(
                ConstantRateOrchestrator::get_sleep_between(
                    config.get_rps(),
                    Utc::now() - start,
                    recv_counter,
                )
                .to_std()
                .unwrap_or(time::Duration::from_nanos(0)),
            );
            work_send.send(true).expect("Could not send work to worker");
        }

        debug!("Sending stop signal to threads");
        for _ in 0..config.get_thread_count() {
            work_send
                .send(false)
                .expect("Failed to send shutoff signal to workers");
        }
    }
}
