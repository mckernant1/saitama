use std::thread;
use std::thread::{JoinHandle, sleep};
use std::time::Instant;
use chrono::Duration;
use crossbeam::channel;
use crossbeam::channel::{Receiver, Sender};
use log::info;
use reqwest::blocking;
use crate::args::ChronoUnit;
use crate::Punch;
use crate::punch::request_record::RequestRecord;

pub fn punch(punch: Punch) {
    let (work_send, work_recv) = channel::bounded::<bool>(50);
    let (output_send, output_recv) = channel::bounded::<RequestRecord>(50);

    info!("Starting {} worker threads", punch.thread_count);

    let worker_threads = (0..punch.thread_count)
        .map(|i| {
            let work_recv = work_recv.clone();
            let output_send = output_send.clone();
            thread::Builder::new()
                .name(format!("worker-thread-{}", i))
                .spawn(move || worker(work_recv, output_send))
        })
        .map(|j| j.expect("Worker Thread failed to launch"))
        .collect::<Vec<JoinHandle<()>>>();

    info!("Starting orchestrator thread");

    let orchestrator_thread = thread::Builder::new()
        .name("orchestrator-thread".to_string())
        .spawn(|| orchestrator(work_send, punch))
        .expect("Orchestrator Thread failed to launch");

    for x in worker_threads {
        x.join().expect("Could not join worker thread")
    }
    orchestrator_thread.join().expect("Could not join orchestrator thread")
}


fn worker(
    work_recv: Receiver<bool>,
    output_send: Sender<RequestRecord>,
) {
    while work_recv.recv().expect("Could not receive from channel") {
        info!("Worker {} Workin!", thread::current().name().unwrap())
    }
}


fn orchestrator(
    work_send: Sender<bool>,
    punch: Punch,
) {
    let start = Instant::now();

    let duration = match punch.duration_unit {
        ChronoUnit::Second => Duration::seconds(punch.duration as i64),
        ChronoUnit::Minute => Duration::minutes(punch.duration as i64),
        ChronoUnit::Hour => Duration::hours(punch.duration as i64)
    }.to_std().expect("Could not convert to std duration");

    let delay = Duration::nanoseconds(((1_f64 / punch.rps as f64) * 1e9) as i64)
        .to_std()
        .expect("Could not get ");
    info!("Computed delay to be {:?}", delay);
    while Instant::now() < start + duration {
        sleep(delay);
        work_send.send(true).expect("Could not send work to worker");
    }

    for _ in 0..punch.thread_count {
        work_send.send(false).expect("Failed to send shutoff signal to workers");
    }
}


fn channel_monitor() {

}
