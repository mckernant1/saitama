use crate::args::args::Punch;
use crate::model::request_record::RequestRecord;
use crate::orchestrator::constant_http_orchestrator::ConstantHttpOrchestrator;
use crate::orchestrator::orchestrator::Orchestrator;
use crate::output::output::OutputHandler;
use crate::output::term_output_handler::TermOutputHandler;
use crate::worker::http_worker::HttpWorker;
use crate::worker::worker::Worker;
use crossbeam::channel;
use log::info;
use std::thread;
use std::thread::JoinHandle;

pub fn punch(punch: Punch) {
    let (work_send, work_recv) = channel::bounded::<bool>(1);
    let (output_send, output_recv) = channel::unbounded::<Option<RequestRecord>>();
    let (feedback_send, feedback_recv) = channel::unbounded();

    info!("Starting {} worker threads", punch.thread_count);

    let output_thread = thread::Builder::new()
        .name("output-thread".to_string())
        .spawn(|| TermOutputHandler::handle_output(output_recv))
        .expect("Unable to start output thread");

    let worker_threads = (0..punch.thread_count)
        .map(|i| {
            let work_recv = work_recv.clone();
            let output_send = output_send.clone();
            let feedback_send = feedback_send.clone();
            let punch = punch.clone();
            let worker = HttpWorker::new(work_recv, output_send, feedback_send, punch);
            thread::Builder::new()
                .name(format!("worker-thread-{}", i))
                .spawn(move || worker.start())
        })
        .map(|j| j.expect("Worker Thread failed to launch"))
        .collect::<Vec<JoinHandle<()>>>();

    info!("Starting orchestrator thread");
    let orchestrator =
        ConstantHttpOrchestrator::new(work_send.clone(), feedback_recv.clone(), punch);
    let orchestrator_thread = thread::Builder::new()
        .name("orchestrator-thread".to_string())
        .spawn(move || orchestrator.start())
        .expect("Orchestrator Thread failed to launch");

    for x in worker_threads {
        x.join().expect("Could not join worker thread")
    }
    output_send.send(None).unwrap();
    orchestrator_thread
        .join()
        .expect("Could not join orchestrator thread");

    output_thread.join().unwrap();
    drop(feedback_recv);
    drop(work_send);
    drop(output_send);
}
