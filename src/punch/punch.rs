use crate::punch::request_record::RequestRecord;
use crate::Punch;
use crossbeam::channel;
use log::info;
use std::thread;
use std::thread::{JoinHandle};
use crate::punch::orchestrator::{Orchestrator};
use crate::punch::output_handler::handle_output;
use crate::punch::worker::Worker;


pub fn punch(punch: Punch) {
    let (work_send, work_recv) = channel::bounded::<bool>(50);
    let (output_send, output_recv) = channel::bounded::<Option<RequestRecord>>(50);

    let (feedback_send, feedback_recv) = channel::bounded(50);

    info!("Starting {} worker threads", punch.thread_count);

    let output_thread = thread::Builder::new()
        .name("output-thread".to_string())
        .spawn(|| handle_output(output_recv))
        .expect("Unable to start output thread");

    let worker_threads = (0..punch.thread_count)
        .map(|i| {
            let work_recv = work_recv.clone();
            let output_send = output_send.clone();
            let feedback_send = feedback_send.clone();
            let punch = punch.clone();
            let worker = Worker::new(
                work_recv,
                output_send,
                feedback_send,
                punch
            );
            thread::Builder::new()
                .name(format!("worker-thread-{}", i))
                .spawn(move || worker.start())
        })
        .map(|j| j.expect("Worker Thread failed to launch"))
        .collect::<Vec<JoinHandle<()>>>();

    info!("Starting orchestrator thread");
    let orchestrator = Orchestrator::new(
        work_send,
        feedback_recv,
        punch
    );
    let orchestrator_thread = thread::Builder::new()
        .name("orchestrator-thread".to_string())
        .spawn(move || orchestrator.start())
        .expect("Orchestrator Thread failed to launch");

    for x in worker_threads {
        x.join().expect("Could not join worker thread")
    }
    orchestrator_thread
        .join()
        .expect("Could not join orchestrator thread");
    output_send.send(None).unwrap();
    output_thread.join().unwrap();
}
