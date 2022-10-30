use crate::impls::constant_rate_orchestrator::{
    ConstantRateOrchestrator, ConstantRateOrchestratorConfig,
};
use crate::traits::orchestrator::Orchestrator;
use crate::traits::output::OutputHandler;
use crate::traits::worker::Worker;
use crossbeam::channel;
use log::info;
use std::fmt::Display;
use std::thread;
use std::thread::JoinHandle;

pub fn run_constant_rate_orchestrator_load_test<C, R, W, P>(config: C)
where
    C: ConstantRateOrchestratorConfig + Send + 'static,
    R: Display + Send + 'static,
    W: Worker<C, R>,
    P: OutputHandler<C, R>,
{
    info!("Starting output thread");

    let (work_send, work_recv) = channel::bounded::<bool>(1);
    let (output_send, output_recv) = channel::unbounded::<Option<R>>();
    let (feedback_send, feedback_recv) = channel::unbounded();

    let config_clone = config.clone();

    let output_thread = thread::Builder::new()
        .name("output-thread".to_string())
        .spawn(move || P::handle_output(output_recv, config_clone))
        .expect("Unable to start output thread");

    info!("Starting worker threads");

    let worker_threads = (0..config.get_thread_count())
        .map(|i| {
            let work_recv = work_recv.clone();
            let output_send = output_send.clone();
            let feedback_send = feedback_send.clone();
            let config = config.clone();

            thread::Builder::new()
                .name(format!("worker-thread-{}", i))
                .spawn(move || W::start(work_recv, output_send, feedback_send, config))
        })
        .map(|j| j.expect("Worker Thread failed to launch"))
        .collect::<Vec<JoinHandle<()>>>();

    info!("Starting orchestrator thread");
    let orchestrator_thread = thread::Builder::new()
        .name("orchestrator-thread".to_string())
        .spawn(move || ConstantRateOrchestrator::start(work_send, feedback_recv, config.clone()))
        .expect("Orchestrator Thread failed to launch");

    for x in worker_threads {
        x.join().expect("Could not join worker thread")
    }

    output_send.send(None).unwrap();
    orchestrator_thread
        .join()
        .expect("Could not join orchestrator thread");
    output_thread.join().unwrap();
}
